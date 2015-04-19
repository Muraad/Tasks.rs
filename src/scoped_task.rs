// Copyright 2015 - Muraad Nofal
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::Arc;
use std::sync::{ONCE_INIT, Mutex};
use std::sync::mpsc::{channel, Receiver};
use threadpool::{ScopedPool, ThreadPool};
use std::ops::{Shl, Shr, Mul};

pub type SafeScopedPool<'p> = Arc<Mutex<ScopedPool<'p>>>;

#[allow(dead_code)] pub struct ScopedTask<'pool, T> {
	pool: SafeScopedPool<'pool>,
	receiver: Receiver<T>,
}

#[allow(dead_code)] impl<'pool, T: Send + 'pool> ScopedTask<'pool,T> {

    /// Create a new ScopedTask
    ///
    /// 
    pub fn new(p: SafeScopedPool<'pool>, r: Receiver<T>) -> ScopedTask<'pool, T> {
        ScopedTask {
            pool: p,
            receiver: r,
        }
    }


    pub fn new_safe_pool(threads: u32) -> SafeScopedPool<'pool> {
        Arc::new(Mutex::new(ScopedPool::new(threads)))
    }

    fn run<F>(pool: &SafeScopedPool<'pool>, job: F) -> ScopedTask<'pool, T>
        where F: Fn() -> T + Send + 'pool
    {
        let (tx, rx) = channel::<T>();
        let lock = pool.lock().unwrap();

        lock.execute(move || {
                    let result = job();
                    tx.send(result);
                });

        ScopedTask::new((*pool).clone(), rx)
    }

    pub fn continue_with<F, U>(self, job: F) -> ScopedTask<'pool, U>
        where F: Fn(T) -> U + Send + 'pool, U: Send + 'pool
    {
        let rcv = self.receiver;

        ScopedTask::run(
            &self.pool.clone(),  
            move || { job(rcv.recv().unwrap()) } )
    }

    pub fn await(self) -> T {
        self.receiver.recv().unwrap()
    }
}


pub trait ScopedTaskDispatcher<'a> {
    fn run<T, F>(&self, job: F) -> ScopedTask<'a, T>
        where F: Fn() -> T + Send + 'a, T : Send + 'a;
}

impl<'a> ScopedTaskDispatcher<'a> for SafeScopedPool<'a> {

    fn run<T, F>(&self, job: F) -> ScopedTask<'a, T>
        where F: Fn() -> T + Send + 'a, T : Send + 'a
    {
        let (tx, rx) = channel::<T>();
        let lock = self.lock().unwrap();

        lock.execute(move || {
                    let result = job();
                    tx.send(result);
                });

        ScopedTask::new((*self).clone(), rx)
    }
}