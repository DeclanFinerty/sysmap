use std::env;

use anyhow::Result;
use colored::Colorize;

use crate::config::{find_sysmap_root, map_path};
use crate::scanner::{scan_directory, ScannerConfig};

/// Execute the update command
pub fn execute(full: bool, verbosity: u8) -> Result<()> {
    let cwd = env::current_dir()?;
    
    let root = find_sysmap_root(&cwd)
        .ok_or_else(|| anyhow::anyhow!(
            "No sysmap found. Run 'sysmap init' first."
        ))?;

    if verbosity > 0 {
        if full {
            println!("{} full rebuild...", "Starting".green().bold());
        } else {
            println!("{} map...", "Updating".green().bold());
        }
    }

    let mut config = ScannerConfig::default();
    config.show_progress = verbosity > 0;
    let map = scan_directory(&root, &config)?;

    // Save updated map
    let map_file = map_path(&root);
    map.save(&map_file)?;

    if verbosity > 0 {
        println!();
        println!("  {} Files: {} scanned, {} indexed",
            "├─".dimmed(),
            map.meta.total_files.to_string().yellow(),
            map.meta.indexed_files.to_string().green()
        );
        println!("  {} Updated in {}ms",
            "└─".dimmed(),
            map.meta.scan_time_ms
        );
    }

    Ok(())
}
