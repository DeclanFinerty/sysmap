use anyhow::Result;
use clap::Parser;
use colored::{control, Colorize};

mod cli;
mod colors;
mod commands;
mod config;
mod map;
mod patterns;
mod scanner;

use cli::{Cli, Commands};

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Handle global flags
    if cli.no_color {
        control::set_override(false);
    }

    let verbosity = if cli.quiet { 0 } else if cli.verbose { 2 } else { 1 };

    match cli.command {
        Commands::Init { path, force } => {
            commands::init::execute(path, force, verbosity)?;
        }
        Commands::Summary { json, yaml } => {
            commands::summary::execute(json, yaml)?;
        }
        Commands::Tree { path, depth, all } => {
            commands::tree::execute(path, depth, all)?;
        }
        Commands::Update { full } => {
            commands::update::execute(full, verbosity)?;
        }
        Commands::Find { query, file_type, language, purpose } => {
            commands::find::execute(query, file_type, language, purpose)?;
        }
    }

    Ok(())
}
