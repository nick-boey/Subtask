use crate::{task_list::render::TaskListState, task_list::TaskList};

mod actions;
mod render;

use ratatui::{
    style::Stylize,
    widgets::{StatefulWidget, Widget},
    DefaultTerminal,
};
use std::io;

#[derive(Debug, Clone, Copy, Default)]
enum AppMode {
    #[default]
    TaskList,
    TodayTasks,
}

#[derive(Debug, Default)]
pub struct App {
    task_list: Option<TaskList>,
    task_list_state: TaskListState,
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
