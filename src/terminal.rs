use std::io::{self, stdout, Stdout, Write};

use crossterm::{
    cursor::{self, MoveDown, MoveTo},
    execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    QueueableCommand,
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
        execute!(self.stdout, MoveTo(0, 0))?;
        Ok(())
    }
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    pub fn clear_screen(&mut self) -> Result<(), std::io::Error> {
        self.stdout
            .queue(cursor::Hide)?
            .queue(Clear(ClearType::All))?
            .queue(cursor::Show)?;

        self.stdout
            .flush()
    }

    pub fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        self.clear_screen()?;
        print!("Goodbye.\r\n");
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
            .queue(cursor::Show)?
            .flush()
    }
}
