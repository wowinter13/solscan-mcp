# 🟣🔎 solscan-mcp: A Solscan API MCP Server in Rust

## Overview

A Model Context Protocol (MCP) server for interacting with the Solscan Pro API for Solana blockchain data. This server provides tools to query token information, account activities, and transaction details on the Solana blockchain via Large Language Models.


## Examples

Simple prompts: 


https://github.com/user-attachments/assets/2f2586b2-ed9d-4d4d-bda4-0154b9b98cde




You can also use it for much more complex queries, like analyzing criminal activity combining multiple MCPs and correct context.

For example, upload a csv list of suspected addresses, then using perplexity-mcp research tool LLM will add info to context window on how investigators define criminal wallets based on their activity (mev, dusting, poisoning, sandwiched, etc) -> solscan-mcp will use this context to investigate the wallets and provide a report.

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

I treat MCPs like useful scripts, as the structure of the APIs they rely on can always change. So, api.rs could definitely be split into multiple APIs based on Solscan namespaces. My main goal was to make it work and be easy to maintain, while ensuring errors are ignored without breaking the flow (unlike many MCPs I've tested in Python/TS, which crash painfully when they don't gracefully handle simple errors).


## License

MIT
