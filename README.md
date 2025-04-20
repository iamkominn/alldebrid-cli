# AllDebrid CLI

A simple command-line tool to unlock links with the AllDebrid API.

## Features

- Unlock links via the AllDebrid API
- Read API key securely from environment variables
- Extract direct download links from the API response

## Installation

1. Clone this repository
2. Copy `.env.example` to `.env` and add your AllDebrid API key
3. Build the project:

```bash
cargo build --release
```

## Usage

```bash
alldebrid-cli "http://example.com/somefile"
```

To include an optional password:

```bash
alldebrid-cli "http://example.com/somefile" "your_password"
```

## Environment Variables

Create a `.env` file in the project root with the following:

```
ALLDEBRID_API_KEY=your_api_key_here
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
