use clap::{Parser, Subcommand};
use colored::*;
use std::path::Path;

/// Rust Terminal File Manager
#[derive(Parser)]
#[command(name = "fmg")]
#[command(about = "A simple file manager CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List directory contents
    #[command(alias = "ls")]
    List {
        /// Path to the directory to list
        #[arg(default_value = ".", value_parser)]
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List { path } => {
            list_directory(path);
        }
    }
}

fn list_directory(path_str: &str) {
    let path = Path::new(path_str);

    if !path.exists() {
        eprintln!("{} Path '{}' does not exist.", "Error:".red().bold(), path_str);
        return;
    }

    if !path.is_dir() {
        eprintln!("{} Path '{}' is not a directory.", "Error:".red().bold(), path_str);
        return;
    }

    match std::fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let file_name = entry.file_name();
                        let file_path = entry.path();
                        if file_path.is_dir() {
                            println!("{}", file_name.to_string_lossy().blue().bold());
                        } else {
                            println!("{}", file_name.to_string_lossy().green());
                        }
                    }
                    Err(e) => {
                        eprintln!("{} Failed to read entry: {}", "Error:".red().bold(), e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{} Failed to read directory '{}': {}", "Error:".red().bold(), path_str, e);
        }
    }
}
