//! Mock data.

extern crate arbeider;

use chrono::NaiveDate;

use arbeider::indicator::define::Period;
use arbeider::mock::market::MarketData;

fn main() {
    let period = Period {
        start: NaiveDate::from_ymd_opt(2017, 1, 1).unwrap(),
        end: NaiveDate::from_ymd_opt(2022, 1, 31).unwrap(),
    };

    for i in 601000..601010 {
        let market = MarketData::new(i, period.clone());
        market.write_parquet(format!("examples/data/{}.parquet", i).as_str());
    }
}