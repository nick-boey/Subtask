mod render;
mod events;

use crate::task_list::state::TaskListState;
use crate::task_list::TaskList;

#[derive(Debug, Default)]
pub(crate) struct TaskListController {
    pub(crate) task_list: TaskList,
    pub(crate) task_list_state: TaskListState,
}
