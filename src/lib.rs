// Copyright 2015 - Muraad Nofal
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Abstraction of a task with continuitations and await

#[macro_use]
extern crate lazy_static;
extern crate threadpool;

pub mod task;
pub mod scoped_task;
