
use std::cell::RefCell;
use std::rc::Rc;
use crate::models::key::KeyEvent;
use crate::models::pane::TextPane;
use crate::models::settings::Settings;
use crate::models::style::StyledText;


pub mod normal;
pub mod command;
pub mod insert;

pub trait ModeObserver {
    fn run_command(&mut self, command: &str);

    fn change_mode(&mut self, mode: &str);

    fn update_status(&mut self, status: (String, String, String));
}



pub trait Mode {
    fn get_name(&self) -> String;

    fn add_settings(&mut self, settings: Rc<RefCell<Settings>>);

    fn refresh(&mut self);

}


pub trait TextMode {
    fn process_keypress(&mut self, key: KeyEvent, pane: &mut dyn TextPane);

    fn update_status(&self, pane: &dyn TextPane) -> (String, String, String);
}

