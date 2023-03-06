//! Cli main.
use arb::{Cli, Commands, SchedulerOperator, WorkerOperator};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scheduler { command } => match command {
            SchedulerOperator::Start(args) => {
                println!("start scheduler");
            }
            SchedulerOperator::Stop(args) => {
                println!("stop scheduler");
            }
            SchedulerOperator::Restart(args) => {
                println!("restart scheduler");
            }
        },
        Commands::Worker { command } => match command {
            WorkerOperator::Start(args) => {
                println!("start worker");
            }
            WorkerOperator::Stop(args) => {
                println!("stop worker");
            }
            WorkerOperator::Restart(args) => {
                println!("restart worker");
            }
            WorkerOperator::List => {
                println!("list worker");
            }
        },
    }
}
