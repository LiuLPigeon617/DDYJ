use std::collections::HashSet;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tauri_plugin_notification::NotificationExt;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

use crate::earthquake::{self, Earthquake, MonitorSettings};

/// 监控状态
pub struct MonitorState {
    pub is_running: bool,
    pub known_ids: HashSet<String>,
    pub settings: MonitorSettings,
}

impl Default for MonitorState {
    fn default() -> Self {
        Self {
            is_running: false,
            known_ids: HashSet::new(),
            settings: MonitorSettings::default(),
        }
    }
}

pub type SharedMonitorState = Arc<Mutex<MonitorState>>;

/// 启动后台监控任务
pub fn start_monitor_task(app: AppHandle, state: SharedMonitorState) {
    tokio::spawn(async move {
        log::info!("地震监控任务已启动");

        loop {
            let poll_interval;
            let notifications_enabled;
            let min_magnitude;
            {
                let mut st = state.lock().await;
                if !st.is_running {
                    log::info!("监控已停止，退出任务");
                    break;
                }
                poll_interval = st.settings.poll_interval_sec;
                notifications_enabled = st.settings.notifications_enabled;
                min_magnitude = st.settings.min_magnitude;
            }

            // 获取最新地震数据
            let all_eqs = earthquake::fetch_all_earthquakes().await;
            {
                let new_eqs: Vec<&Earthquake> = {
                    let mut st = state.lock().await;
                    let mut new_list = Vec::new();
                    for eq in &all_eqs {
                        if eq.magnitude >= min_magnitude && !st.known_ids.contains(&eq.id) {
                            if !st.known_ids.is_empty() {
                                // 只在已有基线后才通知新地震（首次加载不通知）
                                new_list.push(eq);
                            }
                            st.known_ids.insert(eq.id.clone());
                        }
                    }
                    new_list
                };

                // 发送通知和事件
                for eq in new_eqs {
                    log::info!(
                        "检测到新地震: M{:.1} {} ({})",
                        eq.magnitude,
                        eq.location,
                        eq.source
                    );

                    // 向前端发送事件
                    let _ = app.emit("new-earthquake", eq.clone());

                    // 发送系统通知 (高震级)
                    if notifications_enabled && eq.magnitude >= 4.0 {
                        let title = if eq.magnitude >= 6.0 {
                            format!("🚨 强烈地震 M{:.1}", eq.magnitude)
                        } else {
                            format!("⚠️ 地震 M{:.1}", eq.magnitude)
                        };
                        let body = format!(
                            "{}\n深度: {}km\n{}",
                            eq.location, eq.depth, eq.source
                        );
                        let _ = app.notification().builder()
                            .title(&title)
                            .body(&body)
                            .show();
                    }
                }
            }

            // 等待下一次轮询
            time::sleep(Duration::from_secs(poll_interval.max(10))).await;
        }

        log::info!("地震监控任务已结束");
    });
}
