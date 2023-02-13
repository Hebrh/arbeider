//! Calculator for parquet files.

use arrow_array::Float64Array;
use walkdir::WalkDir;

use paste::paste;
use crate::indicator::returns::Returns;

pub struct Calculator {
    /// The parquet file path.
    pub path: String,

    /// Parquet file path list.
    pub filepaths: Vec<String>,
}

/// Pipeline struct for calculate a series of indicator and security match.
pub struct Pipeline {
    /// Security code.
    pub code: i32,

    /// Data origin. Parquet file
    pub filepath: String,

    /// Indicator series.
    pub indicators: Vec<String>,
}

pub struct PipelineResult {
    /// Security code.
    pub code: i32,

    /// Indicator series.
    pub indicators: Vec<String>,

    /// Indicator result.
    pub results: Vec<Float64Array>,
}

impl Default for PipelineResult {
    fn default() -> Self {
        Self {
            code: -1,
            indicators: vec![],
            results: vec![],
        }
    }
}

impl PipelineResult {
    /// Construct a pipeline result.
    pub fn new(code: i32, indicators: Vec<String>, results: Vec<Float64Array>) -> Self {
        Self {
            code,
            indicators,
            results,
        }
    }
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

    /// Calculate a workflow for a series of indicator and security match.
    /// Need to optimize the performance.
    pub fn workflow(&self, pipelines: Vec<Pipeline>) -> Vec<PipelineResult> {
        let mut results: Vec<PipelineResult> = vec![];

        // loop all pipelines
        for pipeline in pipelines {
            // Initialize a Float64 result array
            let mut values:Vec<Float64Array> = vec![Float64Array::from
                (vec![0.0])];

            // loop all indicators
            for indicator in &pipeline.indicators {
                // Construct a return indicator struct
                let filepath = pipeline.filepath.clone();
                let code = pipeline.code.clone();
                let returns = Returns::from_parquet(&filepath, code);

                // calculate the indicator, get result value
                match indicator.as_str() {
                    "day" => {
                        values.push(returns.day());
                    }
                    "cumulative" => {
                        values.push(Float64Array::from(vec![returns.cumulative()]));
                    }
                    "max_drawdown" => {
                        values.push(Float64Array::from(vec![returns.max_drawdown()]));
                    }
                    "square" => {
                        values.push(returns.square());
                    }
                    "volatility" => {
                        values.push(Float64Array::from(vec![returns.volatility()]));
                    }
                    "sharpe_ratio" => {
                        values.push(Float64Array::from(vec![returns.sharpe_ratio()]));
                    }
                    "sortino_ratio" => {
                        values.push(Float64Array::from(vec![returns.sortino_ratio()]));
                    }
                    _ => {
                        panic!("No indicator named {}", indicator);
                    }
                }
            }

            // construct a pipeline result
            let pipeline_result = PipelineResult::new(pipeline.code, pipeline
                .indicators, values);
            results.push(pipeline_result);
        }
        results
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
    fn test_find() {
        // Calculate find file name time
        let mut calculator = Calculator::new("examples/data/".to_string());
        calculator.read_files();

        let start = Instant::now();
        calculator.day_returns("examples/data/600878.parquet");
        let duration = start.elapsed();
        println!("Time elapsed in find() is: {:?}", duration);
    }
}