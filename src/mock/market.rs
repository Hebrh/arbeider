//! Mock market data.

use std::fs::File;
use std::sync::Arc;

use arrow_array::{ArrayRef, Int32Array};
use arrow_array::RecordBatch;
use arrow_schema::{DataType, Field, Schema};
use chrono::NaiveDate;
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::file;
use parquet::file::properties::WriterProperties;

use crate::indicator::define::Period;
use crate::mock::price::{DayPrice, mock_price};

pub struct MarketData<'a> {
    /// The code of the Security.
    pub code: &'a str,
    /// The price of the Security.
    pub day_price: Vec<(f64, NaiveDate)>,
}

impl<'a> MarketData<'a> {
    /// Construct a new market data.
    fn new(code: &'a str, period: Period) -> MarketData {
        MarketData {
            code,
            day_price: mock_price(period.start, period.end),
        }
    }

    /// Write mock market data to parquet file.
    pub fn write_parquet(&self, filepath: &str) {
        MarketData::mock_parquet(filepath, self.code, period);
    }
}

/// Mock market data to parquet file.
fn mock_parquet(filepath: &str, code: &str, period: Period) {
    // Init record batch schema
    let field_price = Field::new("a", DataType::Float64, false);
    let field_date = Field::new("b", DataType::Date64, false);

    let schema = Schema::new(vec![field_price, field_date]);

    // Create parquet file
    let file = File::create(filepath).unwrap();

    // Write properties with compression
    let props = WriterProperties::builder()
        .set_compression(file::writer::Compression::SNAPPY)
        .build();

    // Init day price
    let mut day_price = DayPrice::new(period.start, period.end);

    // Setup writer
    let mut writer = ArrowWriter::try_new(file,
                                          Arc::new(schema),
                                          Some(props)).unwrap();

    // Iterate mock and write
    while let Some(price) = day_price.next() {
        writer.write(&MarketData::mock_record_batch(code, price)).unwrap();
    }

    // Close writer.
    writer.close().unwrap();
}

/// Mock record batch.
fn mock_record_batch(code: &str, price: Vec<(f64, NaiveDate)>) -> RecordBatch {
    let mut code_vec: Vec<&str> = Vec::new();
    let mut price_vec: Vec<f64> = Vec::new();
    let mut date_vec: Vec<i64> = Vec::new();

    for (price, date) in price {
        code_vec.push(code);
        price_vec.push(price);
        date_vec.push(date.and_hms_opt(0, 0, 0).unwrap().timestamp_millis());
    }

    let code_array: ArrayRef = Arc::new(Int32Array::from(code_vec));
    let price_array: ArrayRef = Arc::new(Int32Array::from(price_vec));
    let date_array: ArrayRef = Arc::new(Int32Array::from(date_vec));

    RecordBatch::try_new(Arc::new(Schema::empty()),
                         vec![code_array, price_array, date_array]).unwrap()
}
