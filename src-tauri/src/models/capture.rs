use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Context captured from the screen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureContext {
    pub id: Uuid,
    pub ocr_text: String,
    pub active_window_title: Option<String>,
    pub active_app_name: Option<String>,
    pub captured_at: DateTime<Utc>,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl CaptureContext {
    pub fn new(ocr_text: String, width: u32, height: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            ocr_text,
            active_window_title: None,
            active_app_name: None,
            captured_at: Utc::now(),
            screen_width: width,
            screen_height: height,
        }
    }

    pub fn with_window_info(mut self, title: Option<String>, app_name: Option<String>) -> Self {
        self.active_window_title = title;
        self.active_app_name = app_name;
        self
    }

    /// Format context for LLM consumption
    pub fn format_for_llm(&self) -> String {
        let mut parts = vec![];

        if let Some(ref app) = self.active_app_name {
            parts.push(format!("Active Application: {}", app));
        }

        if let Some(ref title) = self.active_window_title {
            parts.push(format!("Window Title: {}", title));
        }

        parts.push(format!("Screen Content:\n{}", self.ocr_text));

        parts.join("\n\n")
    }
}

/// Raw screen capture data
#[derive(Debug)]
pub struct ScreenCapture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub bytes_per_row: usize,
}
