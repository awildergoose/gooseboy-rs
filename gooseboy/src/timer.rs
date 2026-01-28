use std::time::Duration;

pub struct Timer {
    pub interval: Duration,
    pub last_trigger: Duration,
}

impl Timer {
    /// Create a new timer with the given interval
    #[must_use]
    pub const fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_trigger: Duration::ZERO,
        }
    }

    /// Returns true if the timer should trigger at this `elapsed` time
    pub fn tick(&mut self, elapsed: Duration) -> bool {
        if elapsed.checked_sub(self.last_trigger).unwrap() >= self.interval {
            self.last_trigger = elapsed;
            true
        } else {
            false
        }
    }
}

#[macro_export]
macro_rules! make_timer {
    ($duration:expr) => {
        std::sync::LazyLock::new(|| std::sync::Mutex::new($crate::timer::Timer::new($duration)))
    };
}
