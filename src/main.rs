mod cli;
mod core;
mod display;
mod logo;

use clap::Parser;
use cli::commands::handle_list_command;
use cli::parser::Cli;
use logo::{FileManLogo, LogoConfig};

fn main() {
    // 创建默认的 Logo 配置
    let logo_config = LogoConfig::default();
    
    // 打印 Logo
    FileManLogo::print(&logo_config);
    
    let cli = Cli::parse();

    match &cli.command {
        cli::parser::Commands::List(args) => {
            handle_list_command(args);
        }
    }
}
