#!/bin/bash
set -euo pipefail

# Usage: update_version.sh <new_version>
if [ -z "$1" ]; then
    echo "Usage: $0 <new_version>"
    echo "Example: $0 0.1.2"
    exit 1
fi

NEW_VERSION=$1
CARGO_TOML="Cargo.toml"

if [ ! -f "$CARGO_TOML" ]; then
    echo "Error: $CARGO_TOML file not found"
    exit 1
fi

echo "Updating version to: $NEW_VERSION ..."

# Replace version under [package] only
perl -i -0777 -pe "s/(\[package\]\n(?:.*\n)*?version\s*=\s*\")[^\"]*(\")/\${1}$NEW_VERSION\${2}/m" "$CARGO_TOML"

# Try to update CHANGELOG.md with git-cliff (if available as git subcommand)
if git cliff --version >/dev/null 2>&1; then
    git cliff --unreleased --tag "$NEW_VERSION" --prepend CHANGELOG.md || \
        echo "git-cliff ran but failed; skipping changelog update"
else
    echo "git-cliff not found; skipping automatic CHANGELOG.md update."
    echo "To generate changelog later run: git cliff --unreleased --tag $NEW_VERSION --prepend CHANGELOG.md"
fi

echo "Update completed!"
