use std::time::{Duration, Instant};

use crate::{cfg_blocking, cfg_gen_blocking, cfg_non_blocking};

const MULTIPLIER: f64 = 1.6;
const BACKOFF_INITIAL: Duration = Duration::from_secs(1);
const BACKOFF_MAX: Duration = Duration::from_secs(120);
const JITTER: f64 = 0.23;

pub struct BackoffTimer {
    num_retries: i32,
    backoff: Duration,
    min_timeout: Duration,
    deadline: Instant,
}

impl BackoffTimer {
    cfg_blocking! {
        fn sleep(duration: Duration) {
            std::thread::sleep(duration);
        }
    }

    cfg_non_blocking! {
        async fn sleep(duration: Duration) {
            tokio::time::sleep(duration).await;
        }
    }

    //   public BackoffTimer(Duration minTimeout) {
    //     numRetries = 0;
    //     backoff = backoffInitial;
    //     this.minTimeout = minTimeout;
    //     deadline = Instant.now().plus(backoff);
    //   }
    pub fn new(min_timeout: Duration) -> Self {
        Self {
            num_retries: 0,
            backoff: BACKOFF_INITIAL,
            min_timeout,
            deadline: Instant::now() + BACKOFF_INITIAL,
        }
    }

    //   public Duration getTimeout() {
    //     Duration timeToDeadline = getTimeUntilDeadline();
    //     if (timeToDeadline.compareTo(minTimeout) < 0) return minTimeout;
    //     return timeToDeadline;
    //   }
    pub fn get_timeout(&self) -> Duration {
        let time_to_deadline = self.get_time_until_deadline();
        if time_to_deadline < self.min_timeout {
            self.min_timeout
        } else {
            time_to_deadline
        }
    }

    //   public long getTimeoutMillis() {
    //     return getTimeout().toMillis();
    //   }
    pub fn get_timeout_millis(&self) -> u64 {
        self.get_timeout().as_millis() as u64
    }

    pub fn get_num_retries(&self) -> i32 {
        self.num_retries
    }

    //   public void sleepUntilRetry() throws InterruptedException {
    //     try {
    //       Thread.sleep(getTimeUntilDeadline().toMillis());
    //     } catch (InterruptedException exception) {
    //       Thread.currentThread().interrupt();
    //       throw exception;
    //     }

    //     backoff = Duration.ofNanos((long) (backoff.toNanos() * multiplier));
    //     if (backoff.compareTo(backoffMax) > 0) backoff = backoffMax;

    //     float randomJitter = (ThreadLocalRandom.current().nextFloat() * 2.0F - 1.0F) * jitter + 1.0F;
    //     Duration jitteredBackoff = Duration.ofNanos((long) (backoff.toNanos() * randomJitter));
    //     deadline = Instant.now().plus(jitteredBackoff);
    //     ++numRetries;
    //   }

    cfg_gen_blocking! {
        pub async fn sleep_until_retry(&mut self) {
            Self::sleep(self.get_time_until_deadline()).await;
            self.backoff = Duration::from_nanos((self.backoff.as_nanos() as f64 * MULTIPLIER) as u64);
            if self.backoff > BACKOFF_MAX {
                self.backoff = BACKOFF_MAX;
            }
            let random_jitter = (rand::random::<f64>() * 2.0 - 1.0) * JITTER + 1.0;
            let jittered_backoff =
                Duration::from_nanos((self.backoff.as_nanos() as f64 * random_jitter) as u64);
            self.deadline = Instant::now() + jittered_backoff;
            self.num_retries += 1;
        }
    }

    //   private Duration getTimeUntilDeadline() {
    //     Instant currentTime = Instant.now();
    //     if (currentTime.isAfter(deadline)) return Duration.ZERO;
    //     return Duration.between(currentTime, deadline);
    //   }

    fn get_time_until_deadline(&self) -> Duration {
        let current_time = Instant::now();
        if current_time > self.deadline {
            Duration::ZERO
        } else {
            self.deadline - current_time
        }
    }
}
