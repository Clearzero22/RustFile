mod cli;
mod core;
mod display;

use cli::parser::Cli;
use cli::commands::handle_list_command;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        cli::parser::Commands::List(args) => {
            handle_list_command(args);
        }
    }
}