//! Calculator for parquet files.

use arrow_array::Float64Array;
use walkdir::WalkDir;

use crate::indicator::returns::Returns;

pub struct Calculator {
    /// The parquet file path.
    pub path: String,

    /// Parquet file path list.
    pub filepaths: Vec<String>,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            path: ".".to_string(),
            filepaths: vec![],
        }
    }
}

impl Calculator {
    /// Construct a calculator.
    pub fn new(path: String) -> Self {
        Self {
            path,
            filepaths: vec![],
        }
    }

    /// Read parquet files from path.
    pub fn read_files(&mut self) {
        let walkdir = WalkDir::new(&self.path).into_iter().filter_map(|e| e.ok());
        for entry in walkdir {
            if entry.file_type().is_file() {
                // if no extension
                if entry.path().extension().is_none() {
                    continue;
                }
                // if extension is parquet
                if entry.path().extension().unwrap() == "parquet" {
                    self.filepaths.push(entry.path().to_str().unwrap().to_string());
                }
            }
        }
    }

    // Calculate day returns from a parquet file
    pub fn day_returns(&self, path: &str) -> Float64Array {
        let returns = Returns::from_parquet(path);
        returns.day_returns()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[test]
    fn test_read() {
        // Calculate read file time
        let mut calculator = Calculator::new("examples/data/".to_string());
        calculator.read_files();
        assert!(calculator.filepaths.len() > 0);
    }


    #[test]
    fn test_find(){
        // Calculate find file name time
        let mut calculator = Calculator::new("examples/data/".to_string());
        calculator.read_files();

        let start = Instant::now();
        calculator.day_returns("examples/data/600878.parquet");
        let duration = start.elapsed();
        println!("Time elapsed in find() is: {:?}", duration);
    }
}