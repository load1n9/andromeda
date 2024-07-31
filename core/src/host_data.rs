use std::{
    cell::RefCell,
    collections::HashMap,
    future::Future,
    sync::{
        atomic::{AtomicU32, Ordering},
        mpsc::{Receiver, Sender},
        Arc,
    },
};

use anymap::AnyMap;
use tokio::task::JoinHandle;

use crate::{Interval, IntervalId, MacroTask, TaskId};

/// Data created and used by the Runtime.
pub struct HostData {
    /// Storage used by the built-in functions.
    pub storage: RefCell<AnyMap>,
    /// Send macro tasks to the event loop.
    pub macro_task_tx: Sender<MacroTask>,
    /// Counter of active macro tasks.
    pub macro_task_count: Arc<AtomicU32>,
    /// Registry of active running intervals.
    pub intervals: RefCell<HashMap<IntervalId, Interval>>,
    /// Counter of accumulative intervals. Used for ID generation.
    pub interval_count: Arc<AtomicU32>,
    /// Registry of async tasks.
    pub tasks: RefCell<HashMap<TaskId, JoinHandle<()>>>,
    /// Counter of accumulative created async tasks.  Used for ID generation.
    pub task_count: Arc<AtomicU32>,
}

impl HostData {
    pub fn new() -> (Self, Receiver<MacroTask>) {
        let (macro_task_tx, rx) = std::sync::mpsc::channel();
        (
            Self {
                storage: RefCell::new(AnyMap::new()),
                macro_task_tx,
                macro_task_count: Arc::new(AtomicU32::new(0)),
                interval_count: Arc::default(),
                intervals: RefCell::default(),
                tasks: RefCell::default(),
                task_count: Arc::default(),
            },
            rx,
        )
    }

    /// Get an owned senderto the macro tasks event loop.
    pub fn macro_task_tx(&self) -> Sender<MacroTask> {
        self.macro_task_tx.clone()
    }

    /// Spawn an async task in the Tokio Runtime that self-registers and unregisters automatically.
    /// It's given [TaskId] is returned.
    pub fn spawn_macro_task<F>(&self, future: F) -> TaskId
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let macro_task_count = self.macro_task_count.clone();
        macro_task_count.fetch_add(1, Ordering::Relaxed);

        let task_handle = tokio::spawn(async move {
            future.await;
            macro_task_count.fetch_sub(1, Ordering::Relaxed);
        });

        let task_id = TaskId::from_index(self.task_count.fetch_add(1, Ordering::Relaxed));
        self.tasks.borrow_mut().insert(task_id, task_handle);

        task_id
    }

    /// Abort a MacroTask execution given it's [TaskId].
    pub fn abort_macro_task(&self, task_id: TaskId) {
        let task = self.tasks.borrow_mut().remove(&task_id).unwrap();
        task.abort();
        self.macro_task_count.fetch_sub(1, Ordering::Relaxed);
    }
}
