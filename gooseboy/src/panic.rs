use std::panic;

use crate::log;

pub fn set_panic_handler() {
    panic::set_hook(Box::new(|info| {
        let location = info.location().map_or_else(
            || "<unknown>".into(),
            |l| format!("{}:{}:{}", l.file(), l.line(), l.column()),
        );

        let payload = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(|s| &**s))
            .unwrap_or("<non-string panic>");

        log!("PANIC at {}: {}", location, payload);
    }));
}
