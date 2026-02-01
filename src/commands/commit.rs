use crate::config::Config;
use crate::git::Git;
use crate::llm::LlmClient;
use clap::Args;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::IsTerminal;

#[derive(Args)]
pub struct CommitArgs {
    /// Stage all changes before committing
    #[arg(short = 'a', long)]
    pub all: bool,

    /// Auto-confirm with the first suggestion
    #[arg(short = 'y', long)]
    pub yes: bool,

    /// Only print the generated message, don't commit
    #[arg(long)]
    pub dry_run: bool,

    /// Specify the commit type for conventional commits
    #[arg(long)]
    pub r#type: Option<String>,
}

pub async fn run(args: CommitArgs) -> anyhow::Result<()> {
    // Stage all changes if requested
    if args.all {
        Git::stage_all()?;
        println!("{}", "Staged all changes.".dimmed());
    }

    // Get staged diff
    let diff = match Git::get_staged_diff() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    // Load config
    let config = Config::load()?;

    // Build prompt
    let prompt = build_commit_prompt(&diff, &config, args.r#type.as_deref());

    // Get LLM client
    println!("{}", "Generating commit message...".dimmed());
    let client = match LlmClient::from_config(&config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            eprintln!(
                "{}",
                "Run 'git ai config' to configure your LLM provider.".yellow()
            );
            std::process::exit(1);
        }
    };

    // Generate commit message
    let response = client.generate(&prompt).await?;
    let messages = parse_suggestions(&response);

    if messages.is_empty() {
        eprintln!("{}", "Failed to generate commit message.".red());
        std::process::exit(1);
    }

    // If dry-run, just print and exit
    if args.dry_run {
        println!("\n{}", "Generated commit message(s):".green().bold());
        for (i, msg) in messages.iter().enumerate() {
            println!("\n{}. {}", i + 1, msg);
        }
        return Ok(());
    }

    // Select message
    let selected = if args.yes || messages.len() == 1 || !std::io::stdin().is_terminal() {
        messages[0].clone()
    } else {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a commit message")
            .items(&messages)
            .default(0)
            .interact()?;
        messages[selection].clone()
    };

    // Commit
    Git::commit(&selected)?;
    println!("\n{} {}", "✓".green().bold(), "Committed:".green());
    println!("  {}", selected);

    Ok(())
}

fn build_commit_prompt(diff: &str, config: &Config, commit_type: Option<&str>) -> String {
    let language_instruction = match config.options.language.as_str() {
        "ko" => "Write the commit message in Korean.",
        _ => "Write the commit message in English.",
    };

    let format_instruction = match config.options.format.as_str() {
        "conventional" => {
            let type_hint = if let Some(t) = commit_type {
                format!("Use '{}' as the commit type.", t)
            } else {
                "Choose the appropriate type (feat, fix, docs, style, refactor, test, chore)."
                    .to_string()
            };
            format!(
                "Use the Conventional Commits format: <type>(<optional scope>): <description>. {}",
                type_hint
            )
        }
        _ => "Write a clear, concise commit message.".to_string(),
    };

    format!(
        r#"You are a helpful assistant that generates Git commit messages based on the given diff.

Instructions:
- {language_instruction}
- {format_instruction}
- Keep the subject line under 72 characters.
- Be specific about what changed.
- Generate 3 different suggestions, each on a new line starting with a number (1., 2., 3.).

Git diff:
```
{diff}
```

Generate 3 commit message suggestions:"#,
        language_instruction = language_instruction,
        format_instruction = format_instruction,
        diff = truncate_diff(diff, 4000)
    )
}

fn truncate_diff(diff: &str, max_chars: usize) -> &str {
    if diff.len() <= max_chars {
        diff
    } else {
        &diff[..max_chars]
    }
}

fn parse_suggestions(response: &str) -> Vec<String> {
    let mut suggestions = Vec::new();

    for line in response.lines() {
        let trimmed = line.trim();
        // Match lines starting with "1.", "2.", "3.", etc.
        if let Some(rest) = trimmed
            .strip_prefix("1.")
            .or_else(|| trimmed.strip_prefix("2."))
            .or_else(|| trimmed.strip_prefix("3."))
            .or_else(|| trimmed.strip_prefix("1)"))
            .or_else(|| trimmed.strip_prefix("2)"))
            .or_else(|| trimmed.strip_prefix("3)"))
        {
            let msg = rest.trim().trim_matches('`').trim_matches('"').trim();
            if !msg.is_empty() {
                suggestions.push(msg.to_string());
            }
        }
    }

    // If no numbered suggestions found, try to use the whole response
    if suggestions.is_empty() {
        let msg = response.trim();
        if !msg.is_empty() {
            suggestions.push(msg.to_string());
        }
    }

    suggestions
}
