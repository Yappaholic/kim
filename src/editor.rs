#![warn(clippy::pedantic, clippy::all)]
use super::{terminal::*, view::*};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{
    Event, Event::Key, Event::Resize, KeyCode::Char, KeyEvent, KeyModifiers, read,
};
use crossterm::style::Print;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{execute, queue};
use std::io::{Error, Write, stdout};

#[derive(Default)]
pub struct Editor {
    pub should_quit: bool,
    view: View,
}

impl Editor {
    /// # Panics
    /// On init and deinit
    pub fn run(&mut self, path: Option<String>) -> Result<(), Error> {
        self.view.needs_redraw = true;
        self.view.buffer.load(path)?;
        self.initialize()?;
        self.repl()?;
        self.terminate()
    }

    fn initialize(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, EnterAlternateScreen)?;
        enable_raw_mode()?;
        self.view.clear_screen()?;
        self.view.render()?;

        if self.view.buffer.is_empty() {
            self.welcome()
        } else {
            Ok(())
        }
    }

    fn terminate(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        disable_raw_mode()?;
        self.view.clear_screen()?;
        execute!(stdout, LeaveAlternateScreen)
    }

    fn move_direction(&mut self, direction: MoveDirection) -> Result<(), Error> {
        let cursor = &mut self.view.cursor;
        if self.view.buffer.text.is_empty() {
            return Ok(());
        }
        match direction {
            MoveDirection::Left => {
                if cursor.x == 0 && self.view.offset.x > 0 {
                    self.view.offset.x -= 1;
                    self.view.needs_redraw = true;
                }
                if cursor.x > 0 {
                    cursor.x -= 1;
                }
            }
            MoveDirection::Right => {
                if cursor.x == self.view.term_size.cols - 1 {
                    self.view.offset.x += 1;
                    self.view.needs_redraw = true;
                } else {
                    cursor.x += 1;
                }
            }
            MoveDirection::Up => {
                if cursor.y == 0 && self.view.offset.y > 0 {
                    self.view.offset.y -= 1;
                    self.view.needs_redraw = true;
                }
                if cursor.y > 0 {
                    cursor.y -= 1;
                }
            }
            MoveDirection::Down => {
                let next_offset = self.view.offset.y + 1;
                let buffer_len = self.view.buffer.text.len();
                if cursor.y != self.view.term_size.rows - 1 {
                    cursor.y += 1;
                }

                if self.view.term_size.rows - 2
                    == self.view.buffer.text[next_offset as usize..buffer_len].len() as u16
                {
                    self.view.needs_redraw = false;
                } else if cursor.y == self.view.term_size.rows - 1
                    && next_offset < buffer_len as u16
                {
                    self.view.offset.y += 1;
                    self.view.needs_redraw = true;
                }
            }
            MoveDirection::LineEnd => cursor.x = self.view.term_size.cols - 1,
            MoveDirection::LineStart => cursor.x = 0,
            MoveDirection::FileStart => self.view.cursor = Cursor { x: cursor.x, y: 0 },
            MoveDirection::FileEnd => {
                self.view.cursor = Cursor {
                    x: cursor.x,
                    y: self.view.term_size.rows - 1,
                }
            }
        }
        self.move_cursor()
    }

    fn move_cursor(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, MoveTo(self.view.cursor.x, self.view.cursor.y))?;
        stdout.flush()
    }

    fn welcome(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        let name = "KIM - Kakoune Impoved";
        let ver = "Version 0.1";
        let x = self.view.term_size.cols / 2 - (name.len() / 2) as u16;
        let y = self.view.term_size.rows / 3;
        let ver_x = x + ver.len() as u16 / 2;
        queue!(stdout, MoveTo(x, y))?;
        queue!(stdout, Print(name))?;
        queue!(stdout, MoveTo(ver_x, y + 1))?;
        queue!(stdout, Print(ver))?;
        self.view.zero_cursor()?;
        stdout.flush()
    }

    /// # Errors
    /// Only when working with raw mode
    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            if self.should_quit {
                break;
            }
            if self.view.needs_redraw {
                self.view.render()?;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
            self.refresh_screen()?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Resize(cols, rows) = event {
            self.view.term_size.cols = *cols;
            self.view.term_size.rows = *rows;
            self.view.needs_redraw = true;
        }
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                Char('h') => self.move_direction(MoveDirection::Left)?,
                Char('l') => self.move_direction(MoveDirection::Right)?,
                Char('k') => self.move_direction(MoveDirection::Up)?,
                Char('j') => self.move_direction(MoveDirection::Down)?,
                Char('$') => self.move_direction(MoveDirection::LineEnd)?,
                Char('0') => self.move_direction(MoveDirection::LineStart)?,
                Char('G') => self.move_direction(MoveDirection::FileEnd)?,
                Char('K') => self.move_direction(MoveDirection::FileStart)?,
                _ => {}
            }
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Hide)?;
        if self.should_quit {
            self.view.clear_screen()?;
            queue!(stdout, Print("Goodbye!\r\n"))?;
        }
        queue!(stdout, Show)?;
        stdout.flush()?;
        Ok(())
    }
}
