//! 核心文件操作模块

use crate::display::formatter::{format_system_time, format_tree_prefix};
use colored::*;
use std::path::Path;
use std::time::UNIX_EPOCH;

/// 列出目录内容
pub fn list_directory(path_str: &str, long_format: bool) {
    let path = Path::new(path_str);

    if !path.exists() {
        eprintln!(
            "{} Path '{}' does not exist.",
            "Error:".red().bold(),
            path_str
        );
        return;
    }

    if !path.is_dir() {
        eprintln!(
            "{} Path '{}' is not a directory.",
            "Error:".red().bold(),
            path_str
        );
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
                                        println!(
                                            "{} {:>10} {} {}",
                                            perm_string,
                                            size,
                                            modified_str,
                                            file_name.to_string_lossy().blue().bold()
                                        );
                                    } else {
                                        println!(
                                            "{} {:>10} {} {}",
                                            perm_string,
                                            size,
                                            modified_str,
                                            file_name.to_string_lossy().green()
                                        );
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "{} Failed to get metadata for {}: {}",
                                        "Error:".red().bold(),
                                        file_name.to_string_lossy(),
                                        e
                                    );
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
            eprintln!(
                "{} Failed to read directory '{}': {}",
                "Error:".red().bold(),
                path_str,
                e
            );
        }
    }
}

/// 以树形结构列出目录内容
pub fn list_directory_tree(
    path_str: &str,
    long_format: bool,
    max_depth: usize,
    current_depth: usize,
) {
    list_directory_tree_with_ancestors(path_str, long_format, max_depth, current_depth, &vec![])
}

/// 递归实现树形结构列表，跟踪祖先节点信息
fn list_directory_tree_with_ancestors(
    path_str: &str,
    long_format: bool,
    max_depth: usize,
    current_depth: usize,
    ancestors: &Vec<bool>,
) {
    // Prevent infinite recursion
    if current_depth > max_depth {
        return;
    }

    let path = Path::new(path_str);

    if !path.exists() {
        eprintln!(
            "{} Path '{}' does not exist.",
            "Error:".red().bold(),
            path_str
        );
        return;
    }

    if !path.is_dir() {
        eprintln!(
            "{} Path '{}' is not a directory.",
            "Error:".red().bold(),
            path_str
        );
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

                // Create prefix for tree structure using the new formatter
                let prefix = format_tree_prefix(is_last, ancestors);

                if file_path.is_dir() {
                    if long_format {
                        match entry.metadata() {
                            Ok(metadata) => {
                                let size = metadata.len();
                                let permissions = "d";
                                let perm_string = format!("{}{}", permissions, "---------");
                                let modified = metadata.modified().unwrap_or(UNIX_EPOCH);
                                let modified_str = format_system_time(modified);
                                println!(
                                    "{}{} {:>10} {} {}",
                                    prefix,
                                    perm_string,
                                    size,
                                    modified_str,
                                    file_name.to_string_lossy().blue().bold()
                                );
                            }
                            Err(e) => {
                                eprintln!(
                                    "{} Failed to get metadata for {}: {}",
                                    "Error:".red().bold(),
                                    file_name.to_string_lossy(),
                                    e
                                );
                            }
                        }
                    } else {
                        println!("{}{}", prefix, file_name.to_string_lossy().blue().bold());
                    }

                    // Recursively list subdirectory if depth allows
                    if current_depth < max_depth {
                        // 更新祖先节点信息
                        let mut new_ancestors = ancestors.clone();
                        new_ancestors.push(is_last);
                        list_directory_tree_with_ancestors(
                            file_path.to_str().unwrap_or(""),
                            long_format,
                            max_depth,
                            current_depth + 1,
                            &new_ancestors,
                        );
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
                                println!(
                                    "{}{} {:>10} {} {}",
                                    prefix,
                                    perm_string,
                                    size,
                                    modified_str,
                                    file_name.to_string_lossy().green()
                                );
                            }
                            Err(e) => {
                                eprintln!(
                                    "{} Failed to get metadata for {}: {}",
                                    "Error:".red().bold(),
                                    file_name.to_string_lossy(),
                                    e
                                );
                            }
                        }
                    } else {
                        println!("{}{}", prefix, file_name.to_string_lossy().green());
                    }
                }
            }
        }
        Err(e) => {
            eprintln!(
                "{} Failed to read directory '{}': {}",
                "Error:".red().bold(),
                path_str,
                e
            );
        }
    }
}
