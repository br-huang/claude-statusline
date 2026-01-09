# Claude Statusline

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/yourusername/claude-statusline)](https://github.com/yourusername/claude-statusline/releases)


A high-performance, customizable statusline for [Claude Code](https://claude.ai/code) written in Rust.

 ## ✨ Features

  - **⚡ Blazing Fast** - Written in Rust, renders in <10ms
  - **🎨 Customizable Themes** - TOML-based theme configuration
  - **📊 Rich Information** - Model, tokens, cost, git status, duration
  - **🔌 Modular Segments** - Enable/disable segments as needed
  - **💾 Smart Caching** - 5-second cache for git operations
  - **🖥️ Cross-Platform** - Works on macOS, Linux, and Window

  ## 📦 Installation

  ### Quick Install (Recommended)

  ```bash
  curl -fsSL https://raw.githubusercontent.com/br-huang/claude-statusline/main/install.sh | bash

  From Source

```bash
  # Clone the repository
  git clone https://github.com/yourusername/claude-statusline.git
  cd claude-statusline

  # Build release version
  cargo build --release

  # Install to ~/.claude/bin
  mkdir -p ~/.claude/bin
  cp target/release/claude-statusline ~/.claude/bin/
```
  Using Cargo

```bash
  cargo install claude-statusline
```

## ⚙️ Configuration

Add to your ~/.claude/settings.json:

{
"statusLine": {
    "type": "command",
    "command": "~/.claude/bin/claude-statusline"
}
}

Then restart Claude Code to see your new statusline!

## 🔧 Usage

Command Line Options

claude-statusline [OPTIONS]

Options:
- -t, --theme <THEME>        Path to theme file
- -s, --segments <SEGMENTS>  Comma-separated list of segments to enable
- -d, --debug                Show render time
- -h, --help                 Print help
- -V, --version              Print version

### Examples

```bash
# Use minimal segments
claude-statusline --segments model,tokens,git

# Use custom theme
claude-statusline --theme ~/.config/claude-statusline/mytheme.toml

# Debug mode (show render time)
claude-statusline --debug
```

### 🎨 Themes

  Create a custom theme at ~/.config/claude-statusline/theme.toml:

```toml
theme_name = "my-awesome-theme"
separator = " | "
enabled_segments = ["model", "tokens", "cost", "git", "duration"]

[colors.model]
icon = "\x1b[95m"    # Bright Magenta
label = "\x1b[36m"   # Cyan
value = "\x1b[96m"   # Bright Cyan
reset = "\x1b[0m"

[colors.tokens]
icon = "\x1b[93m"    # Bright Yellow
label = "\x1b[33m"   # Yellow
value = "\x1b[93m"   # Bright Yellow
reset = "\x1b[0m"
```


## 📁 Project Structure

```
  claude-statusline/
  ├── src/
  │   ├── main.rs          # Entry point, CLI parsing
  │   ├── lib.rs           # Library exports
  │   ├── input.rs         # JSON input parsing
  │   ├── segment.rs       # Segment trait definition
  │   ├── theme.rs         # Theme system
  │   ├── config.rs        # Configuration loading
  │   ├── output.rs        # Output rendering
  │   └── segments/
  │       ├── mod.rs       # Segment registry
  │       ├── model.rs     # Model segment
  │       ├── tokens.rs    # Tokens segment
  │       ├── cost.rs      # Cost segment
  │       ├── duration.rs  # Duration segment
  │       └── git.rs       # Git segment (with caching)
  ├── themes/              # Theme files
  ├── Cargo.toml
  └── README.md
```

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (git checkout -b feature/amazing-feature)
3. Commit your changes (git commit -m 'Add amazing feature')
4. Push to the branch (git push origin feature/amazing-feature)
5. Open a Pull Request

Adding a New Segment

1. Create src/segments/mysegment.rs
2. Implement the Segment trait
3. Register in src/segments/mod.rs
4. Add to default theme


##  🗺️ Roadmap

- More built-in themes (Dracula, Nord, Solarized)
- Task progress segment
- Network latency segment
- Plugin system for custom segments
- Publish to crates.io

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- https://claude.ai/code - The amazing AI coding assistant
- https://starship.rs/ - Inspiration for the prompt design
- https://github.com/powerline/powerline - Original statusline concept