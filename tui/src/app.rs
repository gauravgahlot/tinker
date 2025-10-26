use core::time;

use ratatui::crossterm;
use ratatui::{
    crossterm::event::{KeyCode, KeyEventKind},
    DefaultTerminal, Frame,
};

use crate::ui::{Hardware, Splash, Template};

pub(crate) struct App {
    state: AppState,
    current_screen: Screen,
    last_screen: Option<Screen>,
    ns: String,
}

#[derive(PartialEq)]
pub(crate) enum AppState {
    Disconnected,
    Connected,
    Exit,
}

pub(crate) trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

#[derive(Clone, Copy)]
pub(crate) enum Screen {
    Splash(Splash),
    Hardware(Hardware),
    Template(Template),
}

impl Drawable for Screen {
    fn draw(&self, frame: &mut Frame) {
        match self {
            Screen::Hardware(v) => v.draw(frame),
            Screen::Splash(v) => v.draw(frame),
            Screen::Template(v) => v.draw(frame),
        }
    }
}

impl App {
    pub fn new(ns: &str) -> Self {
        Self {
            ns: String::from(ns),
            state: AppState::Disconnected,
            current_screen: Screen::Splash(Splash::new()),
            last_screen: None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        let splash_started = std::time::Instant::now();

        loop {
            if crossterm::event::poll(time::Duration::from_millis(100))? {
                match crossterm::event::read()? {
                    crossterm::event::Event::Key(key) => self.handle_key_event(key)?,
                    _ => {}
                }
            }

            match self.state {
                AppState::Disconnected => {
                    terminal.draw(|frame| self.current_screen.draw(frame))?;
                    if splash_started.elapsed().as_secs() >= 1 {
                        // pretend we bootstrapped
                        self.state = AppState::Connected;
                        self.current_screen = Screen::Hardware(Hardware::new(self.ns.as_str()));
                    }
                }
                AppState::Connected => {
                    terminal.draw(|frame| self.current_screen.draw(frame))?;
                }
                AppState::Exit => break,
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> anyhow::Result<()> {
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
            self.state = AppState::Exit;

            return Ok(());
        }

        if self.state == AppState::Disconnected {
            return Ok(());
        }

        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => self.state = AppState::Exit,
                KeyCode::Esc => {
                    if let Some(s) = self.last_screen {
                        self.current_screen = s;
                    }
                }
                KeyCode::Char('h') => {
                    self.last_screen = Some(self.current_screen);
                    self.current_screen = Screen::Hardware(Hardware::new(self.ns.as_str()))
                }
                KeyCode::Char('t') => {
                    self.last_screen = Some(self.current_screen);
                    self.current_screen = Screen::Template(Template::new())
                }
                _ => {}
            }
        }

        Ok(())
    }
}
