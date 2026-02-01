use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("Failed to serialize config: {0}")]
    SerializeError(#[from] toml::ser::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub provider: ProviderConfig,
    #[serde(default)]
    pub options: OptionsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    #[serde(default = "default_provider")]
    pub name: String,
    #[serde(default = "default_model")]
    pub model: String,
    pub api_key: Option<String>,
    /// Base URL for OpenAI-compatible providers (Together, Groq, etc.)
    pub base_url: Option<String>,
    #[serde(default = "default_ollama_url")]
    pub ollama_url: String,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            name: default_provider(),
            model: default_model(),
            api_key: None,
            base_url: None,
            ollama_url: default_ollama_url(),
        }
    }
}

fn default_provider() -> String {
    "ollama".to_string()
}

fn default_model() -> String {
    "llama3.2".to_string()
}

fn default_ollama_url() -> String {
    "http://localhost:11434".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsConfig {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_format")]
    pub format: String,
}

impl Default for OptionsConfig {
    fn default() -> Self {
        Self {
            language: default_language(),
            format: default_format(),
        }
    }
}

fn default_language() -> String {
    "en".to_string()
}

fn default_format() -> String {
    "conventional".to_string()
}

impl Config {
    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("git-ai")
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    pub fn load() -> Result<Self, ConfigError> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let path = Self::config_path();
        let dir = Self::config_dir();

        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }
}
