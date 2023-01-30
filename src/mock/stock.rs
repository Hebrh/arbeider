//! Mock stock data.

use rand;
use rand::Rng;
use chrono::NaiveDate;
use crate::mock::price::price;

/// Shanghai Stock Exchange
pub const SH_HEADER: [&str; 4]  = ["600", "601", "603", "605"];
/// Shenzhen Stock Exchange
pub const SZ_HEADER: [&str; 2]  = ["000", "002"];
/// Startup Edition stock exchange. 创业版
pub const CY_HEADER: [&str; 1]  = ["300"];
/// Tech board stock exchange. 科创板
pub const KB_HEADER: [&str; 1]  = ["688"];

/// Stock price series
pub struct StockPrice{
    /// The code of the stock.
    code: String,
    /// The price of the stock.
    price: Vec<f64>,
}

impl StockPrice{
    pub fn new(start:NaiveDate, end: NaiveDate )-> StockPrice{
        StockPrice{
            code: code(),
            price: price(start, end),
        }
    }
}


/// mock stock code
fn code()-> String{
    let mut code = String::new();
    let mut rng = rand::thread_rng();

    let header = code_header();
    code.push_str(header.as_str());
    for _ in 0..3 {
        code.push_str(&rng.gen_range(0..10).to_string());
    }
    code
}

/// Randon a stock code header
fn code_header()-> String{
    let mut rng = rand::thread_rng();
    let header = match rng.gen_range(0..4) {
        0 => SH_HEADER[rng.gen_range(0..SH_HEADER.len())].to_string(),
        1 => SZ_HEADER[rng.gen_range(0..SZ_HEADER.len())].to_string(),
        2 => CY_HEADER[rng.gen_range(0..CY_HEADER.len())].to_string(),
        3 => KB_HEADER[rng.gen_range(0..KB_HEADER.len())].to_string(),
        _ => SH_HEADER[rng.gen_range(0..SH_HEADER.len())].to_string(),
    };
    header
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