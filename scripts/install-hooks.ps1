# Install Git hooks for the project

Write-Host "Installing Git hooks..."

# Ensure we are in a git repository
if (-not (Test-Path ".git" -PathType Container)) {
    Write-Error "Error: This script must be run from the root of the Git repository."
    exit 1
}

# Create hooks directory if it doesn't exist
$hooksDir = ".git/hooks"
if (-not (Test-Path $hooksDir)) {
    New-Item -ItemType Directory -Path $hooksDir -Force | Out-Null
}

# PowerShell script for pre-commit hook
$hookScriptContent = @"
# Pre-commit hook: Format code with nightly rustfmt
Write-Host "Running cargo +nightly fmt..."
cargo +nightly fmt --all

# Add formatted files back to staging
git add -u

exit 0
"@

# Create the PowerShell script
$hookScriptPath = ".git/hooks/pre-commit.ps1"
Set-Content -Path $hookScriptPath -Value $hookScriptContent -Encoding UTF8

# Create a wrapper script that Git can execute
$wrapperContent = @"
#!/bin/sh
pwsh -NoProfile -ExecutionPolicy Bypass -File .git/hooks/pre-commit.ps1
"@

$hookPath = ".git/hooks/pre-commit"
Set-Content -Path $hookPath -Value $wrapperContent -Encoding UTF8

Write-Host "✓ Git hooks installed successfully"
Write-Host "Pre-commit hook will automatically format code with rustfmt"
Write-Host ""
Write-Host "Note: Make sure PowerShell 7+ (pwsh) is installed and in your PATH"
