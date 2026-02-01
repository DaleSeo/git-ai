#!/bin/bash
# Check if Cargo.toml and npm/package.json versions are in sync

set -e

CARGO_VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
NPM_VERSION=$(jq -r .version npm/package.json)

echo "Cargo.toml version: $CARGO_VERSION"
echo "npm/package.json version: $NPM_VERSION"

if [ "$CARGO_VERSION" != "$NPM_VERSION" ]; then
  echo ""
  echo "❌ Version mismatch detected!"
  echo "   Run: ./scripts/bump-version.sh <version>"
  exit 1
fi

echo "✅ Versions are in sync"
