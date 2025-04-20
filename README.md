# AllDebrid CLI

A simple command-line tool to unlock links with the AllDebrid API.

## Features

- Unlock links via the AllDebrid API
- Extract direct download links from the API response
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

```bash
alldebrid-cli "http://example.com/somefile"
```

To include an optional password:

```bash
alldebrid-cli "http://example.com/somefile" "your_password"
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