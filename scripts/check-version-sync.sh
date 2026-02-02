#!/bin/bash
# Check if Cargo.toml and npm/package.json versions are in sync

set -e

CARGO_VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
NPM_VERSION=$(jq -r .version npm/package.json)

echo "Cargo.toml version: $CARGO_VERSION"
echo "npm/package.json version: $NPM_VERSION"

MISMATCH=0

if [ "$CARGO_VERSION" != "$NPM_VERSION" ]; then
  echo "❌ Main package version mismatch!"
  MISMATCH=1
fi

# Check platform packages
for platform_dir in npm/platforms/*/; do
  if [ -f "${platform_dir}package.json" ]; then
    PLATFORM_VERSION=$(jq -r .version "${platform_dir}package.json")
    PLATFORM_NAME=$(basename "$platform_dir")
    echo "npm/platforms/$PLATFORM_NAME/package.json version: $PLATFORM_VERSION"

    if [ "$CARGO_VERSION" != "$PLATFORM_VERSION" ]; then
      echo "❌ Platform package $PLATFORM_NAME version mismatch!"
      MISMATCH=1
    fi
  fi
done

if [ $MISMATCH -eq 1 ]; then
  echo ""
  echo "❌ Version mismatch detected!"
  echo "   Run: ./scripts/bump-version.sh <version>"
  exit 1
fi

echo "✅ All versions are in sync"
