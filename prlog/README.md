# prlog

**prlog** - is a lightweight and flexible Rust logging library that works on the principle of "write once, see everywhere". It outputs colored logs to the console and simultaneously stores them in a structured text format for further analysis.

## Content
* [Specifics](#specifics)
* [Quick Start](#quick-start)
* [CLI visualizer](#cli-visualizer)
* [Features](#features)
* [License](#license)

---

## Specifics

- **Two in one:** Output to `stdout` for development and to a file for history.
- **Native visualizer:** Special CLI utility `prlog-cli` for reading log files in a convenient format.
- **Export to JSON:** CLI can convert text logs to JSON array for integration with other tools (e.g. `jq`).
- **Modularity:** Change colors or write to file via Cargo functions to lighten binary.
- **Parsing safety:** Uses `|` delimiter, which allows `:` to be used freely in time and messages.

---

## Quick Start

### 1. Add a dependency
Add `prlog` to your `Cargo.toml`:

```toml
[dependencies]
prlog = "0.1.0"
log = "0.4"
```

### 2. Initialize in code
```rust
use prlog::Config;
use log::{info, warn, error};

fn main() {
    prlog::init(Config::default()).unwrap();

    info!(target: "main", "System enabled");
    warn!(target: "network", "Attempt to reconnect...");
    error!(target: "database", "Error with accessing db");
}
```

## CLI Visualizer
Instead of reading raw text in Notepad, use our built-in tool.

### Installing
```shell
cargo install prlog-cli
```

### Using
View with colors and filtering:

```shell
# Easy view
prlog-cli log.txt

# Filtering by specific module
prlog-cli log.txt --target network

# Export for analytic
prlog-cli app.prlog --json > report.json
```

## Features
You can customize the library to your needs:

* `default` - enable `colors` and `file-log`.
* `colors` - support for color output to the console.
* `file-log` - the ability to write logs to a file.

Example of a minimal version:
```toml
prlog = { version = "0.1.0", default-features = false }
```

## Format of logs
Logs are stored in `DSV` format with the separator `|`:
`LEVEL|TIMESTAMP|TARGET|MESSAGE`

Example:
`INFO|2026-05-09T12:00:00+03:00|main|Hello world`

## License
This project is licensed under the MIT license.