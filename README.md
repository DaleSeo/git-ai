# git-ai

AI-powered Git assistant for commit messages, PR descriptions, and more.

## Installation

### Via npm (recommended)

```sh
npm install -g git-ai
```

Or run directly with npx:

```sh
npx git-ai commit
```

### Build from source

```sh
cargo build --release
cp target/release/git-ai /usr/local/bin/
```

Once installed, you can use it as a Git subcommand:

```sh
git ai commit
git ai pr
git ai config
```

## Commands

### `git ai config`

Manage git-ai configuration.

```sh
git ai config                          # Show current configuration
git ai config --provider openai        # Set LLM provider
git ai config --model gpt-4o           # Set model
git ai config --lang ko                # Set language (en, ko)
git ai config --format conventional    # Set commit format
```

Supported providers:

- `ollama` (default) - Local, offline, free
- `openai` - Requires API key
- `anthropic` - Requires API key

### `git ai commit`

Generate AI-powered commit messages based on staged changes.

```sh
git ai commit              # Generate and select commit message
git ai commit -a           # Stage all changes, then commit
git ai commit -y           # Auto-confirm with first suggestion
git ai commit --dry-run    # Print message without committing
git ai commit --type feat  # Specify conventional commit type
```

### `git ai pr`

Generate PR title and description.

```sh
git ai pr                  # Compare current branch with main
git ai pr --base develop   # Specify base branch
git ai pr --copy           # Copy to clipboard
```

## Configuration

Config file location: `~/.config/git-ai/config.toml`

```toml
[provider]
name = "ollama"
model = "llama3.2"
ollama_url = "http://localhost:11434"
# api_key = "sk-..."  # For OpenAI/Anthropic

[options]
language = "en"
format = "conventional"
```

## Environment Variables

- `OPENAI_API_KEY` - OpenAI API key (alternative to config file)
- `ANTHROPIC_API_KEY` - Anthropic API key (alternative to config file)

## Using OpenAI-Compatible Providers

Together AI, Groq, Fireworks, OpenRouter 등 OpenAI 호환 API를 사용하려면:

```sh
git ai config --provider openai
git ai config --base-url https://api.together.xyz/v1
git ai config --model meta-llama/Llama-3.2-3B-Instruct-Turbo
git ai config --api-key <YOUR_API_KEY>
```

## License

MIT
