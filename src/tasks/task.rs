use chrono::{DateTime, Local};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::StatefulWidget;
use std::cmp::PartialEq;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum TaskStatus {
    NotStarted,
    InProgress(DateTime<Local>),
    Complete(DateTime<Local>),
}

#[derive(Debug, Clone)]
pub enum ExecutionOrder {
    Series,
    Parallel,
}

/// A task to be executed.
#[derive(Debug, Clone)]
pub struct Task {
    /// The unique identifier of the task.
    pub id: Uuid,
    /// The title of the task.
    pub title: String,
    /// A description of the task.
    pub description: String,
    /// True if the task is the next to be executed, false if not.
    pub is_next: bool,
    /// True if the task is on the critical chain, false if not.
    pub is_critical: bool,
    /// The status of the task.
    pub task_status: TaskStatus,
    /// The order in which the subtasks should be executed. Defaults to Series.
    pub execution_order: ExecutionOrder,

    /// The depth at which the task is currently sitting. Used to determine the subtasks.
    pub depth: i8,

    /// The automatically created creation date.
    pub creation_date: DateTime<Local>,
    /// An optional start date for the task.
    pub start_date: Option<DateTime<Local>>,
    /// An optional due date for the task.
    pub due_date: Option<DateTime<Local>>,

    pub expected_duration: Option<i32>,
}

impl Task {
    /// Creates a new instance of a task with a name and description
    pub fn new(title: &str, depth: i8) -> Task {
        Task {
            id: Uuid::new_v4(),
            title: title.to_string(),
            description: String::from(""),
            is_next: false,
            is_critical: false,
            creation_date: Local::now(),
            start_date: None,
            due_date: None,
            task_status: TaskStatus::NotStarted,
            execution_order: ExecutionOrder::Series,
            expected_duration: None,
            depth,
        }
    }

    pub(crate) fn toggle_status(&mut self) {
        match self.task_status {
            TaskStatus::NotStarted => {
                self.task_status = TaskStatus::InProgress(Local::now());
            }
            TaskStatus::InProgress(_) => {
                self.task_status = TaskStatus::Complete(Local::now());
            }
            TaskStatus::Complete(_) => {
                self.task_status = TaskStatus::NotStarted;
            }
        }
    }

    /// Set the execution order to a new value.
    fn execution_order(&mut self, order: ExecutionOrder) {
        self.execution_order = order;
    }

    /// Sets the task to an active status and starts logging the time since it was active.
    fn set_active(&mut self, new_value: bool) {
        if new_value && !self.is_next {
            self.task_status = TaskStatus::InProgress(Local::now())
        }
    }

    /// Calculates the amount of time that has elapsed between when the task was made active and now.
    fn active_time(&self) -> i64 {
        match self.task_status {
            TaskStatus::InProgress(start_time) => {
                let now = Local::now();
                now.signed_duration_since(start_time).num_minutes()
            }
            _ => 0,
        }
    }

    /// Calculates the amount of time that was has elapsed between when the task was first made active to when it was completed.
    fn completion_time(&self) -> Option<i64> {
        match self.task_status {
            TaskStatus::Complete(end_time) => {
                let now = Local::now();
                Option::from(now.signed_duration_since(end_time).num_minutes())
            }
            _ => None,
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl StatefulWidget for &Task {
    type State = TaskState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let symbol = match self.task_status {
            TaskStatus::NotStarted => "○",
            TaskStatus::InProgress(_) => "○",
            TaskStatus::Complete(_) => "●",
        };
        let mut line = Line::from(format!("{} {}\r\n", symbol, self.title));
        if state.selected {
            line = line.underlined();
        }
        match self.task_status {
            TaskStatus::NotStarted => {}
            TaskStatus::InProgress(_) => {
                line = line.yellow().bold();
            }
            TaskStatus::Complete(_) => {
                line = line.green().italic();
            }
        }
        buf.set_line(area.x, area.y, &line, area.width);
    }
}

pub struct TaskState {
    pub(crate) selected: bool,
    visible: bool,
}

impl TaskState {
    pub fn default() -> TaskState {
        TaskState {
            selected: false,
            visible: true,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn new_name_is_correct() {
        let task = super::Task::new(&String::from("Task 1"), 0);
        assert_eq!(task.title, "Task 1");
    }
}
