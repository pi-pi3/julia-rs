
//! Module providing a wrapper for the native Julia task object.

use sys::*;
use error::Result;
use api::{Ref, Function};

wrap_ref! { pub struct Task(Ref); }

impl Task {
    /// Construct a new Task with a Function.
    pub fn with_function(&self, start: &Function) -> Result<Task> {
        let raw = unsafe { jl_new_task(start.lock()?, 0) };
        jl_catch!();
        Ok(Task(Ref::new(raw)))
    }
}
