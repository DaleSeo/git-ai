use crate::config::{Config, Language};
use crate::git::Git;
use crate::llm::LlmClient;
use arboard::Clipboard;
use clap::Args;
use colored::Colorize;

#[derive(Args)]
pub struct PrArgs {
    /// Base branch to compare against (default: main or master)
    #[arg(long, short = 'b')]
    pub base: Option<String>,

    /// Copy the generated PR description to clipboard
    #[arg(long, short = 'c')]
    pub copy: bool,
}

pub async fn run(args: PrArgs) -> anyhow::Result<()> {
    // Determine base branch
    let base = args
        .base
        .unwrap_or_else(|| Git::default_branch().unwrap_or_else(|_| "main".to_string()));

    let current_branch = Git::current_branch()?;
    println!(
        "{} {} → {}",
        "Comparing:".dimmed(),
        base.cyan(),
        current_branch.green()
    );

    // Get diff and commit log
    let diff = match Git::get_branch_diff(&base) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    let commits = Git::get_commit_log(&base).unwrap_or_default();

    if diff.trim().is_empty() && commits.is_empty() {
        eprintln!("{}", "No changes found compared to base branch.".yellow());
        std::process::exit(1);
    }

    // Load config
    let config = Config::load()?;

    // Build prompt
    let prompt = build_pr_prompt(&diff, &commits, &config);

    // Get LLM client
    println!("{}", "Generating PR description...".dimmed());
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

    // Generate PR description
    let response = client.generate(&prompt).await?;

    // Print the result
    println!("\n{}", "━".repeat(60).dimmed());
    println!("{}", response);
    println!("{}", "━".repeat(60).dimmed());

    // Copy to clipboard if requested
    if args.copy {
        match Clipboard::new() {
            Ok(mut clipboard) => {
                if clipboard.set_text(&response).is_ok() {
                    println!("\n{}", "✓ Copied to clipboard!".green());
                } else {
                    eprintln!("{}", "Failed to copy to clipboard.".yellow());
                }
            }
            Err(_) => {
                eprintln!("{}", "Clipboard not available.".yellow());
            }
        }
    } else {
        println!("\n{}", "Tip: Use --copy to copy to clipboard.".dimmed());
    }

    Ok(())
}

fn build_pr_prompt(diff: &str, commits: &[String], config: &Config) -> String {
    let language_instruction = match config.options.language {
        Language::Ko => "Write the PR description in Korean.",
        Language::En => "Write the PR description in English.",
    };

    let commits_section = if !commits.is_empty() {
        format!(
            "Commits in this PR:\n{}",
            commits
                .iter()
                .map(|c| format!("- {}", c))
                .collect::<Vec<_>>()
                .join("\n")
        )
    } else {
        String::new()
    };

    format!(
        r#"You are a helpful assistant that generates Pull Request descriptions.

Instructions:
- {language_instruction}
- Generate a clear, well-structured PR description.
- Include a concise title (first line, without any prefix like "Title:").
- Include a summary section explaining what this PR does.
- Include a list of key changes.
- Keep it professional and informative.

{commits_section}

Git diff (truncated if too long):
```
{diff}
```

Generate the PR title and description:"#,
        language_instruction = language_instruction,
        commits_section = commits_section,
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
