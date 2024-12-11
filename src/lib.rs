use std::time::{ Duration, Instant };
use std::thread;
use std::sync::atomic::{ AtomicBool, Ordering };
use std::sync::Arc;
use std::io::{ Write, stdout };

pub fn time_function<F, T>(name: &str, threshold_secs: u64, f: F) -> T where F: FnOnce() -> T {
    let start = Instant::now();
    let is_running = Arc::new(AtomicBool::new(true));
    let is_running_clone = is_running.clone();

    let name_clone = name.to_string();
    let progress_thread = thread::spawn(move || {
        thread::sleep(Duration::from_secs(threshold_secs));

        while is_running_clone.load(Ordering::Relaxed) {
            let elapsed = start.elapsed();
            print!("\r⏳ {} running for {:?}", name_clone, elapsed);
            stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let result = f();

    is_running.store(false, Ordering::Relaxed);
    let duration = start.elapsed();

    let _ = progress_thread.join();
    println!("\r✅ {} completed in {:?}    ", name, duration);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_timer() {
        let result = time_function("Test", 1, || {
            sleep(Duration::from_secs(3));
            42
        });
        assert_eq!(result, 42);
    }
}
