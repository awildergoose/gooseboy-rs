#![no_main]
use gooseboy::framebuffer::init_fb;
use gooseboy::log;
use gooseboy::{color::Color, framebuffer::clear_framebuffer};

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::executor::SingleExecutor;

pub mod executor;
pub mod task;

struct Yields {
    count: usize,
}

impl Future for Yields {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            Poll::Ready(())
        } else {
            self.count -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[gooseboy::main]
fn main() {
    init_fb();

    let executor = SingleExecutor::new();
    log!("spawning tasks onto runtime");

    executor.spawn(async {
        log!("A: started");
        Yields { count: 2 }.await;
        log!("A: fin");
    });

    executor.spawn(async {
        log!("B: started");
        Yields { count: 1 }.await;
        log!("B: fin");
    });

    executor.run();
    log!("finished running tasks!");
}

#[gooseboy::update]
fn update() {
    clear_framebuffer(Color::BLACK);
}
