use crate::tui::app::TuiApp;
use ratatui::{prelude::*, widgets::Block};

impl Widget for &TuiApp {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title: Line<'_> = Line::from("pkm".bold());
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
    }
}
