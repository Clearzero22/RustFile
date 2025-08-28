//! 输出格式化模块

use std::time::{SystemTime, UNIX_EPOCH};

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

/// 格式化树形结构的前缀
pub fn format_tree_prefix(is_last: bool, ancestors: &[bool]) -> String {
    let mut prefix = String::new();

    // 添加祖先节点的连接线
    for &is_ancestor_last in ancestors {
        if is_ancestor_last {
            prefix.push_str("    "); // 空白占位符
        } else {
            prefix.push_str("│   "); // 垂直连接线
        }
    }

    // 添加当前节点的连接符
    if is_last {
        prefix.push_str("└── "); // L形连接符
    } else {
        prefix.push_str("├── "); // T形连接符
    }

    prefix
}
