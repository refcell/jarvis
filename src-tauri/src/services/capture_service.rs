use crate::models::ScreenCapture;
use anyhow::Result;
use std::process::Command;
use std::path::PathBuf;

/// Service for capturing screen content using screencapture CLI
pub struct CaptureService {
    temp_dir: PathBuf,
}

impl CaptureService {
    pub fn new() -> Self {
        let temp_dir = std::env::temp_dir().join("jarvis_captures");
        let _ = std::fs::create_dir_all(&temp_dir);
        Self { temp_dir }
    }

    /// Check if screen recording permission is granted
    /// On macOS, this is handled by the system when screencapture is used
    pub fn check_permission() -> bool {
        // screencapture will prompt for permission if needed
        true
    }

    /// Request screen recording permission
    pub fn request_permission() {
        // Open System Preferences to the Screen Recording section
        let _ = Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_ScreenCapture")
            .spawn();
    }

    /// Capture a single frame from the main display
    pub async fn capture_frame(&self) -> Result<ScreenCapture> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let capture_path = self.temp_dir.join(format!("capture_{}.png", timestamp));

        // Use screencapture CLI to capture the screen
        let output = Command::new("screencapture")
            .args(["-x", "-C", "-t", "png"])
            .arg(&capture_path)
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "screencapture failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Read the captured image
        let img = image::open(&capture_path)?;
        let rgba = img.to_rgba8();
        let width = rgba.width();
        let height = rgba.height();
        let bytes_per_row = (width * 4) as usize;

        let capture = ScreenCapture {
            width,
            height,
            data: rgba.into_raw(),
            bytes_per_row,
        };

        // Clean up the temp file
        let _ = std::fs::remove_file(&capture_path);

        Ok(capture)
    }
}

impl Default for CaptureService {
    fn default() -> Self {
        Self::new()
    }
}

/// Get active window information using AppleScript
pub fn get_active_window_info() -> Option<(String, String)> {
    let script = r#"
        tell application "System Events"
            set frontApp to first application process whose frontmost is true
            set appName to name of frontApp
            set windowTitle to ""
            try
                set windowTitle to name of front window of frontApp
            end try
            return appName & "|" & windowTitle
        end tell
    "#;

    let output = Command::new("osascript")
        .args(["-e", script])
        .output()
        .ok()?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let parts: Vec<&str> = result.splitn(2, '|').collect();
        if parts.len() >= 2 {
            Some((parts[1].to_string(), parts[0].to_string()))
        } else if !parts.is_empty() {
            Some((String::new(), parts[0].to_string()))
        } else {
            None
        }
    } else {
        None
    }
}
