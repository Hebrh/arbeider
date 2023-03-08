//! Argument struct for the CLI.
// use std::env;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
pub struct Cli {
    #[command(subcommand)]
    pub operator: Operator,
}

#[derive(Subcommand)]
pub enum Operator {
    #[command(author, version, about = "Start worker")]
    Start(StartArgs),

    #[command(about = "Stop worker")]
    Stop(StopArgs),

    #[command(about = "Restart worker")]
    Restart(StopArgs),
}

/// worker numbers argument for start.
#[derive(Args, Debug)]
pub struct StartArgs {
    // scheduler address
    #[arg(
        short,
        long,
        help = "127.0.0.1:27021",
        default_value = "127.0.0.1:27021"
    )]
    pub scheduler: String,
}

/// worker numbers argument for stop, restart.
#[derive(Args, Debug)]
pub struct StopArgs {
    // worker id
    #[arg(short, long)]
    pub id: String,
}
