#![warn(clippy::all, clippy::pedantic)]

use std::io::{self, stdout, Stdout, Write};

use crossterm::{
    cursor::{self, MoveDown, MoveTo}, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}, ExecutableCommand, QueueableCommand
};

pub struct Terminal {
    col_lim: u16,
    row_lim: u16,
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Result<Self, io::Error> {
        match crossterm::terminal::size() {
            Ok(size) => Ok(Terminal {
                col_lim: size.0,
                row_lim: size.1,
                stdout: stdout(),
            }),
            Err(e) => Err(e),
        }
    }
    pub fn initialize(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        self.clear_screen()?;
        self.stdout
            .execute(MoveTo(0, 0))?;
        self.flush_stdout()
    }

    pub fn welcome_message(&mut self) -> Result<(), std::io::Error>{
        let first_line = self.row_lim * 2/3;
        
        assert!(first_line < self.col_lim);


        let message = format!("Nanoo - Version {}\r\n", env!("CARGO_PKG_VERSION"));

        let message_len = message.chars().count();

        assert!(message_len < self.col_lim as usize);

        let col_message = (self.col_lim as usize / 2).saturating_sub(message_len / 2);

        self.stdout
        .queue(MoveTo(first_line, col_message.try_into().unwrap()))?
        .queue(Print(message))?
        .queue(MoveTo(0,0))?;

        self.flush_stdout()

    }


    pub fn flush_stdout(&mut self) -> Result<(), std::io::Error>{
        self.stdout.flush()
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    pub fn clear_screen(&mut self) -> Result<(), std::io::Error> {
        self.stdout
            .queue(cursor::Hide)?
            .queue(Clear(ClearType::All))?
            .queue(cursor::Show)?;

        self.flush_stdout()
    }

    

    pub fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        self.clear_screen()?;
        self.stdout
            .queue(Print("Goodbye.\r\n"))?;
        Ok(())
    }

    pub fn draw_row(&mut self) -> Result<(), std::io::Error> {
        self.stdout
            .queue(cursor::Hide)?
            .queue(MoveTo(0, 0))?;

        for _ in 0..self.row_lim {
            self.stdout
                .queue(Print("~\r"))?
                .queue(MoveDown(1))?;
        }

        self.stdout
            .queue(MoveTo(0, 0))?
            .queue(cursor::Show)?;

        self.flush_stdout()
    }
}
