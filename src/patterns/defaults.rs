use std::collections::HashMap;

/// Pattern for directories that should be collapsed
#[derive(Debug, Clone)]
pub struct CollapsePattern {
    /// Name to match (exact match on directory name)
    pub name: &'static str,
    /// Human-readable reason for collapsing
    pub reason: &'static str,
    /// Optional file that must exist inside to confirm the pattern
    pub indicator: Option<&'static str>,
}

/// Pattern for detecting project types
#[derive(Debug, Clone)]
pub struct ProjectPattern {
    /// Language name
    pub language: &'static str,
    /// Files that indicate this project type (any match)
    pub markers: &'static [&'static str],
    /// Framework detection: (framework_name, package_name_to_look_for)
    pub frameworks: &'static [(&'static str, &'static str)],
}

/// Pattern for detecting file purposes
#[derive(Debug, Clone)]
pub struct PurposePattern {
    /// Glob-like pattern to match
    pub pattern: &'static str,
    /// Purpose label
    pub purpose: &'static str,
}

/// Default patterns for directories to collapse
pub fn default_collapse_patterns() -> Vec<CollapsePattern> {
    vec![
        CollapsePattern {
            name: "node_modules",
            reason: "npm packages",
            indicator: None,
        },
        CollapsePattern {
            name: ".venv",
            reason: "Python virtualenv",
            indicator: Some("pyvenv.cfg"),
        },
        CollapsePattern {
            name: "venv",
            reason: "Python virtualenv",
            indicator: Some("pyvenv.cfg"),
        },
        CollapsePattern {
            name: "env",
            reason: "Python virtualenv",
            indicator: Some("pyvenv.cfg"),
        },
        CollapsePattern {
            name: ".env",
            reason: "Python virtualenv",
            indicator: Some("pyvenv.cfg"),
        },
        CollapsePattern {
            name: "__pycache__",
            reason: "Python bytecode cache",
            indicator: None,
        },
        CollapsePattern {
            name: ".git",
            reason: "Git repository",
            indicator: None,
        },
        CollapsePattern {
            name: "target",
            reason: "Rust build output",
            indicator: None, // Could check for ../Cargo.toml
        },
        CollapsePattern {
            name: "dist",
            reason: "Build output",
            indicator: None,
        },
        CollapsePattern {
            name: "build",
            reason: "Build output",
            indicator: None,
        },
        CollapsePattern {
            name: ".next",
            reason: "Next.js build cache",
            indicator: None,
        },
        CollapsePattern {
            name: ".nuxt",
            reason: "Nuxt build cache",
            indicator: None,
        },
        CollapsePattern {
            name: "vendor",
            reason: "Vendored dependencies",
            indicator: None,
        },
        CollapsePattern {
            name: ".pytest_cache",
            reason: "Pytest cache",
            indicator: None,
        },
        CollapsePattern {
            name: ".mypy_cache",
            reason: "Mypy cache",
            indicator: None,
        },
        CollapsePattern {
            name: ".ruff_cache",
            reason: "Ruff cache",
            indicator: None,
        },
        CollapsePattern {
            name: "coverage",
            reason: "Test coverage data",
            indicator: None,
        },
        CollapsePattern {
            name: ".coverage",
            reason: "Coverage data",
            indicator: None,
        },
        CollapsePattern {
            name: "htmlcov",
            reason: "HTML coverage report",
            indicator: None,
        },
    ]
}

/// Default patterns for files/directories to completely ignore
pub fn default_ignore_patterns() -> Vec<&'static str> {
    vec![
        ".DS_Store",
        "Thumbs.db",
        "*.pyc",
        "*.pyo",
        "*.swp",
        "*.swo",
        "*~",
        ".*.swp",
    ]
}

/// Default patterns for detecting project types
pub fn default_project_patterns() -> Vec<ProjectPattern> {
    vec![
        ProjectPattern {
            language: "rust",
            markers: &["Cargo.toml"],
            frameworks: &[
                ("actix", "actix-web"),
                ("axum", "axum"),
                ("rocket", "rocket"),
                ("tauri", "tauri"),
            ],
        },
        ProjectPattern {
            language: "python",
            markers: &["pyproject.toml", "setup.py", "requirements.txt", "Pipfile"],
            frameworks: &[
                ("flask", "flask"),
                ("django", "django"),
                ("fastapi", "fastapi"),
                ("streamlit", "streamlit"),
                ("pytorch", "torch"),
                ("tensorflow", "tensorflow"),
            ],
        },
        ProjectPattern {
            language: "javascript",
            markers: &["package.json"],
            frameworks: &[
                ("react", "react"),
                ("vue", "vue"),
                ("angular", "@angular/core"),
                ("next", "next"),
                ("express", "express"),
                ("nest", "@nestjs/core"),
            ],
        },
        ProjectPattern {
            language: "typescript",
            markers: &["tsconfig.json"],
            frameworks: &[], // Inherits from JS detection
        },
        ProjectPattern {
            language: "go",
            markers: &["go.mod"],
            frameworks: &[
                ("gin", "github.com/gin-gonic/gin"),
                ("fiber", "github.com/gofiber/fiber"),
            ],
        },
        ProjectPattern {
            language: "java",
            markers: &["pom.xml", "build.gradle", "build.gradle.kts"],
            frameworks: &[
                ("spring", "spring-boot"),
                ("quarkus", "quarkus"),
            ],
        },
        ProjectPattern {
            language: "ruby",
            markers: &["Gemfile"],
            frameworks: &[
                ("rails", "rails"),
                ("sinatra", "sinatra"),
            ],
        },
    ]
}

/// Default patterns for detecting file purposes
pub fn default_purpose_patterns() -> Vec<PurposePattern> {
    vec![
        // Entry points
        PurposePattern { pattern: "main.py", purpose: "entry" },
        PurposePattern { pattern: "app.py", purpose: "entry" },
        PurposePattern { pattern: "__main__.py", purpose: "entry" },
        PurposePattern { pattern: "main.rs", purpose: "entry" },
        PurposePattern { pattern: "lib.rs", purpose: "library" },
        PurposePattern { pattern: "index.js", purpose: "entry" },
        PurposePattern { pattern: "index.ts", purpose: "entry" },
        PurposePattern { pattern: "main.go", purpose: "entry" },
        
        // Tests
        PurposePattern { pattern: "test_*.py", purpose: "test" },
        PurposePattern { pattern: "*_test.py", purpose: "test" },
        PurposePattern { pattern: "*_test.rs", purpose: "test" },
        PurposePattern { pattern: "*_test.go", purpose: "test" },
        PurposePattern { pattern: "*.test.js", purpose: "test" },
        PurposePattern { pattern: "*.test.ts", purpose: "test" },
        PurposePattern { pattern: "*.spec.js", purpose: "test" },
        PurposePattern { pattern: "*.spec.ts", purpose: "test" },
        
        // Config
        PurposePattern { pattern: "config.py", purpose: "config" },
        PurposePattern { pattern: "settings.py", purpose: "config" },
        PurposePattern { pattern: "config.js", purpose: "config" },
        PurposePattern { pattern: "config.ts", purpose: "config" },
        
        // Init files
        PurposePattern { pattern: "__init__.py", purpose: "init" },
        PurposePattern { pattern: "mod.rs", purpose: "module" },
    ]
}

/// Language detection by file extension
pub fn extension_to_language() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("py", "python");
    map.insert("rs", "rust");
    map.insert("js", "javascript");
    map.insert("ts", "typescript");
    map.insert("jsx", "javascript");
    map.insert("tsx", "typescript");
    map.insert("go", "go");
    map.insert("java", "java");
    map.insert("kt", "kotlin");
    map.insert("rb", "ruby");
    map.insert("php", "php");
    map.insert("c", "c");
    map.insert("cpp", "cpp");
    map.insert("cc", "cpp");
    map.insert("h", "c");
    map.insert("hpp", "cpp");
    map.insert("cs", "csharp");
    map.insert("swift", "swift");
    map.insert("scala", "scala");
    map.insert("clj", "clojure");
    map.insert("ex", "elixir");
    map.insert("exs", "elixir");
    map.insert("erl", "erlang");
    map.insert("hs", "haskell");
    map.insert("lua", "lua");
    map.insert("r", "r");
    map.insert("jl", "julia");
    map.insert("sql", "sql");
    map.insert("sh", "shell");
    map.insert("bash", "shell");
    map.insert("zsh", "shell");
    map.insert("fish", "shell");
    map.insert("ps1", "powershell");
    map.insert("yaml", "yaml");
    map.insert("yml", "yaml");
    map.insert("toml", "toml");
    map.insert("json", "json");
    map.insert("xml", "xml");
    map.insert("html", "html");
    map.insert("css", "css");
    map.insert("scss", "scss");
    map.insert("sass", "sass");
    map.insert("less", "less");
    map.insert("md", "markdown");
    map.insert("rst", "rst");
    map.insert("txt", "text");
    map
}
