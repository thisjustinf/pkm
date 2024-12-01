use std::rc::Rc;

use crate::models::note::Note;
use crate::tui::app::TuiApp;
use ratatui::prelude::*;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_ui(frame: &mut Frame, app: &TuiApp) {
    let chunks: Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20), // Title area
                Constraint::Percentage(60), // List area
                Constraint::Percentage(20), // Input area
            ]
            .as_ref(),
        )
        .split(frame.area());

    let notes: Vec<ListItem> = app
        .notes
        .iter()
        .map(|note: &Note| ListItem::new(note.title.clone()))
        .collect();
    let notes_list: List<'_> =
        List::new(notes).block(Block::default().borders(Borders::ALL).title("Notes"));

    frame.render_widget(notes_list, chunks[1]);

    let input: Paragraph<'_> =
        Paragraph::new("").block(Block::default().borders(Borders::ALL).title("Input"));
    frame.render_widget(input, chunks[2]);
}
