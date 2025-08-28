//! CLI命令处理模块

use crate::core::file_ops::{list_directory, list_directory_tree};

/// 处理列表命令
pub fn handle_list_command(args: &super::parser::ListArgs) {
    if args.tree {
        list_directory_tree(&args.path, args.long, args.depth.unwrap_or(usize::MAX), 0);
    } else {
        list_directory(&args.path, args.long);
    }
}
