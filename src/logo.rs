/// 定义 Logo 样式配置
#[derive(Debug, Clone)]
pub struct LogoConfig {
    /// 是否使用彩色输出
    pub colored: bool,
    /// 颜色代码 (ANSI 转义序列)
    pub color_code: &'static str,
    /// 重置颜色的代码
    pub reset_code: &'static str,
}

impl Default for LogoConfig {
    fn default() -> Self {
        Self {
            colored: true,
            color_code: "\x1B[38;5;208m", // 橙色 (可以根据品牌调整)
            reset_code: "\x1B[0m",
        }
    }
}

/// 文件管理 CLI 工具的 Logo 实现
pub struct FileManLogo;

impl FileManLogo {
    /// 打印 Logo 到终端
    pub fn print(config: &LogoConfig) {
        let logo = [
            " ██████╗ ███████╗ ██████╗ ██████╗ ███████╗",
            "██╔════╝ ██╔════╝██╔════╝██╔═══██╗██╔════╝",
            "██║  ███╗█████╗  ██║     ██████╔╝███████╗",
            "██║   ██║██╔══╝  ██║     ██╔══██╗╚════██║",
            "╚██████╔╝███████╗╚██████╗██║  ██║███████║",
            " ╚═════╝ ╚══════╝ ╚═════╝╚═╝  ╚═╝╚══════╝",
            "                                        ",
            "     File Manager CLI - v0.1.0          ",
        ];

        // 根据配置决定是否使用彩色输出
        if config.colored {
            println!("{}", config.color_code);
        }

        // 打印 Logo 每一行
        for line in logo {
            println!("{}", line);
        }

        // 重置终端颜色
        if config.colored {
            println!("{}", config.reset_code);
        }
    }
}

// 测试 Logo 显示
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_logo() {
        let config = LogoConfig::default();
        FileManLogo::print(&config);
        
        // 测试无颜色模式
        let mut no_color_config = LogoConfig::default();
        no_color_config.colored = false;
        FileManLogo::print(&no_color_config);
    }
}