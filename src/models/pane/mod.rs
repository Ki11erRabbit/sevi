use crate::models::file::File;
use crate::models::style::StyledText;
use crate::models::key::KeyEvent;
use crate::models::Rect;

pub mod text;



pub trait Pane {
    fn execute_command(&mut self, command: &str);

    fn get_cursor_position(&self) -> Option<(usize, usize)>;

    fn get_scroll_amount(&self) -> Option<(usize, usize)>;

    fn draw(&self) -> StyledText;

    fn process_keypress(&mut self, key: KeyEvent);

    fn get_status(&self) -> (StyledText, StyledText, StyledText);

    fn refresh(&mut self);

}



pub trait TextPane: Pane {
    fn get_cursor(&self) -> (usize, usize);

    fn change_file(&mut self, file: File) -> File;

    fn can_close(&self) -> bool;

    fn scroll(&mut self, rect: Rect);
}

