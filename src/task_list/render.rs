use crate::task::{render::TaskState, Task};
use crate::task_list::{Direction, TaskList};
use crate::ui::joiner;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::{Line, StatefulWidget},
};
use std::cmp::min;

impl StatefulWidget for &TaskList {
    type State = TaskListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let num_tasks = min(area.height, self.tasks.len() as u16);
        let mut y: u16 = area.y;

        for (pos, task) in self.tasks.iter().enumerate() {
            // Create a joiner for every task except the first
            if pos != 0 {
                let joiner =
                    joiner::Joiner::create(self.neighbour_depth(pos, &Direction::Up), task.depth);
                if let Some(joiner) = joiner {
                    let depth = min(self.neighbour_depth(pos, &Direction::Up), task.depth);
                    buf.set_line(
                        (depth * 2) as u16 + area.x,
                        y,
                        &Line::from(joiner),
                        area.width,
                    );
                };
                y += 1;
            }

            // Create the area that the task will be rendered in and render the task
            let task_area = Rect::new(
                (task.depth * 2) as u16 + area.x,
                y,
                area.width - task.depth as u16 * 2,
                1,
            );

            let mut task_state = TaskState::default();
            if state.selected_pos == pos {
                task_state.selected = true;
            }

            task.render(task_area, buf, &mut task_state);

            y += 1;
            if y - area.y >= num_tasks * 2 {
                break;
            }
        }
    }
}

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
