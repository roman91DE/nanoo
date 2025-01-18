#![warn(clippy::all, clippy::pedantic)]

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::{self};

use crate::terminal::Terminal;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Result<Self, io::Error> {
        match Terminal::new() {
            Ok(terminal) => Ok(Editor {
                should_quit: false,
                terminal,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn run(&mut self) {
        self.terminal
            .initialize()
            .unwrap();
        self.terminal
            .welcome_message()
            .unwrap();
        self.terminal
            .draw_row()
            .unwrap();
        

        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            if self.should_quit {
                self.terminal
                    .refresh_screen()?;
                break;
            }
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
}

// https://flenker.blog/hecto-chapter-3/
