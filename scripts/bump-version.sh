#!/bin/bash
# Bump version in both Cargo.toml and npm/package.json

set -e

if [ -z "$1" ]; then
  echo "Usage: $0 <version|major|minor|patch>"
  echo ""
  echo "Examples:"
  echo "  $0 0.0.4       # Set explicit version"
  echo "  $0 patch       # Bump patch version (0.0.3 → 0.0.4)"
  echo "  $0 minor       # Bump minor version (0.0.3 → 0.1.0)"
  echo "  $0 major       # Bump major version (0.0.3 → 1.0.0)"
  exit 1
fi

VERSION_ARG="$1"

# Check if argument is a semantic keyword (major, minor, patch)
if [[ "$VERSION_ARG" =~ ^(major|minor|patch|premajor|preminor|prepatch|prerelease)$ ]]; then
  echo "Bumping $VERSION_ARG version..."

  # Use npm version to calculate new version
  cd npm
  NEW_VERSION=$(npm version "$VERSION_ARG" --no-git-tag-version | sed 's/^v//')
  cd ..

  echo "Calculated version: $NEW_VERSION"
else
  # Explicit version number provided
  NEW_VERSION="$VERSION_ARG"

  # Validate version format (e.g., 0.0.4, 1.2.3)
  if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?(\+[a-zA-Z0-9.]+)?$ ]]; then
    echo "Error: Invalid version format. Use semantic versioning (e.g., 0.0.4)"
    exit 1
  fi

  echo "Setting version to $NEW_VERSION..."

  # Update npm/package.json first
  cd npm
  npm version "$NEW_VERSION" --no-git-tag-version
  cd ..
fi

# Update Cargo.toml
echo "Updating Cargo.toml..."
if command -v cargo-set-version &> /dev/null; then
  cargo set-version "$NEW_VERSION"
else
  # Fallback to sed if cargo-edit is not installed
  sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
  rm Cargo.toml.bak
fi

echo "✓ Version bumped to $NEW_VERSION"
echo ""
echo "Next steps:"
echo "  1. Verify changes: git diff"
echo "  2. Commit: git add -A && git commit -m 'chore: bump version to $NEW_VERSION'"
echo "  3. Tag: git tag -a v$NEW_VERSION -m \"Release v$NEW_VERSION\""
echo "  4. Push: git push --follow-tags"
