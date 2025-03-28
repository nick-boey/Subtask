use crate::task::ExecutionOrder;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Span;
use ratatui::widgets::Widget;

/// Represents a visual joiner element between tasks.
pub(crate) struct Joiner {
    /// Type of the joiner that is being created.
    executionOrder: ExecutionOrder,
    /// Vector of the y-coordinates below the current position that are being joined to (i.e. the relative position of each rendered subtask).
    subtask_coords: Vec<u16>,
}

/// Renders a new joiner starting at the specified location and connecting the positions given in `subtask_coords`
pub fn render_joiner(
    mut x: u16,
    mut y: u16,
    subtask_coords: Vec<u16>,
    buf: &mut Buffer,
    execution_order: &ExecutionOrder,
) {
    x += 1;
    // Render the first part of the joiner
    buf.set_span(x, y, &Span::from("╮"), 1);
    y += 1;
    match execution_order {
        ExecutionOrder::Series => {
            // Connect first joiner
            buf.set_span(x, y, &Span::from("╰─"), 2);
            // Increment x to align with next tasks
            x += 2;

            if subtask_coords.len() <= 1 {
                return;
            }

            // Connect the rest of the subtasks if there is a space between them
            let mut y_current = y + 1;
            for y_next in &subtask_coords[1..] {
                for y_cursor in y_current..y_next - 1 {
                    buf.set_span(x, y_cursor, &Span::from("│"), 1);
                }
                y_current = y_next + 1;
            }
        }
        ExecutionOrder::Parallel => {
            // Connect to last subtask
            let Some(y_last) = subtask_coords.last() else {
                return;
            };
            buf.set_span(x, *y_last, &Span::from("x─"), 2);

            // Connect to all intermediate subtasks
            for y_next in &subtask_coords[..subtask_coords.len() - 1] {
                buf.set_span(x, *y_next, &Span::from("│"), 1);
            }
        }
    }
}
