mod earthquake;
mod monitor;

use earthquake::{Earthquake, MonitorSettings};
use monitor::{start_monitor_task, SharedMonitorState, MonitorState};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

/// 获取最近地震数据
#[tauri::command]
async fn get_recent_earthquakes() -> Result<Vec<Earthquake>, String> {
    Ok(earthquake::fetch_all_earthquakes().await)
}

/// 获取当前设置
#[tauri::command]
async fn get_settings(state: State<'_, SharedMonitorState>) -> Result<MonitorSettings, String> {
    let st = state.lock().await;
    Ok(st.settings.clone())
}

/// 更新设置
#[tauri::command]
async fn update_settings(
    settings: MonitorSettings,
    state: State<'_, SharedMonitorState>,
) -> Result<(), String> {
    let mut st = state.lock().await;
    st.settings = settings;
    Ok(())
}

/// 启动监控
#[tauri::command]
async fn start_monitoring(
    app: tauri::AppHandle,
    state: State<'_, SharedMonitorState>,
) -> Result<(), String> {
    let mut st = state.lock().await;
    if st.is_running {
        return Ok(());
    }
    st.is_running = true;
    let state_clone = Arc::clone(&state.inner());
    drop(st);
    start_monitor_task(app, state_clone);
    Ok(())
}

/// 停止监控
#[tauri::command]
async fn stop_monitoring(state: State<'_, SharedMonitorState>) -> Result<(), String> {
    let mut st = state.lock().await;
    st.is_running = false;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .manage::<SharedMonitorState>(Arc::new(Mutex::new(MonitorState::default())))
        .invoke_handler(tauri::generate_handler![
            get_recent_earthquakes,
            get_settings,
            update_settings,
            start_monitoring,
            stop_monitoring,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
