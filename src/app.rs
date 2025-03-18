use crate::tasks::{task::Render, task_list::TaskList};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
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

#[derive(Debug, Default)]
pub struct App {
    task_list: Option<TaskList>,
    task_detail_visible: bool,
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
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('t') => self.test(),
            _ => {}
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

    fn instructions(&self) -> Line {
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
            .title_bottom(self.instructions().right_aligned())
            .border_set(border::ROUNDED);

        match &self.task_list {
            // If a task list is available, render it to the screen in a list
            Some(task_list) => {
                let text = task_list.render();
                Paragraph::new(text).block(block).render(area, buf);
            }
            // Prompt the user to create a new task list
            None => {
                let text = Text::from("Press <t> to create a new test task list!");
                Paragraph::new(text)
                    .centered()
                    .block(block)
                    .render(area, buf);
            }
        }
    }
}
