use std::env;
use std::time::Instant;

use anyhow::Result;
use colored::Colorize;

use crate::colors::{colorize_language, colorize_purpose};
use crate::config::{find_sysmap_root, map_path};
use crate::map::{FileNode, SystemMap};

/// Execute the find command
pub fn execute(
    query: String, 
    file_type: Option<String>,
    language: Option<String>,
    purpose: Option<String>,
) -> Result<()> {
    let start = Instant::now();
    let cwd = env::current_dir()?;
    
    let root = find_sysmap_root(&cwd)
        .ok_or_else(|| anyhow::anyhow!(
            "No sysmap found. Run 'sysmap init' first."
        ))?;

    let map = SystemMap::load(&map_path(&root))?;

    let query_lower = query.to_lowercase();
    
    // Normalize file type filter (remove leading dot if present)
    let file_type_normalized = file_type.map(|ft| {
        ft.strip_prefix('.').unwrap_or(&ft).to_lowercase()
    });
    
    // Normalize language filter
    let language_normalized = language.map(|l| l.to_lowercase());
    
    // Normalize purpose filter
    let purpose_normalized = purpose.map(|p| p.to_lowercase());
    
    let mut matches = Vec::new();

    find_matches(
        &map.tree, 
        &query_lower, 
        &file_type_normalized,
        &language_normalized,
        &purpose_normalized,
        &mut matches
    );

    let elapsed = start.elapsed();

    if matches.is_empty() {
        println!("{}", "No matches found.".yellow());
        println!("{}", format!("Search completed in {:?}", elapsed).dimmed());
        return Ok(());
    }

    println!("{} {} matches:", "Found".green().bold(), matches.len());
    println!();

    for (path, name, file_purpose, file_language, lines) in matches {
        let mut info_parts = Vec::new();
        
        if let Some(l) = lines {
            info_parts.push(format!("{} lines", l));
        }
        
        if let Some(p) = &file_purpose {
            info_parts.push(format!("[{}]", colorize_purpose(p)));
        }
        
        if let Some(l) = &file_language {
            info_parts.push(colorize_language(l).to_string());
        }

        let info = if info_parts.is_empty() {
            String::new()
        } else {
            format!("  ({})", info_parts.join(", "))
        };

        // Highlight the match in the name
        let highlighted = highlight_match(&name, &query);
        
        println!("  {}{}{}",
            if path.is_empty() { String::new() } else { format!("{}/", path).dimmed().to_string() },
            highlighted,
            info
        );
    }

    println!();
    println!("{}", format!("Search completed in {:?}", elapsed).dimmed());

    Ok(())
}

fn find_matches(
    node: &FileNode,
    query: &str,
    file_type: &Option<String>,
    language: &Option<String>,
    purpose: &Option<String>,
    matches: &mut Vec<(String, String, Option<String>, Option<String>, Option<usize>)>,
) {
    match node {
        FileNode::File { name, path, purpose: file_purpose, language: file_language, lines, .. } => {
            let name_lower = name.to_lowercase();
            
            // Check file type filter
            if let Some(ft) = file_type {
                let ext = path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");
                if ext.to_lowercase() != *ft {
                    return;
                }
            }
            
            // Check language filter
            if let Some(lang_filter) = language {
                match file_language {
                    Some(file_lang) if file_lang.to_lowercase() == *lang_filter => {}
                    _ => return,
                }
            }
            
            // Check purpose filter
            if let Some(purpose_filter) = purpose {
                match file_purpose {
                    Some(fp) if fp.to_lowercase() == *purpose_filter => {}
                    _ => return,
                }
            }

            // Check if name matches query
            if name_lower.contains(query) {
                let parent = path.parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                
                matches.push((
                    parent,
                    name.clone(),
                    file_purpose.clone(),
                    file_language.clone(),
                    *lines,
                ));
            }
        }
        FileNode::Directory { name, path, children, .. } => {
            // Only match directories if no filters are set
            if file_type.is_none() && language.is_none() && purpose.is_none() {
                let name_lower = name.to_lowercase();
                if name_lower.contains(query) {
                    let parent = path.parent()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default();
                    
                    matches.push((
                        parent,
                        format!("{}/", name),
                        None,
                        None,
                        None,
                    ));
                }
            }

            // Search children
            for child in children {
                find_matches(child, query, file_type, language, purpose, matches);
            }
        }
        FileNode::Collapsed { .. } => {
            // Don't search collapsed directories
        }
    }
}

fn highlight_match(text: &str, query: &str) -> String {
    let text_lower = text.to_lowercase();
    
    if let Some(start) = text_lower.find(query) {
        let end = start + query.len();
        format!(
            "{}{}{}",
            &text[..start],
            text[start..end].yellow().bold(),
            &text[end..]
        )
    } else {
        text.to_string()
    }
}
