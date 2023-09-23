use crate::models::style::StyledText;


pub mod text;



pub trait Pane {
    fn execute_command(&mut self, command: &str);

    fn get_cursor_position(&self) -> Option<(usize, usize)>;

    fn draw(&self) -> StyledText;
}



pub trait TextPane: Pane {

    
}


