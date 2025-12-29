# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-28

### Added

- **Core commands**
  - `sysmap init` - Initialize project mapping
  - `sysmap summary` - Display project overview (human-readable and JSON)
  - `sysmap tree` - Show directory tree with depth control
  - `sysmap find` - Search files with filters
  - `sysmap update` - Refresh existing map

- **Pattern recognition**
  - Automatic collapse of node_modules/, .venv/, __pycache__/, target/, .git/, dist/, build/
  - Multi-language detection (Python, Rust, JavaScript, TypeScript, Go, Java, Ruby)
  - Framework detection (Flask, Django, FastAPI, React, Vue, Next.js, Express, Actix, Rocket)
  - File purpose detection (entry points, modules, tests, config)

- **Find command filters**
  - -t, --type - Filter by file extension
  - -l, --lang - Filter by language  
  - -p, --purpose - Filter by purpose (entry, module, test, config)

- **Output options**
  - --json flag for machine-readable output
  - --no-color flag for plain text
  - -q quiet mode
  - -v verbose mode

- **Visual features**
  - Language-specific colors (Rust=brown, Python=green, JS=yellow, etc.)
  - Purpose-specific colors (entry=gold, module=cyan, test=magenta)
  - Line counts per file
  - Scan timing

## [Unreleased]

### Planned

- Dependency parsing (Cargo.toml, pyproject.toml, package.json)
- Config file support (.sysmap/config.toml)
- Token/character counting for AI context limits
- Data science patterns (notebooks, models, pipelines)
