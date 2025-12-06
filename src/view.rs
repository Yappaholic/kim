use crate::buffer::Buffer;
use crate::terminal::{Cursor, TermSize};
use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, size};
use std::io::{Error, Write, stdout};

#[derive(Default)]
pub struct Offset {
    pub x: u16,
    pub y: u16,
}

#[derive(Default)]
pub struct View {
    pub term_size: TermSize,
    pub cursor: Cursor,
    pub buffer: Buffer,
    pub offset: Offset,
    pub needs_redraw: bool,
}

impl View {
    pub fn render(&mut self) -> Result<(), Error> {
        if self.needs_redraw == false {
            return Ok(());
        }
        let prev_cursor: Cursor = self.cursor.clone();
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::All))?;
        stdout.flush()?;

        (self.term_size.cols, self.term_size.rows) = size()?;

        for row in 0..self.term_size.rows {
            queue!(stdout, MoveTo(0, row))?;
            queue!(stdout, Print("~"))?;
        }
        self.zero_cursor()?;
        let range = self.buffer.text[self.offset.y as usize..self.buffer.text.len()]
            .iter()
            .enumerate();
        for (idx, entry) in range {
            let idx = idx as u16;
            if idx + 1 < self.term_size.rows {
                if entry.is_empty() {
                    queue!(stdout, Clear(ClearType::CurrentLine))?;
                    queue!(stdout, Print("\n"))?;
                    self.cursor.y += 1;
                    queue!(stdout, MoveTo(self.cursor.x, self.cursor.y))?;
                    continue;
                } else {
                    let line = entry.get(self.offset.x as usize..self.term_size.cols as usize);
                    if let Some(line) = line {
                        queue!(stdout, Print(line))?;
                    } else {
                        queue!(stdout, Print(entry))?;
                    }
                }
                self.cursor.y += 1;
                queue!(stdout, MoveTo(self.cursor.x, self.cursor.y))?;
            }
        }
        self.cursor = prev_cursor;
        queue!(stdout, MoveTo(self.cursor.x, self.cursor.y))?;
        //self.zero_cursor()?;
        stdout.flush()?;
        self.needs_redraw = false;
        Ok(())
    }

    pub fn clear_screen(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::All))?;
        self.zero_cursor()?;
        stdout.flush()
    }

    /// Move cursor to 0,0
    pub fn zero_cursor(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, MoveTo(0, 0))?;
        self.cursor = Cursor::default();
        Ok(())
    }
}
