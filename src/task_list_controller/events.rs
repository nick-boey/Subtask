use crate::app::KeyEventHandler;
use crate::task_list_controller::state::{NewTaskDirection, NewTaskState, TaskListMode};
use crate::task_list_controller::TaskListController;
use crossterm::event::{KeyCode, KeyEvent};

impl KeyEventHandler for TaskListController {
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.state.mode {
            TaskListMode::Normal => match key_event.code {
                KeyCode::Char('j') => self.move_cursor_down(),
                KeyCode::Char('k') => self.move_cursor_up(),
                KeyCode::Char('o') => self.start_new_task_mode(NewTaskDirection::Below),
                KeyCode::Char('O') => self.start_new_task_mode(NewTaskDirection::Above),
                KeyCode::Tab => self.demote_task(),
                KeyCode::BackTab => self.promote_task(),
                KeyCode::Char(' ') => self.toggle_task_status(),
                KeyCode::Char('d') => self.delete_task(),
                KeyCode::Char('t') => self.toggle_execution_order(),
                KeyCode::Char('T') => self.test(),
                _ => {}
            },
            TaskListMode::New(_) => match key_event.code {
                KeyCode::Enter => self.new_task(),
                _ => {}
            },
            TaskListMode::Edit(_) => match key_event.code {
                KeyCode::Enter => {
                    todo!();
                }
                _ => {}
            },
            TaskListMode::Selection(_) => {}
        }
    }
}

impl TaskListController {
    /// Moves the cursor up in the task list by one
    fn move_cursor_up(&mut self) {
        let state = &mut self.state;
        if state.selected_pos != 0 {
            state.selected_pos -= 1;
        }
    }

    /// Moves the cursor down in the task list by one
    fn move_cursor_down(&mut self) {
        let state = &mut self.state;
        let task_list = &self.task_list;
        if state.selected_pos < task_list.len() - 1 {
            state.selected_pos += 1;
        }
    }

    fn start_new_task_mode(&mut self, dir: NewTaskDirection) {
        let Some(task) = self.selected_task() else {
            return;
        };

        let state = NewTaskState::new(task.depth, dir);
        self.state.mode = TaskListMode::New(state);
    }

    fn new_task(&mut self) {
        todo!();
    }

    fn promote_task(&mut self) {
        self.task_list.promote_task(self.state.selected_pos);
    }

    fn demote_task(&mut self) {
        self.task_list.demote_task(self.state.selected_pos);
    }

    fn toggle_task_status(&mut self) {
        self.task_list.toggle_task_status(self.state.selected_pos);
    }

    fn toggle_execution_order(&mut self) {
        self.task_list
            .toggle_execution_order(self.state.selected_pos);
    }

    fn delete_task(&mut self) {
        self.task_list.delete_task(self.state.selected_pos);
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
