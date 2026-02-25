# FemtoClaw CLI

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Stable-green.svg)]()

FemtoClaw Command-Line Interface — interactive CLI for the FemtoClaw Industrial Agent Runtime.

## Overview

`femtoclaw-cli` provides the command-line interface for the FemtoClaw Industrial Agent Runtime. It enables users to interact with the runtime directly from the terminal.

This is the primary entry point for local development and testing of FemtoClaw agents.

## Features

- **Interactive REPL**: Chat with the agent interactively
- **Configuration Management**: Flexible runtime configuration
- **Brain Selection**: Choose between different brain backends (echo, openai)
- **Logging Control**: Configurable log levels
- **Command Support**: Run, init, status commands

## Installation

```bash
# Build from source
cargo build -p femtoclaw-cli

# Or install
cargo install femtoclaw-cli
```

## Usage

```bash
# Start interactive REPL (default)
femtoclaw

# With specific brain
femtoclaw --brain openai

# With config file
femtoclawetc/femtoc --config /law.toml

# With debug logging
femtoclaw --log-level debug

# Run a single command
femtoclaw run --prompt "Hello, world"

# Check status
femtoclaw status

# Initialize new configuration
femtoclaw init
```

## Configuration

```toml
# ~/.config/femtoclaw/config.toml
[brain]
type = "openai"
model = "gpt-4.1-mini"

[logging]
level = "info"

[storage]
path = "/var/lib/femtoclaw"
```

## Environment Variables

- `FEMTO_BRAIN` — Brain backend (echo, openai)
- `FEMTO_OPENAI_API_KEY` — OpenAI API key
- `FEMTO_OPENAI_BASE_URL` — OpenAI-compatible API endpoint
- `FEMTO_OPENAI_MODEL` — Model name (default: gpt-4.1-mini)

## Architecture

```
┌─────────────────────────────────────────────┐
│           femtoclaw-cli                      │
│  ┌─────────────────────────────────────┐    │
│  │  CLI Parser (clap)                  │    │
│  │  - Arguments                        │    │
│  │  - Commands                         │    │
│  └─────────────────────────────────────┘    │
│  ┌─────────────────────────────────────┐    │
│  │  Command Handlers                   │    │
│  │  - run                              │    │
│  │  - init                             │    │
│  │  - status                           │    │
│  └─────────────────────────────────────┘    │
└─────────────────┬───────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────┐
│           femtoclaw-core                    │
│  - femtoclaw (runtime)                     │
│  - femtoclaw-protocol (validation)          │
│  - femtoclaw-policy (authorization)         │
│  - femtoclaw-audit (observability)          │
│  - femtoclaw-storage (persistence)          │
└─────────────────────────────────────────────┘
```

## Dependencies

- femtoclaw
- femtoclaw-protocol
- femtoclaw-policy
- femtoclaw-audit
- femtoclaw-storage
- clap 4.x (with derive)
- tokio 1.x
- tracing 0.1
- tracing-subscriber 0.3

## Related Crates

| Crate | Purpose |
|-------|---------|
| `femtoclaw` | Core runtime library |
| `femtoclaw-protocol` | Protocol validation |
| `femtoclaw-policy` | Authorization engine |
| `femtoclaw-audit` | Observability |
| `femtoclaw-storage` | Persistence |
| `femtoclaw-sdk` | Client SDK |
| `femtoclaw-remote` | API server |

## License

Copyright 2026 FemtoClaw

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
