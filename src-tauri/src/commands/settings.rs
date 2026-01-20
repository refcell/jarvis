use crate::models::{Settings, WatchStatus};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    state.get_settings().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_settings(state: State<'_, AppState>, settings: Settings) -> Result<(), String> {
    state.save_settings(&settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_watch_status(state: State<'_, AppState>) -> WatchStatus {
    state.get_watch_status()
}

#[tauri::command]
pub fn set_watching(state: State<'_, AppState>, enabled: bool) -> Result<(), String> {
    state.set_watching(enabled);
    Ok(())
}

#[tauri::command]
pub fn update_capture_interval(
    state: State<'_, AppState>,
    interval_secs: u64,
) -> Result<(), String> {
    let mut settings = state.get_settings().map_err(|e| e.to_string())?;
    settings.capture_interval_secs = interval_secs;
    state.save_settings(&settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_notifications(
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    let mut settings = state.get_settings().map_err(|e| e.to_string())?;
    settings.notifications_enabled = enabled;
    state.save_settings(&settings).map_err(|e| e.to_string())
}
