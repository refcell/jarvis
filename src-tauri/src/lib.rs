mod commands;
mod models;
mod providers;
mod security;
mod services;
mod state;
mod storage;

use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            // Get app data directory
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            // Initialize application state
            let state =
                AppState::new(app_data_dir).expect("Failed to initialize application state");

            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Screen capture commands
            commands::check_screen_permission,
            commands::request_screen_permission,
            commands::capture_screen,
            commands::get_recent_captures,
            // Task commands
            commands::get_active_tasks,
            commands::get_all_tasks,
            commands::get_task,
            commands::create_task,
            commands::update_task_status,
            commands::snooze_task,
            commands::dismiss_task,
            commands::complete_task,
            commands::delete_task,
            // LLM commands
            commands::detect_available_cli_tools,
            commands::analyze_context,
            commands::analyze_and_create_tasks,
            commands::health_check_llm,
            commands::get_llm_config,
            commands::update_llm_config,
            // Keychain commands
            commands::store_api_key,
            commands::get_api_key,
            commands::delete_api_key,
            commands::has_api_key,
            // Settings commands
            commands::get_settings,
            commands::save_settings,
            commands::get_watch_status,
            commands::set_watching,
            commands::update_capture_interval,
            commands::toggle_notifications,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
