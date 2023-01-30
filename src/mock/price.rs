//! Mock stock price.

use rand;
use chrono::NaiveDate;
use rand::Rng;


/// Mock year's stock price.
///
/// Do not consider the holiday.
/// # Arguments
/// * `start` - The start day of the price.
/// * `end` - The end day of the price.
/// # Returns
/// * `Vec<f64>` - The price of the stock from start to end day.
pub fn price(start: NaiveDate, end: NaiveDate)-> Vec<f64>{
    let mut price = Vec::new();
    let mut rng = rand::thread_rng();
    let mut last = rng.gen_range(0.0..100.0);
    let mut delta = rng.gen_range(-0.1..0.1);
    let mut date = start;
    while date <= end {
        last += delta;
        price.push(last);
        date = date.succ_opt().unwrap();
        delta = rng.gen_range(-0.1..0.1);
    }
    price
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_price() {
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2020, 1, 31).unwrap();
        let price = price(start, end);
        println!("{:?}", price);
    }
}