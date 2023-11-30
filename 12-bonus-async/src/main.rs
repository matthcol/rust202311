use futures::executor::block_on;
use std::time::Duration;
use async_std::task;

async fn  task_hello(){
    println!("Hello")
}

async fn  task_hello_long(){
    task::sleep(Duration::from_secs(5)).await;
    println!("Hello after a few time...")
}

async fn compute(x: u64) -> u64 {
    task::sleep(Duration::from_secs(10)).await;
    x*2
} 

async fn async_main() {
    let t1 = task_hello();
    let t2 = task_hello_long();
    let t3 = task_hello();

    let t4 = compute(4u64);

    let (_,_,_,res) = futures::join!(t1,t2,t3,t4);
    println!("Result of computation: {res}");
}

fn main(){
    block_on(async_main())
}
