# Tasks.rs
Simple, elegant task with continuitation and await

## Installation

Currently only via github. Cargo coming soon...

     git clone https://github.com/Muraad/Tasks.rs.git

or using Cargo.toml

```toml
[dependencies.tasks]
git = "https://github.com/Muraad/Tasks.rs"
```

Dependencies are 

```toml
[dependencies]
lazy_static = "*"

[dependencies.threadpool]
version = "*"
features = ["scoped-pool"]
```

Usage

```rust
extern crate tasks
```

## Example

```rust
extern crate tasks;

use tasks::task::{Task};
use std::io;
use std::io::prelude::*;

fn main() {
    let x = 42;
    // Via global DEFAULT_POOL
    for _ in 0..1000 {
        Task::run(|| { println!("Hello World!"); 42 })
            .continue_with(move |msg| format!("Continuitation = {}", msg + x))
            .continue_with(|msg| println!("{}", msg));
    }

    // Continuitations via overloaded ">>" operator
    
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

```
