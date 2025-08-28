//! CLI参数解析模块

use clap::{Parser, Subcommand, Args};

/// Rust Terminal File Manager
#[derive(Parser)]
#[command(name = "fmg")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List directory contents
    #[command(alias = "ls")]
    List(ListArgs),
}

#[derive(Args)]
pub struct ListArgs {
    /// Path to the directory to list
    #[arg(default_value = ".", value_parser)]
    pub path: String,
    
    /// Display detailed information (size, modified time, permissions)
    #[arg(short, long)]
    pub long: bool,
    
    /// Display tree structure
    #[arg(short, long)]
    pub tree: bool,
    
    /// Limit tree depth
    #[arg(short = 'D', long, requires = "tree")] // Changed short flag to 'D' to avoid conflict
    pub depth: Option<usize>,
}