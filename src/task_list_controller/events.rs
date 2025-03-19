use crate::app::KeyEventHandler;
use crate::task_list_controller::TaskListController;
use crossterm::event::{KeyCode, KeyEvent};

impl KeyEventHandler for TaskListController {
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('j') => self.move_cursor_down(),
            KeyCode::Char('k') => self.move_cursor_up(),
            KeyCode::Char(' ') => self.toggle_task_status(),
            KeyCode::Char('t') => self.test(),
            _ => {}
        }
    }
}

impl TaskListController {
    /// Moves the cursor up in the task list by one
    fn move_cursor_up(&mut self) {
        let state = &mut self.task_list_state;
        if state.selected_pos != 0 {
            state.selected_pos -= 1;
        }
    }

    /// Moves the cursor down in the task list by one
    fn move_cursor_down(&mut self) {
        let state = &mut self.task_list_state;
        let task_list = &self.task_list;
        if state.selected_pos < task_list.len() - 1 {
            state.selected_pos += 1;
        }
    }

    fn toggle_task_status(&mut self) {
        let task_list = &mut self.task_list;
        task_list.toggle_task_status(self.task_list_state.selected_pos);
    }

    /// Temporary test function that creates a task list with some tasks
    fn test(&mut self) {
        let task_list = &mut self.task_list;
        task_list.add_new_root_task_at_end("Task 1");
        task_list.add_new_root_task_at_end("Task 2");
        task_list.add_new_root_task_at_end("Task 3");
        task_list.add_new_subtask("Task 1.1", 0);
        task_list.add_new_subtask("Task 1.1.1", 1);
    }
}
