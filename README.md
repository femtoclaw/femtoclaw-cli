# FemtoClaw CLI

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Stable-green.svg)]()

FemtoClaw Command-Line Interface — interactive tool for the industrial agent runtime.

## Overview

`femtoclaw-cli` is the primary entry point for local development, testing, and operational management of FemtoClaw agents. It provides an interactive REPL and single-shot execution commands.

## Features

- **Autonomous REPL**: Chat with the agent and observe multi-step autonomous loops.
- **Durable History**: Automatically persists execution history to local storage.
- **Flexible Configuration**: Switch brains, adjust safety limits, and manage policies.
- **Enterprise-ready CLI**: Built with `clap` for a robust, discoverable interface.

## Installation

```bash
cargo install femtoclaw-cli
```

## Usage

### 1. Start the Autonomous REPL
Enter the interactive loop where the agent can think and execute tools autonomously.
```bash
femtoclaw run
```

### 2. Single-shot Execution
Send a prompt and wait for the final response (the agent will still loop internally if needed).
```bash
femtoclaw once --prompt "List all files in the current directory and count them."
```

### 3. Change Brain Backend
```bash
export FEMTO_BRAIN=openai
femtoclaw run
```

## Related Crates
- `femtoclaw`: Core runtime library.
- `femtoclaw-remote`: Distributed cluster support and API server.

## License
Apache 2.0 — see [LICENSE](LICENSE).
