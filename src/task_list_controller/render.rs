use crate::task_list_controller::TaskListController;
use crate::{
    task::render::TaskState, task_list::state::TaskListState, task_list::Direction, ui::joiner,
};
use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::{Line, StatefulWidget},
};
use std::cmp::min;

impl Widget for &TaskListController {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let task_list = &self.task_list;
        let num_tasks = min(area.height, task_list.tasks.len() as u16);
        let mut y: u16 = area.y;

        for (pos, task) in task_list.tasks.iter().enumerate() {
            // Create a joiner for every task except the first
            if pos != 0 {
                let joiner = joiner::Joiner::create(
                    task_list.neighbour_depth(pos, &Direction::Up),
                    task.depth,
                );
                if let Some(joiner) = joiner {
                    let depth = min(task_list.neighbour_depth(pos, &Direction::Up), task.depth);
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
            if self.task_list_state.selected_pos == pos {
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
