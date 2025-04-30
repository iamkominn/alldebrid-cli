# AllDebrid CLI

A simple command-line tool to unlock links with the AllDebrid API.

## Features

- Unlock links via the AllDebrid API
- Extract direct download links from the API response
- Process multiple URLs from a file
- Cross-platform configuration with support for:
  - Environment variables
  - Local .env file
  - Platform-specific configuration directories

## Installation

1. Clone this repository
2. Build the project:

```bash
cargo build --release
```

3. Copy the binary from `target/release/alldebrid-cli` to a location in your PATH

## Usage

### Process a single URL

```bash
alldebrid-cli "http://example.com/somefile"
```

To include an optional password:

```bash
alldebrid-cli "http://example.com/somefile" "your_password"
```

### Process multiple URLs from a file

Create a text file with one URL per line, then:

```bash
alldebrid-cli -i urls.txt
```

With an optional password:

```bash
alldebrid-cli -i urls.txt "your_password"
```

The file can include comments (lines starting with #) and empty lines, which will be skipped.

### Help

```bash
alldebrid-cli --help
```

## Configuration

The application looks for your AllDebrid API key in the following locations (in order of priority):

1. Environment variable: `ALLDEBRID_API_KEY`
2. `.env` file in the current directory
3. Platform-specific configuration file:
   - **Linux**: `~/.config/alldebrid-cli/alldebrid-cli.conf`
   - **macOS**: `~/Library/Application Support/alldebrid-cli/alldebrid-cli.conf`
   - **Windows**: `C:\Users\Username\AppData\Roaming\alldebrid-cli\alldebrid-cli.conf`

### Creating a Configuration File

```bash
# Linux
mkdir -p ~/.config/alldebrid-cli/
echo "ALLDEBRID_API_KEY=your_api_key_here" > ~/.config/alldebrid-cli/alldebrid-cli.conf
chmod 600 ~/.config/alldebrid-cli/alldebrid-cli.conf

# macOS
mkdir -p ~/Library/Application\ Support/alldebrid-cli/
echo "ALLDEBRID_API_KEY=your_api_key_here" > ~/Library/Application\ Support/alldebrid-cli/alldebrid-cli.conf
chmod 600 ~/Library/Application\ Support/alldebrid-cli/alldebrid-cli.conf

# Windows (PowerShell)
New-Item -Path "$env:APPDATA\alldebrid-cli" -ItemType Directory -Force
Set-Content -Path "$env:APPDATA\alldebrid-cli\alldebrid-cli.conf" -Value "ALLDEBRID_API_KEY=your_api_key_here"
```

### Using Environment Variables

```bash
# Linux/macOS
export ALLDEBRID_API_KEY=your_api_key_here
alldebrid-cli "http://example.com/somefile"

# Windows
set ALLDEBRID_API_KEY=your_api_key_here
alldebrid-cli "http://example.com/somefile"
```

### Using .env File

Create a `.env` file in the same directory as the binary with the following content:

```
ALLDEBRID_API_KEY=your_api_key_here
```

## Cross-Compiling for Raspberry Pi

To build for Raspberry Pi, you can use the `cross` tool:

```bash
# Install cross
cargo install cross

# For Raspberry Pi 3/4 with 64-bit OS
cross build --target aarch64-unknown-linux-gnu --release

# For Raspberry Pi 2/3/4 with 32-bit OS
cross build --target armv7-unknown-linux-gnueabihf --release

# For Raspberry Pi 1/Zero
cross build --target arm-unknown-linux-gnueabihf --release
```

## Building from Source

Requirements:
- Rust toolchain (rustc, cargo)
- Internet connection for downloading dependencies

```bash
# Clone the repository
git clone https://github.com/yourusername/alldebrid-cli.git
cd alldebrid-cli

# Build in release mode
cargo build --release

# The binary will be in target/release/alldebrid-cli
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
