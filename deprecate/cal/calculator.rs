//! Calculator for parquet files.

use arrow_array::Float64Array;

use crate::indicator::returns::Returns;

pub struct Calculator {
    /// The parquet file path.
    pub path: String,
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
        }
    }
}

impl Calculator {
    /// Construct a calculator.
    pub fn new(path: String) -> Self {
        Self { path }
    }

    /// Calculate a workflow for a series of indicator and security match.
    /// Need to optimize the performance.
    pub fn workflow(&self, pipelines: Vec<Pipeline>) -> Vec<PipelineResult> {
        let mut results: Vec<PipelineResult> = vec![];

        // loop all pipelines
        for pipeline in pipelines {
            // Initialize a Float64 result array
            let mut values: Vec<Float64Array> = vec![Float64Array::from(vec![0.0])];

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
            let pipeline_result = PipelineResult::new(pipeline.code, pipeline.indicators, values);
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
    fn test_workflow() {
        let calculator = Calculator::new("examples/data/".to_string());
        let pipelines: Vec<Pipeline> = vec![];

        // read securities code list from parquet file
        let path = "examples/data/funds_prices.parquet";

        // construct a pipeline
        let pipeline = Pipeline {
            code: 000001,
            filepath: path.to_string(),
            indicators: vec![
                "day".to_string(),
                "cumulative".to_string(),
                "max_drawdown".to_string(),
                "square".to_string(),
                "volatility".to_string(),
                "sharpe_ratio".to_string(),
                "sortino_ratio".to_string(),
            ],
        };

        // calculate all pipelines
        let now = Instant::now();
        let results = calculator.workflow(pipelines);
        println!(
            "Time elapsed in calculate all pipelines is: {:?}",
            now.elapsed()
        );

        // loop all results
        for result in results {
            println!("code: {}", result.code);
            println!("indicators: {:?}", result.indicators);
            println!("results: {:?}", result.results);
        }
    }
}
