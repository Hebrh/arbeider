//! Argument struct for the CLI.
// use std::env;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Level one command.
#[derive(Subcommand)]
pub enum Commands {
    /// scheduler
    #[command(author, version, about = "Start worker")]
    Start(NumberArgs),

    /// stop all workers
    #[command(about = "Stop worker")]
    Stop(NumberArgs),

    /// restart worker
    #[command(about = "Restart worker")]
    Restart(NumberArgs),

    /// list all workers
    #[command(about = "List all workers")]
    List,
}

/// worker numbers argument for start stop and restart.
#[derive(Args, Debug)]
pub struct NumberArgs {
    name: Option<u8>,
}

#[derive(Subcommand, PartialEq, Debug)]
pub enum Operator {
    #[command(about = "Start worker")]
    Start,

    // stop all workers
    #[command(about = "Stop all workers")]
    Stop,

    #[command(about = "Restart worker")]
    Restart,

    // list all workers
    #[command(about = "List all workers")]
    List,
}
