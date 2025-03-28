use crate::task::Task;
use crate::task_list_controller::state::TaskListMode;
use crate::task_list_controller::TaskListController;
use crate::ui::joiner::render_joiner;
use crate::{task::render::TaskState, task_list::Direction};
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::{buffer::Buffer, layout::Rect, prelude::StatefulWidget};

impl Widget for &TaskListController {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render the task list without joiners
        let task_list = &self.task_list;
        let mut y: u16 = area.y;

        // Keep track of the x and y coordinates of all rendered tasks
        let mut rendered_tasks: Vec<(u16, u16, usize)> = Vec::new();

        // Render each task in the task list
        for (pos, task) in task_list.tasks.iter().enumerate() {
            let x = (task.depth * 3) as u16 + area.x;
            // Create the area that the task will be rendered in and render the task
            let task_area = Rect::new(x, y, area.width - task.depth as u16 * 3, 1);

            // Create a state for each rendered task based on the current state
            let mut task_state = TaskState::default();
            // A task is only a leaf task if the task below it is at the same or lower depth
            if task_list.neighbour_depth(pos, &Direction::Down) <= task.depth {
                task_state.leaf = true;
            }
            if self.state.selected_pos == pos {
                task_state.selected = true;
            }

            // Render each task and keep track of what has been rendered
            task.render(task_area, buf, &mut task_state);
            rendered_tasks.push((x, y, pos));

            y += 1;
            // TODO: Stop rendering once outside the main area
        }

        // Render subtask joiners
        for (x, y, pos) in &rendered_tasks {
            // If there are no subtasks, do nothing further
            if !task_list.has_subtasks(*pos) {
                continue;
            }
            // Get the top position of the task
            let top = *y;
            let mut rendered_subtasks: Vec<u16> = vec![];

            // Get the direct subtasks of this task that have been rendered to the screen
            let subtasks = task_list.get_direct_subtasks(*pos);
            for subtask_pos in subtasks {
                // Find the subtasks that have been rendered
                let Some((_, subtask_y, _)) =
                    &rendered_tasks.iter().find(|(_, _, p)| *p == subtask_pos)
                else {
                    continue;
                };

                // Keep a vector of the positions of all rendered subtasks.
                rendered_subtasks.push(*subtask_y);
            }

            let Ok(task) = task_list.get_task(*pos) else {
                continue;
            };

            // Render the joiner
            render_joiner(*x, *y, rendered_subtasks, buf, &task.execution_order);
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
