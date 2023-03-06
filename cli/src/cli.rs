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
    #[command(author, version, about = "arb scheduler command")]
    Scheduler {
        #[command(subcommand)]
        command: SchedulerOperator,
    },

    #[command(author, version, about = "arb worker command")]
    Worker {
        #[command(subcommand)]
        command: WorkerOperator,
    },
}

#[derive(Subcommand)]
pub enum WorkerOperator {
    #[command(author, version, about = "Start worker")]
    Start(WorkerArgs),

    #[command(about = "Stop worker")]
    Stop(WorkerArgs),

    #[command(about = "Restart worker")]
    Restart(WorkerArgs),

    #[command(about = "List all workers")]
    List,
}

#[derive(Subcommand)]
pub enum SchedulerOperator {
    #[command(author, version, about = "Start scheduler")]
    Start(SchedulerArgs),

    #[command(about = "Stop scheduler")]
    Stop(SchedulerArgs),

    #[command(about = "Restart scheduler")]
    Restart(SchedulerArgs),
}

/// scheduler Args
#[derive(Args, Debug)]
pub struct SchedulerArgs {
    #[arg(
        short,
        long,
        help = "127.0.0.1:27021",
        default_value = "127.0.0.1:27021"
    )]
    // add help string
    address: Option<String>,
}

/// worker numbers argument for start stop and restart.
#[derive(Args, Debug)]
pub struct WorkerArgs {
    // scheduler address
    #[arg(
        short,
        long,
        help = "127.0.0.1:27021",
        default_value = "127.0.0.1:27021"
    )]
    address: Option<String>,
    #[arg(short, long)]
    name: Option<u8>,
}
