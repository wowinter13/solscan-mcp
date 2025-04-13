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



### Docker



## Code quality Notes

I treat MCPs like useful scripts, as the structure of the APIs they rely on can always change. So, api.rs could definitely be split into multiple APIs based on Solscan namespaces. My main goal was to make it work and be easy to maintain, while ensuring errors are ignored without breaking the flow (unlike many MCPs I’ve tested in Python/TS, which crash painfully when they don’t gracefully handle simple errors).


## License

MIT
