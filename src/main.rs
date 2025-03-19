use crate::app::App;
use std::io;

mod app;
mod task;
mod task_list;
mod ui;
mod task_list_controller;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
