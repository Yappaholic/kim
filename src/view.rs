use crate::buffer::Buffer;
use crate::terminal::{Cursor, TermSize};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, size};
use std::io::{Error, Write, stdout};

#[derive(Default)]
pub struct View {
    pub term_size: TermSize,
    pub cursor: Cursor,
    pub buffer: Buffer,
}

impl View {
    pub fn render(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();

        (self.term_size.cols, self.term_size.rows) = size()?;

        for row in 0..self.term_size.rows {
            queue!(stdout, MoveTo(0, row))?;
            queue!(stdout, Print("~"))?;
        }
        self.zero_cursor()?;
        for entry in self.buffer.text.iter() {
            queue!(stdout, Print(entry))?;
            self.cursor.y += 1;
            queue!(stdout, MoveTo(self.cursor.x, self.cursor.y))?;
        }
        self.zero_cursor()?;
        stdout.flush()?;
        Ok(())
    }

    pub fn clear_screen(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::All))?;
        self.zero_cursor()?;
        stdout.flush()
    }

    pub fn zero_cursor(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, MoveTo(0, 0))?;
        self.cursor = Cursor::default();
        Ok(())
    }
}
