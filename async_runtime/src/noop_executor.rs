use core::ptr;
use std::future::Future;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub struct NoopExecutor;

impl NoopExecutor {
    pub fn run<F: Future>(f: F) -> F::Output {
        let mut boxed_future = Box::pin(f);
        let waker = noop_waker();
        let mut context = Context::from_waker(&waker);
        let result = boxed_future.as_mut().poll(&mut context);
        if let Poll::Ready(a) = result {
            return a;
        } else {
            panic!("This executor doesn't really do anything! You asked it to do too much!");
        }
    }
}

pub struct ConstantFuture {
    value: i32,
}

impl ConstantFuture {
    pub fn new(value: i32) -> Self {
        ConstantFuture { value }
    }
}

impl Future for ConstantFuture {
    type Output = i32;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Poll::Ready(self.value)
    }
}

unsafe fn noop_clone(_p: *const ()) -> RawWaker {
    noop_raw_waker()
}

const RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);

fn noop_raw_waker() -> RawWaker {
    RawWaker::new(ptr::null(), &RAW_WAKER_VTABLE)
}

unsafe fn noop(_p: *const ()) {}

fn noop_waker() -> Waker {
    unsafe { Waker::from_raw(noop_raw_waker()) }
}
