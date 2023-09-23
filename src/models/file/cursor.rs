

enum ColMovement {
    Left,
    Right,
    None,
}

enum RowMovement {
    Up,
    Down,
    None,
}




pub struct Cursor {
    col: usize,
    row: usize,
    col_offset: usize,
    row_offset: usize,
    col_movement: ColMovement,
    row_movement: RowMovement,
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
        }
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        (self.col, self.row)
    }

    pub fn get_relative_cursor(&self) -> (usize, usize) {
        (self.col - self.col_offset, self.row - self.row_offset)
    }

}



