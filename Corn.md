If you don't want to use a loop, you can use `tokio::spawn` along with `tokio::time::sleep` in a recursive async function to schedule the task. Here's an example:  

### Example: Scheduled Task Without a Loop  
```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    tokio::spawn(schedule_task());

    // The main function continues running while the task executes periodically.
    sleep(Duration::from_secs(30)).await; // Keep the program running for some time
}

async fn schedule_task() {
    sleep(Duration::from_secs(5)).await;
    println!("Task executed at {:?}", tokio::time::Instant::now());

    // Recursively call the function to schedule the next execution
    tokio::spawn(schedule_task());
}
```

### Explanation:
1. `schedule_task()` waits for 5 seconds before executing.
2. After execution, it spawns itself again using `tokio::spawn(schedule_task())`.
3. This avoids using an explicit loop while keeping the scheduling continuous.
4. The `main` function runs for 30 seconds to keep the program alive.

Would you like a more cron-like implementation instead?
