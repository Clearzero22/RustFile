mod cli;
mod core;
mod display;

use clap::Parser;
use cli::commands::handle_list_command;
use cli::parser::Cli;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        cli::parser::Commands::List(args) => {
            handle_list_command(args);
        }
    }
}
