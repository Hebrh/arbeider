//! Mock data.

extern crate arbeider;
extern crate arrow_array;
extern crate parquet;

use chrono::NaiveDate;

use arbeider::indicator::define::Period;
use arbeider::mock::market::mock_parquet;

fn main() {
    let period = Period {
        start: NaiveDate::from_ymd_opt(2019, 1, 1).unwrap(),
        end: NaiveDate::from_ymd_opt(2020, 1, 31).unwrap(),
    };

    mock_parquet("examples/data/600001.parquet", 600001, period);
}