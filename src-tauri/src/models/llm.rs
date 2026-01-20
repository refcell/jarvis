use serde::{Deserialize, Serialize};

/// LLM provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LLMProviderType {
    Anthropic,
    OpenAI,
    Ollama,
    ClaudeCLI,
    CbcodeCLI,
    Custom,
}

impl LLMProviderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LLMProviderType::Anthropic => "anthropic",
            LLMProviderType::OpenAI => "openai",
            LLMProviderType::Ollama => "ollama",
            LLMProviderType::ClaudeCLI => "claude_cli",
            LLMProviderType::CbcodeCLI => "cbcode_cli",
            LLMProviderType::Custom => "custom",
        }
    }

    /// Returns true if this provider is a CLI tool that doesn't require an API key
    pub fn is_cli_provider(&self) -> bool {
        matches!(self, LLMProviderType::ClaudeCLI | LLMProviderType::CbcodeCLI)
    }

    /// Returns true if this provider requires an API key
    pub fn requires_api_key(&self) -> bool {
        matches!(self, LLMProviderType::Anthropic | LLMProviderType::OpenAI)
    }
}

/// Configuration for an LLM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub provider_type: LLMProviderType,
    pub model: Option<String>,
    pub endpoint: Option<String>,
    pub api_key_stored: bool,
    pub enabled: bool,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            provider_type: LLMProviderType::ClaudeCLI,
            model: None,
            endpoint: None,
            api_key_stored: false,
            enabled: true,
        }
    }
}

/// Detected CLI tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedCLITools {
    pub claude: bool,
    pub cbcode: bool,
    pub ollama: bool,
}

impl Default for DetectedCLITools {
    fn default() -> Self {
        Self {
            claude: false,
            cbcode: false,
            ollama: false,
        }
    }
}
