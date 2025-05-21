# CLI Assistant 🮀🤖

> Smart developer assistant for analyzing, reviewing, generating and tracking code from terminal

---

## 📦 Features

* `analyze` — get suggestions for improvements from AI
* `review` — apply AI-powered fixes step-by-step (with diff)
* `generate` — generate Rust code snippets by prompt
* `track` — start/stop project time tracking (SQLite)
* `search` — fetch answers from StackOverflow
* `repl` — interactive session (multi-command mode)

---

## ⚙️ Requirements

* Rust ≥ 1.75
* Internet connection (for AI features)
* `.env` file in project root with:

```env
OPENAI_API_KEY=sk-...
```

---

## 🚀 Installation

```bash
git clone https://github.com/yourname/cli-assistant
cd cli-assistant
cargo build --release
```

Or install globally:

```bash
cargo install --path .
```

---

## ▶️ Usage Examples

### Analyze a file

```bash
cli-assistant analyze ./src/main.rs --flag fast
```

### Review with AI (loop + confirmation)

```bash
cli-assistant review ./src/main.rs --max-loop 3
```

### Dry run mode

```bash
cli-assistant review ./src/main.rs --dry-run
```

### Auto-apply without confirmation

```bash
cli-assistant review ./src/main.rs --no-confirm
```

### Save review diff to specific directory

```bash
cli-assistant review ./src/main.rs --output-dir ./review_logs
```

### Generate Rust code

```bash
cli-assistant generate "http server using hyper"
```

### Start interactive REPL

```bash
cli-assistant repl
```

---

## 🔍 Logging

Enable logging via environment variable:

```bash
RUST_LOG=info cli-assistant review ./src/main.rs
```

Or more fine-grained:

```bash
RUST_LOG=cli_assistant::commands::review=debug
```

---

## 📚 Dev Notes

* Uses `tracing` for logging
* Uses `dotenvy` for API keys
* Uses `mockito` for testing AI API client
* AI backed by OpenAI Chat Completions

---

## 🚧 Planned Features

* [ ] Git integration (hooks, diff parsing)
* [ ] Static analysis via Clippy
* [ ] Multi-language support
* [ ] VSCode extension
* [ ] Local AI model support (Ollama, LM Studio)

---

Happy hacking! 🚀
