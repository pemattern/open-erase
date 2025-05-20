use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

pub struct Window {
    active: bool,
}

impl Widget for &Window {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
