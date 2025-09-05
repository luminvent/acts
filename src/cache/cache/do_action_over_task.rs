use std::sync::Arc;
use tracing::info;
use crate::{Act, ActError, Action, Error, Result};
use crate::data::MessageStatus;
use crate::event::EventAction;
use crate::scheduler::{Task, Runtime, TaskState, NodeKind, ActTask};
use crate::utils::consts;

pub fn do_action_over_task(task: &Task, action: &Action, runtime: &Arc<Runtime>) -> Result<()> {
  let task = Arc::new(task.clone());

  let ctx = task.create_context();

  ctx.set_action(&action)?;

  info!("update task={:?}", ctx.task());

  match action.event {
    EventAction::Push => {
      let act_name = ctx.get_var::<String>("act").unwrap_or("irq".to_string());
      let key = ctx.get_var::<String>("key").unwrap_or_default();
      let act = Act {
        id: ctx.get_var::<String>("id").unwrap_or_default(),
        name: ctx.get_var::<String>("name").unwrap_or_default(),
        tag: ctx.get_var::<String>("tag").unwrap_or_default(),
        key: key.clone(),
        act: act_name.clone(),
        inputs: ctx.get_var("with").unwrap_or_default(),
        rets: ctx.get_var("rets").unwrap_or_default(),
        outputs: ctx.get_var("outputs").unwrap_or_default(),
        ..Default::default()
      };

      // check key property
      if (act_name == "irq"
        || act_name == "pack"
        || act_name == "call"
        || act_name == "cmd"
        || act_name == "msg")
        && key.is_empty()
      {
        return Err(ActError::Action(
          "cannot find 'key' in act".to_string(),
        ));
      }

      ctx.append_act(&act)?;
    }
    EventAction::Remove => {
      task.set_state(TaskState::Removed, runtime);
      task.next(&ctx)?;
    }
    EventAction::Submit => {
      task.set_state(TaskState::Submitted, runtime);
      task.next(&ctx)?;
    }
    EventAction::Next => {
      if task.state().is_completed() {
        return Err(ActError::Action(format!(
          "task '{}:{}' is already completed",
          task.pid, task.id
        )));
      }
      task.set_state(TaskState::Completed, runtime);
      task.next(&ctx)?;
    }
    EventAction::Back => {
      if task.state().is_completed() {
        return Err(ActError::Action(format!(
          "task '{}:{}' is already completed",
          task.pid, task.id
        )));
      }
      let nid = ctx
        .get_var::<String>(consts::ACT_TO)
        .ok_or(ActError::Action(
          "cannot find 'to' value in options".to_string(),
        ))?;

      let mut path_tasks = Vec::new();
      let task = task.backs(
        &|t| t.node().kind() == NodeKind::Step && t.node().id() == nid,
        &mut path_tasks,
      );

      let task = task.ok_or(ActError::Action(format!(
        "cannot find history task by nid '{}'",
        nid
      )))?;

      ctx.back_task(&ctx.task(), &path_tasks)?;
      ctx.redo_task(&task)?;
    }
    EventAction::Cancel => {
      // find the parent step task
      let mut step = ctx.task().parent();
      while let Some(task) = &step {
        if task.is_kind(NodeKind::Step) {
          break;
        }
        step = task.parent();
      }

      let task = step.ok_or(ActError::Action(format!(
        "cannot find parent step task by tid '{}'",
        ctx.task().id,
      )))?;
      if !task.state().is_success() {
        return Err(ActError::Action(format!(
          "task('{}') is not allowed to cancel",
          task.id
        )));
      }
      // get the neartest next step tasks
      let mut path_tasks = Vec::new();
      let nexts = task.follows(
        &|t| t.is_kind(NodeKind::Step) && t.is_acts(),
        &mut path_tasks,
      );
      if nexts.is_empty() {
        return Err(ActError::Action("cannot find cancelled tasks".to_string()));
      }

      // mark the path tasks as completed
      for p in path_tasks {
        if p.state().is_running() {
          p.set_state(TaskState::Completed, runtime);
          ctx.emit_task(&p)?;
        } else if p.state().is_pending() {
          p.set_state(TaskState::Skipped, runtime);
          ctx.emit_task(&p)?;
        }
      }

      for next in &nexts {
        ctx.undo_task(next)?;
      }
      ctx.redo_task(&task)?;
    }
    EventAction::Abort => {
      if task.state().is_completed() {
        return Err(ActError::Action(format!(
          "task '{}:{}' is already completed",
          task.pid, task.id
        )));
      }
      ctx.abort_task(&ctx.task())?;
    }
    EventAction::Skip => {
      if task.state().is_completed() {
        return Err(ActError::Action(format!(
          "task '{}:{}' is already completed",
          task.pid, task.id
        )));
      }

      for task in task.siblings() {
        if task.state().is_completed() {
          continue;
        }
        task.set_state(TaskState::Skipped, runtime);
        ctx.emit_task(&task)?;
      }

      // set both current act and parent step to skip
      task.set_state(TaskState::Skipped, runtime);
      task.runtime().cache().create_or_update_task(&task)?;
      task.next(&ctx)?;
    }
    EventAction::Error => {
      let ecode = ctx
        .get_var::<String>(consts::ACT_ERR_CODE)
        .ok_or(ActError::Action(format!(
          "cannot find '{}' in options",
          consts::ACT_ERR_MESSAGE
        )))?;

      let error = ctx
        .get_var::<String>(consts::ACT_ERR_MESSAGE)
        .unwrap_or("".to_string());

      let err = Error::new(&error, &ecode);
      println!("error: {err:?}");
      let task = &ctx.task();
      if task.state().is_completed() {
        return Err(ActError::Action(format!(
          "task '{}:{}' is already completed",
          task.pid, task.id
        )));
      }
      let parent = task.parent().ok_or(ActError::Action(format!(
        "cannot find task parent by tid '{}'",
        task.id
      )))?;

      for sub in parent.siblings().iter() {
        if sub.state().is_completed() {
          continue;
        }
        sub.set_state(TaskState::Skipped, runtime);
        ctx.emit_task(sub)?;
      }

      task.proc().set_data(&ctx.vars());

      task.set_err(&err, runtime);
      task.error(&ctx)?;
    }
    EventAction::SetVars => {
      if task.state().is_completed() {
        return Err(ActError::Action(format!(
          "task '{}:{}' is already completed",
          task.pid, task.id
        )));
      }

      task.set_data(&ctx.vars());
      ctx.runtime.cache().create_or_update_task(&task)?;
    }
    EventAction::SetProcessVars => {
      if task.state().is_completed() {
        return Err(ActError::Action(format!(
          "task '{}:{}' is already completed",
          task.pid, task.id
        )));
      }

      task.proc().set_data(&ctx.vars());
      if let Some(task) = task.proc().root() {
        ctx.runtime.cache().create_or_update_task(&task)?;
      }
      ctx.runtime.cache().push_proc(&task.proc());
    }
  };

  if action.event != EventAction::Push {
    // update the message status after doing action
    ctx.runtime.cache().store().set_message_with(
      &action.pid,
      &action.tid,
      MessageStatus::Completed,
    )?;
  }

  Ok(())
}
