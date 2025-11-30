#![warn(clippy::pedantic, clippy::all)]
#[derive(Default)]
pub struct TermSize {
    pub cols: u16,
    pub rows: u16,
}

#[derive(Clone, Copy)]
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
    pub x: u16,
    pub y: u16,
}
