use crate::app::App;
use crate::ui::help;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    prelude::StatefulWidget,
    prelude::{Line, Stylize, Widget},
    symbols::border,
    widgets::Block,
    Frame,
};

impl App {
    pub(crate) fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Subtask ".bold());
        let block = Block::bordered()
            .title(title)
            .title_bottom(footing_prompts().right_aligned())
            .border_set(border::ROUNDED);
        let inner_area = block.inner(area);
        block.render(area, buf);

        self.task_list_controller.render(inner_area, buf);

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

fn footing_prompts() -> Line<'static> {
    let instructions = Line::from(vec![
        " ? ".into(),
        "<Help> ".green().bold(),
        " q ".into(),
        "<Quit> ".red().bold(),
    ]);
    instructions
}
