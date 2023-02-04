//! Security returns.

use std::fs::File;
use std::io;
use arrow_array::{ArrayRef, Float64Array};
use arrow_select::window::shift;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

pub struct Returns {
    /// Price array
    prices: Float64Array
}

impl Returns {
    /// Create a new Returns struct.
    pub fn new(prices: Float64Array) -> Self {
        Self { prices }
    }

    /// Get prices array from parquet file.
    pub fn from_parquet(path: &str) -> Self {
        let prices = read_price(path).unwrap();
        Self { prices }
    }

    /// Calculate returns.
    pub fn day_returns(&self) -> Float64Array {
        let pre_prices: ArrayRef = shift(&self.prices, 1).unwrap();
        let pre_array = pre_prices.as_any().downcast_ref::<Float64Array>().unwrap();

        let returns = self.prices.iter().zip(pre_array.iter())
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
}

/// Read prices array from parquet.
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

        for i in 0..close.len() {
            builder.append_value(close.value(i));
        }
    }

    Ok(builder.finish())
}
