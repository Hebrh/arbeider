//! Calculate returns of a portfolio.

use std::time::Instant;

use arbeider::indicator::returns::Returns;

fn main() {
    // read a stock price from parquet file
    let path = "examples/data/600878.parquet";

    // New Return class
    let returns = Returns::from_parquet(path);

    // calculate returns
    let start = Instant::now();
    let result = returns.day_returns();
    let duration = start.elapsed();
    println!("returns length:{:?}", result.len());
    println!("returns {:?}", result);
    println!("cal_returns time: {:?}", duration);
}