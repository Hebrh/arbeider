//! Clean data from csv file.

use std::fs::{File, OpenOptions};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use arrow_array::{Array, Float64Array, Int32Array, Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::arrow_writer::ArrowWriter;

fn main() {
    // one_parquet();
    // let list = code_list();
    split();
}

// split code and write to multiple parquet files
// code : 0-3000; 3000-6000; 6000-9000; 9000-12000;
// 12000-15000; 15000-18000; above 18000
fn split() {
    let split_points = [2000, 4000, 6000, 11000, 130000, 500000];

    // reader
    let in_file = File::open("./examples/data/one.parquet").unwrap();
    let builder = ParquetRecordBatchReaderBuilder::try_new(in_file).unwrap();
    let reader = builder.build().unwrap();

    // writer schema.
    let schema = schema();

    // construct 7 writers
    let mut writers: Vec<ArrowWriter<File>> = Vec::new();
    for i in 0..(split_points.len() + 1) {
        let file_name = if i < split_points.len() {
            format!("./examples/data/{:06}.parquet", split_points[i])
        } else {
            "./examples/data/above.parquet".to_string()
        };

        let out_file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name)
            .unwrap();

        let writer = ArrowWriter::try_new(out_file, Arc::new(schema.clone()), None).unwrap();
        writers.push(writer);
    }

    // batch read and write to different files
    for batch in reader {
        // add sleep to free cpu
        thread::sleep(Duration::from_millis(100));

        let batch_clone = batch.unwrap();

        let code_array = batch_clone.column(0).clone();
        let category_array = batch_clone.column(1).clone();
        let price_array = batch_clone.column(2).clone();
        let date_array = batch_clone.column(3).clone();

        // construct 7 list to store records
        let mut code_lists: Vec<Vec<i32>> = vec![vec![]; 7];
        let mut category_lists: Vec<Vec<i32>> = vec![vec![]; 7];
        let mut price_lists: Vec<Vec<f64>> = vec![vec![]; 7];
        let mut date_lists: Vec<Vec<i64>> = vec![vec![]; 7];

        // loop read and write
        for i in 0..code_array.len() {
            let code = code_array
                .as_any()
                .downcast_ref::<Int32Array>()
                .unwrap()
                .value(i);

            // write to file
            let category = category_array
                .as_any()
                .downcast_ref::<Int32Array>()
                .unwrap()
                .value(i);

            let price = price_array
                .as_any()
                .downcast_ref::<Float64Array>()
                .unwrap()
                .value(i);

            let date = date_array
                .as_any()
                .downcast_ref::<Int64Array>()
                .unwrap()
                .value(i);

            // cal index
            let index = if code < split_points[0] {
                0
            } else if code < split_points[1] {
                1
            } else if code < split_points[2] {
                2
            } else if code < split_points[3] {
                3
            } else if code < split_points[4] {
                4
            } else if code < split_points[5] {
                5
            } else {
                6
            };

            // push data to list
            code_lists[index].push(code);
            category_lists[index].push(category);
            price_lists[index].push(price);
            date_lists[index].push(date);
        }

        // flush record to file
        for i in 0..7 {
            let code_array = Int32Array::from(code_lists[i].clone());
            let category_array = Int32Array::from(category_lists[i].clone());
            let price_array = Float64Array::from(price_lists[i].clone());
            let date_array = Int64Array::from(date_lists[i].clone());

            let batch = RecordBatch::try_new(
                Arc::new(schema.clone()),
                vec![
                    Arc::new(code_array),
                    Arc::new(category_array),
                    Arc::new(price_array),
                    Arc::new(date_array),
                ],
            )
            .unwrap();

            writers[i].write(&batch).unwrap();
            writers[i].flush().unwrap();
        }
    }

    // close writers
    for writer in writers {
        writer.close().unwrap();
    }
}

fn schema() -> Schema {
    let field_code = Field::new("code", DataType::Int32, false);
    let field_category = Field::new("category", DataType::Int32, false);
    let field_price = Field::new("price", DataType::Float64, false);
    let field_date = Field::new("date", DataType::Int64, false);

    Schema::new(vec![field_code, field_category, field_price, field_date])
}

#[allow(dead_code)]
// Split code and write to one parquet file
fn one_parquet() {
    // new reader
    let in_file = File::open("./examples/data/funds.parquet").unwrap();
    let builder = ParquetRecordBatchReaderBuilder::try_new(in_file).unwrap();

    let reader = builder.build().unwrap();

    // writer schema.
    let field_code = Field::new("code", DataType::Int32, false);
    let field_category = Field::new("category", DataType::Int32, false);
    let field_price = Field::new("price", DataType::Float64, false);
    let field_date = Field::new("date", DataType::Int64, false);

    let writer_schema = Schema::new(vec![field_code, field_category, field_price, field_date]);

    // output file
    let mut out_file: File = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./examples/data/one.parquet")
        .unwrap();

    // writer handle
    let mut writer =
        ArrowWriter::try_new(&mut out_file, Arc::new(writer_schema.clone()), None).unwrap();

    for batch in reader {
        let records: RecordBatch = batch.unwrap();

        let code_array = records.column(0).clone();
        let date_array = records.column(1).clone();
        let price_array = records.column(2).clone();

        // construct new code_array without category
        let mut code_list: Vec<i32> = Vec::new();
        let mut code_error = vec![];

        for i in 0..code_array.len() {
            let code = code_array
                .as_any()
                .downcast_ref::<StringArray>()
                .unwrap()
                .value(i);
            let code_split = code.split('.').next().unwrap().parse::<i32>();
            let only_code = match code_split {
                Ok(_v) => _v,
                Err(_e) => {
                    code_error.push(i);
                    continue;
                }
            };
            // append to code list
            code_list.push(only_code);
        }
        let new_code_array = Int32Array::from(code_list);

        // split code_array and new category
        let mut category_list: Vec<i32> = Vec::new();
        for i in 0..code_array.len() {
            let code = code_array
                .as_any()
                .downcast_ref::<StringArray>()
                .unwrap()
                .value(i);
            let category = code.split('.').last().unwrap();

            if code_error.contains(&i) {
                continue;
            }

            if category == "SH" {
                category_list.push(1);
            } else if category == "SZ" {
                category_list.push(2);
            } else if category == "CY" {
                category_list.push(3);
            } else if category == "KB" {
                category_list.push(4);
            } else if category == "OF" {
                // funds
                category_list.push(5);
            } else {
                category_list.push(0);
            }
        }
        let category_array = Int32Array::from(category_list);

        // construct new price_array
        let mut price_list: Vec<f64> = Vec::new();
        for i in 0..price_array.len() {
            let price = price_array
                .as_any()
                .downcast_ref::<Float64Array>()
                .unwrap()
                .value(i);

            if code_error.contains(&i) {
                continue;
            }

            price_list.push(price);
        }
        let price_array = Float64Array::from(price_list);

        // construct new date_array
        let mut date_list: Vec<i64> = Vec::new();
        for i in 0..date_array.len() {
            let date = date_array
                .as_any()
                .downcast_ref::<Int64Array>()
                .unwrap()
                .value(i);

            if code_error.contains(&i) {
                continue;
            }

            date_list.push(date);
        }
        let date_array = Int64Array::from(date_list);

        // construct new batch
        let new_batch = RecordBatch::try_new(
            Arc::new(writer_schema.clone()),
            vec![
                Arc::new(new_code_array),
                Arc::new(category_array),
                Arc::new(price_array),
                Arc::new(date_array),
            ],
        )
        .unwrap();

        writer.write(&new_batch).unwrap();
    }

    // close writer
    writer.close().unwrap();
}

#[allow(dead_code)]
// get all code and counts from funds.parquet
fn code_list() -> Vec<(i32, i32)> {
    let mut code_list: Vec<(i32, i32)> = Vec::new();

    // new reader
    let in_file = File::open("./examples/data/funds.parquet").unwrap();
    let builder = ParquetRecordBatchReaderBuilder::try_new(in_file).unwrap();

    let reader = builder.build().unwrap();

    // loop check all batch
    for batch in reader {
        // add sleep
        thread::sleep(Duration::from_millis(100));

        let records: RecordBatch = batch.unwrap();
        let code_array = records.column(0).clone();

        for i in 0..code_array.len() {
            let code = code_array
                .as_any()
                .downcast_ref::<StringArray>()
                .unwrap()
                .value(i);
            let code_split = code.split('.').next().unwrap().parse::<i32>();
            let only_code = match code_split {
                Ok(_v) => _v,
                Err(_e) => {
                    continue;
                }
            };

            // check if has same code in code_list
            let mut has_code = false;
            for (code, count) in code_list.iter_mut() {
                if *code == only_code {
                    println!("match :{code},{count}");
                    *count += 1;
                    has_code = true;
                    break;
                }
            }

            // if not has same code, append to code list
            if !has_code {
                code_list.push((only_code, 1));
            }
        }
    }

    code_list
}
