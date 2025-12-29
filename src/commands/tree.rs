use std::env;
use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;

use crate::colors::{colorize_language, colorize_purpose};
use crate::config::{find_sysmap_root, map_path};
use crate::map::{FileNode, SystemMap};

/// Execute the tree command
pub fn execute(path: Option<PathBuf>, depth: usize, show_all: bool) -> Result<()> {
    let cwd = env::current_dir()?;
    
    let root = find_sysmap_root(&cwd)
        .ok_or_else(|| anyhow::anyhow!(
            "No sysmap found. Run 'sysmap init' first."
        ))?;

    let map = SystemMap::load(&map_path(&root))?;

    // Find the starting node
    let start_node = if let Some(ref subpath) = path {
        find_node(&map.tree, subpath)
            .ok_or_else(|| anyhow::anyhow!("Path not found: {}", subpath.display()))?
    } else {
        &map.tree
    };

    print_tree(start_node, "", true, 0, depth, show_all);

    Ok(())
}

fn find_node<'a>(node: &'a FileNode, target: &PathBuf) -> Option<&'a FileNode> {
    // Normalize the target path
    let target_str = target.to_string_lossy();
    let target_parts: Vec<&str> = target_str
        .split(|c| c == '/' || c == '\\')
        .filter(|s| !s.is_empty())
        .collect();

    if target_parts.is_empty() {
        return Some(node);
    }

    find_node_recursive(node, &target_parts, 0)
}

fn find_node_recursive<'a>(node: &'a FileNode, parts: &[&str], index: usize) -> Option<&'a FileNode> {
    if index >= parts.len() {
        return Some(node);
    }

    let target_name = parts[index];

    if let Some(children) = node.children() {
        for child in children {
            if child.name() == target_name {
                if index == parts.len() - 1 {
                    return Some(child);
                }
                return find_node_recursive(child, parts, index + 1);
            }
        }
    }

    None
}

fn print_tree(node: &FileNode, prefix: &str, is_last: bool, current_depth: usize, max_depth: usize, show_all: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    
    match node {
        FileNode::File { name, lines, purpose, language, .. } => {
            let mut info_parts = Vec::new();
            
            if let Some(l) = lines {
                info_parts.push(format!("{} lines", l));
            }
            
            if let Some(p) = purpose {
                info_parts.push(format!("[{}]", colorize_purpose(p)));
            }
            
            if let Some(lang) = language {
                info_parts.push(colorize_language(lang).to_string());
            }

            let info = if info_parts.is_empty() {
                String::new()
            } else {
                format!(" ({})", info_parts.join(", "))
            };

            println!("{}{}{}{}", prefix, connector, name, info.dimmed());
        }
        
        FileNode::Directory { name, children, .. } => {
            // Print this directory
            if current_depth == 0 {
                println!("{}/", name.bold());
            } else {
                println!("{}{}{}/", prefix, connector, name.bold());
            }

            // Check depth
            if current_depth >= max_depth {
                let child_prefix = if is_last {
                    format!("{}    ", prefix)
                } else {
                    format!("{}│   ", prefix)
                };
                let file_count = count_files(children);
                if file_count > 0 {
                    println!("{}└── {} ({} items)", 
                        child_prefix,
                        "...".dimmed(),
                        file_count.to_string().dimmed()
                    );
                }
                return;
            }

            let child_prefix = if current_depth == 0 {
                String::new()
            } else if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            // Sort children: directories first, then files
            let mut sorted_children: Vec<&FileNode> = children.iter().collect();
            sorted_children.sort_by(|a, b| {
                match (a.is_directory() || a.is_collapsed(), b.is_directory() || b.is_collapsed()) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name().cmp(b.name()),
                }
            });

            for (i, child) in sorted_children.iter().enumerate() {
                let child_is_last = i == sorted_children.len() - 1;
                print_tree(child, &child_prefix, child_is_last, current_depth + 1, max_depth, show_all);
            }
        }
        
        FileNode::Collapsed { name, reason, file_count, .. } => {
            if show_all {
                // In show_all mode, this shouldn't happen as we'd rescan
                // For now, just show as collapsed
                println!(
                    "{}{}{}/  {}",
                    prefix,
                    connector,
                    name.dimmed(),
                    format!("[{}: {} files]", reason, file_count).dimmed()
                );
            } else {
                println!(
                    "{}{}{}/  {}",
                    prefix,
                    connector,
                    name.dimmed(),
                    format!("[{}: {} files]", reason, file_count).dimmed()
                );
            }
        }
    }
}

fn count_files(nodes: &[FileNode]) -> usize {
    let mut count = 0;
    for node in nodes {
        match node {
            FileNode::File { .. } => count += 1,
            FileNode::Directory { children, .. } => count += 1 + count_files(children),
            FileNode::Collapsed { file_count, .. } => count += file_count,
        }
    }
    count
}
