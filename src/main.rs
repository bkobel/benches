use std::thread;
use std::env;
use std::time::Instant;
use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;

fn fibonacci(n: u32) -> u64 {
    //println!("thread {} :: iteration{}", thread, n);
    match n {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 2,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_test() {
    let iterations: u32 = 47; // Adjust this number based on your CPU strength
    let num_threads = 32; // For a quad-core CPU. Adjust as per your CPU's core count.

    println!("Running Fibonacci({}) on {} threads", iterations, num_threads);

    let start_time = Instant::now();

    let mut handles = vec![];

    for _ in 0..num_threads {
        let handle = thread::spawn(move || {
            fibonacci(iterations)
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        let _ = handle.join().unwrap();
    }

    let elapsed = start_time.elapsed();
    println!(
        "Calculated Fibonacci({}) on {} threads. Total computation took {:?}",
        iterations, num_threads, elapsed
    );
}

fn matrix_multiplication() {
    let arrays_count = 250;
    let array_size = 2000;
    let num_threads = rayon::current_num_threads();
    println!("Running matrices multiplication. Arrays: {} of size: {} on {} threads", arrays_count, array_size, num_threads);

    let between = Uniform::from(1000..=9999);
    let mut rng = rand::thread_rng();

    let start_time = Instant::now();
    let matrices = vec![vec![vec![between.sample(&mut rng); array_size]; array_size]; arrays_count];
    let _: Vec<_> = (0..arrays_count).into_par_iter().map(|l| {
        let mut local_result = vec![vec![0; array_size]; array_size];
        for k in 0..arrays_count {
            for i in 0..array_size {
                for j in 0..array_size {
                    local_result[i][j] = matrices[l][i][j] * matrices[k][i][j];
                }
            }
        }

        local_result
    })
    .enumerate()
    .inspect(|(index, _)| {
        print!("\rProcessed matrix at index: {}    ", index);
    })
    .collect();

    let elapsed = start_time.elapsed();
    println!(
        "\rMultiplied {} arrays each of {} values. Total computation took {:?}",
        arrays_count,
        array_size,
        elapsed
    );
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"-f".to_string()) {
        fibonacci_test();
    }

    if args.contains(&"-m".to_string()) {
        matrix_multiplication();
    }
}
