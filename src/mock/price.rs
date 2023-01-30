//! Mock stock price.

use rand;
use chrono::{NaiveDate, Days};
use rand::Rng;

pub struct DayPrice{
    start: NaiveDate,
    end: NaiveDate,
    now: NaiveDate,
}

impl DayPrice{
    pub fn new(start: NaiveDate, end: NaiveDate)-> DayPrice{
        DayPrice{
            start,
            end,
            now: start,
        }
    }
}

impl Iterator for DayPrice{
    type Item = Vec<(f64, NaiveDate)>;

    fn next(&mut self) -> Option<Self::Item> {
        let batch_size = 1024;

        println!("now: {}, end: {}", self.now, self.end);

        // over the end
        if self.now > self.end {
            return None;
        }

        // will to the end
        if self.now.checked_add_days(Days::new(batch_size)).unwrap()  > self.end {
            let price = mock_price(self.now, self.end);
            self.now = self.end.checked_add_days(Days::new(1)).unwrap();
            return Some(price);
        }

        // have enough size to read
        let add_days = self.now.checked_add_days(Days::new(batch_size)).unwrap();
        let price = mock_price(self.now, add_days);
        self.now = add_days.checked_add_days(Days::new(1)).unwrap();
        return Some(price);
    }
}

/// Mock stock price from start day to end day.
///
/// Do not consider the holiday.
/// # Arguments
/// * `start` - The start day of the price.
/// * `end` - The end day of the price.
/// # Returns
/// * `Vec<f64,NaiveDate>` - The price of the stock from start to end day.
///                        - The day of the price.
pub fn mock_price(start: NaiveDate, end: NaiveDate)-> Vec<(f64, NaiveDate)>{
    // price list with day
    let mut day_price = Vec::new();
    let mut rng = rand::thread_rng();
    let mut last = rng.gen_range(0.0..100.0);

    // price change range
    let mut delta = rng.gen_range(-0.1..0.1);
    let mut date = start;

    // loop until the end day
    while date <= end {
        last += delta;
        day_price.push((last,date));
        date = date.succ_opt().unwrap();
        delta = rng.gen_range(-0.1..0.1);
    }
    day_price
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_price() {
        let start = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2020, 1, 31).unwrap();
        let price = mock_price(start, end);
        println!("{:?}", price);
    }

    #[test]
    fn test_day_price() {
        let start = NaiveDate::from_ymd_opt(2016, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2020, 1, 31).unwrap();

        // init day price
        let mut day_price = DayPrice::new(start, end);

        // iterator mock price
        while let Some(price) = day_price.next() {
            println!("{:?}", price.len());
        }
    }
}