use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

use anyhow::Result;
use chrono::{DateTime, Utc};
use indicatif::{ProgressBar, ProgressStyle};

use crate::map::{FileNode, MatchedPattern, ScanMeta, SystemMap};
use crate::patterns::{
    self, default_collapse_patterns, default_ignore_patterns, default_purpose_patterns,
    extension_to_language, should_collapse, should_ignore, CollapsePattern,
};

use super::{detect_project_type, count_dir_contents};

/// Scanner configuration
pub struct ScannerConfig {
    /// Whether to show progress
    pub show_progress: bool,
    /// Maximum depth to scan
    pub max_depth: Option<usize>,
    /// Whether to respect gitignore
    pub respect_gitignore: bool,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            show_progress: true,
            max_depth: Some(20),
            respect_gitignore: true,
        }
    }
}

/// Scan a directory and build a SystemMap
pub fn scan_directory(root: &Path, config: &ScannerConfig) -> Result<SystemMap> {
    let start = Instant::now();
    let root = root.canonicalize()?;
    
    let collapse_patterns = default_collapse_patterns();
    let ignore_patterns = default_ignore_patterns();
    let purpose_patterns = default_purpose_patterns();
    let ext_to_lang = extension_to_language();

    // Set up progress bar
    let progress = if config.show_progress {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message("Scanning...");
        Some(pb)
    } else {
        None
    };

    let mut map = SystemMap::new(root.clone());
    let mut patterns_matched: Vec<MatchedPattern> = Vec::new();
    let mut total_files = 0usize;
    let mut total_dirs = 0usize;
    let mut indexed_files = 0usize;

    // Build the tree recursively
    let tree = scan_dir_recursive(
        &root,
        &root,
        &collapse_patterns,
        &ignore_patterns,
        &purpose_patterns,
        &ext_to_lang,
        &mut patterns_matched,
        &mut total_files,
        &mut total_dirs,
        &mut indexed_files,
        0,
        config.max_depth.unwrap_or(20),
        config.respect_gitignore,
        &progress,
    )?;

    if let Some(pb) = &progress {
        pb.finish_and_clear();
    }

    // Detect project type
    let project_type = detect_project_type(&root);

    map.tree = tree;
    map.project_type = project_type;
    map.patterns_matched = patterns_matched;
    map.meta = ScanMeta {
        total_files,
        indexed_files,
        total_dirs,
        scan_time_ms: start.elapsed().as_millis() as u64,
    };

    Ok(map)
}

fn scan_dir_recursive(
    path: &Path,
    root: &Path,
    collapse_patterns: &[CollapsePattern],
    ignore_patterns: &[&str],
    purpose_patterns: &[patterns::PurposePattern],
    ext_to_lang: &HashMap<&str, &str>,
    patterns_matched: &mut Vec<MatchedPattern>,
    total_files: &mut usize,
    total_dirs: &mut usize,
    indexed_files: &mut usize,
    depth: usize,
    max_depth: usize,
    respect_gitignore: bool,
    progress: &Option<ProgressBar>,
) -> Result<FileNode> {
    let dir_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "root".to_string());

    *total_dirs += 1;

    if let Some(pb) = progress {
        pb.set_message(format!("Scanning: {}", path.display()));
    }

    // Check if this directory should be collapsed
    if let Some(pattern) = should_collapse(&dir_name, path, collapse_patterns) {
        let (file_count, dir_count) = count_dir_contents(path);
        *total_files += file_count;
        
        patterns_matched.push(MatchedPattern {
            pattern: pattern.name.to_string(),
            path: path.strip_prefix(root).unwrap_or(path).to_path_buf(),
            files_collapsed: file_count,
            dirs_collapsed: dir_count,
        });

        return Ok(FileNode::Collapsed {
            name: dir_name,
            path: path.strip_prefix(root).unwrap_or(path).to_path_buf(),
            reason: pattern.reason.to_string(),
            file_count,
            dir_count,
        });
    }

    // Don't go deeper than max_depth
    if depth >= max_depth {
        let (file_count, dir_count) = count_dir_contents(path);
        return Ok(FileNode::Collapsed {
            name: dir_name,
            path: path.strip_prefix(root).unwrap_or(path).to_path_buf(),
            reason: "max depth reached".to_string(),
            file_count,
            dir_count,
        });
    }

    // Read directory contents
    let mut children = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .collect();
    
    // Sort entries for consistent output
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for entry in entries {
        let entry_path = entry.path();
        let entry_name = entry.file_name().to_string_lossy().to_string();

        // Skip ignored files
        if should_ignore(&entry_name, ignore_patterns) {
            continue;
        }

        // Skip hidden files (except specific important ones like .env.example)
        if entry_name.starts_with('.') 
            && !matches!(entry_name.as_str(), ".env.example" | ".gitignore" | ".dockerignore") 
        {
            // Still check for collapsible directories like .git, .venv
            if entry_path.is_dir() {
                if let Some(pattern) = should_collapse(&entry_name, &entry_path, collapse_patterns) {
                    let (file_count, dir_count) = count_dir_contents(&entry_path);
                    *total_files += file_count;
                    *total_dirs += 1;
                    
                    patterns_matched.push(MatchedPattern {
                        pattern: pattern.name.to_string(),
                        path: entry_path.strip_prefix(root).unwrap_or(&entry_path).to_path_buf(),
                        files_collapsed: file_count,
                        dirs_collapsed: dir_count,
                    });

                    children.push(FileNode::Collapsed {
                        name: entry_name,
                        path: entry_path.strip_prefix(root).unwrap_or(&entry_path).to_path_buf(),
                        reason: pattern.reason.to_string(),
                        file_count,
                        dir_count,
                    });
                }
            }
            continue;
        }

        if entry_path.is_dir() {
            let child = scan_dir_recursive(
                &entry_path,
                root,
                collapse_patterns,
                ignore_patterns,
                purpose_patterns,
                ext_to_lang,
                patterns_matched,
                total_files,
                total_dirs,
                indexed_files,
                depth + 1,
                max_depth,
                respect_gitignore,
                progress,
            )?;
            children.push(child);
        } else if entry_path.is_file() {
            *total_files += 1;
            *indexed_files += 1;

            // Get file metadata
            let metadata = entry_path.metadata().ok();
            let modified = metadata
                .as_ref()
                .and_then(|m| m.modified().ok())
                .map(|t| DateTime::<Utc>::from(t));

            // Detect language from extension
            let language = entry_path
                .extension()
                .and_then(|e| e.to_str())
                .and_then(|e| ext_to_lang.get(e))
                .map(|s| s.to_string());

            // Detect purpose
            let purpose = patterns::detect_purpose(&entry_name, purpose_patterns)
                .map(|s| s.to_string());

            // Count lines for code files
            let lines = if is_text_file(&entry_path) {
                count_lines(&entry_path).ok()
            } else {
                None
            };

            children.push(FileNode::File {
                name: entry_name,
                path: entry_path.strip_prefix(root).unwrap_or(&entry_path).to_path_buf(),
                lines,
                language,
                purpose,
                modified,
            });
        }
    }

    Ok(FileNode::Directory {
        name: dir_name,
        path: path.strip_prefix(root).unwrap_or(path).to_path_buf(),
        children,
    })
}

/// Check if a file is likely a text file based on extension
fn is_text_file(path: &Path) -> bool {
    let text_extensions = [
        "py", "rs", "js", "ts", "jsx", "tsx", "go", "java", "kt", "rb", "php",
        "c", "cpp", "cc", "h", "hpp", "cs", "swift", "scala", "clj", "ex", "exs",
        "erl", "hs", "lua", "r", "jl", "sql", "sh", "bash", "zsh", "fish", "ps1",
        "yaml", "yml", "toml", "json", "xml", "html", "css", "scss", "sass", "less",
        "md", "rst", "txt", "cfg", "ini", "conf", "env",
    ];

    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| text_extensions.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Count lines in a file
fn count_lines(path: &Path) -> Result<usize> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}
