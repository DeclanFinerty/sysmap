use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

/// Name of the sysmap directory
pub const SYSMAP_DIR: &str = ".sysmap";

/// Name of the map file
pub const MAP_FILE: &str = "map.json";

/// Name of the config file (for future use)
#[allow(dead_code)]
pub const CONFIG_FILE: &str = "config.toml";

/// Find the sysmap root directory by looking for .sysmap folder
pub fn find_sysmap_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    
    loop {
        let sysmap_dir = current.join(SYSMAP_DIR);
        if sysmap_dir.is_dir() {
            return Some(current);
        }
        
        if !current.pop() {
            return None;
        }
    }
}

/// Get the path to the .sysmap directory
pub fn sysmap_dir(root: &Path) -> PathBuf {
    root.join(SYSMAP_DIR)
}

/// Get the path to the map.json file
pub fn map_path(root: &Path) -> PathBuf {
    sysmap_dir(root).join(MAP_FILE)
}

/// Get the path to the config.toml file (for future use)
#[allow(dead_code)]
pub fn config_path(root: &Path) -> PathBuf {
    sysmap_dir(root).join(CONFIG_FILE)
}

/// Ensure the .sysmap directory exists
pub fn ensure_sysmap_dir(root: &Path) -> Result<PathBuf> {
    let dir = sysmap_dir(root);
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create directory: {}", dir.display()))?;
    Ok(dir)
}

/// Check if a sysmap has been initialized in the given directory
pub fn is_initialized(root: &Path) -> bool {
    map_path(root).exists()
}
