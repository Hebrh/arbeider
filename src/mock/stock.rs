//! Mock stock data.

use chrono::NaiveDate;
use rand;
use rand::Rng;

use crate::indicator::define::StockCategory;
use crate::mock::price::mock_price;

/// Shanghai Stock Exchange
pub const SH_HEADER: [&str; 4] = ["600", "601", "603", "605"];
/// Shenzhen Stock Exchange
pub const SZ_HEADER: [&str; 2] = ["000", "002"];
/// Startup Edition stock exchange. 创业版
pub const CY_HEADER: [&str; 1] = ["300"];
/// Tech board stock exchange. 科创板
pub const KB_HEADER: [&str; 1] = ["688"];

/// Stock price series
#[allow(dead_code)]
pub struct StockPrice {
    /// The code of the stock.Need map to actual stock code.
    code: i32,
    /// The category of the stock.
    category: StockCategory,
    /// The price of the stock.
    price: Vec<(f64, NaiveDate)>,
}

impl StockPrice {
    pub fn new(
        start: NaiveDate,
        end: NaiveDate,
        category: StockCategory) -> Self {
        // random a stock code
        let code: i32 = rand::thread_rng().gen_range(0..10000);

        // Construct StockPrice
        StockPrice {
            code,
            category,
            price: mock_price(start, end),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let code = code();
        println!("{}", code);
    }
}