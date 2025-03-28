use crate::task_list_controller::TaskListController;
mod events;
mod render;

use crossterm::event::KeyEvent;
use ratatui::DefaultTerminal;
use std::io;
#[derive(Debug, Clone, Copy, Default)]
enum AppMode {
    #[default]
    TaskList,
    TodayTasks,
}

#[derive(Debug, Default)]
pub struct App {
    task_list_controller: TaskListController,
    task_detail_visible: bool,
    help_visible: bool,
    mode: AppMode,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
}
pub(crate) trait KeyEventHandler {
    fn handle_key_event(&mut self, key_event: KeyEvent);
}
