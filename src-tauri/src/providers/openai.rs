use super::{LLMProvider, TASK_DETECTION_PROMPT};
use crate::models::{CaptureContext, DetectedTask};
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: model.unwrap_or_else(|| "gpt-4o".to_string()),
        }
    }
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: Option<String>,
}

#[derive(Deserialize)]
struct TasksResponse {
    tasks: Vec<DetectedTask>,
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn analyze_context(&self, context: &CaptureContext) -> Result<Vec<DetectedTask>> {
        let request = OpenAIRequest {
            model: self.model.clone(),
            max_tokens: 1024,
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: TASK_DETECTION_PROMPT.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: context.format_for_llm(),
                },
            ],
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }

        let response: OpenAIResponse = response.json().await?;

        let text = response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))?;

        let tasks_response: TasksResponse = serde_json::from_str(&text)
            .map_err(|e| anyhow::anyhow!("Failed to parse LLM response: {} - Response: {}", e, text))?;

        Ok(tasks_response.tasks)
    }

    async fn health_check(&self) -> Result<bool> {
        let response = self
            .client
            .get("https://api.openai.com/v1/models")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    fn provider_name(&self) -> &str {
        "OpenAI"
    }
}
