# Screenshot Automation

An automated screenshot capture tool for Windows that periodically takes screenshots while automatically sending specified arrow key inputs.

## Features

- 🖼️ Automatic screenshot capture (BMP format)
- ⌨️ Automated arrow key input (Left/Right)
- ⏱️ Customizable iteration count
- 🛑 Interruptible with Space or ESC key
- 📁 Customizable output directory and file naming

## Use Case

Automatically capture game screens while progressing through content using arrow key inputs.

## Requirements

- Windows OS
- Rust 1.70 or higher

## Installation

```bash
git clone https://github.com/MatsudaYoshio/screenshot-automation.git
cd screenshot-automation
cargo build --release
```

### For Developers

Before committing, format your code:

```powershell
cargo +nightly fmt
```

Or install Git hooks for automatic formatting:

For Windows PowerShell:
```powershell
# In PowerShell
.\scripts\install-hooks.ps1
```

For Git Bash or WSL:
```bash
# In Git Bash or WSL
./scripts/install-hooks.sh
```

## Usage

```bash
cargo run
```

Or use the compiled executable:

```bash
.\target\release\screenshot_automation.exe
```

### Execution Flow

1. **Configuration Input**
   - Select arrow key (Up/Down/Left/Right)
   - Specify output directory (default: `screenshots`)
   - Specify file name prefix (default: `screenshot`)
   - Specify maximum iteration count (default: 10)

2. **10-Second Countdown**
   - A 10-second countdown is displayed as preparation time
   - Use this time to focus on the target application

3. **Automation Execution**
   - Capture screenshot
   - Wait 1 second
   - Send specified arrow key
   - Wait 3 seconds
   - Proceed to next iteration

4. **Interruption**
   - Press Space or ESC key to interrupt

## Output

Screenshots are saved in the specified directory with the following format:

```
{prefix}_{00001}.bmp
{prefix}_{00002}.bmp
{prefix}_{00003}.bmp
...
```

Example: `screenshot_00001.bmp`, `screenshot_00002.bmp`

## Tech Stack

- **Rust** - Systems programming language
- **Windows API** - Screenshot capture and keyboard input
- **winapi crate** - Rust bindings for Windows API

## License

MIT License

## Notes

- This tool is Windows-only
- Screenshots are saved in BMP format (uncompressed)
- Administrator privileges are not required, but key sending may not work with some applications
