use std::thread;
use std::time::Instant;

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

fn main() {
    let iterations: u32 = 47; // Adjust this number based on your CPU strength
    let num_threads = 32; // For a quad-core CPU. Adjust as per your CPU's core count.

    let start_time = Instant::now();

    let mut handles = vec![];

    for thread in 0..num_threads {
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
