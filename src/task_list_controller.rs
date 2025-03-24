mod events;
mod render;
pub(crate) mod state;

use crate::task::Task;
use crate::task_list::TaskList;
use crate::task_list_controller::state::TaskListMode;
use state::TaskListState;

#[derive(Debug, Default)]
pub(crate) struct TaskListController {
    pub(crate) task_list: TaskList,
    pub(crate) state: TaskListState,
}

impl TaskListController {
    /// Get the currently selected task, if there is one.
    pub(crate) fn selected_task(&self) -> Option<&Task> {
        match self.state.mode {
            // Don't provide a task back if in selection mode, as there may be many selected tasks.
            TaskListMode::Selection(_) => None,
            _ => {
                let Ok(result) = self.task_list.get_task(self.state.selected_pos) else {
                    return None;
                };
                Some(result)
            }
        }
    }
}
