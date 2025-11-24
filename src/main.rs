#![warn(clippy::pedantic, clippy::all)]
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use std::io::{Error, Write, stdout};

#[derive(Default)]
pub struct TermSize {
    cols: u16,
    rows: u16,
}

pub enum MoveDirection {
    Left,
    Right,
    Down,
    Up,
    LineStart,
    LineEnd,
    FileEnd,
    FileStart,
}

#[derive(Default)]
pub struct Cursor {
    x: u16,
    y: u16,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    term_size: TermSize,
    cursor: Cursor,
}

impl Editor {
    /// # Panics
    /// On init and deinit
    pub fn run(&mut self) {
        self.initialize().unwrap();
        let result = self.repl();
        self.terminate().unwrap();
        result.unwrap();
    }

    fn initialize(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        self.clear_screen()?;
        self.draw_rows()?;
        self.welcome()
    }

    fn terminate(&mut self) -> Result<(), Error> {
        disable_raw_mode()?;
        self.clear_screen()
    }

    fn clear_screen(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::All))?;
        self.zero_cursor()?;
        stdout.flush()
    }

    fn zero_cursor(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, MoveTo(0, 0))?;
        self.cursor = Cursor::default();
        Ok(())
    }

    fn move_direction(&mut self, direction: MoveDirection) -> Result<(), Error> {
        match direction {
            MoveDirection::Left => {
                if self.cursor.x > 0 {
                    self.cursor.x -= 1
                }
            }
            MoveDirection::Right => {
                if self.cursor.x < self.term_size.cols - 1 {
                    self.cursor.x += 1
                }
            }
            MoveDirection::Up => {
                if self.cursor.y > 0 {
                    self.cursor.y -= 1
                }
            }
            MoveDirection::Down => {
                if self.cursor.y < self.term_size.rows - 1 {
                    self.cursor.y += 1
                }
            }
            MoveDirection::LineEnd => self.cursor.x = self.term_size.cols - 1,
            MoveDirection::LineStart => self.cursor.x = 0,
            MoveDirection::FileStart => {
                self.cursor = Cursor {
                    x: self.cursor.x,
                    y: 0,
                }
            }
            MoveDirection::FileEnd => {
                self.cursor = Cursor {
                    x: self.cursor.x,
                    y: self.term_size.rows - 1,
                }
            }
        }
        self.move_cursor()
    }

    fn move_cursor(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, MoveTo(self.cursor.x, self.cursor.y))?;
        stdout.flush()
    }

    fn welcome(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();
        let name = "KIM - Kakoune Impoved";
        let ver = "Version 0.1";
        let x = self.term_size.cols / 2 - (name.len() / 2) as u16;
        let y = self.term_size.rows / 3;
        let ver_x = x + ver.len() as u16 / 2;
        queue!(stdout, MoveTo(x, y))?;
        queue!(stdout, Print(name))?;
        queue!(stdout, MoveTo(ver_x, y + 1))?;
        queue!(stdout, Print(ver))?;
        self.zero_cursor()?;
        stdout.flush()
    }

    fn draw_rows(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();

        (self.term_size.cols, self.term_size.rows) = size()?;

        for row in 0..self.term_size.rows {
            queue!(stdout, MoveTo(0, row))?;
            queue!(stdout, Print("~"))?;
        }
        queue!(stdout, MoveTo(0, 0))?;
        stdout.flush()?;
        Ok(())
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
            self.clear_screen()?;
            queue!(stdout, Print("Goodbye!\r\n"))?;
        }
        queue!(stdout, Show)?;
        stdout.flush()?;
        Ok(())
    }
}

fn main() {
    Editor::default().run();
}
