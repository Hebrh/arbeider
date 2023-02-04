//! Calculate returns of a portfolio.

use std::fs::File;
use std::io;
use arrow_array::{Array, ArrayRef, Float64Array};
use arrow_select::window::shift;
use std::time::Instant;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

fn main(){
    // read a stock price from parquet file
    let path = "examples/data/600878.parquet";
    let prices: Float64Array = read_price(path).unwrap();
    println!("price length:{:?}", prices.len());

    // calculate returns
    let start = Instant::now();
    let result = cal_returns(&prices);
    let duration = start.elapsed();
    println!("returns length:{:?}", result.len());
    println!("returns {:?}", result);
    println!("cal_returns time: {:?}", duration);
}

fn cal_returns(prices: &Float64Array) -> Float64Array {
    let pre_prices: ArrayRef = shift(&prices, 1).unwrap();
    let pre_array = pre_prices.as_any().downcast_ref::<Float64Array>().unwrap();

    let returns = prices.iter().zip(pre_array.iter())
        .map(|(a, b)| {
            if let (Some(a), Some(b)) = (a, b) {
                Some((a - b) / b)
            } else {
                None
            }
        })
        .collect::<Float64Array>();

    returns
}

/// Read a stock close price from parquet file.
fn read_price(path: &str) -> Result<Float64Array, io::Error> {
    let file = File::open(path).unwrap();

    // New a reader builder
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();

    // New a reader
    let mut reader = builder.build().unwrap();

    // New a builder
    let mut builder = Float64Array::builder(0);

    // Read a batch and push to price vector
    while let Some(batch) = reader.next() {

        let batch_data = batch.unwrap();

        // column 1 is close price: code-price-date
        let close = batch_data.column(1).as_any().downcast_ref::<Float64Array>()
            .unwrap();

        builder.extend(close);
    }

    Ok(builder.finish())
}