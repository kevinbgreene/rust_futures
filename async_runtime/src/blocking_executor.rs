use std::{
    fmt::Debug,
    fs::read_to_string,
    future::Future,
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake},
    thread::{self, Thread},
};

struct SharedState {
    running: bool,
    contents: Option<String>,
}

pub struct ReadFileFuture {
    path: String,
    state: Arc<Mutex<SharedState>>,
}

impl ReadFileFuture {
    pub fn new(path: &str) -> Self {
        ReadFileFuture {
            path: path.to_string(),
            state: Arc::new(Mutex::new(SharedState {
                running: false,
                contents: None,
            })),
        }
    }
}

impl Future for ReadFileFuture {
    type Output = String;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        
        if !state.running {
            state.running = true;
            let thread_state = self.state.clone();
            let thread_waker = cx.waker().clone();
            let path = self.path.clone();
            thread::spawn(move || {
                let contents = read_to_string(path).expect("Unable to read file.");
                let mut shared_state = thread_state.lock().unwrap();
                shared_state.contents = Some(contents);
                thread_waker.wake();
            });
        }

        if let Some(val) = &state.contents {
            Poll::Ready(val.to_owned())
        } else {
            Poll::Pending
        }
    }
}

pub struct BlockingExecutor;

impl BlockingExecutor {
    pub fn run<F>(f: F) -> F::Output
    where
        F: Future,
        F::Output: Debug,
    {
        let mut boxed_future = Box::pin(f);
        let waker = Arc::new(ThreadWaker {
            thread: thread::current(),
        })
        .into();
        let mut cx = Context::from_waker(&waker);

        loop {
            let result = boxed_future.as_mut().poll(&mut cx);
            println!("poll = {:?}", result);
            match result {
                Poll::Ready(res) => return res,
                Poll::Pending => thread::park(),
            }
        }
    }
}

struct ThreadWaker {
    thread: Thread,
}

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.thread.unpark();
    }
}

// type BoxedFuture<T> = Pin<Box<dyn Future<Output = T>>>;

// struct Task {
//     future: BoxedFuture<()>,
//     sender: mpsc::SyncSender<Task>,
// }
