use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Widget};
use ratatui::Frame;

#[derive(Default)]
pub struct SplashScreen {}

impl SplashScreen {
    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &SplashScreen {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let logo = vec![
            "                                              ".into(),
            "     ██                ██                     ".into(),
            "  ████████ ██  ██████  ██  ██   ████   █████  ".into(),
            "     ██       ██    ██ ████    ██▄▄▄  ██      ".into(),
            "     ██    ██ ██    ██ ██  ██  ██     ██      ".into(),
            "     ██    ██ ██    ██ ██   ██  ████  ██      ".into(),
            "                                              ".into(),
        ];

        let [logo_area, _msg_area] =
            Layout::vertical([Constraint::Percentage(80), Constraint::Percentage(20)])
                .margin(1)
                .areas(area);

        // Block::bordered()
        //     .title("Message".to_span().into_centered_line())
        //     .border_type(ratatui::widgets::BorderType::Thick)
        //     .style(Style::new().fg(Color::LightYellow))
        //     .render(msg_area, buf);

        Paragraph::new(logo)
            .style(Style::new().fg(Color::Green))
            .alignment(ratatui::layout::Alignment::Center)
            .render(logo_area, buf);

        // Line::from("SplashScreen goes here!!")
        // .bold()
        // .render(area, buf);
    }
}
