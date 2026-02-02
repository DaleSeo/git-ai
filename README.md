# git-ai

AI-powered Git assistant for commit messages, PR descriptions, and more.

## Installation

```sh
npm install -g @daleseo/git-ai
```

Or use with npx:

```sh
npx @daleseo/git-ai commit
```

## Quick Start

```sh
# Generate commit message from staged changes
git ai commit

# Generate PR description
git ai pr
```

By default, git-ai uses **Ollama** (local, free). To use other providers like OpenAI or Anthropic, see [LLM Provider Setup](#llm-provider-setup) below.

## Commands

### `git ai commit`

Generate AI-powered commit messages.

```sh
git ai commit              # Interactive selection
git ai commit -a           # Stage all changes first
git ai commit -y           # Auto-confirm
git ai commit --dry-run    # Preview only
```

### `git ai pr`

Generate PR title and description.

```sh
git ai pr                  # Generate PR description
git ai pr --copy           # Copy to clipboard
```

### `git ai config`

Manage configuration.

```sh
git ai config              # Show current config
git ai config --provider openai
git ai config --model gpt-4o
git ai config --format gitmoji
```

## LLM Provider Setup

### Ollama (Default)

Local, offline, and free. No configuration needed.

```sh
# Install Ollama from https://ollama.ai
ollama pull llama3.2

# git-ai uses Ollama by default, so you can start using it immediately
git ai commit
```

### OpenAI

Requires API key from https://platform.openai.com/api-keys

```sh
git ai config --provider openai
git ai config --model gpt-4o
git ai config --api-key <YOUR_OPENAI_API_KEY>
```

Or use environment variable:

```sh
export OPENAI_API_KEY="sk-..."
git ai config --provider openai
git ai config --model gpt-4o
```

### Anthropic Claude

Requires API key from https://console.anthropic.com/

```sh
git ai config --provider anthropic
git ai config --model claude-3-5-sonnet-20241022
git ai config --api-key <YOUR_ANTHROPIC_API_KEY>
```

Or use environment variable:

```sh
export ANTHROPIC_API_KEY="sk-ant-..."
git ai config --provider anthropic
git ai config --model claude-3-5-sonnet-20241022
```

### Together AI

OpenAI-compatible API with competitive pricing.

```sh
git ai config --provider openai
git ai config --base-url https://api.together.xyz/v1
git ai config --model meta-llama/Llama-3.2-3B-Instruct-Turbo
git ai config --api-key <YOUR_TOGETHER_API_KEY>
```

### Groq

Fast inference with OpenAI-compatible API.

```sh
git ai config --provider openai
git ai config --base-url https://api.groq.com/openai/v1
git ai config --model llama-3.1-8b-instant
git ai config --api-key <YOUR_GROQ_API_KEY>
```

### Other OpenAI-Compatible Providers

Any provider with OpenAI-compatible API can be used:

```sh
git ai config --provider openai
git ai config --base-url <PROVIDER_BASE_URL>
git ai config --model <MODEL_NAME>
git ai config --api-key <YOUR_API_KEY>
```

## Commit Message Formats

- `conventional` (default) - `feat: add feature`
- `conventional-scoped` - `feat(api): add feature`
- `gitmoji` - `✨ feat: add feature`
- `free` - Free-form

## License

MIT

---

For detailed documentation, see [CLAUDE.md](./CLAUDE.md).
