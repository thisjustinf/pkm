use crate::tui::app::App;
use ratatui::symbols::border;
use ratatui::{prelude::*, widgets::Block};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // let title: Line<'_> = Line::from("pkm".bold());
        // let block = Block::bordered()
        //     .title(title.centered())
        //     .title_bottom(instructions.centered())
        //     .border_set(border::THICK);
        todo!()
    }
}
