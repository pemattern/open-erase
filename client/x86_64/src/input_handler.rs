use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
    },
    thread::{self, JoinHandle},
    time::Duration,
};
use termion::input::TermRead;

use crate::message::Message;

pub struct InputHandler;

impl InputHandler {
    pub fn listen(sender: Sender<Message>, exit: Arc<AtomicBool>) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut keys = termion::async_stdin().keys();
            loop {
                if let Some(key) = keys.next() {
                    sender.send(Message::Input(key.unwrap())).unwrap();
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
                std::thread::sleep(Duration::from_millis(25));
            }
        })
    }
}
