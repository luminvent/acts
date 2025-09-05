mod do_action_over_task;

use crate::scheduler::{NodeKind, TaskState};
use crate::{scheduler::{Process, Runtime, Task}, store::Store, ActError, Action, Engine, Message, ProcInfo, Result, ShareLock, StoreAdapter, TaskInfo, Vars};
use moka::sync::Cache as MokaCache;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use tracing::{debug, instrument};
use crate::cache::cache::do_action_over_task::do_action_over_task;
use crate::event::EventAction;

#[derive(Clone)]
pub struct Cache {
    cap: usize,
    procs: MokaCache<String, Arc<Process>>,
    store: ShareLock<Arc<Store>>,
}

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache")
            .field("cap", &self.cap())
            .field("count", &self.count())
            .finish()
    }
}

impl Cache {
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            procs: MokaCache::new(cap as u64),
            store: Arc::new(RwLock::new(Store::default())),
        }
    }

    pub fn store(&self) -> Arc<Store> {
        self.store.read().unwrap().clone()
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn count(&self) -> usize {
        self.procs.run_pending_tasks();
        self.procs.entry_count() as usize
    }

    pub fn init(&self, engine: &Engine) {
        debug!("cache::init");
        #[cfg(feature = "store")]
        {
            let config = engine.config();
            *self.store.write().unwrap() =
                Arc::new(Store::local(&config.data_dir, &config.db_name));
        }
        if let Some(store) = engine.adapter().store() {
            *self.store.write().unwrap() = Arc::new(Store::create(store));
        }
    }

    pub fn close(&self) {
        self.store.read().unwrap().close();
    }

    #[instrument]
    pub fn push_proc(&self, proc: &Arc<Process>) {
        self.push_proc_pri(proc, true);
    }

    pub fn do_tick_on_running_processes(&self, rt: &Arc<Runtime>) {
        let store = self.store.read().unwrap();
        if let Ok(processes) = store.load_running_processes(rt) {
            for process in processes {
                process.do_tick()
            }
        }
    }

    pub fn get_process_state(&self, pid: &str, runtime: &Arc<Runtime>) -> Option<TaskState> {
        self.store
            .read()
            .unwrap()
            .load_proc(pid, runtime)
            .ok()
            .and_then(|process| process.map(|process| process.state()))
    }

    pub fn get_process_info(&self, pid: &str, runtime: &Arc<Runtime>) -> Result<ProcInfo> {
        let process = self.get_process(pid, runtime)?;

        let mut tasks: Vec<TaskInfo> = process.tasks().iter().map(TaskInfo::from).collect();
        tasks.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        let mut process_info = process.deref().info().clone();
        process_info.tasks = tasks;
        Ok(process_info)
    }

    pub fn run_hooks_for_task(&self, task_info: &TaskInfo, runtime: &Arc<Runtime>) -> Result<()> {
        let task = self.get_task(&task_info.pid, &task_info.id, runtime)?;

        let context = task.create_context();
        task.run_hooks(&context)?;

        Ok(())
    }

    pub fn create_message(&self, task_info: &TaskInfo, runtime: &Arc<Runtime>) -> Result<Message> {
        let task = self.get_task(&task_info.pid, &task_info.id, runtime)?;
        Ok(task.create_message())
    }

    pub fn do_action(&self, action: &Action, runtime: &Arc<Runtime>) -> Result<()> {
        let task = self.get_task(&action.pid, &action.tid, runtime)?;

        if action.event == EventAction::Push {
            if !task.is_kind(NodeKind::Step) {
                return Err(ActError::Action(format!(
                    "The task '{}' is not an Step task",
                    action.tid
                )));
            }
        } else if !task.is_kind(NodeKind::Act) {
            return Err(ActError::Action(format!(
                "The task '{}' is not an Act task",
                action.tid
            )));
        }

        // check the outputs
        task.outputs();
        // check act return
        let rets = task.node().content.rets();

        let mut action = action.clone();

        if !rets.is_empty() {
            let mut options = Vars::new();
            for (ref key, _) in &rets {
                if !action.options.contains_key(key) {
                    return Err(ActError::Action(format!(
                        "the options is not satisfied with act's rets '{}' in task({})",
                        key, action.tid
                    )));
                }
                let value = action.options.get_value(key).unwrap();
                options.set(key, value.clone());
            }

            // reset the options by rets definition
            action.options = options;
        }

        do_action_over_task(&task, &action, &runtime)?;

        Ok(())
    }

    #[instrument]
    pub fn remove(&self, pid: &str) -> Result<bool> {
        debug!("remove pid={pid}");
        self.procs.remove(pid);
        self.store.read().unwrap().remove_proc(pid)?;
        Ok(true)
    }

    #[instrument(skip(on_load))]
    pub fn restore<F: Fn(&Arc<Process>)>(&self, rt: &Arc<Runtime>, on_load: F) -> Result<()> {
        debug!("restore");
        let store = self.store.read().unwrap();
        let cap = self.cap();
        let count = self.count();
        let mut check_point = cap / 2;
        if check_point == 0 {
            check_point = cap;
        }
        if count < check_point {
            let cap = cap - count;
            for ref proc in store.load(cap, rt)? {
                if !self.procs.contains_key(proc.id()) {
                    self.push_proc_pri(proc, false);
                    on_load(proc);
                }
            }
        }
        Ok(())
    }

    #[instrument]
    pub fn create_or_update_task(&self, task: &Arc<Task>) -> Result<()> {
        self.push_task_pri(task, true)
    }

    #[cfg(test)]
    pub fn uncache(&self, pid: &str) {
        self.procs.remove(pid);
    }

    fn get_process(&self, process_id: &str, runtime: &Arc<Runtime>) -> Result<Arc<Process>> {
        let store = self.store.read().unwrap();

        store.load_proc(&process_id, runtime).and_then(|process| process.ok_or(
            ActError::Runtime(format!("cannot find process '{process_id}'"))
        ))
    }

    fn get_task(&self, process_id: &str, task_id: &str, runtime: &Arc<Runtime>) -> Result<Arc<Task>> {
        let process = self.get_process(process_id, runtime)?;

        process.task(task_id).ok_or(
            ActError::Runtime(format!("cannot find task '{task_id}' on process '{process_id}'"))
        )
    }

    pub(super) fn push_proc_pri(&self, proc: &Arc<Process>, save: bool) {
        debug!("push process pid={}", proc.id());
        if save {
            let store = self.store.read().unwrap();
            store.upsert_proc(proc).expect("fail to upsert process");
        }
        self.procs.insert(proc.id().to_string(), proc.clone());
    }

    pub(super) fn push_task_pri(&self, task: &Arc<Task>, save: bool) -> Result<()> {
        let p = task.proc();
        if save {
            let store = self.store.read().unwrap();
            // update process when updating the task
            let mut proc = store.procs().find(&task.pid)?;
            proc.end_time = p.end_time();
            proc.state = p.state().into();
            store.procs().update(&proc)?;

            store.upsert_task(task)?;
        }

        if let Some(proc) = self.procs.get(&task.pid) {
            proc.set_pure_state(p.state());
            proc.set_end_time(p.end_time());
            proc.push_task(task.clone());
        }

        Ok(())
    }
}
