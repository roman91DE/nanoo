use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

pub enum Command {
    Quit,
    Save,
    InsertChar(char),
    MoveCursorUp,
    MoveCursorDown,
    MoveCursorLeft,
    MoveCursorRight,
}

pub fn handle_input() -> Option<Command> {
    if let Event::Key(KeyEvent {
        code, modifiers, ..
    }) = event::read().unwrap()
    {
        match (modifiers, code) {
            (KeyModifiers::CONTROL, KeyCode::Char('x')) => Some(Command::Quit),
            (KeyModifiers::CONTROL, KeyCode::Char('s')) => Some(Command::Save),
            (KeyModifiers::NONE, KeyCode::Char(c)) => Some(Command::InsertChar(c)),
            (KeyModifiers::NONE, KeyCode::Up) => Some(Command::MoveCursorUp),
            (KeyModifiers::NONE, KeyCode::Down) => Some(Command::MoveCursorDown),
            (KeyModifiers::NONE, KeyCode::Left) => Some(Command::MoveCursorLeft),
            (KeyModifiers::NONE, KeyCode::Right) => Some(Command::MoveCursorRight),
            _ => None, 
        }
    } else {
        None
    }
}
