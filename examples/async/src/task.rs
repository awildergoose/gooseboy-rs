use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{RawWaker, RawWakerVTable};

pub(crate) struct Task {
    pub(crate) future: RefCell<Pin<Box<dyn Future<Output = ()>>>>,
    pub(crate) queue: Rc<RefCell<VecDeque<Rc<Self>>>>,
}

pub(crate) const TASK_VTABLE: RawWakerVTable =
    RawWakerVTable::new(task_clone, task_wake, task_wake_by_ref, task_drop);

unsafe fn task_clone(ptr: *const ()) -> RawWaker {
    let rc = unsafe { Rc::from_raw(ptr.cast::<Task>()) };
    let cloned = rc.clone();
    let _ = Rc::into_raw(rc);
    RawWaker::new(Rc::into_raw(cloned).cast::<()>(), &TASK_VTABLE)
}

unsafe fn task_wake(ptr: *const ()) {
    let rc = unsafe { Rc::from_raw(ptr.cast::<Task>()) };
    let cloned = rc.clone();
    rc.queue.borrow_mut().push_back(cloned);
}

unsafe fn task_wake_by_ref(ptr: *const ()) {
    let rc = unsafe { Rc::from_raw(ptr.cast::<Task>()) };
    rc.queue.borrow_mut().push_back(rc.clone());
    let _ = Rc::into_raw(rc);
}

unsafe fn task_drop(ptr: *const ()) {
    drop(unsafe { Rc::from_raw(ptr.cast::<Task>()) });
}
