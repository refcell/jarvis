use super::{LLMProvider, TASK_DETECTION_PROMPT};
use crate::models::{CaptureContext, DetectedTask};
use anyhow::Result;
use async_trait::async_trait;
use std::process::Stdio;
use tokio::process::Command;
use serde::Deserialize;

pub struct CLIProvider {
    command: String,
    name: String,
}

/// Get the PATH environment variable with common binary locations added.
/// macOS GUI apps don't inherit the shell's PATH, so we need to include
/// common locations like /opt/homebrew/bin for Homebrew on Apple Silicon.
fn get_augmented_path() -> String {
    let current_path = std::env::var("PATH").unwrap_or_default();
    let additional_paths = [
        "/opt/homebrew/bin",
        "/usr/local/bin",
        "/usr/bin",
        "/bin",
        "/usr/sbin",
        "/sbin",
    ];

    let mut paths: Vec<&str> = additional_paths.to_vec();
    if !current_path.is_empty() {
        paths.push(&current_path);
    }
    paths.join(":")
}

impl CLIProvider {
    pub fn new_claude() -> Self {
        Self {
            command: "claude".to_string(),
            name: "Claude CLI".to_string(),
        }
    }

    pub fn new_cbcode() -> Self {
        Self {
            command: "cbcode".to_string(),
            name: "cbcode CLI".to_string(),
        }
    }
}

#[derive(Deserialize)]
struct TasksResponse {
    tasks: Vec<DetectedTask>,
}

#[async_trait]
impl LLMProvider for CLIProvider {
    async fn analyze_context(&self, context: &CaptureContext) -> Result<Vec<DetectedTask>> {
        let prompt = format!(
            "{}\n\n---\n\nScreen Context:\n{}",
            TASK_DETECTION_PROMPT,
            context.format_for_llm()
        );

        let child = Command::new(&self.command)
            .arg("-p")
            .arg(&prompt)
            .env("PATH", get_augmented_path())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = child.wait_with_output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("{} failed: {}", self.command, stderr));
        }

        let response = String::from_utf8_lossy(&output.stdout);

        // Try to extract JSON from the response
        let json_start = response.find('{');
        let json_end = response.rfind('}');

        let json_str = match (json_start, json_end) {
            (Some(start), Some(end)) if end >= start => &response[start..=end],
            _ => &response,
        };

        let tasks_response: TasksResponse = serde_json::from_str(json_str)
            .map_err(|e| anyhow::anyhow!("Failed to parse CLI response: {} - Response: {}", e, response))?;

        Ok(tasks_response.tasks)
    }

    async fn health_check(&self) -> Result<bool> {
        let output = Command::new("which")
            .arg(&self.command)
            .env("PATH", get_augmented_path())
            .output()
            .await?;

        Ok(output.status.success())
    }

    fn provider_name(&self) -> &str {
        &self.name
    }
}
