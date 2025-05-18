use std::io::{self, Stdin, Stdout};

use ratatui::{
    Frame, Terminal,
    buffer::Buffer,
    layout::Rect,
    prelude::TermionBackend,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use termion::{
    event::Key,
    input::{Keys, TermRead},
    raw::{IntoRawMode, RawTerminal},
    screen::{AlternateScreen, IntoAlternateScreen},
};

type Tty = Terminal<TermionBackend<AlternateScreen<RawTerminal<Stdout>>>>;

fn main() -> io::Result<()> {
    let writer = std::io::stdout().into_raw_mode()?.into_alternate_screen()?;
    let backend = TermionBackend::new(writer);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    App::default().run(&mut terminal)
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut Tty) -> io::Result<()> {
        let mut keys = std::io::stdin().keys();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events(&mut keys)?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self, keys: &mut Keys<Stdin>) -> io::Result<()> {
        if let Some(key) = keys.next() {
            match key? {
                Key::Left => self.decrement_counter(),
                Key::Right => self.increment_counter(),
                Key::Char('q') => self.exit(),
                _ => {}
            };
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::PLAIN);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
