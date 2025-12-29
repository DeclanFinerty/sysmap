use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "sysmap")]
#[command(author, version, about = "Intelligent project mapping for AI agents and humans")]
#[command(propagate_version = true)]
pub struct Cli {
    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Quiet mode - minimal output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Verbose mode - extra detail
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project map
    #[command(visible_alias = "i")]
    Init {
        /// Directory to map (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Overwrite existing .sysmap directory
        #[arg(short, long)]
        force: bool,
    },

    /// Display compressed project summary
    #[command(visible_alias = "s")]
    Summary {
        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Output as YAML
        #[arg(long)]
        yaml: bool,
    },

    /// Display directory tree with pattern awareness
    #[command(visible_alias = "t")]
    Tree {
        /// Subdirectory to show (defaults to project root)
        path: Option<PathBuf>,

        /// Maximum depth (default: 3)
        #[arg(short, long, default_value = "3")]
        depth: usize,

        /// Show collapsed directories expanded
        #[arg(short, long)]
        all: bool,
    },

    /// Update existing map incrementally
    #[command(visible_alias = "u")]
    Update {
        /// Force full rebuild instead of incremental
        #[arg(long)]
        full: bool,
    },

    /// Search the map for files
    #[command(visible_alias = "f")]
    Find {
        /// Search term (filename, pattern, or keyword)
        query: String,

        /// Filter by file extension (py, rs, js, etc.)
        #[arg(short = 't', long = "type")]
        file_type: Option<String>,

        /// Filter by language (python, rust, javascript, etc.)
        #[arg(short = 'l', long = "lang")]
        language: Option<String>,

        /// Filter by purpose (entry, module, test, config)
        #[arg(short = 'p', long = "purpose")]
        purpose: Option<String>,
    },
}
