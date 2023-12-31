use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::models::key::KeyEvent;
use crate::models::pane::TextPane;
use crate::models::settings::Settings;


pub mod normal;
pub mod command;
pub mod insert;
pub mod selection;
pub mod search;
pub mod mirror;
pub mod pair;


pub trait Mode {
    fn get_name(&self) -> String;

    fn add_settings(&mut self, settings: Rc<RefCell<Settings>>);

    fn refresh(&mut self);

    fn add_special(&mut self, something: &dyn Any);

    fn get_special(&self) -> Option<&dyn Any>;

    fn influence_cursor(&self) -> Option<usize>;


}


pub trait TextMode: Mode {
    fn process_keypress(&mut self, key: KeyEvent, pane: &mut dyn TextPane);

    fn update_status(&self, pane: &dyn TextPane) -> (String, String, String);

    /// This is called when the mode is started
    fn start(&mut self, pane: &mut dyn TextPane);
}

