# Version Management Scripts

## Prerequisites (Optional)

For cleaner Cargo.toml updates, install `cargo-edit`:
```sh
cargo install cargo-edit
```

The script will automatically use `cargo set-version` if available, otherwise falls back to `sed`.

## Quick Reference

### Bump version

Using semantic keywords (recommended):
```sh
./scripts/bump-version.sh patch   # 0.0.3 → 0.0.4
./scripts/bump-version.sh minor   # 0.0.3 → 0.1.0
./scripts/bump-version.sh major   # 0.0.3 → 1.0.0
```

Or set explicit version:
```sh
./scripts/bump-version.sh 0.0.4
```

Updates version in both:
- `Cargo.toml`
- `npm/package.json`

### Check version sync
```sh
./scripts/check-version-sync.sh
```

Verifies that Cargo.toml and npm/package.json have the same version.

## Release Workflow

```sh
# 1. Bump version
./scripts/bump-version.sh patch

# 2. Verify changes
git diff

# 3. Commit
git add -A
git commit -m 'chore: bump version to 0.0.4'

# 4. Tag and push
git tag -a v0.0.4 -m "Release v0.0.4"
git push --follow-tags
```

GitHub Actions will automatically:
- Build binaries for all platforms
- Create GitHub release
- Verify version sync
- Publish to npm
