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

pub type SafeThreadPool = Arc<Mutex<ThreadPool>>;

lazy_static! {
    pub static ref THREAD_POOL: SafeThreadPool = Arc::new(Mutex::new(ThreadPool::new(4)));
}

#[allow(dead_code)] pub struct Task<T> {
	pool: SafeThreadPool,
	receiver: Receiver<T>,
}

/// A task that is running in a thread pool.
///
/// When a task is finished it can be awaited or a task 
/// can be started.
///
/// # Example
///
/// ```rust
/// use tasks::Task;
/// 
/// let pool = ThreadPool::new(4);
///
/// let x = 42;
/// // Via global DEFAULT_POOL
/// for _ in 0..10 {
///     Task::run(|| { println!("From DEFAULT..."); 42 })
///         .continue_with(move |msg| format!("Continuitation = {}", msg + x))
///         .continue_with(|msg| println!("{}", msg));
/// }
///
/// // Continuitations using ">>" operator overloading
/// let finished_task = Task::run(|| 42 )
///                     >> { |i| format!("{}, {}", i, 43) }
///                     >> { |s| format!("{}, {}", s, 44) }
///                     >> { |s| format!("{}, {}", s, 45) }
///                     >> { |s| format!("{}, {}", s, 46) }
///                     >> { |s| format!("{}, {}", s, 47) }
///                     >> { |s| { println!("{}", s); "Finished" } };
///
///    // ... do other work
///    
///    // wait for the task to be completed and get the result message
///    // The calling thread is blocking here
///    let finished_msg = finished_task.await();
///    println!("Finished message: {}", finished_msg);
///
/// ```
#[allow(dead_code)] impl<T: Send + 'static> Task<T> {

    /// Runs a new task on the (static) default THREAD_POOL
    ///
    /// Given a function/lambda "run" is creating and starting a new task
    ///
    /// # Examples
    /// ```
    /// use tasks::Task;
    ///
    /// // Returns a Task<int>.
    /// let task = Task::run(|| { println!("Hello World"); 42});
    /// ``` 
    pub fn run<F>(job: F)  -> Task<T>
        where F: Fn() -> T + Send + 'static {
        Task::run_on_pool(&THREAD_POOL, job)
    }

    /// Create and start a new task
    pub fn run_on_pool<F>(pool: &SafeThreadPool, job: F)  -> Task<T>
        where F: Fn() -> T + Send + 'static
    {
        let (tx, rx) = channel::<T>();
        let lock = pool.lock().unwrap();

        lock.execute(move || {
                    let result = job();
                    tx.send(result);
                });
        Task::new((*pool).clone(), rx)
    }

    /// Creates a new SafeThreadPool
    pub fn new_safe_pool(threads: usize) -> SafeThreadPool {
        Arc::new(Mutex::new(ThreadPool::new(threads)))
    }

    /// Creates a new SafeScopedPool
    //pub fn new_safe_scoped_pool<'a>(threads: u32) -> SafeScopedPool<'a> {
    //    Arc::new(Mutex::new(ScopedPool::new(threads)))
    //}

    /// Add a continuitation to this task.
    ///
    /// TODO: Add some checks if task is already finished and receiver is "empty"
    pub fn continue_with<F, U>(self, job: F) -> Task<U>
        where F: Fn(T) -> U + Send + 'static, U: Send + 'static
    {
        Task::run_on_pool(
            &self.pool.clone(),  
            move || { job(self.receiver.recv().unwrap()) })
    }

    /// Wait for the result of this task.
    ///
    /// Blocks the calling thread
    pub fn await(self) -> T {
        self.receiver.recv().unwrap()
    }

    /// Creates a new Task
    ///
    /// A created Task<T> is connected to a thread pool.
    /// The reference to the pool is used to add continuitation
    /// tasks via the Task<T> object itself.
    /// The Receiver<T> of the task is the receiver end of the
    /// result channel of this task, and can be used to wait for the
    /// task completion.
    /// TODO: Think about reusing the Receiver/Channel? (Events/KernelObjects?Expensive?)
    fn new(p: SafeThreadPool, r: Receiver<T>) -> Task<T> {
        Task {
            pool: p,
            receiver: r,
        }
    }
}

pub trait TaskDispatcher {
    fn run<T, F>(&self, job: F) -> Task<T>
        where F: Fn() -> T + Send, T : Send;
}

impl TaskDispatcher for SafeThreadPool {

    fn run<T, F>(&self, job: F) -> Task<T>
        where F: Fn() -> T + Send + 'static, T : Send + 'static
    {
        let (tx, rx) = channel::<T>();
        let lock = self.lock().unwrap();

        lock.execute(move || {
                    let result = job();
                    tx.send(result);
                });

        Task::new((*self).clone(), rx)
    }
}

impl<T: Send + 'static, F: Fn(T) -> () + Send + 'static> Shl<F> for Task<T> {
    type Output = Task<()>;

    fn shl(self, job: F) -> Task<()> {
        self.continue_with(job)
    }
}

impl<T: Send + 'static, U: Send + 'static, F: Fn(T) -> U + Send + 'static> Shr<F> for Task<T> {
    type Output = Task<U>;

    fn shr(self, job: F) -> Task<U> {
        self.continue_with(job)
    }
}