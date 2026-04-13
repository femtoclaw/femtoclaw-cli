# 💻 FemtoClaw CLI

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Stable-green.svg)]()

The **FemtoClaw CLI** is the primary administrative and development tool for the industrial agent runtime. It provides an interactive REPL for testing autonomous loops and a robust command-line interface for operational tasks.

---

## 🚀 Key Features

- **Autonomous REPL**: Directly interact with the agent's iterative reasoning loop.
- **Durable Local History**: Automatically persists execution history to local storage via WAL.
- **Brain Hot-Swapping**: Switch between brain backends (echo, openai, ollama) with zero reconfiguration.
- **Governance Tools**: Commands to inspect policies, capabilities, and audit logs.

---

## 🛠️ Usage

### 1. Launch the Autonomous REPL
```bash
femtoclaw run
```

### 2. Single-shot Execution (Once)
Execute a specific instruction and exit after the goal is reached or limits are hit.
```bash
femtoclaw once --prompt "Generate a system health report and count running processes."
```

---

## 📄 Related Crates
- **[femtoclaw](../femtoclaw)**: The core runtime engine.
- **[femtoclaw-remote](../femtoclaw-remote)**: Distributed coordination server.

Copyright © 2026 FemtoClaw Project.
