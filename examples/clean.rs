//! Clean data from csv file.

use std::fs::File;
use std::sync::Arc;
use arrow_schema::{DataType, Field, Schema};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::file::properties::WriterProperties;
use parquet::arrow::arrow_writer::ArrowWriter;

fn main(){

    let in_file = File::open("/examples/data/funds_prices.parquet").unwrap();

    // write to a new file
    let out_file = File::create("data.parquet").unwrap();

    // default writer properties
    let props = WriterProperties::builder().build();

    // writer schema.
    let field_code= Field::new("code",DataType::Int32, false);
    let field_category = Field::new("category", DataType::Int32, false);
    let field_price= Field::new("price", DataType::Float64, false);
    let field_date = Field::new("date", DataType::Int64, false);

    let writer_schema = Schema::new(vec![field_code, field_category, field_price, field_date]);

    // new writer
    let mut writer = ArrowWriter::try_new(out_file, Arc::new(writer_schema),
                                          Some(props)).unwrap();


    let builder = ParquetRecordBatchReaderBuilder::try_new(in_file).unwrap();
    println!("Converted arrow schema is: {}", builder.schema());

    let mut reader = builder.build().unwrap();

    let mut count = 0;
    while let Some(batch) = reader.next() {
        println!("Read {} records.", batch.unwrap().num_rows());

        count = count + 1;

        if count > 3 {
            break;
        }

        // three columns names: SRC_SECU_CODE, PRICE_DATE, F_NAV_ADJUSTED
        // 000001.OF    20011218          1.0000

        // get column by name
    }
}