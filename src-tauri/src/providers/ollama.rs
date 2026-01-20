use super::{LLMProvider, TASK_DETECTION_PROMPT};
use crate::models::{CaptureContext, DetectedTask};
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OllamaProvider {
    client: Client,
    endpoint: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(endpoint: String, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            endpoint,
            model: model.unwrap_or_else(|| "llama3.2".to_string()),
        }
    }
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[derive(Deserialize)]
struct TasksResponse {
    tasks: Vec<DetectedTask>,
}

#[async_trait]
impl LLMProvider for OllamaProvider {
    async fn analyze_context(&self, context: &CaptureContext) -> Result<Vec<DetectedTask>> {
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: context.format_for_llm(),
            system: TASK_DETECTION_PROMPT.to_string(),
            stream: false,
        };

        let response = self
            .client
            .post(format!("{}/api/generate", self.endpoint))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Ollama API error: {}", error_text));
        }

        let response: OllamaResponse = response.json().await?;

        // Try to extract JSON from the response (Ollama might include extra text)
        let json_start = response.response.find('{');
        let json_end = response.response.rfind('}');

        let json_str = match (json_start, json_end) {
            (Some(start), Some(end)) if end >= start => &response.response[start..=end],
            _ => &response.response,
        };

        let tasks_response: TasksResponse = serde_json::from_str(json_str)
            .map_err(|e| anyhow::anyhow!("Failed to parse LLM response: {} - Response: {}", e, response.response))?;

        Ok(tasks_response.tasks)
    }

    async fn health_check(&self) -> Result<bool> {
        let response = self
            .client
            .get(format!("{}/api/tags", self.endpoint))
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    fn provider_name(&self) -> &str {
        "Ollama"
    }
}
