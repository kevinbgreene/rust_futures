use crate::{
    blocking_executor::{BlockingExecutor, ReadFileFuture},
    noop_executor::{ConstantFuture, NoopExecutor},
};
use async_std::fs::read_to_string;

mod blocking_executor;
mod noop_executor;

async fn identity(value: i32) -> i32 {
    value
}

async fn read_file() -> String {
    let contents = read_to_string("test.txt").await;
    contents.expect("Error opening file")
}

fn main() {
    let future = ConstantFuture::new(5);
    let result = NoopExecutor::run(future);
    println!("result = {}", result);

    let result = NoopExecutor::run(identity(6));
    println!("result = {}", result);

    // Panic!
    // NoopExecutor::run(read_file());

    let result = BlockingExecutor::run(identity(9));
    println!("result = {}", result);

    let result = BlockingExecutor::run(read_file());
    println!("result = {}", result);

    let result = BlockingExecutor::run(ReadFileFuture::new("test.txt"));
    println!("result = {}", result);
}
