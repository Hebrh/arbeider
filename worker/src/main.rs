//! Worker main.

use clap::Parser;
use worker::{Cli, Executor, Operator};

fn main() {
    let cli = Cli::parse();

    match &cli.operator {
        Operator::Start(args) => {
            println!("start worker:{args:?}");
            // start executor
            let executor = Executor::new(args.scheduler.clone());
            executor.start();
        }
        Operator::Stop(args) => {
            println!("stop worker:{args:?}");
        }
        Operator::Restart(args) => {
            println!("restart worker:{args:?}");
        }
    }
}
