// Copyright 2015 - Muraad Nofal
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate tasks;
extern crate threadpool;

use tasks::scoped_task::{ScopedTask, ScopedTaskDispatcher, SafeScopedPool};
use threadpool::{ScopedPool, ThreadPool};
use std::io;
use std::io::prelude::*;

fn main() {
    let scoped_pool: SafeScopedPool = ScopedTask::<()>::new_safe_pool(4);

    for i in 0..100 {
        scoped_pool.run(move || { println!("From DEFAULT..."); 42 + i })
                   .continue_with(|msg| format!("Continuitation = {}", msg + 42))
                   .continue_with(|msg| println!("{}", msg));
    }

    let finished_task = scoped_pool.run(|| { println!("From DEFAULT..."); 42})
                          .continue_with(|i| format!("{}, {}", i, 43))
                          .continue_with(|i| format!("{}, {}", i, 44))
                          .continue_with(|i| format!("{}, {}", i, 45))
                          .continue_with(|i| format!("{}, {}", i, 46))
                          .continue_with(|i| format!("{}, {}", i, 47))
                          .continue_with(|i| format!("{}, {}", i, 48))
                          .continue_with(|i| format!("{}, {}", i, 49))
                          .continue_with(|s| { println!("{}", s); "Finished" });

    let finished_msg = finished_task.await();
    println!("Finished message: {}", finished_msg);
}