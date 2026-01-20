use super::{LLMProvider, TASK_DETECTION_PROMPT};
use crate::models::{CaptureContext, DetectedTask};
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl AnthropicProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: model.unwrap_or_else(|| "claude-sonnet-4-20250514".to_string()),
        }
    }
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    system: String,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: Option<String>,
}

#[derive(Deserialize)]
struct TasksResponse {
    tasks: Vec<DetectedTask>,
}

#[async_trait]
impl LLMProvider for AnthropicProvider {
    async fn analyze_context(&self, context: &CaptureContext) -> Result<Vec<DetectedTask>> {
        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 1024,
            system: TASK_DETECTION_PROMPT.to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: context.format_for_llm(),
            }],
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Anthropic API error: {}", error_text));
        }

        let response: AnthropicResponse = response.json().await?;

        let text = response
            .content
            .into_iter()
            .filter_map(|c| c.text)
            .collect::<Vec<_>>()
            .join("");

        let tasks_response: TasksResponse = serde_json::from_str(&text)
            .map_err(|e| anyhow::anyhow!("Failed to parse LLM response: {} - Response: {}", e, text))?;

        Ok(tasks_response.tasks)
    }

    async fn health_check(&self) -> Result<bool> {
        let response = self
            .client
            .get("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .send()
            .await?;

        // 405 is expected for GET on messages endpoint
        Ok(response.status().as_u16() == 405 || response.status().is_success())
    }

    fn provider_name(&self) -> &str {
        "Anthropic"
    }
}
