//! Calculate returns of a portfolio.

use std::fs::File;
use std::io;
use arrow_array::Float64Array;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

fn main(){
    // read a stock price from parquet file
    let path = "examples/data/600878.parquet";
    let prices = read_price(path).unwrap();
    println!("price length:{:?}", prices.len());
}

/// Read a stock close price from parquet file.
fn read_price(path: &str) -> Result<Vec<f64>, io::Error> {
    let file = File::open(path).unwrap();

    // New a reader builder
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();

    // New a reader
    let mut reader = builder.build().unwrap();

    // Read a batch and push to price vector
    let mut price = Vec::new();
    while let Some(batch) = reader.next() {

        let batch_data = batch.unwrap();

        // column 1 is close price: code-price-date
        let close = batch_data.column(1).as_any().downcast_ref::<Float64Array>()
            .unwrap();
        for i in 0..close.len(){
            price.push(close.value(i));
        }
    }

    Ok(price)
}