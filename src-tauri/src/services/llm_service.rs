use crate::models::{CaptureContext, DetectedTask, LLMConfig, LLMProviderType};
use crate::providers::{AnthropicProvider, CLIProvider, LLMProvider, OllamaProvider, OpenAIProvider};
use anyhow::Result;
use std::sync::Arc;

/// Service for managing LLM providers and analyzing context
pub struct LLMService {
    provider: Arc<dyn LLMProvider>,
}

impl LLMService {
    pub fn new(config: &LLMConfig, api_key: Option<String>) -> Result<Self> {
        let provider: Arc<dyn LLMProvider> = match config.provider_type {
            LLMProviderType::Anthropic => {
                let key = api_key.ok_or_else(|| anyhow::anyhow!("Anthropic API key required"))?;
                Arc::new(AnthropicProvider::new(key, config.model.clone()))
            }
            LLMProviderType::OpenAI => {
                let key = api_key.ok_or_else(|| anyhow::anyhow!("OpenAI API key required"))?;
                Arc::new(OpenAIProvider::new(key, config.model.clone()))
            }
            LLMProviderType::Ollama => {
                let endpoint = config.endpoint.clone().unwrap_or_else(|| "http://localhost:11434".to_string());
                Arc::new(OllamaProvider::new(endpoint, config.model.clone()))
            }
            LLMProviderType::ClaudeCLI => Arc::new(CLIProvider::new_claude()),
            LLMProviderType::CbcodeCLI => Arc::new(CLIProvider::new_cbcode()),
            LLMProviderType::Custom => {
                let endpoint = config.endpoint.clone().ok_or_else(|| anyhow::anyhow!("Custom endpoint required"))?;
                Arc::new(OllamaProvider::new(endpoint, config.model.clone()))
            }
        };

        Ok(Self { provider })
    }

    /// Analyze captured context and detect tasks
    pub async fn analyze_context(&self, context: &CaptureContext) -> Result<Vec<DetectedTask>> {
        self.provider.analyze_context(context).await
    }

    /// Check if the provider is healthy
    pub async fn health_check(&self) -> Result<bool> {
        self.provider.health_check().await
    }

    /// Get the provider name
    pub fn provider_name(&self) -> &str {
        self.provider.provider_name()
    }
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

/// Detect available CLI tools
pub fn detect_cli_tools() -> crate::models::DetectedCLITools {
    use std::process::Command;

    let path = get_augmented_path();

    let claude = Command::new("which")
        .arg("claude")
        .env("PATH", &path)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let cbcode = Command::new("which")
        .arg("cbcode")
        .env("PATH", &path)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let ollama = Command::new("which")
        .arg("ollama")
        .env("PATH", &path)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    crate::models::DetectedCLITools {
        claude,
        cbcode,
        ollama,
    }
}
