
use std::collections::HashMap;
use crate::models::key::KeyEvent;
use crate::models::pane::TextPane;


pub mod normal;

pub trait ModeObserver {
    fn run_command(&mut self, command: &str);

    fn change_mode(&mut self, mode: &str);

    fn update_status(&mut self, status: (String, String, String));
}



pub trait Mode {
    fn get_name(&self) -> String;

    fn add_keybindings(&mut self, bindings: HashMap<Vec<KeyEvent>, String>);

    fn refresh(&mut self);

}


pub trait TextMode {
    fn process_keypress(&mut self, key: KeyEvent, pane: &mut dyn TextPane);
}

