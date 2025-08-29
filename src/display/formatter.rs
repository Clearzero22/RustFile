//! 输出格式化模块

use std::time::{SystemTime, UNIX_EPOCH};
use colored::*;

/// 格式化系统时间
pub fn format_system_time(system_time: SystemTime) -> String {
    match system_time.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            // Format as YYYY-MM-DD HH:MM
            let secs = duration.as_secs();
            let datetime = time::OffsetDateTime::from_unix_timestamp(secs as i64)
                .unwrap_or(time::OffsetDateTime::UNIX_EPOCH);
            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}",
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

/// 格式化树形结构的前缀，并根据深度应用颜色
pub fn format_tree_prefix(is_last: bool, ancestors: &[bool], depth: usize) -> String {
    let mut prefix = String::new();
    
    // 定义颜色列表，用于不同层级
    let colors = [
        Color::Blue,   // 深度 0
        Color::Green,  // 深度 1
        Color::Yellow, // 深度 2
        Color::Magenta, // 深度 3 (使用 Magenta 替代 BrightPurple)
        Color::Cyan,   // 深度 4
        Color::Red,    // 深度 5
    ];
    let current_color = colors[depth % colors.len()];

    // 添加祖先节点的连接线并着色
    for &is_ancestor_last in ancestors {
        if is_ancestor_last {
            prefix.push_str(&"    ".color(current_color).to_string()); // 空白占位符
        } else {
            prefix.push_str(&"│   ".color(current_color).to_string()); // 垂直连接线
        }
    }

    // 添加当前节点的连接符并着色
    if is_last {
        prefix.push_str(&"└── ".color(current_color).to_string()); // L形连接符
        } else {
        prefix.push_str(&"├── ".color(current_color).to_string()); // T形连接符
    }

    prefix
}
