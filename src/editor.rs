use std::io;

use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;
use ratatui::DefaultTerminal;
use ratatui::Frame;

use crate::file;
use crate::input;

pub struct Editor {
    buffer: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
    running: bool,
    filename: Option<String>, // Optional filename for saving/loading
}

impl Editor {
    pub fn new(filename: Option<String>) -> Self {
        let buffer = if let Some(ref file) = filename {
            file::load_file(file).unwrap_or_else(|_| vec![String::new()])
        } else {
            vec![String::new()] // Start with a new empty buffer
        };

        Self {
            buffer,
            cursor_row: 0,
            cursor_col: 0,
            running: true,
            filename,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            if let Some(command) = input::handle_input() {
                self.handle_command(command);
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_command(&mut self, command: input::Command) {
        match command {
            input::Command::Quit => self.running = false,
            input::Command::InsertChar(c) => self.insert_char(c),
            input::Command::MoveCursorUp => self.move_cursor_up(),
            input::Command::MoveCursorDown => self.move_cursor_down(),
            input::Command::MoveCursorLeft => self.move_cursor_left(),
            input::Command::MoveCursorRight => self.move_cursor_right(),
            input::Command::Save => {
                self.save_file().expect("Error during File saving");
            }
        }
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.cursor_col = self.cursor_col.min(self.buffer[self.cursor_row].len());
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cursor_row + 1 < self.buffer.len() {
            self.cursor_row += 1;
            self.cursor_col = self.cursor_col.min(self.buffer[self.cursor_row].len());
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.cursor_col = self.buffer[self.cursor_row].len();
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_col < self.buffer[self.cursor_row].len() {
            self.cursor_col += 1;
        } else if self.cursor_row + 1 < self.buffer.len() {
            self.cursor_row += 1;
            self.cursor_col = 0;
        }
    }

    fn insert_char(&mut self, c: char) {
        if self.buffer.is_empty() {
            self.buffer.push(String::new());
        }
        self.buffer[self.cursor_row].insert(self.cursor_col, c);
        self.cursor_col += 1;
    }

    fn save_file(&self) -> Result<(), std::io::Error> {
        match &self.filename {
            Some(name) => file::save_file(name, &self.buffer),
            None => file::save_file("new.txt", &self.buffer),
        }
    }
}

impl Widget for &Editor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(ratatui::style::Stylize::bold("Nanoo"));
        let instructions = Line::from(vec![" Quit ".into(), "<Ctrl> + <Q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        Paragraph::new(self.buffer.concat())
            .centered()
            .block(block)
            .render(area, buf);
    }
}
