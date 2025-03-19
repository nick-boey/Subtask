/// Contains the application state of the list.
#[derive(Debug, Default, Clone)]
pub struct TaskListState {
    /// The position of the currently selected task in the list.
    pub(crate) selected_pos: usize,
}

impl TaskListState {
    /// Creates a default TaskListState
    pub fn default() -> TaskListState {
        TaskListState { selected_pos: 0 }
    }
}
