mod anthropic;
mod ollama;
mod openai;

use crate::config::Config;
use thiserror::Error;

pub use anthropic::AnthropicClient;
pub use ollama::OllamaClient;
pub use openai::OpenAIClient;

#[derive(Error, Debug)]
pub enum LlmError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Missing API key for {0}")]
    MissingApiKey(String),
    #[error("Unknown provider: {0}")]
    UnknownProvider(String),
}

#[async_trait::async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String, LlmError>;
}

pub struct LlmClient {
    provider: Box<dyn LlmProvider>,
}

impl LlmClient {
    pub fn from_config(config: &Config) -> Result<Self, LlmError> {
        let provider: Box<dyn LlmProvider> = match config.provider.name.as_str() {
            "openai" => {
                let api_key = config
                    .provider
                    .api_key
                    .clone()
                    .or_else(|| std::env::var("OPENAI_API_KEY").ok())
                    .ok_or_else(|| LlmError::MissingApiKey("OpenAI".to_string()))?;
                Box::new(OpenAIClient::new(
                    api_key,
                    config.provider.model.clone(),
                    config.provider.base_url.clone(),
                ))
            }
            "anthropic" => {
                let api_key = config
                    .provider
                    .api_key
                    .clone()
                    .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
                    .ok_or_else(|| LlmError::MissingApiKey("Anthropic".to_string()))?;
                Box::new(AnthropicClient::new(api_key, config.provider.model.clone()))
            }
            "ollama" => Box::new(OllamaClient::new(
                config.provider.ollama_url.clone(),
                config.provider.model.clone(),
            )),
            other => return Err(LlmError::UnknownProvider(other.to_string())),
        };

        Ok(Self { provider })
    }

    pub async fn generate(&self, prompt: &str) -> Result<String, LlmError> {
        self.provider.generate(prompt).await
    }
}
