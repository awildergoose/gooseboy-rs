use crate::test;
use core::time::Duration;
use gooseboy::timer::Timer;

pub fn test_timer() {
    let mut t = Timer::new(Duration::from_millis(100));

    test!(
        "timer:tick_initial_false",
        !t.tick(Duration::from_millis(0))
    );
    test!("timer:tick_50ms_false", !t.tick(Duration::from_millis(50)));
    test!("timer:tick_150ms_true", t.tick(Duration::from_millis(150)));
}
