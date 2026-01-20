use crate::models::{CaptureContext, DetectedTask};
use anyhow::Result;
use async_trait::async_trait;

/// Trait for LLM providers
#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Analyze screen context and detect actionable tasks
    async fn analyze_context(&self, context: &CaptureContext) -> Result<Vec<DetectedTask>>;

    /// Check if the provider is healthy and accessible
    async fn health_check(&self) -> Result<bool>;

    /// Get the provider name for display
    fn provider_name(&self) -> &str;
}

/// System prompt for task detection
pub const TASK_DETECTION_PROMPT: &str = r#"You are an AI assistant that analyzes screen content to detect actionable tasks.

Given the screen context below, identify any actionable tasks the user should complete. Look for:
- TODO items or tasks mentioned in text
- Unread messages or notifications that need responses
- Calendar events or deadlines
- Error messages that need attention
- Forms that need to be filled out
- Reminders or notes

For each task found, provide:
1. A clear, concise title (max 50 characters)
2. A brief description of what needs to be done
3. A priority score from 0.1 to 1.0 based on urgency and importance

Respond in JSON format:
{
  "tasks": [
    {
      "title": "Task title",
      "description": "Brief description of the task",
      "priority": 0.8
    }
  ]
}

If no actionable tasks are found, return: {"tasks": []}
"#;
