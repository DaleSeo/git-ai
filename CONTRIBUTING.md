# Contributing to git-ai

Thank you for your interest in contributing to git-ai! This guide will help you set up your development environment and understand the testing workflow.

## Development Setup

### Prerequisites

- Rust 1.70+ (for `IsTerminal` trait)
- Git
- Optional: Ollama for local testing

### Building from Source

```sh
# Clone the repository
git clone https://github.com/DaleSeo/git-ai.git
cd git-ai

# Build the project
cargo build --release

# The binary will be at:
./target/release/git-ai
```

## Testing

### Unit Tests

```sh
cargo test
```

### Integration Testing

git-ai behaves differently in TTY (interactive terminal) vs non-TTY (CI/CD, scripts) environments. Test both scenarios:

#### 1. Non-TTY Environment (Claude Code, CI/CD)

In non-interactive environments, `git ai commit` automatically selects the first suggestion without prompting:

```sh
# Create test changes
echo "# Test" >> README.md
git add README.md

# Test message generation (no commit)
./target/release/git-ai commit --dry-run

# Test actual commit (auto-selects first suggestion)
./target/release/git-ai commit

# Clean up
git reset HEAD~1
git restore README.md
```

#### 2. TTY Environment (Normal Terminal)

In interactive terminals, users can choose from multiple suggestions with arrow keys:

```sh
# In a regular terminal (Terminal.app, iTerm, etc.)
cd /path/to/git-ai

# Create test changes
echo "# Test" >> README.md
git add README.md

# Interactive selection UI
./target/release/git-ai commit
# Use ↑↓ arrows to select, Enter to confirm

# Clean up
git reset HEAD~1
git restore README.md
```

#### 3. Force Auto-Selection (Testing --yes flag)

```sh
# Skip interactive prompt even in TTY
./target/release/git-ai commit --yes
```

### Testing LLM Providers

#### Local Testing (Ollama - Default)

```sh
# Install Ollama
# https://ollama.ai

# Pull a model
ollama pull llama3.2

# Configure git-ai (default)
git ai config --provider ollama --model llama3.2

# Test
git ai commit --dry-run
```

#### OpenAI

```sh
git ai config --provider openai
git ai config --model gpt-4o
git ai config --api-key sk-...

# Or use environment variable
export OPENAI_API_KEY=sk-...
```

#### Anthropic Claude

```sh
git ai config --provider anthropic
git ai config --model claude-3-5-sonnet-20241022
git ai config --api-key sk-ant-...

# Or use environment variable
export ANTHROPIC_API_KEY=sk-ant-...
```

#### OpenAI-Compatible Providers (Together AI, Groq, etc.)

```sh
# Together AI
git ai config --provider openai
git ai config --base-url https://api.together.xyz/v1
git ai config --model meta-llama/Llama-3.2-3B-Instruct-Turbo
git ai config --api-key <YOUR_API_KEY>

# Groq
git ai config --base-url https://api.groq.com/openai/v1
git ai config --model llama-3.2-3b-preview
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Follow Rust best practices from [rust-best-practices](https://github.com/DaleSeo/skills/blob/main/rust-best-practices/SKILL.md)

## Pull Request Guidelines

1. Create a new branch: `git checkout -b feature/your-feature`
2. Make your changes
3. Test both TTY and non-TTY scenarios
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Check for warnings: `cargo clippy`
7. Commit with conventional commits format:
   ```sh
   git ai commit --type feat  # or fix, docs, refactor, etc.
   ```
8. Push and create a PR:
   ```sh
   git push -u origin feature/your-feature
   git ai pr --copy  # Generates PR description
   ```

## Architecture Overview

```
src/
├── main.rs           # CLI entry point (clap)
├── config.rs         # Config management (~/.config/git-ai/config.toml)
├── git.rs            # Git command wrapper
├── llm/
│   ├── mod.rs        # LLM provider abstraction
│   ├── openai.rs     # OpenAI-compatible API
│   ├── anthropic.rs  # Anthropic Claude
│   └── ollama.rs     # Local Ollama
└── commands/
    ├── config.rs     # git ai config
    ├── commit.rs     # git ai commit (TTY detection here)
    └── pr.rs         # git ai pr
```

### Key Design Decisions

- **TTY Detection**: `std::io::IsTerminal` trait determines interactive vs non-interactive mode
- **Config Location**: `~/.config/git-ai/config.toml` (XDG Base Directory)
- **Provider Abstraction**: Unified `LlmClient` trait for all providers
- **Error Handling**: `anyhow` for user-facing errors, graceful degradation

## Release Process

See [CLAUDE.md](./CLAUDE.md) for version management and deployment workflow.

## Questions?

- Open an issue: https://github.com/DaleSeo/git-ai/issues
- Check existing docs: [README.md](./README.md), [CLAUDE.md](./CLAUDE.md)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
