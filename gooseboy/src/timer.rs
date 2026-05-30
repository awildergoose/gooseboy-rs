use std::time::Duration;

/// A timer that triggers on intervals.
pub struct Timer {
    /// The interval.
    pub interval: Duration,
    /// The last time it triggered.
    pub last_trigger: Duration,
}

impl Timer {
    /// Create a new timer with the given `interval`.
    #[must_use]
    pub const fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_trigger: Duration::ZERO,
        }
    }

    /// Returns true if the timer should trigger at this `elapsed` time.
    pub fn tick(&mut self, elapsed: Duration) -> bool {
        #[allow(clippy::unchecked_time_subtraction)]
        if elapsed - self.last_trigger >= self.interval {
            self.last_trigger = elapsed;
            true
        } else {
            false
        }
    }
}

/// Creates a new time, returns a `LazyLock<Mutex<Timer>>`
#[macro_export]
macro_rules! make_timer {
    ($duration:expr) => {
        std::sync::LazyLock::new(|| std::sync::Mutex::new($crate::timer::Timer::new($duration)))
    };
}
