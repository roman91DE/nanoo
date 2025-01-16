use std::io::{self, stdout, Stdout};

use crossterm::{cursor::{MoveDown, MoveTo}, execute, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};

pub struct Terminal {
    terminal_size: (u16, u16),
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Result<Self, io::Error> {
        match crossterm::terminal::size() {
            Ok(size) => Ok(Terminal {
                terminal_size: size,
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
        execute!(self.stdout, Clear(ClearType::All))
    }

    pub fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        self.clear_screen()?;
        print!("Goodbye.\r\n");
        Ok(())
    }

    pub fn draw_row(&mut self) -> Result<(), std::io::Error> {
        execute!(self.stdout, MoveTo(0, 0))?;

        for _ in 0..self.terminal_size.1 {
            execute!(self.stdout, MoveDown(1))?;
            print!("~");
        }
        execute!(self.stdout, MoveTo(0, 0))?;
        Ok(())
    }
}