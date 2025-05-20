use std::{
    io::{self, Stdout},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
};

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

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
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
                    Key::Left => self.decrement_counter(),
                    Key::Right => self.increment_counter(),
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
