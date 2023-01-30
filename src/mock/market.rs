//! Mock market data.

use chrono::NaiveDate;
use crate::mock::price::mock_price;
use parquet::file;

pub struct MarketData{
    /// The code of the Security.
    code: String,
    /// The price of the Security.
    day_price: Vec<(f64, NaiveDate)>,
}

impl MarketData{
    fn mock(code: String, start: NaiveDate, end: NaiveDate)-> MarketData{
        MarketData{
            code ,
            day_price: mock_price(start, end),
        }
    }

    fn mock_parquet(filepath: String, code: String, start: NaiveDate, end: NaiveDate) {

    }
}