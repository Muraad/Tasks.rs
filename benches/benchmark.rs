#![feature(test)]

extern crate test;
extern crate tasks;
extern crate threadpool;


pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    //use super::*;
    use test::Bencher;
    use tasks::task::Task;
    use threadpool::{ScopedPool, ThreadPool};
    use std::thread;


    #[bench]
    fn bench_tasks(b: &mut Bencher) {
        println!("Starting bench_tasks...");
        b.iter(|| {
            for _ in 0..1000 {
                Task::run(|| 42 );
            }
        });
        println!("bench_tasks finished");
    }

    #[bench]
    fn bench_threads(b: &mut Bencher) {
        println!("Starting bench_threads...");
        b.iter(|| {
            for _ in 0..1000 {
                thread::spawn(|| 42 );
            }
        });
        println!("bench_threads finished");
    }
}