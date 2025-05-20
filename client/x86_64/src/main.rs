mod app;
mod args;
mod input_handler;
mod message;
mod widgets;

use std::io;

use app::App;
use args::Args;
use clap::Parser;
use ratatui::{Terminal, prelude::TermionBackend};
use termion::{raw::IntoRawMode, screen::IntoAlternateScreen};

fn main() -> io::Result<()> {
    let _args = Args::parse();
    let writer = io::stdout().into_raw_mode()?.into_alternate_screen()?;
    let backend = TermionBackend::new(writer);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    App::default().run(&mut terminal)?;
    terminal.clear()?;
    Ok(())
}
