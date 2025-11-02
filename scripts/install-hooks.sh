#!/bin/bash
# Install Git hooks for the project

echo "Installing Git hooks..."

# Ensure we are in a git repository
if [ ! -d ".git" ]; then
    echo "Error: This script must be run from the root of the Git repository." >&2
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Create pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Pre-commit hook: Format code with nightly rustfmt

set -e

echo "Running cargo +nightly fmt..."
if ! cargo +nightly fmt --all; then
    echo "Error: cargo +nightly fmt failed. Please fix the errors and try again." >&2
    exit 1
fi

# Add formatted files back to staging
git add -u

exit 0
EOF

# Make executable
chmod +x .git/hooks/pre-commit

echo "✓ Git hooks installed successfully"
echo "Pre-commit hook will automatically format code with rustfmt"
