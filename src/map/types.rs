use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The complete system map for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMap {
    /// Schema version for forward compatibility
    pub version: String,

    /// Absolute path to project root
    pub root: PathBuf,

    /// Detected project type information
    pub project_type: ProjectType,

    /// When this map was created/updated
    pub scanned_at: DateTime<Utc>,

    /// The file tree structure
    pub tree: FileNode,

    /// Patterns that were matched and collapsed
    pub patterns_matched: Vec<MatchedPattern>,

    /// Metadata about the scan
    pub meta: ScanMeta,
}

/// Information about the detected project type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectType {
    /// All detected languages in the project (first is primary)
    pub languages: Vec<String>,

    /// Framework if detected (flask, react, etc.)
    pub framework: Option<String>,

    /// Files that led to this detection
    pub detected_from: Vec<String>,
}

/// A node in the file tree (either file, directory, or collapsed)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FileNode {
    /// A regular file
    File {
        name: String,
        path: PathBuf,
        lines: Option<usize>,
        language: Option<String>,
        purpose: Option<String>,
        modified: Option<DateTime<Utc>>,
    },

    /// A directory with children
    Directory {
        name: String,
        path: PathBuf,
        children: Vec<FileNode>,
    },

    /// A collapsed directory (e.g., node_modules, .venv)
    Collapsed {
        name: String,
        path: PathBuf,
        reason: String,
        file_count: usize,
        dir_count: usize,
    },
}

impl FileNode {
    pub fn name(&self) -> &str {
        match self {
            FileNode::File { name, .. } => name,
            FileNode::Directory { name, .. } => name,
            FileNode::Collapsed { name, .. } => name,
        }
    }

    #[allow(dead_code)]
    pub fn path(&self) -> &PathBuf {
        match self {
            FileNode::File { path, .. } => path,
            FileNode::Directory { path, .. } => path,
            FileNode::Collapsed { path, .. } => path,
        }
    }

    pub fn is_directory(&self) -> bool {
        matches!(self, FileNode::Directory { .. })
    }

    pub fn is_collapsed(&self) -> bool {
        matches!(self, FileNode::Collapsed { .. })
    }

    pub fn children(&self) -> Option<&Vec<FileNode>> {
        match self {
            FileNode::Directory { children, .. } => Some(children),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn children_mut(&mut self) -> Option<&mut Vec<FileNode>> {
        match self {
            FileNode::Directory { children, .. } => Some(children),
            _ => None,
        }
    }
}

/// Record of a pattern that was matched
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedPattern {
    pub pattern: String,
    pub path: PathBuf,
    pub files_collapsed: usize,
    pub dirs_collapsed: usize,
}

/// Metadata about the scanning process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanMeta {
    /// Total files found before collapsing
    pub total_files: usize,

    /// Files actually indexed (after collapsing)
    pub indexed_files: usize,

    /// Total directories found
    pub total_dirs: usize,

    /// Time taken to scan (milliseconds)
    pub scan_time_ms: u64,
}

impl Default for ProjectType {
    fn default() -> Self {
        Self {
            languages: Vec::new(),
            framework: None,
            detected_from: Vec::new(),
        }
    }
}

impl SystemMap {
    /// Create a new empty system map
    pub fn new(root: PathBuf) -> Self {
        Self {
            version: "1.0".to_string(),
            root: root.clone(),
            project_type: ProjectType::default(),
            scanned_at: Utc::now(),
            tree: FileNode::Directory {
                name: root
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "root".to_string()),
                path: root,
                children: Vec::new(),
            },
            patterns_matched: Vec::new(),
            meta: ScanMeta {
                total_files: 0,
                indexed_files: 0,
                total_dirs: 0,
                scan_time_ms: 0,
            },
        }
    }

    /// Save the map to a JSON file
    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load a map from a JSON file
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let map = serde_json::from_str(&json)?;
        Ok(map)
    }
}
