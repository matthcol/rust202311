use std::{time::Duration, thread::{self, spawn}, sync::mpsc::channel};

use rayon::prelude::*;

fn demo_thread_hello() {
    println!("*** Thread Hello ***");
    let job = spawn(|| println!("Fizz"));
    println!("Buz");
    job.join().unwrap();
    println!()
}

fn demo_thread_simple_fun() {
    println!("*** Thread call simple function ***");
    let x = 4;
    let job = spawn(move || {
        thread::sleep(Duration::from_secs(2));
        x * x + 1
    });
    if let Ok(res) = job.join() {
        println!("Other thread has computed: {res}")
    } else {
        eprintln!("Something bad happened")
    }
    println!()
}

fn demo_thread_compute_vec_parallel(){
    println!("*** Thread compute in // ***");
    let v: Vec<i32> = (0..10_000).collect();
    let n_workers = 10;
    let chunk = v.len() / n_workers;
    (0..n_workers).into_par_iter().for_each(|i| {
        let i_min = i*chunk;
        let i_max = (i+1)*chunk;
        let s: i32 = v[i_min..i_max].iter().sum();
        thread::sleep(Duration::from_secs(3));
        println!("sum[{i_min}..{i_max}] = {s}")
    });
    println!()
}

fn demo_thread_compute_vec_parallel_channel(){
    println!("*** Thread compute in // and receive results via channel ***");
    let v: Vec<i32> = (0..10_000).collect();
    let n_workers = 10;
    let chunk = v.len() / n_workers;
    let (sender, receiver) = channel();
    (0..n_workers).into_par_iter().for_each_with(sender, |s, i| {
        let i_min = i*chunk;
        let i_max = (i+1)*chunk;
        let sum: i32 = v[i_min..i_max].iter().sum();
        thread::sleep(Duration::from_secs(3));
        println!("sum[{i_min}..{i_max}] = {sum}");
        s.send(sum).unwrap()
    });
    let sums: Vec<i32> = receiver.iter().collect();
    println!("Sums: {sums:?}");
    let final_sum: i32 = sums.iter().sum();
    println!("Final sum: {final_sum}");
    println!()
}

fn main() {
    demo_thread_hello();
    demo_thread_simple_fun();
    demo_thread_compute_vec_parallel();
    demo_thread_compute_vec_parallel_channel()
}
