# sysmap - System Map CLI Tool

## Project Overview

`sysmap` is a CLI tool that generates intelligent, compressed representations of project directories. It's designed to help AI agents (and humans) quickly understand project structure without reading every file.

**Core value proposition**: Reduce the "cold start" problem for AI coding assistants by providing pre-computed, pattern-aware project maps.

---

## CLI Interface

### Global Options

```
sysmap [OPTIONS] <COMMAND>

Options:
  -q, --quiet       Suppress non-essential output
  -v, --verbose     Increase output detail
  --no-color        Disable colored output
  -h, --help        Print help
  -V, --version     Print version
```

---

### Commands

#### `sysmap init`

Build initial map for a project directory.

```
sysmap init [OPTIONS] [PATH]

Arguments:
  [PATH]  Directory to map (default: current directory)

Options:
  --force           Overwrite existing .sysmap/ directory
  --no-gitignore    Don't respect .gitignore patterns
  --max-depth <N>   Maximum directory depth to scan (default: 20)
```

**Behavior**:
- Creates `.sysmap/` directory in project root
- Stores `map.json` (full map data)
- Stores `config.toml` (user-overridable patterns)
- Respects `.gitignore` by default
- Shows progress bar during scan

**Example output**:
```
Scanning /home/user/projects/flask-api...
  ├─ Detected: Python project (pyproject.toml)
  ├─ Files: 847 scanned, 34 indexed (813 collapsed/ignored)
  ├─ Patterns: .venv (3,421 files), __pycache__ (12 dirs)
  └─ Dependencies: 8 found in pyproject.toml

Map saved to .sysmap/map.json
Run 'sysmap summary' to view project overview.
```

---

#### `sysmap update`

Incrementally update an existing map.

```
sysmap update [OPTIONS]

Options:
  --full            Force full rebuild instead of incremental
```

**Behavior**:
- Checks file modification times against stored map
- Only re-processes changed files
- Updates dependency graph if relevant files changed
- Fast for small changes (sub-second typical)

**Example output**:
```
Updating map...
  ├─ Changed: 3 files
  ├─ Added: 1 file (src/routes/orders.py)
  ├─ Removed: 0 files
  └─ Updated in 0.12s
```

---

#### `sysmap summary`

Display compressed project overview.

```
sysmap summary [OPTIONS]

Options:
  --json            Output as JSON
  --yaml            Output as YAML
  --tokens          Show estimated token count
```

**Behavior**:
- Outputs human-readable summary by default
- Target: under 500 tokens for typical project
- Includes: project type, structure overview, key files, dependencies

**Example output (default)**:
```
Project: flask-api
Type: Python (Flask)
Root: /home/user/projects/flask-api

Structure:
  src/           12 Python files (1,847 lines)
  tests/         8 test files
  config.yaml    Application configuration
  .env.example   Environment template

Entry points:
  src/app.py     Flask application entry

Key directories:
  src/routes/    API endpoints (auth, users, products, health)
  src/models/    Database models (user, product, base)
  src/services/  Business logic (database, email)

Dependencies (from pyproject.toml):
  Flask, SQLAlchemy, pytest, python-dotenv, gunicorn

Collapsed (not indexed):
  .venv/         Python virtualenv (3,421 files)
  __pycache__/   Bytecode cache (12 directories)
  .git/          Git repository
```

**Example output (--json)**:
```json
{
  "name": "flask-api",
  "type": "python",
  "framework": "flask",
  "root": "/home/user/projects/flask-api",
  "structure": {
    "source_dirs": [
      {"path": "src/", "files": 12, "lines": 1847, "language": "python"}
    ],
    "test_dirs": [
      {"path": "tests/", "files": 8}
    ],
    "config_files": ["config.yaml", ".env.example"]
  },
  "entry_points": ["src/app.py"],
  "key_directories": [
    {"path": "src/routes/", "purpose": "API endpoints", "contents": ["auth", "users", "products", "health"]},
    {"path": "src/models/", "purpose": "Database models", "contents": ["user", "product", "base"]},
    {"path": "src/services/", "purpose": "Business logic", "contents": ["database", "email"]}
  ],
  "dependencies": {
    "source": "pyproject.toml",
    "packages": ["Flask", "SQLAlchemy", "pytest", "python-dotenv", "gunicorn"]
  },
  "collapsed": [
    {"path": ".venv/", "reason": "Python virtualenv", "file_count": 3421},
    {"path": "__pycache__/", "reason": "Bytecode cache", "dir_count": 12},
    {"path": ".git/", "reason": "Git repository"}
  ],
  "meta": {
    "indexed_files": 34,
    "total_files": 847,
    "last_updated": "2025-01-15T10:30:00Z"
  }
}
```

---

#### `sysmap tree`

Display directory tree with pattern awareness.

```
sysmap tree [OPTIONS] [PATH]

Arguments:
  [PATH]  Subdirectory to show (default: project root)

Options:
  -d, --depth <N>   Maximum depth (default: 3)
  -a, --all         Show collapsed directories expanded
  --files-only      Hide directories, show only files
  --dirs-only       Hide files, show only directories
```

**Behavior**:
- Shows tree with collapsed patterns indicated
- Annotates files with basic metadata (lines, purpose if detected)
- Respects map data for annotations

**Example output**:
```
flask-api/
├── src/
│   ├── __init__.py
│   ├── app.py (87 lines) [entry]
│   ├── routes/
│   │   ├── __init__.py (blueprint registration)
│   │   ├── auth.py (142 lines)
│   │   ├── users.py (98 lines)
│   │   ├── products.py (156 lines)
│   │   └── health.py (23 lines)
│   ├── models/
│   │   ├── __init__.py
│   │   ├── base.py (45 lines)
│   │   ├── user.py (89 lines)
│   │   └── product.py (67 lines)
│   └── services/
│       ├── database.py (120 lines)
│       └── email.py (78 lines)
├── tests/ (8 files)
├── config.yaml
├── pyproject.toml
├── .env.example
├── .venv/ [collapsed: virtualenv, 3421 files]
└── .git/ [collapsed: git]
```

---

#### `sysmap deps`

Show dependency relationships.

```
sysmap deps [OPTIONS] [FILE]

Arguments:
  [FILE]  File to analyze (optional, shows all if omitted)

Options:
  --reverse         Show what depends ON this file (reverse deps)
  --depth <N>       How many levels of dependencies (default: 1)
  --json            Output as JSON
```

**Behavior**:
- Parses imports/requires to build dependency graph
- For v1: focuses on internal project dependencies
- External packages shown separately

**Example output**:
```
$ sysmap deps src/routes/users.py

src/routes/users.py
  imports:
    ├── src/models/user.py
    ├── src/services/database.py
    └── (external) flask, sqlalchemy

$ sysmap deps src/models/user.py --reverse

src/models/user.py
  imported by:
    ├── src/routes/users.py
    ├── src/routes/auth.py
    └── tests/test_user.py
```

---

#### `sysmap find`

Search the map for files matching criteria.

```
sysmap find [OPTIONS] <QUERY>

Arguments:
  <QUERY>  Search term (filename, pattern, or keyword)

Options:
  -t, --type <TYPE>   Filter by file type (py, rs, js, etc.)
  -d, --dir <PATH>    Search within directory
  --json              Output as JSON
```

**Behavior**:
- Searches filenames and paths
- For v1: basic substring/glob matching
- Future: content keyword index

**Example output**:
```
$ sysmap find user

Found 5 matches:
  src/models/user.py         Model: User database model
  src/routes/users.py        Routes: User API endpoints
  tests/test_user.py         Tests: User model tests
  tests/test_user_routes.py  Tests: User route tests
  docs/user-guide.md         Docs: User documentation
```

---

## Data Model

### Stored Map Structure (`map.json`)

```json
{
  "version": "1.0",
  "root": "/absolute/path/to/project",
  "project_type": {
    "language": "python",
    "framework": "flask",
    "detected_from": ["pyproject.toml", "src/app.py"]
  },
  "scanned_at": "2025-01-15T10:30:00Z",
  "tree": {
    "type": "directory",
    "name": "flask-api",
    "children": [
      {
        "type": "directory",
        "name": "src",
        "children": [
          {
            "type": "file",
            "name": "app.py",
            "lines": 87,
            "language": "python",
            "purpose": "entry",
            "modified": "2025-01-14T08:00:00Z"
          }
        ]
      },
      {
        "type": "collapsed",
        "name": ".venv",
        "reason": "python_virtualenv",
        "file_count": 3421,
        "indicator": "pyvenv.cfg"
      }
    ]
  },
  "dependencies": {
    "internal": {
      "src/routes/users.py": ["src/models/user.py", "src/services/database.py"],
      "src/routes/auth.py": ["src/models/user.py", "src/services/database.py"]
    },
    "external": {
      "source": "pyproject.toml",
      "packages": ["Flask", "SQLAlchemy", "pytest"]
    }
  },
  "patterns_matched": [
    {"pattern": "python_virtualenv", "path": ".venv/", "files_collapsed": 3421},
    {"pattern": "pycache", "paths": ["src/__pycache__/", "tests/__pycache__/"]},
    {"pattern": "git", "path": ".git/"}
  ]
}
```

### Configuration (`config.toml`)

```toml
# .sysmap/config.toml
# User-overridable pattern configuration

[collapse]
# Patterns to collapse (don't index individual files)
# Format: name = { match = "...", reason = "...", indicator = "..." }

node_modules = { match = "node_modules", reason = "npm packages", indicator = "../package.json" }
__pycache__ = { match = "__pycache__", reason = "Python bytecode" }
".venv" = { match = ".venv", reason = "Python virtualenv", indicator = "pyvenv.cfg" }
venv = { match = "venv", reason = "Python virtualenv", indicator = "pyvenv.cfg" }
target = { match = "target", reason = "Rust build output", indicator = "../Cargo.toml" }
".git" = { match = ".git", reason = "Git internals" }
dist = { match = "dist", reason = "Build output" }
build = { match = "build", reason = "Build output" }

[ignore]
# Patterns to completely ignore (not shown at all)
".DS_Store" = {}
"*.pyc" = {}
"*.swp" = {}
"thumbs.db" = {}

[project_types]
# How to detect project types
# Format: name = { markers = [...], framework_hints = {...} }

[project_types.python]
markers = ["pyproject.toml", "setup.py", "requirements.txt", "Pipfile"]
framework_hints = { "flask" = "flask", "django" = "django", "fastapi" = "fastapi" }

[project_types.rust]
markers = ["Cargo.toml"]

[project_types.node]
markers = ["package.json"]
framework_hints = { "react" = "react", "vue" = "vue", "next" = "next" }

[project_types.go]
markers = ["go.mod"]

[purposes]
# File purpose detection patterns
# Format: pattern = "purpose_label"
"app.py" = "entry"
"main.py" = "entry"
"main.rs" = "entry"
"lib.rs" = "library"
"index.js" = "entry"
"index.ts" = "entry"
"*_test.py" = "test"
"test_*.py" = "test"
"*_test.go" = "test"
"*.spec.ts" = "test"
"*.test.ts" = "test"
```

---

## File Structure

After `sysmap init`, project structure:

```
project/
├── .sysmap/
│   ├── map.json        # Full map data
│   ├── config.toml     # Pattern configuration (user-editable)
│   └── cache/          # Incremental update cache (internal)
│       └── checksums   # File modification tracking
├── src/
│   └── ...
└── ...
```

Add to `.gitignore`:
```
.sysmap/cache/
```

(The map.json and config.toml can optionally be committed for team sharing)

---

## Implementation Phases

### Phase 1: Core Scanning (MVP)
- [ ] CLI skeleton with clap
- [ ] Directory walker with gitignore support
- [ ] Pattern matching for collapse/ignore
- [ ] Project type detection
- [ ] Basic tree output
- [ ] JSON map storage
- [ ] `init`, `tree`, `summary` commands

### Phase 2: Intelligence
- [ ] Dependency parsing (Python imports, Rust use, JS require/import)
- [ ] `deps` command
- [ ] File purpose detection
- [ ] Line counting by language
- [ ] `find` command

### Phase 3: Polish
- [ ] `update` command (incremental)
- [ ] Progress bars and better UX
- [ ] Colored output
- [ ] YAML output option
- [ ] Custom config support
- [ ] Error handling and edge cases

### Phase 4: Integration (Future)
- [ ] `watch` command (daemon mode)
- [ ] MCP server wrapper
- [ ] Content keyword indexing
- [ ] Cross-project maps

---

## Rust Crates

Core dependencies:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }      # CLI parsing
walkdir = "2"                                         # Directory traversal
ignore = "0.4"                                        # Gitignore support
serde = { version = "1", features = ["derive"] }     # Serialization
serde_json = "1"                                      # JSON
toml = "0.8"                                          # Config parsing
chrono = { version = "0.4", features = ["serde"] }   # Timestamps
colored = "2"                                         # Terminal colors
indicatif = "0.17"                                    # Progress bars

# Phase 2+
petgraph = "0.6"                                      # Dependency graph
regex = "1"                                           # Pattern matching
```

---

## Success Criteria

The tool is successful if:

1. `sysmap summary` produces output that fits in <500 tokens for typical projects
2. An AI agent given the summary can correctly identify where to add new code
3. `sysmap init` completes in <5 seconds for projects with <10,000 files
4. `sysmap update` completes in <1 second for small changes
5. Pattern collapse reduces indexed files by >80% for projects with large dependency directories

---

## Deferred Ideas (Tracked)

For potential future versions:

1. **Whole-system mapping**: Extend beyond single project to map user's entire development environment
2. **MCP server**: Expose sysmap as tools for AI agents to call directly
3. **Semantic search**: Embed file purposes/contents for natural language queries
4. **Learning/adaptive**: Track which files are actually accessed, weight importance
5. **Team sharing**: Sync maps across team members
6. **Content indexing**: Index keywords/symbols within files for deeper search
