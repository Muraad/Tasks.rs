// Copyright 2015 - Muraad Nofal
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate tasks;
extern crate threadpool;

use tasks::task::{Task};
use threadpool::{ScopedPool, ThreadPool};
use std::io;
use std::io::prelude::*;

fn main() {
    let x = 42;
    // Via global DEFAULT_POOL
    for _ in 0..1000 {
        Task::run(|| { println!("From DEFAULT..."); 42 })
            .continue_with(move |msg| format!("Continuitation = {}", msg + x))
            .continue_with(|msg| println!("{}", msg));
    }


    let finished_task = Task::run(|| 42 )
                        >> { |i| format!("{}, {}", i, 43) }
                        >> { |s| format!("{}, {}", s, 44) }
                        >> { |s| format!("{}, {}", s, 45) }
                        >> { |s| format!("{}, {}", s, 46) }
                        >> { |s| format!("{}, {}", s, 47) }
                        >> { |s| { println!("{}", s); "Finished" } };

    let finished_msg = finished_task.await();
    println!("Finished message: {}", finished_msg);
}