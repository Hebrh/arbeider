//! Worker main.
use aworker::{Cli, Commands};
use clap::{Args, Parser, Subcommand};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start(args) => {
            println!("start worker");
            println!("args: {args:?}");
        }
        Commands::Stop(args) => {
            println!("stop worker");
            println!("args: {args:?}");
        }
        Commands::Restart(args) => {
            println!("restart worker");
            println!("args: {args:?}");
        }
        Commands::List => {
            println!("list all workers");
        }
    }
}
