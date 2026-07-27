use chrono::Utc;
use serde::{Deserialize, Serialize};

/// 统一的地震数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Earthquake {
    pub id: String,
    pub magnitude: f64,
    pub depth: f64,
    pub location: String,
    pub latitude: f64,
    pub longitude: f64,
    pub time: String,
    pub source: String,
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorSettings {
    pub min_magnitude: f64,
    pub poll_interval_sec: u64,
    pub notifications_enabled: bool,
    pub sound_enabled: bool,
    pub region: String,
}

impl Default for MonitorSettings {
    fn default() -> Self {
        Self {
            min_magnitude: 2.5,
            poll_interval_sec: 60,
            notifications_enabled: true,
            sound_enabled: true,
            region: "all".to_string(),
        }
    }
}

/// USGS GeoJSON 响应结构
#[derive(Debug, Deserialize)]
struct UsgsResponse {
    features: Vec<UsgsFeature>,
}

#[derive(Debug, Deserialize)]
struct UsgsFeature {
    id: String,
    properties: UsgsProperties,
    geometry: UsgsGeometry,
}

#[derive(Debug, Deserialize)]
struct UsgsProperties {
    mag: Option<f64>,
    place: Option<String>,
    time: i64,
}

#[derive(Debug, Deserialize)]
struct UsgsGeometry {
    coordinates: Vec<f64>, // [lon, lat, depth]
}

/// CENC (中国地震台网中心) 响应结构
#[derive(Debug, Deserialize)]
struct CencResponse {
    shuju: Option<Vec<CencRecord>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct CencRecord {
    cata_id: Option<String>,
    #[serde(rename = "M")]
    m: Option<String>,
    #[serde(rename = "EPI_LAT")]
    epi_lat: Option<String>,
    #[serde(rename = "EPI_LON")]
    epi_lon: Option<String>,
    #[serde(rename = "EPI_DEPTH")]
    epi_depth: Option<String>,
    #[serde(rename = "LOCATION_C")]
    location_c: Option<String>,
    #[serde(rename = "O_TIME")]
    o_time: Option<String>,
}

/// 从所有数据源获取地震数据
pub async fn fetch_all_earthquakes() -> Vec<Earthquake> {
    let mut all = Vec::new();

    // 从 USGS 获取全球数据
    match fetch_usgs().await {
        Ok(data) => {
            log::info!("USGS: 获取到 {} 条记录", data.len());
            all.extend(data);
        }
        Err(e) => {
            log::warn!("USGS 获取失败: {}", e);
        }
    }

    // 从 CENC 获取中国区域数据
    match fetch_cenc().await {
        Ok(data) => {
            log::info!("CENC: 获取到 {} 条记录", data.len());
            all.extend(data);
        }
        Err(e) => {
            log::warn!("CENC 获取失败: {}", e);
        }
    }

    // 去重 (USGS 和 CENC 可能有重叠)
    let mut seen = std::collections::HashSet::new();
    all.retain(|eq| {
        let key = format!("{:.3}_{:.3}_{:.1}", eq.longitude, eq.latitude, eq.magnitude);
        seen.insert(key)
    });

    all
}

/// 从 USGS 获取地震数据 (全球，过去24小时)
/// API 文档: https://earthquake.usgs.gov/fdsnws/event/1/
async fn fetch_usgs() -> Result<Vec<Earthquake>, Box<dyn std::error::Error + Send + Sync>> {
    let url = "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/all_day.geojson";
    let client = reqwest::Client::builder()
        .user_agent("DDYJ-EarthquakeApp/0.1")
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let resp: UsgsResponse = client.get(url).send().await?.json().await?;

    let earthquakes = resp
        .features
        .into_iter()
        .filter_map(|f| {
            let mag = f.properties.mag?;
            let coords = &f.geometry.coordinates;
            if coords.len() < 3 {
                return None;
            }
            Some(Earthquake {
                id: f.id,
                magnitude: mag,
                depth: coords[2],
                location: f.properties.place.unwrap_or_else(|| "未知位置".to_string()),
                latitude: coords[1],
                longitude: coords[0],
                time: chrono::DateTime::from_timestamp_millis(f.properties.time)
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default(),
                source: "USGS".to_string(),
            })
        })
        .collect();

    Ok(earthquakes)
}

/// 从中国地震台网中心 (CENC) 获取地震数据
/// 数据源: https://www.ceic.ac.cn/
async fn fetch_cenc() -> Result<Vec<Earthquake>, Box<dyn std::error::Error + Send + Sync>> {
    let url = "https://www.ceic.ac.cn/ajax/speedsearch";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X)")
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let now = Utc::now();
    let start = now.format("%Y-%m-%d").to_string();
    let end = (now + chrono::Duration::days(1)).format("%Y-%m-%d").to_string();

    let params = [
        ("page", "1"),
        ("start", start.as_str()),
        ("end", end.as_str()),
        ("jingdu1", "-180"),
        ("jingdu2", "180"),
        ("weidu1", "-90"),
        ("weidu2", "90"),
        ("height1", "0"),
        ("height2", "1000"),
        ("zhenji1", "0"),
        ("zhenji2", "10"),
    ];

    let resp: CencResponse = client.post(url).form(&params).send().await?.json().await?;

    let earthquakes = resp
        .shuju
        .unwrap_or_default()
        .into_iter()
        .filter_map(|r| {
            let mag = r.m?.parse::<f64>().ok()?;
            let lat = r.epi_lat?.parse::<f64>().ok()?;
            let lon = r.epi_lon?.parse::<f64>().ok()?;
            let depth = r.epi_depth?.parse::<f64>().ok()?;
            let id = r.cata_id.unwrap_or_else(|| format!("cenc_{}_{}_{}", lat, lon, mag));
            let location = r.location_c.unwrap_or_else(|| "未知位置".to_string());
            let time = r
                .o_time
                .as_deref()
                .and_then(|t| {
                    chrono::NaiveDateTime::parse_from_str(t, "%Y-%m-%d %H:%M:%S")
                        .ok()
                        .map(|dt| dt.and_utc().to_rfc3339())
                })
                .unwrap_or_default();

            Some(Earthquake {
                id,
                magnitude: mag,
                depth,
                location,
                latitude: lat,
                longitude: lon,
                time,
                source: "CENC".to_string(),
            })
        })
        .collect();

    Ok(earthquakes)
}
