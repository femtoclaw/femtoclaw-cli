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
- **Brain Selection**: Choose between different brain backends (echo, openai, openrouter)
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
femtoclaw --config /etc/femtoclaw.toml

# With debug logging
femtoclaw --log-level debug

# Run a single command
femtoclaw run --prompt "Hello, world"

# Check status
femtoclaw status

# Initialize new configuration
femtoclaw init
```

## Windows How-To: Open Notepad With A Random Haiku (OpenRouter)

Use this flow on Windows PowerShell to have FemtoClaw generate a tool call that writes a random haiku to a temp file and opens it in Notepad.

### 1. Build the CLI

```powershell
cd d:\code\femtoclaw\femtoclaw-cli
cargo build --release
```

### 2. Set OpenRouter environment variables

```powershell
$env:FEMTO_OPENROUTER_API_KEY = "YOUR_OPENROUTER_API_KEY"
$env:FEMTO_OPENROUTER_MODEL = "openai/gpt-4.1-mini"
$env:FEMTO_OPENROUTER_TIMEOUT_SECS = "120"
```

Notes:
- `openai/gpt-4.1-mini` via OpenRouter is recommended for reliable JSON tool-call output.
- `arcee-ai/trinity-large-preview:free` is another available option
- `openrouter/free` may work but can be less consistent with strict tool format.

### 3. Set a strict prompt

```powershell
$prompt = 'Return ONLY tool_call JSON. The tool name MUST be exactly "shell". Use bin "powershell" and argv to: create 3 full haikus, pick one randomly, save it to Join-Path $env:TEMP ''femtoclaw_haiku.txt'', open notepad for that file, and output the selected haiku text.'
```

### 4. Run with safe PowerShell argument passing

```powershell
& ".\target\release\femtoclaw.exe" @('--brain','openrouter','run','--prompt',$prompt)
```

### 5. Verify output file

```powershell
Get-Content "$env:TEMP\femtoclaw_haiku.txt"
```

Troubleshooting:
- `unexpected argument ...`: use the array invocation form shown above (`@(...)`) so PowerShell does not split your prompt.
- `Capability denied: DENIED_CAPABILITY_NOT_FOUND`: ensure prompt says tool name must be exactly `shell`.
- `shell bin not allowed`: rebuild after updating allowlist in `femtoclaw/src/tools/shell.rs`.
- `arg too long`: update to latest code where shell arg length limit is increased.

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

- `FEMTO_BRAIN` - Brain backend (for example: echo, openai, openrouter)
- `FEMTO_OPENAI_API_KEY` - OpenAI API key
- `FEMTO_OPENAI_BASE_URL` - OpenAI-compatible API endpoint
- `FEMTO_OPENAI_MODEL` - OpenAI model name (default: gpt-4.1-mini)
- `FEMTO_OPENROUTER_API_KEY` - OpenRouter API key
- `FEMTO_OPENROUTER_MODEL` - OpenRouter model (for example: openai/gpt-4.1-mini)
- `FEMTO_OPENROUTER_TIMEOUT_SECS` - OpenRouter timeout in seconds (default: 180)

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
