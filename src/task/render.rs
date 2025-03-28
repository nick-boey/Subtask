use crate::task::{ExecutionOrder, Task, TaskStatus};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::{Line, StatefulWidget},
    style::Stylize,
};

impl StatefulWidget for &Task {
    type State = TaskState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let symbol = match self.task_status {
            TaskStatus::NotStarted => "○",
            TaskStatus::InProgress(_) => "○",
            TaskStatus::Complete(_) => "●",
        };

        // Add an extra space to the left of the task title if it is a branch task to allow room for the joiner.
        let mut line = match state.leaf {
            true => Line::from(format!("{} {}\r\n", symbol, self.title)),
            false => Line::from(format!("{}  {}\r\n", symbol, self.title)),
        };

        if state.selected {
            line = line.underlined();
        }

        if !state.next {
            line = line.dark_gray();
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

/// Holds the current state of a task that is being rendered.
pub struct TaskState {
    /// True if the task is currently selected.
    pub selected: bool,
    /// True if the task is currently visible.
    pub visible: bool,
    /// True if the task is a leaf task false if it is a branch task
    pub leaf: bool,
    /// True if the task is a candidate to be completed next
    pub next: bool,
}

impl TaskState {
    pub fn default() -> TaskState {
        TaskState {
            selected: false,
            visible: true,
            leaf: false,
            next: false,
        }
    }
}
