use crate::app::{App, AppMode};
use crate::task_list::TaskList;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

impl App {
    pub(crate) fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn test(&mut self) {
        self.task_list = Some(TaskList::new("Task List"));
        let task_list = self.task_list.as_mut().unwrap();
        task_list.add_new_root_task_at_end("Task 1");
        task_list.add_new_root_task_at_end("Task 2");
        task_list.add_new_root_task_at_end("Task 3");
        task_list.add_new_subtask("Task 1.1", 0);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            // Global key commands
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('t') => self.test(),
            // Toggle help overlay
            KeyCode::Char('?') => self.help_visible = !self.help_visible,
            _ => match self.mode {
                AppMode::TaskList => match key_event.code {
                    KeyCode::Char('j') => self.move_cursor_down(),
                    KeyCode::Char('k') => self.move_cursor_up(),
                    KeyCode::Char(' ') => self.toggle_task_status(),
                    _ => {}
                },
                AppMode::TodayTasks => {}
            },
        }
    }

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
        if let Some(task_list) = &self.task_list {
            if state.selected_pos < task_list.len() - 1 {
                state.selected_pos += 1;
            }
        }
    }

    fn toggle_task_status(&mut self) {
        if let Some(task_list) = &mut self.task_list {
            task_list.toggle_task_status(self.task_list_state.selected_pos);
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
