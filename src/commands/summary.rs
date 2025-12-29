use std::collections::HashMap;
use std::env;

use anyhow::Result;
use colored::Colorize;

use crate::config::{find_sysmap_root, map_path};
use crate::map::{FileNode, SystemMap};

/// Execute the summary command
pub fn execute(json: bool, yaml: bool) -> Result<()> {
    let cwd = env::current_dir()?;
    
    let root = find_sysmap_root(&cwd)
        .ok_or_else(|| anyhow::anyhow!(
            "No sysmap found. Run 'sysmap init' first."
        ))?;

    let map = SystemMap::load(&map_path(&root))?;

    if json {
        print_json_summary(&map)?;
    } else if yaml {
        print_yaml_summary(&map)?;
    } else {
        print_human_summary(&map);
    }

    Ok(())
}

fn print_human_summary(map: &SystemMap) {
    // Project header
    let project_name = map.root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Build type string - show all languages if multiple
    let project_type = if map.project_type.languages.is_empty() {
        "Unknown".to_string()
    } else if map.project_type.languages.len() > 1 {
        let langs: Vec<String> = map.project_type.languages.iter()
            .map(|l| capitalize(l))
            .collect();
        let type_str = langs.join(", ");
        match &map.project_type.framework {
            Some(fw) => format!("{} ({})", type_str, capitalize(fw)),
            None => type_str,
        }
    } else {
        let lang = &map.project_type.languages[0];
        match &map.project_type.framework {
            Some(fw) => format!("{} ({})", capitalize(lang), capitalize(fw)),
            None => capitalize(lang),
        }
    };

    println!("{} {}", "Project:".bold(), project_name);
    println!("{} {}", "Type:".bold(), project_type);
    println!();

    // Analyze structure
    let analysis = analyze_tree(&map.tree);

    // Structure section
    println!("{}", "Structure:".bold());
    
    // Show source directories with their subdirectories
    for (dir_name, stats) in &analysis.source_dirs {
        println!("  {:<14} {} {} files{}",
            format!("{}/", dir_name),
            stats.file_count.to_string().yellow(),
            stats.primary_language.as_deref().unwrap_or(""),
            stats.lines.map(|l| format!(" ({} lines)", l)).unwrap_or_default()
        );
    }

    // Show test directories
    for (dir_name, stats) in &analysis.test_dirs {
        println!("  {:<14} {} test files",
            format!("{}/", dir_name),
            stats.file_count.to_string().yellow()
        );
    }

    // Show config files
    if !analysis.config_files.is_empty() {
        println!("  {:<14} {}",
            "Config:",
            analysis.config_files.join(", ")
        );
    }

    // Entry points
    if !analysis.entry_points.is_empty() {
        println!();
        println!("{}", "Entry points:".bold());
        for entry in &analysis.entry_points {
            println!("  {}", entry);
        }
    }

    // Key directories with interesting contents
    if !analysis.key_dirs.is_empty() {
        println!();
        println!("{}", "Key directories:".bold());
        for (dir_path, contents) in &analysis.key_dirs {
            println!("  {:<14} {}",
                format!("{}/", dir_path),
                contents.join(", ")
            );
        }
    }

    // Dependencies
    if !analysis.dependencies.is_empty() {
        println!();
        println!("{}", "Dependencies:".bold());
        println!("  {}", analysis.dependencies.join(", "));
    }
    
    // File types and purposes (metadata)
    if !analysis.purposes_found.is_empty() || !analysis.languages_found.is_empty() {
        println!();
        println!("{}", "File metadata:".bold());
        if !analysis.purposes_found.is_empty() {
            println!("  {:<14} {}",
                "Purposes:",
                analysis.purposes_found.join(", ")
            );
        }
        if !analysis.languages_found.is_empty() {
            println!("  {:<14} {}",
                "Languages:",
                analysis.languages_found.join(", ")
            );
        }
    }

    // Collapsed directories
    if !map.patterns_matched.is_empty() {
        println!();
        println!("{}", "Collapsed:".bold().dimmed());
        for pattern in &map.patterns_matched {
            println!("  {:<14} {} ({} files)",
                format!("{}/", pattern.path.display()).dimmed(),
                pattern.pattern.dimmed(),
                pattern.files_collapsed.to_string().dimmed()
            );
        }
    }
}

fn print_json_summary(map: &SystemMap) -> Result<()> {
    let analysis = analyze_tree(&map.tree);
    
    let summary = serde_json::json!({
        "name": map.root.file_name().map(|n| n.to_string_lossy().to_string()),
        "languages": map.project_type.languages,
        "framework": map.project_type.framework,
        "structure": {
            "source_dirs": analysis.source_dirs.iter().map(|(name, stats)| {
                serde_json::json!({
                    "path": name,
                    "files": stats.file_count,
                    "lines": stats.lines,
                    "language": stats.primary_language
                })
            }).collect::<Vec<_>>(),
            "test_dirs": analysis.test_dirs.iter().map(|(name, stats)| {
                serde_json::json!({
                    "path": name,
                    "files": stats.file_count
                })
            }).collect::<Vec<_>>(),
            "config_files": analysis.config_files
        },
        "entry_points": analysis.entry_points,
        "key_directories": analysis.key_dirs.iter().map(|(path, contents)| {
            serde_json::json!({
                "path": path,
                "contents": contents
            })
        }).collect::<Vec<_>>(),
        "dependencies": {
            "packages": analysis.dependencies
        },
        "collapsed": map.patterns_matched.iter().map(|p| {
            serde_json::json!({
                "path": p.path,
                "reason": p.pattern,
                "file_count": p.files_collapsed
            })
        }).collect::<Vec<_>>(),
        "meta": {
            "indexed_files": map.meta.indexed_files,
            "total_files": map.meta.total_files,
            "last_updated": map.scanned_at,
            "purposes_found": analysis.purposes_found,
            "file_languages": analysis.languages_found
        }
    });

    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

fn print_yaml_summary(map: &SystemMap) -> Result<()> {
    // For now, just output JSON - YAML support can be added later
    // This keeps dependencies minimal for MVP
    eprintln!("{}", "YAML output not yet implemented, showing JSON:".yellow());
    print_json_summary(map)
}

// ============ Analysis helpers ============

struct DirStats {
    file_count: usize,
    lines: Option<usize>,
    primary_language: Option<String>,
}

struct TreeAnalysis {
    source_dirs: Vec<(String, DirStats)>,
    test_dirs: Vec<(String, DirStats)>,
    config_files: Vec<String>,
    entry_points: Vec<String>,
    key_dirs: Vec<(String, Vec<String>)>,
    dependencies: Vec<String>,
    purposes_found: Vec<String>,
    languages_found: Vec<String>,
}

fn analyze_tree(tree: &FileNode) -> TreeAnalysis {
    let mut analysis = TreeAnalysis {
        source_dirs: Vec::new(),
        test_dirs: Vec::new(),
        config_files: Vec::new(),
        entry_points: Vec::new(),
        key_dirs: Vec::new(),
        dependencies: Vec::new(),
        purposes_found: Vec::new(),
        languages_found: Vec::new(),
    };

    // Known source directory names
    let source_dir_names = ["src", "lib", "app", "pkg", "internal", "cmd"];
    let test_dir_names = ["tests", "test", "spec", "__tests__"];
    let config_extensions = ["yaml", "yml", "toml", "json", "ini", "cfg"];
    let config_names = ["config", "settings", ".env.example", "Makefile", "Dockerfile"];
    
    // Collect all purposes and languages
    collect_metadata(tree, &mut analysis.purposes_found, &mut analysis.languages_found);

    if let FileNode::Directory { children, .. } = tree {
        for child in children {
            match child {
                FileNode::Directory { name, children: dir_children, .. } => {
                    let stats = compute_dir_stats(dir_children);

                    if source_dir_names.contains(&name.as_str()) {
                        analysis.source_dirs.push((name.clone(), stats));
                        
                        // Look for key subdirectories
                        let key_subdirs = find_key_subdirs(dir_children);
                        if !key_subdirs.is_empty() {
                            for (subdir_name, contents) in key_subdirs {
                                analysis.key_dirs.push((
                                    format!("{}/{}", name, subdir_name),
                                    contents
                                ));
                            }
                        }
                    } else if test_dir_names.contains(&name.as_str()) {
                        analysis.test_dirs.push((name.clone(), stats));
                    }
                }
                FileNode::File { name, purpose, .. } => {
                    // Check for config files
                    let is_config = config_names.iter().any(|c| name.contains(c))
                        || name.split('.').last()
                            .map(|ext| config_extensions.contains(&ext))
                            .unwrap_or(false);
                    
                    if is_config && !name.starts_with('.') {
                        analysis.config_files.push(name.clone());
                    }

                    // Check for entry points
                    if purpose.as_deref() == Some("entry") {
                        analysis.entry_points.push(name.clone());
                    }

                    // Parse dependency files
                    if name == "pyproject.toml" || name == "requirements.txt" {
                        // Would parse dependencies here in a full implementation
                    } else if name == "package.json" {
                        // Would parse dependencies here
                    } else if name == "Cargo.toml" {
                        // Would parse dependencies here
                    }
                }
                _ => {}
            }
        }
    }

    // Sort for consistent output
    analysis.config_files.sort();

    analysis
}

fn compute_dir_stats(children: &[FileNode]) -> DirStats {
    let mut file_count = 0;
    let mut total_lines = 0;
    let mut lang_counts: HashMap<String, usize> = HashMap::new();

    count_recursive(children, &mut file_count, &mut total_lines, &mut lang_counts);

    let primary_language = lang_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(lang, _)| lang);

    DirStats {
        file_count,
        lines: if total_lines > 0 { Some(total_lines) } else { None },
        primary_language,
    }
}

fn count_recursive(
    nodes: &[FileNode],
    file_count: &mut usize,
    total_lines: &mut usize,
    lang_counts: &mut HashMap<String, usize>,
) {
    for node in nodes {
        match node {
            FileNode::File { lines, language, .. } => {
                *file_count += 1;
                if let Some(l) = lines {
                    *total_lines += l;
                }
                if let Some(lang) = language {
                    *lang_counts.entry(lang.clone()).or_insert(0) += 1;
                }
            }
            FileNode::Directory { children, .. } => {
                count_recursive(children, file_count, total_lines, lang_counts);
            }
            FileNode::Collapsed { file_count: fc, .. } => {
                // Don't count collapsed files in detail
                *file_count += fc;
            }
        }
    }
}

fn find_key_subdirs(children: &[FileNode]) -> Vec<(String, Vec<String>)> {
    let mut result = Vec::new();
    
    // Show all subdirectories that have code files
    for child in children {
        if let FileNode::Directory { name, children: subchildren, .. } = child {
            // Skip hidden directories and common non-code directories
            if name.starts_with('.') || name == "__pycache__" {
                continue;
            }
            
            // Get file names (without extensions) for display
            let file_names: Vec<String> = subchildren
                .iter()
                .filter_map(|c| {
                    if let FileNode::File { name, .. } = c {
                        // Remove extension for cleaner display
                        let base = name.split('.').next().unwrap_or(name);
                        // Skip init/mod files
                        if base != "__init__" && base != "mod" && base != "index" {
                            return Some(base.to_string());
                        }
                    }
                    None
                })
                .collect();
            
            // Include directory if it has files
            if !file_names.is_empty() {
                result.push((name.clone(), file_names));
            }
        }
    }

    result
}

fn collect_metadata(node: &FileNode, purposes: &mut Vec<String>, languages: &mut Vec<String>) {
    match node {
        FileNode::File { purpose, language, .. } => {
            if let Some(p) = purpose {
                if !purposes.contains(p) {
                    purposes.push(p.clone());
                }
            }
            if let Some(l) = language {
                if !languages.contains(l) {
                    languages.push(l.clone());
                }
            }
        }
        FileNode::Directory { children, .. } => {
            for child in children {
                collect_metadata(child, purposes, languages);
            }
        }
        FileNode::Collapsed { .. } => {}
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(chars).collect(),
    }
}
