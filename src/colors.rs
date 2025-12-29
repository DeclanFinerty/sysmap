use colored::{ColoredString, Colorize};

/// Get a colored string for a language name
pub fn colorize_language(lang: &str) -> ColoredString {
    match lang.to_lowercase().as_str() {
        "python" => lang.green(),
        "rust" => lang.truecolor(183, 65, 14), // Rust orange/brown
        "javascript" => lang.yellow(),
        "typescript" => lang.blue(),
        "java" => lang.red(),
        "c" => lang.truecolor(85, 85, 255), // Light blue
        "cpp" | "c++" => lang.truecolor(0, 89, 156), // Darker blue
        "go" => lang.cyan(),
        "ruby" => lang.truecolor(204, 52, 45), // Ruby red
        "php" => lang.truecolor(119, 123, 180), // PHP purple
        "swift" => lang.truecolor(255, 149, 0), // Swift orange
        "kotlin" => lang.truecolor(179, 129, 255), // Kotlin purple
        "r" => lang.truecolor(25, 118, 210), // R blue
        "haskell" => lang.truecolor(94, 80, 134), // Haskell purple
        "scala" => lang.red(),
        "perl" => lang.truecolor(57, 69, 126), // Perl blue
        "julia" => lang.truecolor(149, 88, 178), // Julia purple
        "shell" | "bash" | "zsh" => lang.truecolor(78, 154, 6), // Shell green
        "lua" => lang.truecolor(0, 0, 128), // Lua dark blue
        "elixir" => lang.truecolor(110, 74, 126), // Elixir purple
        "clojure" => lang.truecolor(99, 177, 42), // Clojure green
        "html" => lang.truecolor(227, 76, 38), // HTML orange
        "css" => lang.truecolor(38, 77, 228), // CSS blue
        "scss" | "sass" => lang.truecolor(205, 103, 153), // Sass pink
        "json" => lang.truecolor(250, 200, 50), // JSON yellow
        "yaml" | "yml" => lang.truecolor(203, 23, 30), // YAML red
        "toml" => lang.truecolor(156, 66, 33), // TOML brown
        "markdown" | "md" => lang.white(),
        "sql" => lang.truecolor(255, 160, 0), // SQL orange
        _ => lang.normal(),
    }
}

/// Get a colored string for a file purpose
pub fn colorize_purpose(purpose: &str) -> ColoredString {
    match purpose.to_lowercase().as_str() {
        "entry" => purpose.truecolor(255, 215, 0), // Gold for entry points
        "module" => purpose.cyan(),
        "test" => purpose.magenta(),
        "config" => purpose.truecolor(255, 165, 0), // Orange for config
        "library" => purpose.blue(),
        "init" => purpose.truecolor(128, 128, 128), // Gray for init files
        _ => purpose.normal(),
    }
}

/// List of all known purpose types (for future use)
#[allow(dead_code)]
pub fn known_purposes() -> Vec<&'static str> {
    vec!["entry", "module", "test", "config", "library", "init"]
}

/// Map file extension to language for filtering (for future use)
#[allow(dead_code)]
pub fn extension_to_language_name(ext: &str) -> Option<&'static str> {
    match ext.to_lowercase().as_str() {
        "py" => Some("python"),
        "rs" => Some("rust"),
        "js" => Some("javascript"),
        "ts" => Some("typescript"),
        "jsx" => Some("javascript"),
        "tsx" => Some("typescript"),
        "go" => Some("go"),
        "java" => Some("java"),
        "kt" => Some("kotlin"),
        "rb" => Some("ruby"),
        "php" => Some("php"),
        "c" => Some("c"),
        "cpp" | "cc" | "cxx" => Some("cpp"),
        "h" | "hpp" => Some("c"),
        "cs" => Some("csharp"),
        "swift" => Some("swift"),
        "scala" => Some("scala"),
        "clj" => Some("clojure"),
        "ex" | "exs" => Some("elixir"),
        "erl" => Some("erlang"),
        "hs" => Some("haskell"),
        "lua" => Some("lua"),
        "r" => Some("r"),
        "jl" => Some("julia"),
        "sql" => Some("sql"),
        "sh" | "bash" | "zsh" => Some("shell"),
        "ps1" => Some("powershell"),
        _ => None,
    }
}
