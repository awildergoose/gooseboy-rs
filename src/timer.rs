use std::time::Duration;

pub struct Timer {
    pub interval: Duration,
    pub last_trigger: Duration,
}

impl Timer {
    /// Create a new timer with the given interval
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_trigger: Duration::ZERO,
        }
    }

    /// Returns true if the timer should trigger at this `elapsed` time
    pub fn tick(&mut self, elapsed: Duration) -> bool {
        if elapsed - self.last_trigger >= self.interval {
            self.last_trigger = elapsed;
            true
        } else {
            false
        }
    }
}
