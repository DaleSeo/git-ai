use crate::config::{AutoStage, Config, Format, Language};
use crate::git::Git;
use crate::llm::LlmClient;
use clap::Args;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
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
    // Load config early to check auto_stage setting
    let config = Config::load()?;

    // Stage all changes if requested
    if args.all {
        Git::stage_all()?;
        println!("{}", "Staged all changes.".dimmed());
    }

    // Get staged diff
    let diff = match Git::get_staged_diff() {
        Ok(d) => d,
        Err(crate::git::GitError::NoStagedChanges) => {
            // Check for unstaged or untracked changes
            let has_unstaged = Git::has_unstaged_changes().unwrap_or(false);
            let has_untracked = Git::has_untracked_files().unwrap_or(false);

            if has_unstaged || has_untracked {
                // Determine whether to auto-stage based on config
                let should_stage = match config.options.auto_stage {
                    AutoStage::Always => true,
                    AutoStage::Never => {
                        eprintln!("{} {}", "Error:".red().bold(), "No staged changes");
                        eprintln!();
                        eprintln!("{}", "You have unstaged changes. Try:".yellow());
                        eprintln!("  {} stage specific files first", "git add <files>".cyan());
                        eprintln!(
                            "  {} stage all changes and commit",
                            "git ai commit -a".cyan()
                        );
                        std::process::exit(1);
                    }
                    AutoStage::Ask => {
                        // Ask user if they want to stage all changes
                        if std::io::stdin().is_terminal() {
                            // Interactive prompt in TTY
                            Confirm::with_theme(&ColorfulTheme::default())
                                .with_prompt("No staged changes. Stage all changes and continue?")
                                .default(true)
                                .interact()
                                .unwrap_or(false)
                        } else {
                            // Auto-stage in non-TTY (like -a flag)
                            true
                        }
                    }
                };

                if should_stage {
                    Git::stage_all()?;
                    println!("{}", "Staged all changes.".dimmed());
                    // Retry getting staged diff
                    Git::get_staged_diff()?
                } else {
                    eprintln!("{}", "Aborted.".yellow());
                    std::process::exit(1);
                }
            } else {
                eprintln!("{} {}", "Error:".red().bold(), "No changes to commit");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

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
    let language_instruction = match config.options.language {
        Language::Ko => "Write the commit message in Korean.",
        Language::En => "Write the commit message in English.",
    };

    let format_instruction = match config.options.format {
        Format::Conventional => {
            let type_hint = if let Some(t) = commit_type {
                format!("Use '{}' as the commit type.", t)
            } else {
                "Choose the appropriate type: feat, fix, docs, style, refactor, test, chore"
                    .to_string()
            };
            format!(
                r#"Use Conventional Commits format WITHOUT scope. Follow this pattern EXACTLY:
  - type: description

Examples:
  - feat: add user authentication
  - fix: resolve timeout issue
  - docs: update README with examples

IMPORTANT:
  - Do NOT use scope: ❌ feat(api): description
  - Only use type and description: ✅ feat: description
  - {}"#,
                type_hint
            )
        }
        Format::ConventionalScoped => {
            let type_hint = if let Some(t) = commit_type {
                format!("Use '{}' as the commit type.", t)
            } else {
                "Choose the appropriate type: feat, fix, docs, style, refactor, test, chore"
                    .to_string()
            };
            format!(
                r#"Use Conventional Commits format WITH scope. Follow this pattern EXACTLY:
  - type(scope): description

Examples:
  - feat(auth): add user authentication
  - fix(api): resolve timeout issue
  - docs(readme): update installation guide

IMPORTANT:
  - Always include scope: ✅ feat(api): description
  - Never omit scope: ❌ feat: description
  - Never nest parentheses: ❌ feat(api): fix)
  - {}"#,
                type_hint
            )
        }
        Format::Gitmoji => {
            format!(
                r#"Use Gitmoji with Conventional Commits format. Follow this pattern EXACTLY:
  - emoji type: description

Gitmoji mapping:
  - ✨ feat: new feature
  - 🐛 fix: bug fix
  - 📝 docs: documentation
  - 💄 style: formatting, styling
  - ♻️ refactor: code refactoring
  - ✅ test: adding tests
  - 🔧 chore: maintenance

Examples:
  - ✨ feat: add user authentication
  - 🐛 fix: resolve timeout issue
  - 📝 docs: update README with examples
  - ♻️ refactor: simplify error handling

IMPORTANT:
  - Always start with the emoji: ✅ ✨ feat: description
  - Never omit emoji: ❌ feat: description
  - Use the correct emoji for the type
  - Keep type keyword after emoji for clarity"#
            )
        }
        Format::Free => "Write a clear, concise commit message.".to_string(),
    };

    format!(
        r#"You are a helpful assistant that generates Git commit messages based on the given diff.

Instructions:
- {language_instruction}
- {format_instruction}
- Keep the subject line under 72 characters
- Be specific about what changed
- Generate 3 different suggestions
- Output ONLY the commit messages, one per line, starting with "1. ", "2. ", "3. "
- Do NOT include any explanations, markdown formatting, or extra text

Git diff:
```
{diff}
```

Output format (follow EXACTLY):
1. type: description
2. type(scope): description
3. type: description"#,
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
