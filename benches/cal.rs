use std::thread;
use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};

use arbeider::cal::calculator::Calculator;

fn day_returns(i: usize) {
    // Construct a calculator
    let calculator = Calculator::new("examples/data/".to_string());

    // New file path
    let filepath = format!("examples/data/600{:03}.parquet", i);
    let path = filepath.as_str();

    // Calculate day returns
    calculator.day_returns(path);
}

fn multi_returns() {
    // Thread handlers
    let mut handles = vec![];

    // spawn 10 threads
    for i in 100..130 {
        // Fork a new thread to calculate day returns
        let handle = thread::spawn(move || day_returns(i));

        // Add thread handler to vector
        handles.push(handle);
    }

    // count handler if finished
    let mut count = 0;

    // wait for all thread to finish
    for handle in &handles {
        if count == handles.len() {
            break;
        }

        if handle.is_finished() {
            count += 1;
        }
        // sleep 1 millisecond
        thread::sleep(Duration::from_millis(1));
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day returns", |b| b.iter(|| multi_returns()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);