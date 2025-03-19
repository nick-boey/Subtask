use crate::task::{Task, TaskStatus};
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
    pub selected: bool,
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
