use clap::ValueEnum;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    En,
    Ko,
}

impl Default for Language {
    fn default() -> Self {
        Self::En
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::En => write!(f, "en"),
            Self::Ko => write!(f, "ko"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Format {
    /// Conventional Commits without scope: type: description
    Conventional,
    /// Conventional Commits with scope: type(scope): description
    ConventionalScoped,
    /// Gitmoji with conventional: 🎨 feat: description
    Gitmoji,
    /// Free-form commit message
    Free,
}

impl Default for Format {
    fn default() -> Self {
        Self::Conventional
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conventional => write!(f, "conventional"),
            Self::ConventionalScoped => write!(f, "conventional-scoped"),
            Self::Gitmoji => write!(f, "gitmoji"),
            Self::Free => write!(f, "free"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum AutoStage {
    Ask,
    Always,
    Never,
}

impl Default for AutoStage {
    fn default() -> Self {
        Self::Ask
    }
}

impl std::fmt::Display for AutoStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ask => write!(f, "ask"),
            Self::Always => write!(f, "always"),
            Self::Never => write!(f, "never"),
        }
    }
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
    #[serde(default)]
    pub language: Language,
    #[serde(default)]
    pub format: Format,
    #[serde(default)]
    pub auto_stage: AutoStage,
}

impl Default for OptionsConfig {
    fn default() -> Self {
        Self {
            language: Language::default(),
            format: Format::default(),
            auto_stage: AutoStage::default(),
        }
    }
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
