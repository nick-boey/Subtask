use crate::app::{App, AppMode, KeyEventHandler};
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

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            // Global key commands
            KeyCode::Char('q') => self.exit(),
            // Toggle help overlay
            KeyCode::Char('?') => self.help_visible = !self.help_visible,
            _ => match self.mode {
                AppMode::TaskList => self.task_list_controller.handle_key_event(key_event),
                AppMode::TodayTasks => {}
            },
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
