//! Mock market data.

use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use arrow_array::{ArrayRef, Float64Array, Int32Array, Int64Array};
use arrow_array::RecordBatch;
use arrow_schema::{DataType, Field, Schema};
use chrono::NaiveDate;
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::file::properties::WriterProperties;
use parquet::basic::Compression;

use crate::indicator::define::Period;
use crate::mock::price::{DayPrice, mock_price};

pub struct MarketData {
    /// The code of the Security. 证券代码
    pub code: i32,
    /// The period of the Security.
    pub period: Period,
    /// The price of the Security.
    pub day_price: Vec<(f64, NaiveDate)>,
}

impl MarketData {
    /// Construct a new market data.
    pub fn new(code: i32, period: Period) -> MarketData {
        MarketData {
            code,
            period: period.clone(),
            day_price: mock_price(period.start, period.end),
        }
    }

    /// Write mock market data to parquet file.
    pub fn write_parquet(&self, filepath: &str) {
        mock_parquet(filepath, self.code.clone(), self.period.clone());
    }
}

/// Mock market data to parquet file.
pub fn mock_parquet(filepath: &str, code: i32, period: Period) {
    // Init record batch schema
    let field_code= Field::new("code",DataType::Int32, false);
    let field_price= Field::new("price", DataType::Float64, false);
    let field_date = Field::new("date", DataType::Int64, false);

    let schema = Schema::new(vec![field_code, field_price, field_date]);

    // Create parquet file
    let path = Path::new(filepath);
    let file = File::create(path).unwrap();

    // Write properties with compression
    let props = WriterProperties::builder()
        .set_compression(Compression::SNAPPY)
        .build();

    // Init day price
    let mut day_price = DayPrice::new(period.start, period.end);

    // Setup writer
    let batch_schema = schema.clone();
    let mut writer = ArrowWriter::try_new(file,
                                          Arc::new(schema),
                                          Some(props)).unwrap();

    // Iterate mock and write
    while let Some(price) = day_price.next() {
        writer.write(&mock_batch(batch_schema.clone(), code, price)).unwrap();
    }

    // Close writer.
    writer.close().unwrap();
}

/// Mock record batch.
fn mock_batch(schema: Schema, code: i32, price: Vec<(f64, NaiveDate)>) ->
                                                                       RecordBatch {
    let mut code_vec: Vec<i32> = Vec::new();
    let mut price_vec: Vec<f64> = Vec::new();
    let mut date_vec: Vec<i64> = Vec::new();

    for (price, date) in price {
        code_vec.push(code);
        price_vec.push(price);
        date_vec.push(date.and_hms_opt(0, 0, 0).unwrap().timestamp_millis());
    }


    let code_array: ArrayRef = Arc::new(Int32Array::from(code_vec));
    let price_array: ArrayRef = Arc::new(Float64Array::from(price_vec));
    let date_array: ArrayRef = Arc::new(Int64Array::from(date_vec));

    RecordBatch::try_new(Arc::new(schema),
                         vec![code_array, price_array, date_array]).unwrap()
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::indicator::define::Period;
    use crate::mock::market::mock_parquet;

    #[test]
    fn test_mock() {
        let period = Period {
            start: NaiveDate::from_ymd_opt(2019, 1, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2019, 1, 31).unwrap(),
        };

        mock_parquet("examples/data/600000.parquet", 600001, period);
    }
}