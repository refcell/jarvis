use crate::models::CaptureContext;
use crate::services::{get_active_window_info, CaptureService, OcrService};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn check_screen_permission() -> bool {
    CaptureService::check_permission()
}

#[tauri::command]
pub fn request_screen_permission() {
    CaptureService::request_permission();
}

#[tauri::command]
pub async fn capture_screen(state: State<'_, AppState>) -> Result<CaptureContext, String> {
    let capture_service = CaptureService::new();

    // Capture frame
    let frame = capture_service
        .capture_frame()
        .await
        .map_err(|e| e.to_string())?;

    // Perform OCR
    let ocr_service = OcrService::new().map_err(|e| e.to_string())?;
    let ocr_text = ocr_service
        .extract_text(&frame)
        .map_err(|e| e.to_string())?;

    // Get window info
    let (window_title, app_name) = get_active_window_info().unzip();

    // Create context
    let context = CaptureContext::new(ocr_text, frame.width, frame.height)
        .with_window_info(window_title, app_name);

    // Store in database
    state
        .context_repository()
        .insert(&context)
        .map_err(|e| e.to_string())?;

    Ok(context)
}

#[tauri::command]
pub fn get_recent_captures(
    state: State<'_, AppState>,
    limit: usize,
) -> Result<Vec<CaptureContext>, String> {
    state
        .context_repository()
        .get_recent(limit)
        .map_err(|e| e.to_string())
}
