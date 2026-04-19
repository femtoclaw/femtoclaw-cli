# 💻 FemtoClaw CLI: The Industrial Command Center

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Stable-green.svg)]()

The **FemtoClaw CLI** (`femtoclaw`) is the primary administrative and operational interface for the FemtoClaw industrial agent runtime. It provides a robust, terminal-native environment for managing autonomous reasoning loops, inspecting capability policies, and performing low-level system audits with deterministic execution authority.

Designed for systems engineers, security auditors, and automation developers, the CLI bridges the gap between probabilistic inference and physical system execution.

---

## 🚀 Key Features

### 1. Autonomous REPL
The core of the CLI is an interactive **Autonomous REPL**. Unlike standard chat interfaces, this REPL supports multi-step execution cycles where the agent can "think," execute tools, observe results, and iterate autonomously until a user-defined goal is achieved.

### 2. Durable Local Context
Leveraging the **Write-Ahead Log (WAL)**, the CLI ensures that interactive sessions are crash-resilient. History is persisted to local storage, allowing for state reconstruction across restarts and providing a local audit trail of all manual interactions.

### 3. Capability Inspection
Directly query the available "Claws" (capabilities) and their current authorization status. The CLI allows operators to verify what an agent *can* do before initiating a task.

### 4. Zero-Config Brain Hot-Swapping
Switch between inference providers (OpenAI, Anthropic, Ollama, etc.) using environment variables or runtime flags without modifying local configuration files.

---

## 🛠️ Installation & Setup

### From Source (Recommended)
```bash
git clone https://github.com/femtoclaw/femtoclaw
cd femtoclaw/femtoclaw-cli
cargo install --path .
```

### Verification
```bash
femtoclaw --version
```

---

## 📖 Command Reference

### `femtoclaw run`
Starts the interactive autonomous loop. This is the primary mode for general-purpose automation and experimentation.
- **Flags**:
  - `--brain <TYPE>`: Override the default brain (e.g., `openai`, `echo`).
  - `--iterations <LIMIT>`: Set a safety cap on the number of autonomous steps (default: 10).

### `femtoclaw once`
Executes a single instruction and terminates once the agent reaches a completion state. Ideal for CI/CD pipelines and script wrappers.
```bash
femtoclaw once --prompt "Check disk usage on /var/log and alert if > 90%."
```

### `femtoclaw replay`
Utility command to inspect and replay the local Write-Ahead Log.
```bash
# View the last 10 system mutations
femtoclaw replay --tail 10
```

---

## ⚙️ Configuration

The CLI searches for a configuration file at `~/.config/femtoclaw/config.toml`.

```toml
[brain]
backend = "openai"
model = "gpt-4o"

[runtime]
max_iterations = 15
memory_limit = 2000

[storage]
wal_enabled = true
path = "~/.local/share/femtoclaw/log.wal"
```

### Environment Variables
For headless or containerized environments, use:
- `FEMTO_BRAIN`: Set the inference provider.
- `FEMTO_OPENAI_API_KEY`: Authentication for OpenAI.
- `FEMTO_LOG_LEVEL`: Set to `debug` for full protocol validation traces.

---

## 🛡️ Security Posture
The CLI inherits all safety guarantees of the `femtoclaw` core runtime:
- **Strict JSON Enforcement**: Rejects any non-JSON output from the brain.
- **Argv Sandboxing**: Shell commands are executed as raw argument vectors, preventing shell-injection attacks common in natural language prompts.
- **Permission Mapping**: Every CLI-initiated action is subject to the local policy engine rules.

---

## 📄 Related Ecosystem
- **[femtoclaw-core](../femtoclaw)**: The underlying execution authority.
- **[femtoclaw-remote](../femtoclaw-remote)**: For exposing the CLI capabilities over a network.

Copyright © 2026 FemtoClaw Project.
