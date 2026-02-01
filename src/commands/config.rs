use crate::config::Config;
use clap::Args;
use colored::Colorize;

#[derive(Args)]
pub struct ConfigArgs {
    /// Set the LLM provider (openai, anthropic, ollama)
    #[arg(long)]
    pub provider: Option<String>,

    /// Set the model name
    #[arg(long)]
    pub model: Option<String>,

    /// Set the API key
    #[arg(long)]
    pub api_key: Option<String>,

    /// Set the base URL for OpenAI-compatible providers (Together, Groq, etc.)
    #[arg(long)]
    pub base_url: Option<String>,

    /// Set the language for generated messages (en, ko)
    #[arg(long)]
    pub lang: Option<String>,

    /// Set the commit message format (conventional, free)
    #[arg(long)]
    pub format: Option<String>,

    /// Set the Ollama server URL
    #[arg(long)]
    pub ollama_url: Option<String>,
}

pub async fn run(args: ConfigArgs) -> anyhow::Result<()> {
    let mut config = Config::load().unwrap_or_default();
    let mut changed = false;

    if let Some(provider) = args.provider {
        config.provider.name = provider;
        changed = true;
    }

    if let Some(model) = args.model {
        config.provider.model = model;
        changed = true;
    }

    if let Some(api_key) = args.api_key {
        config.provider.api_key = Some(api_key);
        changed = true;
    }

    if let Some(base_url) = args.base_url {
        config.provider.base_url = Some(base_url);
        changed = true;
    }

    if let Some(lang) = args.lang {
        config.options.language = lang;
        changed = true;
    }

    if let Some(format) = args.format {
        config.options.format = format;
        changed = true;
    }

    if let Some(ollama_url) = args.ollama_url {
        config.provider.ollama_url = ollama_url;
        changed = true;
    }

    if changed {
        config.save()?;
        println!("{}", "Configuration saved!".green());
    }

    // Display current configuration
    println!("\n{}", "Current Configuration:".bold());
    println!("─────────────────────────────────");
    println!("  {} {}", "Provider:".cyan(), config.provider.name);
    println!("  {} {}", "Model:".cyan(), config.provider.model);
    println!(
        "  {} {}",
        "API Key:".cyan(),
        if config.provider.api_key.is_some() {
            "********"
        } else {
            "(not set)"
        }
    );
    if config.provider.name == "openai" {
        if let Some(ref base_url) = config.provider.base_url {
            println!("  {} {}", "Base URL:".cyan(), base_url);
        }
    }
    if config.provider.name == "ollama" {
        println!("  {} {}", "Ollama URL:".cyan(), config.provider.ollama_url);
    }
    println!("  {} {}", "Language:".cyan(), config.options.language);
    println!("  {} {}", "Format:".cyan(), config.options.format);
    println!("─────────────────────────────────");
    println!(
        "\n  Config file: {}",
        Config::config_path().display().to_string().dimmed()
    );

    Ok(())
}
