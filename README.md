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

# Configure LLM provider
git ai config --provider openai
git ai config --api-key <YOUR_API_KEY>
```

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

## Supported Providers

- **Ollama** (default) - Local, offline, free
- **OpenAI** - Requires API key
- **Anthropic** - Requires API key
- **OpenAI-compatible** - Together AI, Groq, etc.

## Commit Message Formats

- `conventional` (default) - `feat: add feature`
- `conventional-scoped` - `feat(api): add feature`
- `gitmoji` - `✨ feat: add feature`
- `free` - Free-form

## License

MIT

---

For detailed documentation, see [CLAUDE.md](./CLAUDE.md).
