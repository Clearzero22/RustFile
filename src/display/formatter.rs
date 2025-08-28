//! 输出格式化模块

use std::time::{SystemTime, UNIX_EPOCH};

/// 格式化系统时间
pub fn format_system_time(system_time: SystemTime) -> String {
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