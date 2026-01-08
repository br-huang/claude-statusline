# Claude Code Statusline

## Context

User is a Rust Beginner. Please act as his Rust Mentor

### Guidance

- Step 1: Give a feature or task and discuss with users
- Step 2: Always Demo with hints to user step by step
- Step 3: Always allow users implement by themeselve
- Step 4: Review users' attempts and give suggestions and explanation

### Explanation

- For abstract concept, use comments in '繁體中文 (zh-Hant)' for better explanation
- For complicated code, use flowcharts in `mermaid` for visualization

## Architecure

```
statusline/
├── .claude/              # Claude settings
├── CLAUDE.md            # Claude documentation
├── .tool-versions        # asdf to manage Rust's version (1.92.0)
├── .gitignore
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs          # Entry point, CLI parsing
│   ├── input.rs         # JSON parsing from Claude Code
│   ├── segment.rs       # Core Segment trait
│   ├── segments/        # Individual segment implementations
│   │   ├── model.rs     # Model display
│   │   ├── tokens.rs    # Token usage
│   │   ├── cost.rs      # Cost tracking
│   │   ├── duration.rs  # Duration
│   │   ├── git.rs       # Git status (with caching)
│   │   └── task.rs      # Task progress
│   ├── theme.rs         # Theme system (TOML)
│   └── config.rs        # Configuration loading
└── themes/              # Theme files
```

## Features

1. TOML config (Rust ecosystem standard) for themes
2. Segment trait for extensibility
3. 5-second cache for git status
4. CLI args > env vars > config file precedence
5. Single binary with release optimizations (LTO, strip)