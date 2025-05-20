mod app;
mod input_handler;
mod message;

use std::io;

use app::App;
use ratatui::{Terminal, prelude::TermionBackend};
use termion::{raw::IntoRawMode, screen::IntoAlternateScreen};

fn main() -> io::Result<()> {
    let writer = io::stdout().into_raw_mode()?.into_alternate_screen()?;
    let backend = TermionBackend::new(writer);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    App::default().run(&mut terminal)?;
    terminal.clear()
}
