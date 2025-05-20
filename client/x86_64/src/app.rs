use std::{
    io::{self, Stdout},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
};

use clap::crate_version;
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
use termion::{event::Key, raw::RawTerminal, screen::AlternateScreen};

use crate::{input_handler::InputHandler, message::Message};

const APP_TITLE: &str = concat!(" OpenErase ", crate_version!(), " ");

#[derive(Debug, Default)]
pub struct App {
    exit: Arc<AtomicBool>,
}

type Tty = Terminal<TermionBackend<AlternateScreen<RawTerminal<Stdout>>>>;

impl App {
    pub fn run(&mut self, terminal: &mut Tty) -> io::Result<()> {
        let (sender, receiver) = mpsc::channel();
        let input_thread = InputHandler::listen(sender.clone(), self.exit.clone());
        while !self.exit.load(Ordering::SeqCst) {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_messages(&receiver);
        }
        input_thread.join().unwrap();
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_messages(&mut self, receiver: &mpsc::Receiver<Message>) {
        if let Ok(message) = receiver.recv() {
            match message {
                Message::Input(key) => match key {
                    Key::Char('q') => self.exit(),
                    _ => {}
                },
                Message::Resize => {}
            }
        }
    }

    fn exit(&mut self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(APP_TITLE);
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::PLAIN);

        Paragraph::new(open_erase_lib::audit::pci::get_pci())
            .block(block)
            .render(area, buf);
    }
}
