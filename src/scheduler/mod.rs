mod context;
mod process;
mod queue;
mod runtime;
#[allow(clippy::module_inception)]
mod scheduler;
mod state;
mod tree;

#[cfg(test)]
mod tests;

use async_trait::async_trait;

pub use crate::Result;
pub use context::Context;
pub use process::{Process, StatementBatch, Task, TaskLifeCycle};
pub use runtime::Runtime;
pub use scheduler::Scheduler;
pub use state::TaskState;
pub use tree::{Node, NodeContent, NodeData, NodeKind, NodeTree};

#[async_trait]
pub trait ActTask: Clone + Send {
    fn init(&self, _ctx: &Context) -> Result<()> {
        Ok(())
    }

    fn run(&self, _ctx: &Context) -> Result<()> {
        Ok(())
    }

    fn next(&self, _ctx: &Context) -> Result<bool> {
        Ok(false)
    }

    fn review(&self, _ctx: &Context) -> Result<bool> {
        Ok(true)
    }

    fn error(&self, ctx: &Context) -> Result<()> {
        ctx.emit_error()
    }
}
