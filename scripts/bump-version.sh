#!/bin/bash
# Bump version in both Cargo.toml and npm/package.json

set -e

if [ -z "$1" ]; then
  echo "Usage: $0 <version>"
  echo "Example: $0 0.0.4"
  exit 1
fi

NEW_VERSION="$1"

# Validate version format (e.g., 0.0.4, 1.2.3)
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "Error: Invalid version format. Use semantic versioning (e.g., 0.0.4)"
  exit 1
fi

echo "Bumping version to $NEW_VERSION..."

# Update Cargo.toml
echo "Updating Cargo.toml..."
if command -v cargo-set-version &> /dev/null; then
  cargo set-version "$NEW_VERSION"
else
  # Fallback to sed if cargo-edit is not installed
  sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
  rm Cargo.toml.bak
fi

# Update npm/package.json
echo "Updating npm/package.json..."
cd npm
npm version "$NEW_VERSION" --no-git-tag-version
cd ..

echo "✓ Version bumped to $NEW_VERSION"
echo ""
echo "Next steps:"
echo "  1. Verify changes: git diff"
echo "  2. Commit: git add -A && git commit -m 'chore: bump version to $NEW_VERSION'"
echo "  3. Tag: git tag v$NEW_VERSION"
echo "  4. Push: git push && git push --tags"
