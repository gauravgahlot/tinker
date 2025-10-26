use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Widget};
use ratatui::Frame;

#[derive(Clone, Copy)]
pub struct Template;

impl Template {
    pub fn new() -> Self {
        Self
    }
}

impl crate::app::Drawable for Template {
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &Template {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("Let's tinker with Templates!!")
            .style(Style::new().fg(Color::Green))
            .alignment(ratatui::layout::Alignment::Center)
            .render(area, buf);
    }
}
