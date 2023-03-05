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
    /// Schema of the market data.
    pub schema: Schema,
}

impl MarketData {
    /// Construct a new market data.
    pub fn new(code: i32, period: Period) -> MarketData {
        MarketData {
            code,
            period: period.clone(),
            day_price: mock_price(period.start, period.end),
            schema: MarketData::schema(),
        }
    }

    /// Set market data schema.
    /// code - int32; price - float64; date - int64
    pub fn schema() -> Schema {
        // Init record batch schema
        let field_code= Field::new("code",DataType::Int32, false);
        let field_price= Field::new("price", DataType::Float64, false);
        let field_date = Field::new("date", DataType::Int64, false);

        Schema::new(vec![field_code, field_price, field_date])
    }

    /// Mock record batch.
    /// date type change to timestamp. i64
    fn mock_batch(&self, price: Vec<(f64, NaiveDate)>) ->
    RecordBatch {
        let mut code_vec: Vec<i32> = Vec::new();
        let mut price_vec: Vec<f64> = Vec::new();
        let mut date_vec: Vec<i64> = Vec::new();

        for (price, date) in price {
            code_vec.push(self.code);
            price_vec.push(price);
            date_vec.push(date.and_hms_opt(0, 0, 0).unwrap().timestamp_millis());
        }


        let code_array: ArrayRef = Arc::new(Int32Array::from(code_vec));
        let price_array: ArrayRef = Arc::new(Float64Array::from(price_vec));
        let date_array: ArrayRef = Arc::new(Int64Array::from(date_vec));

        RecordBatch::try_new(Arc::new(self.schema.clone()),
                             vec![code_array, price_array, date_array]).unwrap()
    }

    /// Mock market data to parquet file.
    /// Each code will generate a parquet file.
    /// # Arguments
    /// * `filepath` - The path of the parquet file.
    fn mock_parquet(&self, filepath: &str) {
        let schema = self.schema.clone();

        // Create parquet file
        let path = Path::new(filepath);
        let file = File::create(path).unwrap();

        // Write properties with compression
        let props = WriterProperties::builder()
            .set_compression(Compression::SNAPPY)
            .build();

        // Init day price
        let mut day_price = DayPrice::new(self.period.start, self.period.end);

        // Setup writer
        let mut writer = ArrowWriter::try_new(file,
                                              Arc::new(schema),
                                              Some(props)).unwrap();

        // Iterate mock and write
        while let Some(price) = day_price.next() {
            let batch = self.mock_batch(price.clone());
            writer.write(&batch).unwrap();
        }

        // Close writer.
        writer.close().unwrap();
    }

    /// Write mock market data to parquet file.
    pub fn write_parquet(&self, filepath: &str) {
        self.mock_parquet(filepath);
    }
}

/// Mock market data to parquet file.
/// All code will generate a parquet file.
pub fn one_parquet(filepath: &str, codes:Vec<(i32, Period)>) {
    // Init schema
    let schema = MarketData::schema();

    // Create parquet file
    let path = Path::new(filepath);
    let file = File::create(path).unwrap();

    // Write properties with compression
    let props = WriterProperties::builder()
        .set_compression(Compression::SNAPPY)
        .build();

    // Setup writer
    let mut writer = ArrowWriter::try_new(file,
                                          Arc::new(schema),
                                          Some(props)).unwrap();

    // Iterate mock and write
    for (code, period) in codes {
        let market = MarketData::new(code, period);
        let mut day_price = DayPrice::new(market.period.start, market.period.end);

        while let Some(price) = day_price.next() {
            let batch = market.mock_batch(price.clone());
            writer.write(&batch).unwrap();
        }
    }

    // Close writer.
    writer.close().unwrap();
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::indicator::define::Period;
    use crate::mock::market::MarketData;

    #[test]
    fn test_mock() {
        let period = Period {
            start: NaiveDate::from_ymd_opt(2019, 1, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2019, 1, 31).unwrap(),
        };

        let market = MarketData::new(600000, period);
        market.write_parquet("600000.parquet");
    }
}