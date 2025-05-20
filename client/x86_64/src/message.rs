use termion::event::Key;

pub enum Message {
    Input(Key),
    Resize,
}
