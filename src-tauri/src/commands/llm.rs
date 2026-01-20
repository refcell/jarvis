use crate::models::{CaptureContext, DetectedCLITools, DetectedTask, LLMConfig};
use crate::security::KeychainManager;
use crate::services::{detect_cli_tools, LLMService};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn detect_available_cli_tools() -> DetectedCLITools {
    detect_cli_tools()
}

#[tauri::command]
pub async fn analyze_context(
    state: State<'_, AppState>,
    context: CaptureContext,
) -> Result<Vec<DetectedTask>, String> {
    let settings = state.get_settings().map_err(|e| e.to_string())?;

    // CLI providers don't need API keys
    let api_key = if settings.llm_config.provider_type.is_cli_provider() {
        None
    } else {
        KeychainManager::get_api_key(settings.llm_config.provider_type.as_str())
            .map_err(|e| e.to_string())?
    };

    let llm_service =
        LLMService::new(&settings.llm_config, api_key).map_err(|e| e.to_string())?;

    llm_service
        .analyze_context(&context)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn analyze_and_create_tasks(
    state: State<'_, AppState>,
    context: CaptureContext,
) -> Result<Vec<crate::models::Task>, String> {
    let settings = state.get_settings().map_err(|e| e.to_string())?;

    // CLI providers don't need API keys
    let api_key = if settings.llm_config.provider_type.is_cli_provider() {
        None
    } else {
        KeychainManager::get_api_key(settings.llm_config.provider_type.as_str())
            .map_err(|e| e.to_string())?
    };

    let llm_service =
        LLMService::new(&settings.llm_config, api_key).map_err(|e| e.to_string())?;

    let detected_tasks = llm_service
        .analyze_context(&context)
        .await
        .map_err(|e| e.to_string())?;

    let context_str = context.format_for_llm();
    let mut created_tasks = vec![];

    for detected in detected_tasks {
        let mut task = detected.into_task(context_str.clone());
        if let Some(ref title) = context.active_window_title {
            task = task.with_source_window(title.clone());
        }

        state
            .task_repository()
            .insert(&task)
            .map_err(|e| e.to_string())?;

        created_tasks.push(task);
    }

    Ok(created_tasks)
}

#[tauri::command]
pub async fn health_check_llm(state: State<'_, AppState>) -> Result<bool, String> {
    let settings = state.get_settings().map_err(|e| e.to_string())?;

    // CLI providers don't need API keys
    let api_key = if settings.llm_config.provider_type.is_cli_provider() {
        None
    } else {
        KeychainManager::get_api_key(settings.llm_config.provider_type.as_str())
            .map_err(|e| e.to_string())?
    };

    let llm_service =
        LLMService::new(&settings.llm_config, api_key).map_err(|e| e.to_string())?;

    llm_service.health_check().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_llm_config(state: State<'_, AppState>) -> Result<LLMConfig, String> {
    let settings = state.get_settings().map_err(|e| e.to_string())?;
    Ok(settings.llm_config)
}

#[tauri::command]
pub fn update_llm_config(
    state: State<'_, AppState>,
    config: LLMConfig,
) -> Result<(), String> {
    let mut settings = state.get_settings().map_err(|e| e.to_string())?;
    settings.llm_config = config;
    state.save_settings(&settings).map_err(|e| e.to_string())
}
