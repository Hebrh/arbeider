//! Security returns.

use arrow_array::Float64Array;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::fs::File;
use std::io;

#[allow(dead_code)]
pub struct Returns {
    /// Price array
    prices: Float64Array,

    /// Security code
    code: i32,
}

impl Returns {
    /// Create a new Returns struct.
    pub fn new(prices: Float64Array, code: i32) -> Self {
        Self { prices, code }
    }

    /// Get prices array from parquet file.
    pub fn from_parquet(path: &str, code: i32) -> Self {
        let prices = read_price(path).unwrap();
        Self { prices, code }
    }

    /// Calculate returns.
    pub fn day(&self) -> Float64Array {
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

    /// Cumulative returns.
    pub fn cumulative(&self) -> f64 {
        // Calculate cumulative returns from index 1
        let mut cum_ret = 0.0;
        for i in 1..self.prices.len() {
            let pre_price = self.prices.value(i - 1);
            let price = self.prices.value(i);
            let ret = (price - pre_price) / pre_price;
            cum_ret += ret;
        }
        cum_ret
    }

    /// Max drawdown.
    pub fn max_drawdown(&self) -> f64 {
        // Calculate max drawdown from index 1
        let mut max_drawdown = 0.0;
        let mut max_price = self.prices.value(0);
        for i in 1..self.prices.len() {
            let price = self.prices.value(i);
            if price > max_price {
                max_price = price;
            }
            let drawdown = (max_price - price) / max_price;
            if drawdown > max_drawdown {
                max_drawdown = drawdown;
            }
        }
        max_drawdown
    }

    /// Square returns.
    pub fn square(&self) -> Float64Array {
        // New a builder
        let mut builder = Float64Array::builder(0);

        // Calculate square returns from index 1
        for i in 1..self.prices.len() {
            let pre_price = self.prices.value(i - 1);
            let price = self.prices.value(i);
            let ret = (price - pre_price) / pre_price;
            builder.append_value(ret * ret);
        }
        builder.finish()
    }

    /// Volatility.
    pub fn volatility(&self) -> f64 {
        // Calculate day return.
        let day_returns = self.day();

        // Calculate square returns.
        let square_returns = self.square();

        // Calculate volatility.
        // sum day returns
        let sum_day = day_returns.values().iter().sum::<f64>();
        // let sum_day = day_returns.iter().sum::<f64>();
        let sum_square = square_returns.values().iter().sum::<f64>();

        // Calculate volatility.
        let mean = sum_day / (self.prices.len() - 1) as f64;
        (sum_square / (self.prices.len() - 1) as f64 - mean * mean).sqrt()
    }

    /// Sharpe ratio.
    pub fn sharpe_ratio(&self) -> f64 {
        // Calculate day return sum.
        let sum_day = self.day().values().iter().sum::<f64>();

        // Calculate returns square sum.
        let sum_square = self.square().values().iter().sum::<f64>();

        // Calculate mean.
        let mean = sum_day / (self.prices.len() - 1) as f64;

        // Calculate std.
        let std = (sum_square / (self.prices.len() - 1) as f64 - mean * mean).sqrt();

        // Calculate sharpe ratio.
        mean / std
    }

    /// Sortino ratio.
    pub fn sortino_ratio(&self) -> f64 {
        // Calculate sortino ratio from index 1
        let mut cum_ret = 0.0;
        let mut cum_ret2 = 0.0;
        for i in 1..self.prices.len() {
            let pre_price = self.prices.value(i - 1);
            let price = self.prices.value(i);
            let ret = (price - pre_price) / pre_price;
            cum_ret += ret;
            if ret < 0.0 {
                cum_ret2 += ret * ret;
            }
        }
        let mean = cum_ret / (self.prices.len() - 1) as f64;
        let std = (cum_ret2 / (self.prices.len() - 1) as f64 - mean * mean).sqrt();
        mean / std
    }
}

/// Read prices array from parquet.
fn read_price(path: &str) -> Result<Float64Array, io::Error> {
    let file = File::open(path).unwrap();

    // New a reader builder
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();

    // New a reader
    let reader = builder.build().unwrap();

    // New a builder
    let mut builder = Float64Array::builder(0);

    // Read a batch and push to price vector
    for batch in reader {
        let batch_data = batch.unwrap();

        // column 1 is close price: code-price-date
        let close = batch_data
            .column(1)
            .as_any()
            .downcast_ref::<Float64Array>()
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
        let returns = Returns::from_parquet(path, 600878);
        let mut duration = start.elapsed();
        println!("read parquet time: {duration:?}");

        // calculate returns
        start = Instant::now();
        let result = returns.day();
        duration = start.elapsed();
        println!("returns length:{:?}", result.len());
        println!("cal_returns time: {duration:?}");

        assert!(!result.is_empty());
    }

    #[test]
    fn test_read_time() {
        // read a stock price from parquet file
        let mut path = "examples/data/600.parquet";

        let mut start = Instant::now();
        let mut returns = Returns::from_parquet(path, 600878);
        let mut duration = start.elapsed();
        println!("read {path} parquet time: {duration:?}");
        println!("returns length:{:?}", returns.prices.len());

        path = "examples/data/600601.parquet";
        start = Instant::now();
        returns = Returns::from_parquet(path, 600878);
        duration = start.elapsed();
        println!("read {path} parquet time: {duration:?}");

        assert!(!returns.prices.is_empty());
        // result:
        // read examples/data/600.parquet parquet time: 466.52525ms
        // returns length:1855143
        // read examples/data/600601.parquet parquet time: 1.749459ms
    }
}
