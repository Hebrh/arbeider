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
    use arrow_array::Int32Array;
    use arrow_select::window::shift;

    #[test]
    fn test_split() {
        let mut calculator = Calculator::new("examples/data/".to_string(), 50);
        calculator.split();
        println!("{:?}", calculator.files.len());
        assert_eq!(calculator.files.len(), 20);
        assert_eq!(calculator.files[0].len(), 50);
        assert_eq!(calculator.files[1].len(), 50);
        // assert_eq!(calculator.files[19].len(), 48);
    }

    #[test]
    fn test_shift() {
        let a: Int32Array = vec![Some(1),
                                 Some(2),
                                 Some(3),
                                 Some(4)].into();
        let res = shift(&a, 1).unwrap();
        println!("res:{:?}", res);

        let b = shift(&a, 1).unwrap();
        println!("b:{:?}", b);

        let b_array = b.as_any().downcast_ref::<Int32Array>().unwrap();

        let c= a.iter().zip(b_array.iter())
            .map(|(a, b)| {
                println!("a:{:?}, b:{:?}", a, b);
                if let (Some(a), Some(b)) = (a, b) {
                    Some(a - b)
                } else {
                    None
                }
            })
            .collect::<Int32Array>();

        println!("c:{:?}", c);

        let expected: Int32Array = vec![None,
                                        Some(1),
                                        Some(2),
                                        Some(3),
                                        ].into();
        assert_eq!(res.as_ref(), &expected);
    }
}