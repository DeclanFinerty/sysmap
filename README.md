# sysmap

Intelligent project mapping for AI agents and humans.

`sysmap` generates compressed, pattern-aware representations of project directories. It helps AI coding assistants (and humans) quickly understand project structure without reading every file.

## Features

- **Pattern collapse**: Recognizes `node_modules/`, `.venv/`, `__pycache__/`, `target/`, `.git/` and collapses them
- **Multi-language detection**: Identifies all languages in a project (Python, Rust, JavaScript, TypeScript, Go, Java, Ruby)
- **Framework detection**: Detects Flask, Django, FastAPI, React, Vue, Next.js, and more
- **File purpose detection**: Identifies entry points, tests, config files, modules
- **Line counting**: Shows lines of code per file and directory
- **JSON output**: Machine-readable format for AI consumption
- **Fast**: Scans thousands of files in milliseconds

## Installation

```bash
cargo install sysmap
```

Or build from source:

```bash
git clone https://github.com/yourusername/sysmap
cd sysmap
cargo build --release
```

## Quick Start

```bash
# Initialize map in current directory
sysmap init

# View project summary
sysmap summary

# View directory tree
sysmap tree

# Search for files
sysmap find user

# Update after changes
sysmap update
```

## Commands

### `sysmap init [PATH]`

Build initial map for a project directory.

```bash
sysmap init              # Map current directory
sysmap init ./my-project # Map specific directory
sysmap init --force      # Overwrite existing map
```

### `sysmap summary`

Display compressed project overview.

```bash
sysmap summary         # Human-readable output
sysmap summary --json  # JSON for AI agents
```

Example output:
```
Project: flask-api
Type: Python (Flask)

Structure:
  src/           12 python files (1,847 lines)
  tests/         8 test files
  Config:        config.yaml, pyproject.toml

Key directories:
  src/routes/    auth, users, products, health
  src/models/    user, product, base
  src/services/  database, email

Collapsed:
  .venv/         Python virtualenv (3,421 files)
```

### `sysmap tree [PATH]`

Display directory tree with pattern awareness.

```bash
sysmap tree            # Full project tree
sysmap tree src/       # Subtree only
sysmap tree -d 2       # Limit depth
```

### `sysmap find <QUERY>`

Search the map for files.

```bash
sysmap find user           # Search by name
sysmap find user -t py     # Filter by file type
sysmap find config -t json # Find JSON config files
```

### `sysmap update`

Incrementally update an existing map.

```bash
sysmap update       # Update map
sysmap update --full # Force full rebuild
```

## How It Works

### Pattern Recognition

sysmap recognizes common project patterns and collapses them:

| Pattern | Action |
|---------|--------|
| `node_modules/` | Collapse (npm packages) |
| `.venv/`, `venv/` | Collapse (Python virtualenv) |
| `__pycache__/` | Collapse (Python bytecode) |
| `target/` | Collapse (Rust build output) |
| `.git/` | Collapse (Git internals) |
| `dist/`, `build/` | Collapse (Build output) |

### Project Detection

Automatically detects project type from marker files:

- **Rust**: `Cargo.toml`
- **Python**: `pyproject.toml`, `setup.py`, `requirements.txt`
- **JavaScript/TypeScript**: `package.json`, `tsconfig.json`
- **Go**: `go.mod`
- **Java**: `pom.xml`, `build.gradle`
- **Ruby**: `Gemfile`

### File Purposes

Identifies common file purposes:

- Entry points: `main.py`, `app.py`, `index.js`, `main.rs`
- Tests: `test_*.py`, `*_test.rs`, `*.spec.ts`
- Config: `config.yaml`, `settings.py`
- Modules: `mod.rs`, `__init__.py`

## Use with AI Agents

The `--json` output is designed for AI consumption:

```bash
# Copy to clipboard for pasting to AI
sysmap summary --json | clip

# Or use in scripts
sysmap summary --json > project-context.json
```

## Data Storage

After `sysmap init`, a `.sysmap/` directory is created:

```
.sysmap/
└── map.json    # Full project map
```

The `map.json` contains the complete file tree with metadata. The summary command generates a compressed view from this data.

## Development

```bash
# Build
cargo build --release

# Run tests
cargo test

# Run test script (Windows PowerShell)
.\test.ps1
```

## License

MIT
