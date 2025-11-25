#![warn(clippy::pedantic, clippy::all)]
use super::{terminal::*, view::*};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use std::io::{Error, Write, stdout};

#[derive(Default)]
pub struct Editor {
    pub should_quit: bool,
    view: View,
}

impl Editor {
    /// # Panics
    /// On init and deinit
    pub fn run(&mut self, path: Option<&str>) -> Result<(), Error> {
        self.view.buffer.load(path)?;
        self.initialize()?;
        self.repl()?;
        self.terminate()
    }

    fn initialize(&mut self) -> Result<(), Error> {
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
        disable_raw_mode()?;
        self.view.clear_screen()
    }

    fn move_direction(&mut self, direction: MoveDirection) -> Result<(), Error> {
        let cursor = &self.view.cursor;
        match direction {
            MoveDirection::Left => {
                if cursor.x > 0 {
                    self.view.cursor.x -= 1
                }
            }
            MoveDirection::Right => {
                if cursor.x < self.view.term_size.cols - 1 {
                    self.view.cursor.x += 1
                }
            }
            MoveDirection::Up => {
                if cursor.y > 0 {
                    self.view.cursor.y -= 1
                }
            }
            MoveDirection::Down => {
                if cursor.y < self.view.term_size.rows - 1 {
                    self.view.cursor.y += 1
                }
            }
            MoveDirection::LineEnd => self.view.cursor.x = self.view.term_size.cols - 1,
            MoveDirection::LineStart => self.view.cursor.x = 0,
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
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                Char('h') => self.move_direction(MoveDirection::Left).unwrap(),
                Char('l') => self.move_direction(MoveDirection::Right).unwrap(),
                Char('k') => self.move_direction(MoveDirection::Up).unwrap(),
                Char('j') => self.move_direction(MoveDirection::Down).unwrap(),
                Char('$') => self.move_direction(MoveDirection::LineEnd).unwrap(),
                Char('0') => self.move_direction(MoveDirection::LineStart).unwrap(),
                Char('G') => self.move_direction(MoveDirection::FileEnd).unwrap(),
                Char('K') => self.move_direction(MoveDirection::FileStart).unwrap(),
                _ => {}
            }
        }
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
