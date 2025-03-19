use crate::app::App;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use std::io;

mod app;
mod tasks;
mod ui;
mod help;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
