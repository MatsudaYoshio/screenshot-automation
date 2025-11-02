#!/bin/bash
# Install Git hooks for the project

echo "Installing Git hooks..."

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Create pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Pre-commit hook: Format code with nightly rustfmt

echo "Running cargo +nightly fmt..."
cargo +nightly fmt --all

# Add formatted files back to staging
git add -u

exit 0
EOF

# Make executable
chmod +x .git/hooks/pre-commit

echo "✓ Git hooks installed successfully"
echo "Pre-commit hook will automatically format code with rustfmt"
