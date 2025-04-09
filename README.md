# Progress Timer

A simple Rust utility for monitoring execution time of long-running operations with real-time progress updates.

## Features

- Real-time progress monitoring
- Single-line updates using carriage returns
- Configurable time threshold for progress display
- Clean and simple API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
progress_timer = { git = "https://github.com/ninouGx/progress-timer" }
```

## Usage

```rust
use progress_timer::time_function;

fn main() {
    // Will show progress if execution takes more than 2 seconds
    let result = time_function("Long task", 2, || {
        // Your long-running operation here
        std::thread::sleep(std::time::Duration::from_secs(5));
        42
    });
}
```

Output example:
```
⏳ Long task running for 2.001s
⏳ Long task running for 3.002s
⏳ Long task running for 4.001s
✅ Long task completed in 5.003s
```

## License

MIT
