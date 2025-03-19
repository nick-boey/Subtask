use crate::help;
use crate::tasks::task_list::TaskList;
use crate::tasks::task_list::TaskListState;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::prelude::StatefulWidget;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
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

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
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

    fn test(&mut self) {
        self.task_list = Some(TaskList::new("Task List"));
        let task_list = self.task_list.as_mut().unwrap();
        task_list.add_new_root_task_at_end("Task 1");
        task_list.add_new_root_task_at_end("Task 2");
        task_list.add_new_root_task_at_end("Task 3");
        task_list.add_new_subtask("Task 1.1", 0);
    }

    fn footing_prompts(&self) -> Line {
        let instructions = Line::from(vec![
            " ? ".into(),
            "<Help> ".green().bold(),
            " q ".into(),
            "<Quit> ".red().bold(),
        ]);
        instructions
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Subtask ".bold());
        let block = Block::bordered()
            .title(title)
            .title_bottom(self.footing_prompts().right_aligned())
            .border_set(border::ROUNDED);
        let inner_area = block.inner(area);
        block.render(area, buf);

        // Render task list
        match &self.task_list {
            // If a task list is available, render it to the screen in a list
            Some(task_list) => {
                task_list.render(inner_area, buf, &mut self.task_list_state.clone());
            }
            // Prompt the user to create a new task list
            None => {
                let text = Text::from("Press <t> to create a new test task list!");
                text.render(inner_area, buf);
            }
        }

        // Render help overlay
        if self.help_visible {
            let [centre_area] = Layout::horizontal([Constraint::Percentage(80)])
                .flex(Flex::Center)
                .areas(area);
            let [centre_area] = Layout::vertical([Constraint::Percentage(80)])
                .flex(Flex::Center)
                .areas(centre_area);
            help::render_help(centre_area, buf);
        }
    }
}
