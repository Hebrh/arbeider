//! Calculator for parquet files.

use walkdir::WalkDir;

pub struct Calculator {
    /// The parquet file path.
    pub path: String,

    /// Parquet files chunk size.
    pub chunk: usize,

    /// Parquet files list.
    pub files: Vec<Vec<String>>,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            path: ".".to_string(),
            chunk: 10,
            files: vec![],
        }
    }
}

impl Calculator {
    /// Construct a calculator.
    pub fn new(path: String, chunk: usize) -> Self {
        Self {
            path,
            chunk,
            files: vec![],
        }
    }

    /// Count parquet files and split into files by chunk size.
    pub fn split(&mut self) {
        let mut files = vec![];
        let mut chunk = vec![];
        let walkdir = WalkDir::new(&self.path).into_iter().filter_map(|e| e.ok());
        for entry in walkdir {
            if entry.file_type().is_file() {
                // if no extension
                if entry.path().extension().is_none() {
                    continue;
                }
                // if extension is parquet
                if entry.path().extension().unwrap() == "parquet" {
                    chunk.push(entry.path().to_str().unwrap().to_string());
                }
            }
            // chunk is full, push to files and construct a new chunk
            if chunk.len() == self.chunk {
                files.push(chunk);
                chunk = vec![];
            }
        }

        // left files
        if chunk.len() > 0 {
            files.push(chunk);
        }
        self.files = files;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let mut calculator = Calculator::new("examples/data/".to_string(), 50);
        calculator.split();
        println!("{:?}", calculator.files.len());
        assert_eq!(calculator.files.len(), 20);
        assert_eq!(calculator.files[0].len(), 50);
        assert_eq!(calculator.files[1].len(), 50);
        assert_eq!(calculator.files[19].len(), 48);
    }
}