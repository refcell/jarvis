use crate::models::ScreenCapture;
use anyhow::Result;
use std::process::Command;
use std::path::PathBuf;

/// Service for performing OCR on screen captures using macOS Vision framework via shortcuts
pub struct OcrService {
    temp_dir: PathBuf,
}

impl OcrService {
    pub fn new() -> Result<Self> {
        let temp_dir = std::env::temp_dir().join("jarvis_ocr");
        std::fs::create_dir_all(&temp_dir)?;
        Ok(Self { temp_dir })
    }

    /// Extract text from a screen capture using macOS Vision framework
    pub fn extract_text(&self, capture: &ScreenCapture) -> Result<String> {
        // Save capture to temp file
        let timestamp = chrono::Utc::now().timestamp_millis();
        let image_path = self.temp_dir.join(format!("ocr_{}.png", timestamp));

        // Create image from raw data
        let img = image::RgbaImage::from_raw(
            capture.width,
            capture.height,
            capture.data.clone(),
        )
        .ok_or_else(|| anyhow::anyhow!("Failed to create image from capture data"))?;

        img.save(&image_path)?;

        let result = self.extract_text_from_file(&image_path);

        // Clean up
        let _ = std::fs::remove_file(&image_path);

        result
    }

    /// Extract text from an image file using Vision framework via shortcuts
    pub fn extract_text_from_file(&self, path: &std::path::Path) -> Result<String> {
        // Use AppleScript to invoke Vision framework OCR
        let script = format!(
            r#"
            use framework "Vision"
            use framework "Foundation"
            use scripting additions

            set imagePath to "{}"
            set theImage to current application's NSImage's alloc()'s initWithContentsOfFile:imagePath

            if theImage is missing value then
                return ""
            end if

            set theBitmap to current application's NSBitmapImageRep's imageRepWithData:(theImage's TIFFRepresentation())
            set theCGImage to theBitmap's CGImage()

            set theRequest to current application's VNRecognizeTextRequest's alloc()'s init()
            theRequest's setRecognitionLevel:(current application's VNRequestTextRecognitionLevelAccurate)

            set theHandler to current application's VNImageRequestHandler's alloc()'s initWithCGImage:theCGImage options:(current application's NSDictionary's dictionary())
            theHandler's performRequests:(current application's NSArray's arrayWithObject:theRequest) |error|:(missing value)

            set theResults to theRequest's results()
            set theText to ""

            repeat with observation in theResults
                set theText to theText & ((observation's topCandidates:1)'s firstObject()'s |string|() as text) & linefeed
            end repeat

            return theText
            "#,
            path.display()
        );

        let output = Command::new("osascript")
            .args(["-l", "AppleScript", "-e", &script])
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            // Fallback: return empty string if OCR fails
            log::warn!("OCR failed: {}", String::from_utf8_lossy(&output.stderr));
            Ok(String::new())
        }
    }
}

impl Default for OcrService {
    fn default() -> Self {
        Self::new().expect("Failed to initialize OCR service")
    }
}
