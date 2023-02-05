//! Mock data to one file.
extern crate arbeider;
extern crate arrow_array;
extern crate parquet;

use chrono::NaiveDate;

use arbeider::indicator::define::Period;
use arbeider::mock::market::one_parquet;

fn main() {
    let period = Period {
        start: NaiveDate::from_ymd_opt(2017, 1, 1).unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 31).unwrap(),
    };

    let mut codes = vec![];
    for i in 600000..600999 {
        codes.push((i, period.clone()));
    }

    one_parquet("examples/data/600.parquet", codes);
}