# git-ai

AI-powered Git CLI for commit messages, PR descriptions, and more.

## Project Info

- **Repository**: https://github.com/DaleSeo/git-ai
- **Language**: Rust
- **Distribution**: npm (`git-ai`) + GitHub Releases

## Architecture

```
src/
├── main.rs           # CLI entry point (clap)
├── config.rs         # Config management (~/.config/git-ai/config.toml)
├── git.rs            # Git command wrapper
├── llm/
│   ├── mod.rs        # LLM provider abstraction
│   ├── openai.rs     # OpenAI-compatible API (supports Together, Groq, etc.)
│   ├── anthropic.rs  # Anthropic Claude
│   └── ollama.rs     # Local Ollama
└── commands/
    ├── config.rs     # git ai config
    ├── commit.rs     # git ai commit
    └── pr.rs         # git ai pr

npm/                  # npm package wrapper for binary distribution
.github/workflows/    # GitHub Actions for cross-platform builds
```

## Commands

```sh
git ai config                    # Show/set configuration
git ai config --provider openai --base-url https://api.together.xyz/v1
git ai commit                    # Generate commit message from staged diff
git ai commit --dry-run          # Preview without committing
git ai pr                        # Generate PR title and description
git ai pr --copy                 # Copy to clipboard
```

## Supported LLM Providers

| Provider    | Config                                                        |
| ----------- | ------------------------------------------------------------- |
| OpenAI      | `--provider openai`                                           |
| Together AI | `--provider openai --base-url https://api.together.xyz/v1`    |
| Groq        | `--provider openai --base-url https://api.groq.com/openai/v1` |
| Anthropic   | `--provider anthropic`                                        |
| Ollama      | `--provider ollama`                                           |

## Development

```sh
# Build
cargo build --release

# Test locally
./target/release/git-ai --help
./target/release/git-ai commit --dry-run
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
- [ ] `git ai review` - 코드 리뷰 피드백
- [ ] `git ai changelog` - CHANGELOG 자동 생성
- [ ] `git ai explain` - 커밋/diff 설명
- [ ] `git ai hook install` - Git hook 설치
