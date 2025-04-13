# 🟣🔎 solscan-mcp: A Solscan API MCP Server in Rust

## Overview

A Model Context Protocol (MCP) server for interacting with the Solscan Pro API for Solana blockchain data. This server provides tools to query token information, account activities, and transaction details on the Solana blockchain via Large Language Models.


## Features

For detailed documentation of all available tools, see [TOOLS.md](TOOLS.md).

## Installation

Prerequisites:
- Rust toolchain (install via [rustup](https://rustup.rs/)) – for regular usage
- Docker – for Docker usage
- Solscan Pro API key. You can obtain one from [Solscan APIs](https://solscan.io/apis).

### Regular

```bash
cargo install solscan-mcp

where solscan-mcp # -> /Users/$username/.cargo/bin/solscan-mcp
```

Add the following to your `claude_desktop_config.json` or `claude_config.json`:

```json
{
  "mcpServers": {
    "solscan-mcp": {
      "command": "/Users/$username/.cargo/bin/solscan-mcp",
      "args": [],
      "env": {
        "SOLSCAN_API_KEY": "your_solscan_api_key"
      }
    }
  }
}
```

### Docker

WIP, will be available soon.



## Code quality Notes

I treat MCPs like useful scripts, as the structure of the APIs they rely on can always change. So, api.rs could definitely be split into multiple APIs based on Solscan namespaces. My main goal was to make it work and be easy to maintain, while ensuring errors are ignored without breaking the flow (unlike many MCPs I’ve tested in Python/TS, which crash painfully when they don’t gracefully handle simple errors).


## License

MIT
