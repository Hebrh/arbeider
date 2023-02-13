//! Calculate all funds returns.

use arbeider::cal::calculator::Calculator;

fn all_returns() {
    // Construct a calculator
    let mut calculator = Calculator::new("examples/data/".to_string());

    // read securities code list from parquet file
    let path = "examples/data/funds_prices.parquet";



}