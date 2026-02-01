use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Git command failed: {0}")]
    CommandFailed(String),
    #[error("Not a git repository")]
    NotARepository,
    #[error("No staged changes")]
    NoStagedChanges,
    #[error("Failed to execute git: {0}")]
    ExecutionError(#[from] std::io::Error),
}

pub struct Git;

impl Git {
    /// Check if we're in a git repository
    pub fn is_repository() -> bool {
        Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Get the diff of staged changes
    pub fn get_staged_diff() -> Result<String, GitError> {
        if !Self::is_repository() {
            return Err(GitError::NotARepository);
        }

        let output = Command::new("git")
            .args(["diff", "--cached", "--no-color"])
            .output()?;

        if !output.status.success() {
            return Err(GitError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let diff = String::from_utf8_lossy(&output.stdout).to_string();
        if diff.trim().is_empty() {
            return Err(GitError::NoStagedChanges);
        }

        Ok(diff)
    }

    /// Check if there are unstaged changes
    pub fn has_unstaged_changes() -> Result<bool, GitError> {
        let output = Command::new("git")
            .args(["diff", "--no-color"])
            .output()?;

        if !output.status.success() {
            return Err(GitError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let diff = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(!diff.trim().is_empty())
    }

    /// Check if there are untracked files
    pub fn has_untracked_files() -> Result<bool, GitError> {
        let output = Command::new("git")
            .args(["ls-files", "--others", "--exclude-standard"])
            .output()?;

        if !output.status.success() {
            return Err(GitError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let files = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(!files.trim().is_empty())
    }

    /// Stage all changes
    pub fn stage_all() -> Result<(), GitError> {
        let output = Command::new("git").args(["add", "-A"]).output()?;

        if !output.status.success() {
            return Err(GitError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    /// Get the diff between current branch and base branch
    pub fn get_branch_diff(base: &str) -> Result<String, GitError> {
        if !Self::is_repository() {
            return Err(GitError::NotARepository);
        }

        let output = Command::new("git")
            .args(["diff", &format!("{}...HEAD", base), "--no-color"])
            .output()?;

        if !output.status.success() {
            return Err(GitError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Get commit log between base and HEAD
    pub fn get_commit_log(base: &str) -> Result<Vec<String>, GitError> {
        let output = Command::new("git")
            .args([
                "log",
                &format!("{}..HEAD", base),
                "--pretty=format:%s",
                "--no-color",
            ])
            .output()?;

        if !output.status.success() {
            return Err(GitError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let log = String::from_utf8_lossy(&output.stdout);
        Ok(log.lines().map(|s| s.to_string()).collect())
    }

    /// Get current branch name
    pub fn current_branch() -> Result<String, GitError> {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()?;

        if !output.status.success() {
            return Err(GitError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Get default branch (main or master)
    pub fn default_branch() -> Result<String, GitError> {
        // Try to get the default branch from remote
        let output = Command::new("git")
            .args(["symbolic-ref", "refs/remotes/origin/HEAD", "--short"])
            .output()?;

        if output.status.success() {
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            // Remove "origin/" prefix
            return Ok(branch
                .strip_prefix("origin/")
                .unwrap_or(&branch)
                .to_string());
        }

        // Fallback: check if main or master exists
        for branch in ["main", "master"] {
            let output = Command::new("git")
                .args(["rev-parse", "--verify", branch])
                .output()?;
            if output.status.success() {
                return Ok(branch.to_string());
            }
        }

        Err(GitError::CommandFailed(
            "Could not determine default branch".to_string(),
        ))
    }

    /// Create a commit with the given message
    pub fn commit(message: &str) -> Result<(), GitError> {
        let output = Command::new("git")
            .args(["commit", "-m", message])
            .output()?;

        if !output.status.success() {
            return Err(GitError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }
}
