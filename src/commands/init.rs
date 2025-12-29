use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use colored::Colorize;

use crate::config::{ensure_sysmap_dir, is_initialized, map_path, sysmap_dir};
use crate::scanner::{scan_directory, ScannerConfig};

/// Execute the init command
pub fn execute(path: PathBuf, force: bool, verbosity: u8) -> Result<()> {
    // Resolve the path
    let root = path
        .canonicalize()
        .with_context(|| format!("Cannot access path: {}", path.display()))?;

    // Check if already initialized
    if is_initialized(&root) && !force {
        bail!(
            "Already initialized at {}. Use --force to reinitialize.",
            sysmap_dir(&root).display()
        );
    }

    if verbosity > 0 {
        println!(
            "{} {}",
            "Scanning".green().bold(),
            root.display()
        );
    }

    // Scan the directory
    let mut config = ScannerConfig::default();
    config.show_progress = verbosity > 0;
    let map = scan_directory(&root, &config)?;

    // Create .sysmap directory
    ensure_sysmap_dir(&root)?;

    // Save the map
    let map_file = map_path(&root);
    map.save(&map_file)?;

    // Print summary (unless quiet)
    if verbosity > 0 {
        println!();
        let primary_lang = map.project_type.languages.first()
            .map(|s| s.as_str())
            .unwrap_or("unknown");
        println!("  {} Detected: {} project{}", 
            "├─".dimmed(),
            primary_lang.cyan(),
            map.project_type.framework.as_ref()
                .map(|f| format!(" ({})", f))
                .unwrap_or_default()
        );
        println!("  {} Files: {} scanned, {} indexed ({} collapsed)",
            "├─".dimmed(),
            map.meta.total_files.to_string().yellow(),
            map.meta.indexed_files.to_string().green(),
            (map.meta.total_files - map.meta.indexed_files).to_string().dimmed()
        );
        
        if !map.patterns_matched.is_empty() {
            let pattern_summary: Vec<String> = map.patterns_matched
                .iter()
                .map(|p| format!("{} ({} files)", p.pattern, p.files_collapsed))
                .collect();
            println!("  {} Patterns: {}",
                "├─".dimmed(),
                pattern_summary.join(", ").dimmed()
            );
        }

        println!("  {} Scan time: {}ms",
            "└─".dimmed(),
            map.meta.scan_time_ms
        );

        println!();
        println!(
            "{} {}",
            "Map saved to".green(),
            map_file.display().to_string().dimmed()
        );
        println!(
            "Run '{}' to view project overview.",
            "sysmap summary".cyan()
        );
    }

    Ok(())
}
