use std::collections::VecDeque;
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub struct ApiThrottler {
    call_times: VecDeque<Instant>,
    max_calls_per_minute: usize,
}

impl ApiThrottler {
    pub fn new(max_calls_per_minute: usize) -> Self {
        Self {
            call_times: VecDeque::new(),
            max_calls_per_minute,
        }
    }

    pub async fn throttle(&mut self) {
        let now = Instant::now();
        let one_minute_ago = now - Duration::from_secs(60);

        // Remove old call times (older than 1 minute)
        while let Some(&front_time) = self.call_times.front() {
            if front_time < one_minute_ago {
                self.call_times.pop_front();
            } else {
                break;
            }
        }

        // Check if we need to wait
        if self.call_times.len() >= self.max_calls_per_minute {
            if let Some(&oldest_call) = self.call_times.front() {
                let wait_until = oldest_call + Duration::from_secs(60);
                if wait_until > now {
                    let wait_duration = wait_until - now;
                    println!("⏳ API rate limit reached. Waiting {wait_duration:?}...");

                    // Show a simple spinner animation
                    let spinner_task = tokio::spawn(async move {
                        let chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
                        let mut i = 0;
                        loop {
                            print!("\r{} Please wait...", chars[i % chars.len()]);
                            tokio::time::sleep(Duration::from_millis(100)).await;
                            i += 1;
                        }
                    });

                    sleep(wait_duration).await;
                    spinner_task.abort();
                    print!("\r                    \r"); // Clear the spinner line
                }
            }
        }

        // Record this call
        self.call_times.push_back(now);
    }
}
