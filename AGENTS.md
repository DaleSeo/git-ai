# git-ai

AI-powered Git CLI for commit messages, PR descriptions, and more.

## Project Info

- **Repository**: https://github.com/DaleSeo/git-ai
- **Language**: Rust
- **Distribution**: npm (`@daleseo/git-ai`) + GitHub Releases

## Architecture

```
src/
├── main.rs           # CLI entry point (clap)
├── config.rs         # Config management + Enums (Language, Format, AutoStage)
├── git.rs            # Git command wrapper
├── llm/
│   ├── mod.rs        # LLM provider abstraction
│   ├── openai.rs     # OpenAI-compatible API (supports Together, Groq, etc.)
│   ├── anthropic.rs  # Anthropic Claude
│   └── ollama.rs     # Local Ollama
└── commands/
    ├── config.rs     # git ai config
    ├── commit.rs     # git ai commit (TTY detection, auto-stage logic)
    └── pr.rs         # git ai pr

npm/                  # npm package wrapper for binary distribution
.github/workflows/    # GitHub Actions for cross-platform builds
```

## Commands

### Configuration

```sh
git ai config                               # Show current configuration
git ai config --provider openai             # Set LLM provider
git ai config --model gpt-4o                # Set model
git ai config --lang ko                     # Language (en, ko)
git ai config --format gitmoji              # Commit format
git ai config --auto-stage always           # Auto-stage behavior
git ai config --base-url https://api.together.xyz/v1  # For OpenAI-compatible providers
```

### Commit Messages

```sh
git ai commit                          # Generate commit message from staged diff
git ai commit -a                       # Stage all changes and commit
git ai commit -y                       # Auto-confirm with first suggestion
git ai commit --dry-run                # Preview without committing
git ai commit --type feat              # Specify conventional commit type
```

### Pull Requests

```sh
git ai pr                              # Generate PR title and description
git ai pr --base develop               # Specify base branch
git ai pr --copy                       # Copy to clipboard
```

## Configuration

### Config File

Location: `~/.config/git-ai/config.toml` (Linux/macOS) or `%APPDATA%\git-ai\config.toml` (Windows)

```toml
[provider]
name = "ollama"                              # ollama, openai, anthropic
model = "llama3.2"                           # Model name
api_key = "sk-..."                           # API key (optional, can use env var)
base_url = "https://api.together.xyz/v1"     # For OpenAI-compatible providers
ollama_url = "http://localhost:11434"        # Ollama server URL

[options]
language = "en"                              # en, ko
format = "conventional"                      # conventional, conventional-scoped, gitmoji, free
auto_stage = "ask"                           # ask, always, never
```

### Environment Variables

Alternative to config file:

```sh
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
```

### Commit Message Formats

#### `conventional` (default)

Simple conventional commits without scope:

```
feat: add user authentication
fix: resolve timeout issue
docs: update README
```

#### `conventional-scoped`

Conventional commits with scope (required):

```
feat(auth): add user authentication
fix(api): resolve timeout issue
docs(readme): update installation guide
```

#### `gitmoji`

Gitmoji with conventional commits:

```
✨ feat: add user authentication
🐛 fix: resolve timeout issue
📝 docs: update README
💄 style: improve UI
♻️ refactor: simplify logic
✅ test: add unit tests
🔧 chore: update dependencies
```

Gitmoji mapping:
- ✨ feat (new feature)
- 🐛 fix (bug fix)
- 📝 docs (documentation)
- 💄 style (formatting, styling)
- ♻️ refactor (code refactoring)
- ✅ test (adding tests)
- 🔧 chore (maintenance)

#### `free`

Free-form commit message (no constraints).

### Auto-Stage Behavior

Controls what happens when there are no staged changes:

- `ask` (default): Interactive prompt in TTY, auto-stage in non-TTY
- `always`: Always auto-stage unstaged changes
- `never`: Show error + help message (require manual staging)

## Supported LLM Providers

| Provider    | Config                                                        |
| ----------- | ------------------------------------------------------------- |
| Ollama      | `--provider ollama` (default, local, free)                    |
| OpenAI      | `--provider openai`                                           |
| Anthropic   | `--provider anthropic`                                        |
| Together AI | `--provider openai --base-url https://api.together.xyz/v1`    |
| Groq        | `--provider openai --base-url https://api.groq.com/openai/v1` |
| Fireworks   | `--provider openai --base-url https://api.fireworks.ai/inference/v1` |

### Setup Examples

#### Ollama (Local, Free)

```sh
# Install Ollama from https://ollama.ai
ollama pull llama3.2

git ai config --provider ollama
git ai config --model llama3.2
```

#### OpenAI

```sh
git ai config --provider openai
git ai config --model gpt-4o
git ai config --api-key sk-...
```

#### Together AI

```sh
git ai config --provider openai
git ai config --base-url https://api.together.xyz/v1
git ai config --model meta-llama/Llama-3.2-3B-Instruct-Turbo
git ai config --api-key <YOUR_API_KEY>
```

#### Anthropic Claude

```sh
git ai config --provider anthropic
git ai config --model claude-3-5-sonnet-20241022
git ai config --api-key sk-ant-...
```

## Development

### Build

```sh
cargo build --release
```

### Testing

git-ai behaves differently in TTY (interactive terminal) vs non-TTY (CI/CD, scripts) environments.

#### Non-TTY Environment (Claude Code, CI/CD)

Auto-selects first suggestion without prompting:

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

#### TTY Environment (Normal Terminal)

Interactive selection UI with arrow keys:

```sh
# In a regular terminal (Terminal.app, iTerm, etc.)
echo "# Test" >> README.md
git add README.md

# Interactive selection UI (↑↓ arrows, Enter to confirm)
./target/release/git-ai commit

# Clean up
git reset HEAD~1
git restore README.md
```

#### Force Auto-Selection

```sh
# Skip interactive prompt even in TTY
./target/release/git-ai commit --yes
```

### Testing Auto-Stage Behavior

```sh
# Set auto-stage to always
git ai config --auto-stage always

# Test with unstaged changes (should auto-stage)
echo "# Test" >> README.md
./target/release/git-ai commit --dry-run

# Set to never (should show error)
git ai config --auto-stage never
./target/release/git-ai commit  # Error: No staged changes

# Reset to default
git ai config --auto-stage ask
```

## Deployment

### Version Management

Update version in both Cargo.toml and npm/package.json:

```sh
./scripts/bump-version.sh patch   # 0.0.3 → 0.0.4
./scripts/bump-version.sh minor   # 0.0.3 → 0.1.0
./scripts/bump-version.sh major   # 0.0.3 → 1.0.0
./scripts/bump-version.sh 0.0.4   # explicit version
```

Verify versions are in sync:

```sh
./scripts/check-version-sync.sh
```

### Release Process

1. Bump version: `./scripts/bump-version.sh patch`
2. Review: `git diff`
3. Commit: `git add -A && git commit -m 'chore: bump version to 0.0.4'`
4. Tag: `git tag -a v0.0.4 -m "Release v0.0.4"`
5. Push: `git push --follow-tags`

GitHub Actions will auto-build and publish to npm.

## TODO

- [ ] npm 배포 (NPM_TOKEN 설정 필요)
- [ ] Conventional format scope 프롬프트 개선 (여전히 scope 포함됨)
- [ ] `git ai review` - 코드 리뷰 피드백
- [ ] `git ai changelog` - CHANGELOG 자동 생성
- [ ] `git ai explain` - 커밋/diff 설명
- [ ] `git ai hook install` - Git hook 설치
