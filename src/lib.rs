use std::time::{ Duration, Instant };
use std::thread;
use std::sync::atomic::{ AtomicBool, Ordering };
use std::sync::Arc;
use std::io::{ Write, stdout };

fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let millis = duration.subsec_millis();

    if total_secs < 60 {
        format!("{}.{:03}s", total_secs, millis)
    } else if total_secs < 3600 {
        let minutes = total_secs / 60;
        let secs = total_secs % 60;
        format!("{}m {:02}.{:03}s", minutes, secs, millis)
    } else {
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let secs = total_secs % 60;
        format!("{}h {:02}m {:02}.{:03}s", hours, minutes, secs, millis)
    }
}

pub fn time_function<F, T>(name: &str, threshold: Duration, update_interval: Duration, f: F) -> T
    where F: FnOnce() -> T
{
    let start = Instant::now();
    let is_running = Arc::new(AtomicBool::new(true));
    let is_running_clone = is_running.clone();

    let name_clone = name.to_string();
    let progress_thread = thread::spawn(move || {
        thread::sleep(threshold);

        while is_running_clone.load(Ordering::Relaxed) {
            let elapsed = start.elapsed();
            print!("\r⏳ {} running for {}", name_clone, format_duration(elapsed));
            stdout().flush().unwrap();
            thread::sleep(update_interval);
        }
    });

    let result = f();

    is_running.store(false, Ordering::Relaxed);
    let duration = start.elapsed();

    let _ = progress_thread.join();
    println!("\r✅ {} completed in {:?}    ", name, format_duration(duration));

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_timer() {
        let result = time_function("Test", Duration::from_secs(1), Duration::from_millis(500), || {
            sleep(Duration::from_secs(3));
            42
        });
        assert_eq!(result, 42);
    }
}
