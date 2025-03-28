use crate::app::App;
use std::io;
use tracing::debug;

mod app;
mod debug;
mod task;
mod task_list;
mod task_list_controller;
mod ui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
