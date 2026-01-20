use serde::{Deserialize, Serialize};
use super::LLMConfig;

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Screen capture interval in seconds
    pub capture_interval_secs: u64,
    /// Whether screen watching is enabled
    pub watching_enabled: bool,
    /// Whether notifications are enabled (disabled by default)
    pub notifications_enabled: bool,
    /// LLM provider configuration
    pub llm_config: LLMConfig,
    /// Priority decay rate per hour (default 0.95 = 5% decay per hour)
    pub priority_decay_rate: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            capture_interval_secs: 30,
            watching_enabled: false,
            notifications_enabled: false,
            llm_config: LLMConfig::default(),
            priority_decay_rate: 0.95,
        }
    }
}

/// Watch status for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchStatus {
    pub is_watching: bool,
    pub last_capture_at: Option<chrono::DateTime<chrono::Utc>>,
    pub captures_since_start: u64,
    pub tasks_detected_since_start: u64,
}

impl Default for WatchStatus {
    fn default() -> Self {
        Self {
            is_watching: false,
            last_capture_at: None,
            captures_since_start: 0,
            tasks_detected_since_start: 0,
        }
    }
}
