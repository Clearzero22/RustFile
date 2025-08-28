use clap::{Parser, Subcommand, Args};
use colored::*;
use std::path::Path;
use std::time::{UNIX_EPOCH, SystemTime};

/// Rust Terminal File Manager
#[derive(Parser)]
#[command(name = "fmg")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List directory contents
    #[command(alias = "ls")]
    List(ListArgs),
}

#[derive(Args)]
struct ListArgs {
    /// Path to the directory to list
    #[arg(default_value = ".", value_parser)]
    path: String,
    
    /// Display detailed information (size, modified time, permissions)
    #[arg(short, long)]
    long: bool,
    
    /// Display tree structure
    #[arg(short, long)]
    tree: bool,
    
    /// Limit tree depth
    #[arg(short = 'D', long, requires = "tree")] // Changed short flag to 'D' to avoid conflict
    depth: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List(args) => {
            if args.tree {
                list_directory_tree(&args.path, args.long, args.depth.unwrap_or(usize::MAX), 0);
            } else {
                list_directory(&args.path, args.long);
            }
        }
    }
}

fn format_system_time(system_time: SystemTime) -> String {
    match system_time.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            // Format as YYYY-MM-DD HH:MM
            let secs = duration.as_secs();
            let datetime = time::OffsetDateTime::from_unix_timestamp(secs as i64).unwrap_or(time::OffsetDateTime::UNIX_EPOCH);
            format!("{:04}-{:02}-{:02} {:02}:{:02}", 
                datetime.year(), 
                datetime.month() as u8, 
                datetime.day(), 
                datetime.hour(), 
                datetime.minute()
            )
        }
        Err(_) => "???".to_string(),
    }
}

fn list_directory(path_str: &str, long_format: bool) {
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
                        
                        if long_format {
                            match entry.metadata() {
                                Ok(metadata) => {
                                    let size = metadata.len();
                                    // Simplified permissions display (Unix-style)
                                    let permissions = if metadata.is_dir() {
                                        "d".to_string()
                                    } else {
                                        "-".to_string()
                                    };
                                    let perm_string = format!("{}{}", permissions, "---------"); // Placeholder for actual perms
                                    
                                    // Modified time (formatted)
                                    let modified = metadata.modified().unwrap_or(UNIX_EPOCH);
                                    let modified_str = format_system_time(modified);
                                    
                                    if metadata.is_dir() {
                                        println!("{} {:>10} {} {}", perm_string, size, modified_str, file_name.to_string_lossy().blue().bold());
                                    } else {
                                        println!("{} {:>10} {} {}", perm_string, size, modified_str, file_name.to_string_lossy().green());
                                    }
                                }
                                Err(e) => {
                                    eprintln!("{} Failed to get metadata for {}: {}", "Error:".red().bold(), file_name.to_string_lossy(), e);
                                }
                            }
                        } else {
                            if file_path.is_dir() {
                                println!("{}", file_name.to_string_lossy().blue().bold());
                            } else {
                                println!("{}", file_name.to_string_lossy().green());
                            }
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

fn list_directory_tree(path_str: &str, long_format: bool, max_depth: usize, current_depth: usize) {
    // Prevent infinite recursion
    if current_depth > max_depth {
        return;
    }
    
    let path = Path::new(path_str);
    
    if !path.exists() {
        eprintln!("{} Path '{}' does not exist.", "Error:".red().bold(), path_str);
        return;
    }
    
    if !path.is_dir() {
        eprintln!("{} Path '{}' is not a directory.", "Error:".red().bold(), path_str);
        return;
    }
    
    // For root level (current_depth == 0), we don't print the directory name itself
    // Read directory entries
    match std::fs::read_dir(path) {
        Ok(entries) => {
            // Filter out entries that might cause issues and collect them
            let entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            let count = entries.len();
            
            for (i, entry) in entries.iter().enumerate() {
                let file_name = entry.file_name();
                let file_path = entry.path();
                let is_last = i == count - 1;
                
                // Create prefix for tree structure
                let prefix = if current_depth == 0 {
                    if is_last { "└── ".to_string() } else { "├── ".to_string() }
                } else {
                    let mut p = "  ".repeat(current_depth - 1);
                    if is_last {
                        p.push_str("└── ");
                    } else {
                        p.push_str("├── ");
                    }
                    p
                };
                
                if file_path.is_dir() {
                    if long_format {
                        match entry.metadata() {
                            Ok(metadata) => {
                                let size = metadata.len();
                                let permissions = "d";
                                let perm_string = format!("{}{}", permissions, "---------");
                                let modified = metadata.modified().unwrap_or(UNIX_EPOCH);
                                let modified_str = format_system_time(modified);
                                println!("{}{} {:>10} {} {}", prefix, perm_string, size, modified_str, file_name.to_string_lossy().blue().bold());
                            }
                            Err(e) => {
                                eprintln!("{} Failed to get metadata for {}: {}", "Error:".red().bold(), file_name.to_string_lossy(), e);
                            }
                        }
                    } else {
                        println!("{}{}", prefix, file_name.to_string_lossy().blue().bold());
                    }
                    
                    // Recursively list subdirectory if depth allows
                    if current_depth < max_depth {
                        list_directory_tree(file_path.to_str().unwrap_or(""), long_format, max_depth, current_depth + 1);
                    }
                } else {
                    if long_format {
                        match entry.metadata() {
                            Ok(metadata) => {
                                let size = metadata.len();
                                let permissions = "-";
                                let perm_string = format!("{}{}", permissions, "---------");
                                let modified = metadata.modified().unwrap_or(UNIX_EPOCH);
                                let modified_str = format_system_time(modified);
                                println!("{}{} {:>10} {} {}", prefix, perm_string, size, modified_str, file_name.to_string_lossy().green());
                            }
                            Err(e) => {
                                eprintln!("{} Failed to get metadata for {}: {}", "Error:".red().bold(), file_name.to_string_lossy(), e);
                            }
                        }
                    } else {
                        println!("{}{}", prefix, file_name.to_string_lossy().green());
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{} Failed to read directory '{}': {}", "Error:".red().bold(), path_str, e);
        }
    }
}
