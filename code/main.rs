
use clap::Parser;
use std::path::PathBuf;
use crate::logo::{FileManLogo, LogoConfig};

mod logo;

/// 终端文件管理 CLI 工具
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 不显示彩色 Logo
    #[clap(short, long)]
    no_color: bool,
    
    /// 操作的目标路径
    #[clap(value_parser)]
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    
    // 配置并显示 Logo
    let mut logo_config = LogoConfig::default();
    logo_config.colored = !args.no_color;
    FileManLogo::print(&logo_config);
    
    // 后续的文件管理逻辑...
    println!("\n开始文件管理操作...");
}
