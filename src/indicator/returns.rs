//! Security returns.

use std::fs::File;
use std::io;
use arrow_array::Float64Array;
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
        // New a builder
        let mut builder = Float64Array::builder(0);

        // Calculate returns from index 1
        for i in 1..self.prices.len() {
            let pre_price = self.prices.value(i - 1);
            let price = self.prices.value(i);
            let ret = (price - pre_price) / pre_price;
            builder.append_value(ret);
        }
        builder.finish()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_day_returns() {
        // read a stock price from parquet file
        let path = "examples/data/600878.parquet";


        let mut start = Instant::now();
        let returns = Returns::from_parquet(path);
        let mut duration = start.elapsed();
        println!("read parquet time: {:?}", duration);

        // calculate returns
        start = Instant::now();
        let result = returns.day_returns();
        duration = start.elapsed();
        println!("returns length:{:?}", result.len());
        println!("cal_returns time: {:?}", duration);

        assert!(result.len() > 0);
    }
}
