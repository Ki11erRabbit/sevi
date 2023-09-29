
use crate::models::Rect;
use crate::models::file::File;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ColMovement {
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RowMovement {
    Up,
    Down,
    None,
}

pub enum CursorMovement {
    Up,
    Down,
    Left,
    Right,
    LineStart,
    LineEnd,
    FileStart,
    FileEnd,
    PageUp,
    PageDown,
    HalfPageUp,
    HalfPageDown,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    col: usize,
    row: usize,
    col_offset: usize,
    row_offset: usize,
    col_movement: ColMovement,
    row_movement: RowMovement,
    number_line_width: usize,
    gutter_width: usize,
}


impl Cursor {
    pub const fn new() -> Self {
        Self {
            col: 0,
            row: 0,
            col_offset: 0,
            row_offset: 0,
            col_movement: ColMovement::None,
            row_movement: RowMovement::None,
            number_line_width: 0,
            gutter_width: 0,
        }
    }

    pub fn set_number_line_width(&mut self, width: usize) {
        self.number_line_width = width;
    }

    pub fn set_gutter_width(&mut self, width: usize) {
        self.gutter_width = width;
    }

    pub fn get_cursor(self) -> (usize, usize) {
        (self.col, self.row)
    }

    pub fn get_relative_cursor(self) -> (usize, usize) {
        (self.col - self.col_offset + self.number_line_width + self.gutter_width, self.row - self.row_offset)
    }

    pub fn get_scroll_amount(self) -> (usize, usize) {
        (self.col_offset, self.row_offset)
    }

    pub fn scroll(&mut self, rect: Rect) {

        match self.col_movement {
            ColMovement::Right if rect.width != 0 && ((self.col) - self.col_offset) >= rect.width => {
                self.col_offset = (self.col).saturating_sub(rect.width) + 1;
            }
            ColMovement::Left if (self.col.saturating_sub(self.col_offset)) == 0 => {
                self.col_offset = self.col;
            }
            _ => {}
        }

        match self.row_movement {
            RowMovement::Down if rect.height != 0 && ((self.row) - self.row_offset) >= rect.height => {
                self.row_offset = (self.row).saturating_sub(rect.height) + 1;
            }
            RowMovement::Up if (self.row.saturating_sub(self.row_offset)) == 0 => {
                self.row_offset = self.row;
            }
            _ => {}
        }
    }

    pub fn set_cursor(&mut self, col: usize, row: usize) {
        self.col = col;
        self.row = row;
    }


    pub fn move_cursor(&mut self, direction: CursorMovement, mut n: usize, file: &File) {
        let number_of_lines = file.get_line_count();

        let number_of_cols = if let Some(cols) = file.get_row_len(self.row) {
            cols
        } else {
            0
        };

        match direction {
            CursorMovement::Up => {
                self.row_movement = RowMovement::Up;
                self.row = self.row.saturating_sub(n);
            }
            CursorMovement::Down => {
                if self.row < number_of_lines {
                    let new_row =(self.row + n) % number_of_lines;
                    if new_row < self.row {
                        self.row = number_of_lines.saturating_sub(1)
                    } else {
                        self.row = new_row;
                    }
                }
                self.row_movement = RowMovement::Down;

            }
            CursorMovement::Left => {
                self.col_movement = ColMovement::Left;
                self.col = self.col.saturating_sub(n);
            }
            CursorMovement::Right => {
                self.col_movement = ColMovement::Right;
                if self.col < number_of_cols {
                    let new_col = (self.col + n) % (number_of_cols + 1);
                    if new_col < self.col {
                        self.col = number_of_cols;
                    } else {
                        self.col = new_col;
                    }
                } else {
                    self.col = number_of_cols;
                }
            }
            CursorMovement::LineStart => {
                self.col_movement = ColMovement::Left;
                self.col = 0;
            }
            CursorMovement::LineEnd => {
                self.col_movement = ColMovement::Right;
                self.col = number_of_cols;
            }
            CursorMovement::FileStart => {
                self.row_movement = RowMovement::Up;
                self.row = 0;
            }
            CursorMovement::FileEnd => {
                self.row_movement = RowMovement::Down;
                self.row = number_of_lines - 2;

                let number_of_cols = if let Some(cols) = file.get_row_len(self.row) {
                    cols
                } else {
                    0
                };

                self.col = number_of_cols;
            }
            /*CursorMovement::PageUp => {
                self.row_movement = RowMovement::Up;
                if self.row > 0 {
                    self.row = self.row.saturating_sub(n);
                }
            }
            CursorMovement::PageDown => {
                self.row_movement = RowMovement::Down;
                if self.row < number_of_lines - 1 {
                    self.row = self.row.saturating_add(n);
                }
            }
            CursorMovement::HalfPageUp => {
                self.row_movement = RowMovement::Up;
                if self.row > 0 {
                    self.row = self.row.saturating_sub(n / 2);
                }
            }
            CursorMovement::HalfPageDown => {
                self.row_movement = RowMovement::Down;
                if self.row < number_of_lines - 1 {
                    self.row = self.row.saturating_add(n / 2);
                }
            }*/
            _ => {}
        }


    }

}



