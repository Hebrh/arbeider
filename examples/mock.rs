//! Mock data.

extern crate arbeider;
extern crate arrow_array;
extern crate parquet;

use chrono::NaiveDate;

use arbeider::indicator::define::Period;
use arbeider::mock::market::mock_parquet;

fn main() {
    let period = Period {
        start: NaiveDate::from_ymd_opt(2017, 1, 1).unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 31).unwrap(),
    };

    for i in 600000..600999 {
        mock_parquet(format!("examples/data/{}.parquet", i).as_str(), i, period);
    }
}