
use sys::*;
use error::Result;
use api::{JlValue, Function};

jlvalues! {
    pub struct Task(jl_task_t);
}

impl Task {
    pub fn with_function(&self, start: &Function) -> Result<Task> {
        let raw = unsafe {
            jl_new_task(start.lock()?, 0)
        };
        jl_catch!();
        Task::new(raw)
    }
}
