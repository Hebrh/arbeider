//! Clean data from csv file.

use std::fs::File;
use std::sync::Arc;
use arrow_array::{StringArray, Date32Array, Int64Array, Date64Array, Float64Array, Int32Array, RecordBatch, Array};
use arrow_array::types::{Date32Type, Date64Type};
use arrow_schema::{DataType, Field, Schema};
use chrono::NaiveDate;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::file::properties::WriterProperties;
use parquet::arrow::arrow_writer::ArrowWriter;

fn main(){

    let in_file = File::open("./examples/data/funds_prices.parquet").unwrap();

    // write to a new file
    let out_file = File::create("./examples/data/output.parquet").unwrap();

    // default writer properties
    let props = WriterProperties::builder().build();

    // writer schema.
    let field_code= Field::new("code",DataType::Int32, false);
    let field_category = Field::new("category", DataType::Utf8, false);
    let field_price= Field::new("price", DataType::Float64, false);
    let field_date = Field::new("date", DataType::Int64, false);

    let writer_schema = Schema::new(vec![field_code, field_category, field_price, field_date]);

    // new writer
    let mut writer = ArrowWriter::try_new(out_file, Arc::new(writer_schema.clone()), Some(props)).unwrap();


    let builder = ParquetRecordBatchReaderBuilder::try_new(in_file).unwrap();

    let mut reader = builder.build().unwrap();

    let mut count = 0;
    while let Some(batch) = reader.next() {

        let records:RecordBatch =  batch.unwrap();

        // get record batch column by index
        let original_code = records.column(0).clone();
        let original_date = records.column(1).clone();
        let original_price = records.column(2).clone();

        // split code from '000001.OF' to '000001' and 'OF'
        let code_str = original_code.as_any().downcast_ref::<StringArray>().unwrap();
        let code_int = code_str.iter().map(|x| x.unwrap().split('.').next()
            .unwrap()
            .parse::<i32>().unwrap()).collect::<Vec<i32>>();

        // code category. like 'OF', 'SH', 'SZ'
        let code_category: Vec<&str> = code_str.iter().map(|x| x.unwrap().split('.').last().unwrap()).collect();

        // date to NaiveDate
        let date_int = original_date.as_any().downcast_ref::<Int64Array>().unwrap();

        let naive_date = date_int.iter().map(|x|{
            // change int to string
            let date_str= x.unwrap().to_string();

            // get year, month, day
            let year = date_str[0..4].parse::<i32>().unwrap();
            let month = date_str[4..6].parse::<u32>().unwrap();
            let day = date_str[6..8].parse::<u32>().unwrap();

            // construct Naive Date
            let naive = NaiveDate::from_ymd_opt(year, month, day).unwrap();

            // convert NaiveDate to i64
            let date = naive.and_hms_opt(0, 0, 0).unwrap();
            let timestamp = date.timestamp_millis();
            timestamp

        }).collect::<Vec<i64>>();

        // new a record batch
        let code_new = Int32Array::from(code_int);
        let category_new = StringArray::from(code_category);
        let price_new =  original_price.as_any().downcast_ref::<Float64Array>().unwrap().clone();
        let date_new = Int64Array::from(naive_date);

        let records_new = RecordBatch::try_new(
            Arc::new(writer_schema.clone()),
            vec![Arc::new(code_new), Arc::new(category_new), Arc::new(price_new), Arc::new(date_new)]).unwrap();


        // write to parquet file with four columns code, category, price, date
        writer.write(&records_new).unwrap();

        count = count + 1;

        if count > 10 {
            break;
        }
    }

    writer.close().unwrap();
}