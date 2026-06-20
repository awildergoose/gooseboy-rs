use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::rc::Rc;
use std::task::{Context, RawWaker, Waker};

use crate::task::{TASK_VTABLE, Task};

/// A single-threaded async executor, meant to be used for when other crates require async.
pub struct SingleExecutor {
    queue: Rc<RefCell<VecDeque<Rc<Task>>>>,
}

impl SingleExecutor {
    #[must_use]
    pub fn new() -> Self {
        Self {
            queue: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    /// Spawns a new future.
    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        let task = Rc::new(Task {
            future: RefCell::new(Box::pin(future)),
            queue: self.queue.clone(),
        });
        self.queue.borrow_mut().push_back(task);
    }

    /// Runs all the futures sequentially.
    pub fn run(&self) {
        loop {
            let task = {
                let mut q = self.queue.borrow_mut();
                q.pop_front()
            };

            let Some(task) = task else { break };

            let raw_waker = RawWaker::new(Rc::into_raw(task.clone()).cast::<()>(), &TASK_VTABLE);
            let waker = unsafe { Waker::from_raw(raw_waker) };
            let mut context = Context::from_waker(&waker);

            let mut future = task.future.borrow_mut();
            let _ = future.as_mut().poll(&mut context);
        }
    }
}

impl Default for SingleExecutor {
    fn default() -> Self {
        Self::new()
    }
}
