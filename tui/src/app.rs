use core::time;

use ratatui::crossterm;
use ratatui::crossterm::event::{KeyCode, KeyEventKind};
use ratatui::prelude::Stylize;
use ratatui::{text::Line, widgets::Widget, DefaultTerminal, Frame};

#[derive(Default)]
pub(crate) struct App {
    ns: String,
    exit: bool,
}

impl App {
    pub fn new(ns: &str) -> Self {
        Self {
            ns: String::from(ns),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        while !self.exit {
            if crossterm::event::poll(time::Duration::from_millis(100))? {
                match crossterm::event::read()? {
                    crossterm::event::Event::Key(key) => self.handle_key_event(key)?,
                    _ => {}
                }
            } else {
                terminal.draw(|frame| self.draw(frame))?;
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> anyhow::Result<()> {
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
            self.exit = true;
        }

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Line::from(format!("Let's tinker in {} namespace!!", self.ns.as_str()))
            .bold()
            .render(area, buf);
    }
}
