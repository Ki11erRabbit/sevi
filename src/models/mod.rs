use std::cmp;
use crate::models::style::StyledText;

pub mod key;

pub mod style;
pub mod settings;

pub mod file;

pub mod pane;
pub mod mode;
pub mod cursor;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Id {
    Pane(usize),
    Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message<'a> {
    AppClose,
    Redraw,
    OpenFile(String),
    MoveCursor(Option<(u16, u16)>),
    Scroll(Option<(u16, u16)>),
    Key(key::KeyEvent),
    Lines(StyledText<'a>),
    Status(StyledText<'a>, StyledText<'a>, StyledText<'a>),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub const fn area(self) -> usize {
        self.width * self.height
    }

    pub const fn left(self) -> usize {
        self.x
    }

    pub const fn right(self) -> usize {
        self.x + self.width
    }

    pub const fn top(self) -> usize {
        self.y
    }

    pub const fn bottom(self) -> usize {
        self.y + self.height
    }

    pub fn union(self, other: Self) -> Self {
        let x1 = cmp::min(self.x, other.x);
        let y1 = cmp::min(self.y, other.y);
        let x2 = cmp::max(self.x + self.width, other.x + other.width);
        let y2 = cmp::max(self.y + self.height, other.y + other.height);

        Self {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    pub fn intersection(self, other: Rect) -> Rect {
        let x1 = cmp::max(self.x, other.x);
        let y1 = cmp::max(self.y, other.y);
        let x2 = cmp::min(self.x + self.width, other.x + other.width);
        let y2 = cmp::min(self.y + self.height, other.y + other.height);

        Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    pub const fn insersects(self, other: Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

impl Into<tuirealm::tui::prelude::Rect> for Rect {
    fn into(self) -> tuirealm::tui::prelude::Rect {
        tuirealm::tui::prelude::Rect::new(self.x as u16, self.y as u16, self.width as u16, self.height as u16)
    }
}

impl From<tuirealm::tui::prelude::Rect> for Rect {
    fn from(rect: tuirealm::tui::prelude::Rect) -> Self {
        Self {
            x: rect.x as usize,
            y: rect.y as usize,
            width: rect.width as usize,
            height: rect.height as usize,
        }
    }
}

