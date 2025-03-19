use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
};
use std::collections::HashMap;

/// Renders the help menu into the given buffer.
pub fn render_help(area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .border_set(border::ROUNDED)
        .blue()
        .title(" Help ".bold());
    Clear.render(area, buf);

    let key_maps = [
        KeyMap::global_key_commands_default(),
        KeyMap::task_list_key_commands_default(),
    ];

    let key_map_lines = key_maps
        .iter()
        .flat_map(|key_map| key_map.to_lines())
        .collect::<Vec<Line>>();

    let help_text = Paragraph::new(key_map_lines).block(block);
    help_text.render(area, buf);
}

struct KeyMap {
    title: String,
    commands: HashMap<String, String>,
}

impl KeyMap {
    pub fn global_key_commands_default() -> KeyMap {
        let mut map = KeyMap {
            title: String::from("Global commands"),
            commands: HashMap::new(),
        };
        map.insert_command("q", "Quit the application")
            .insert_command("?", "Toggle the help menu");
        map
    }

    pub fn task_list_key_commands_default() -> KeyMap {
        let mut map = KeyMap {
            title: String::from("Task list commands"),
            commands: HashMap::new(),
        };
        map.insert_command("j", "Go up one task")
            .insert_command("k", "Go down one task")
            .insert_command("h", "Go up one level")
            .insert_command("l", "Go down one level")
            .insert_command("i", "Edit the task title from the beginning")
            .insert_command("a", "Edit the task title from the end")
            .insert_command("c", "Delete the task title and start editing")
            .insert_command("o", "Add a new task below")
            .insert_command("O", "Add a new task above")
            .insert_command("d", "Delete the current task")
            .insert_command("v", "Enter selection mode.")
            .insert_command("TAB", "Move the task down one level")
            .insert_command("SHIFT+TAB", "Move the task up one level")
            .insert_command("SPACE", "Change the task status");
        map
    }

    fn insert_command(&mut self, key: &str, description: &str) -> &mut Self {
        self.commands
            .insert(key.to_string(), description.to_string());
        self
    }

    fn to_lines(&self) -> Vec<Line> {
        let mut lines = vec![];
        lines.push(Line::from(self.title.clone()).bold());
        lines.extend(self.commands.iter().map(|(key, description)| {
            Line::from(vec![
                "  ".into(),
                "<".into(),
                key.clone().bold(),
                "> ".into(),
                description.clone().into(),
            ])
        }));
        lines
    }
}
