use crate::task_list_controller::state::TaskListMode;
use crate::task_list_controller::TaskListController;
use crate::{task::render::TaskState, task_list::Direction, ui::joiner};
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::{Line, StatefulWidget},
};
use std::cmp::min;
use tui_input::{Input, InputRequest, StateChanged};

impl Widget for &TaskListController {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let task_list = &self.task_list;
        let num_tasks = min(area.height / 2, task_list.tasks.len() as u16);
        let mut y: u16 = area.y;

        // Render each task in the task list
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

            // Create a state for each rendered task based on the current state
            let mut task_state = TaskState::default();
            if self.state.selected_pos == pos {
                task_state.selected = true;
            }

            // Render each task
            task.render(task_area, buf, &mut task_state);

            y += 1;
            // Stop rendering once outside the main window area
            if y - area.y >= num_tasks * 2 {
                break;
            }
        }
    }
}

impl TaskListController {
    fn render_input(&mut self, area: Rect, buf: &mut Buffer) {
        if let TaskListMode::New(state) = &self.state.mode {
            let text: Text = state.title.clone().into();
            let block = Block::default().on_dark_gray();
            let input = Paragraph::new(text).block(block);
            input.render(area, buf);
        }
    }
}
