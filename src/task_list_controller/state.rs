/// Contains the application state of the list.
#[derive(Debug, Default, Clone)]
pub struct TaskListState {
    /// The position of the currently selected task in the list.
    pub(crate) selected_pos: usize,
    pub(crate) mode: TaskListMode,
}

/// The mode that the application is currently in within the task list.
#[derive(Debug, Default, Clone)]
pub(crate) enum TaskListMode {
    #[default]
    /// The default mode where the user can navigate the list.
    Normal,
    /// The user is currently entering a new task.
    New(NewTaskState),
    /// The user is currently editing an existing task.
    Edit(EditTaskState),
    /// The user is selecting a continuous group of tasks.
    Selection(TaskSelectionState),
}

/// The state of the application when in task entry mode.
#[derive(Debug, Clone)]
pub struct NewTaskState {
    pub(crate) title: String,
    pub(crate) depth: i8,
    pub(crate) dir: NewTaskDirection,
}

impl NewTaskState {
    pub fn new(depth: i8, dir: NewTaskDirection) -> NewTaskState {
        NewTaskState {
            title: String::new(),
            depth,
            dir,
        }
    }
}

#[derive(Debug, Clone)]
pub enum NewTaskDirection {
    Above,
    Below,
}

#[derive(Debug, Clone)]
pub struct EditTaskState {
    pub(crate) pos: usize,
    pub(crate) title: String,
}

impl EditTaskState {
    pub fn new(pos: usize, title: String) -> EditTaskState {
        EditTaskState { pos, title }
    }
}

/// The state of the application when in task selectino mode.
#[derive(Debug, Clone)]
pub struct TaskSelectionState {
    pub(crate) pos_start: usize,
    pub(crate) pos_end: usize,
}

impl TaskSelectionState {
    pub fn new(pos_start: usize, pos_end: usize) -> TaskSelectionState {
        TaskSelectionState { pos_start, pos_end }
    }
}
