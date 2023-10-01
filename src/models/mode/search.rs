use std::any::Any;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fmt;
use std::rc::Rc;
use crate::models::key::{Key, KeyEvent};
use crate::models::mode::{Mode, TextMode};
use crate::models::pane::TextPane;
use crate::models::settings::Settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchType {
    Forward,
    Backward,
}

impl fmt::Display for SearchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchType::Forward => write!(f, "/"),
            SearchType::Backward => write!(f, "?"),
        }
    }
}

pub struct SearchMode {
    search_type: SearchType,
    search_string: String,
    edit_pos: usize,
    settings: Option<Rc<RefCell<Settings>>>,
    key_buffer: Vec<KeyEvent>,
    found_pos: BTreeSet<(usize, usize)>,
    number_buffer: String,
}


impl SearchMode {
    pub fn new() -> Self {
        Self {
            search_type: SearchType::Forward,
            search_string: String::new(),
            edit_pos: 0,
            settings: None,
            key_buffer: Vec::new(),
            found_pos: BTreeSet::new(),
            number_buffer: String::new(),
        }
    }

    fn execute_command(&mut self, command: &str, pane: &mut dyn TextPane) {
        let mut command_args = command.split_whitespace();
        let command_name = command_args.next().unwrap_or("");

        match command_name {
            "cancel" => {
                self.search_string.clear();
                self.edit_pos = 0;
                pane.execute_command("change_mode Normal");
                pane.execute_command("clear_selection");
            }
            "left" => {
                //Todo: make sure that we move by the right byte size
                self.edit_pos = self.edit_pos.saturating_sub(1);
            }
            "right" => {
                if self.edit_pos < self.search_string.len() {
                    //Todo: make sure that we move by the right byte size
                    self.edit_pos += 1;
                }
            }
            "up" => {
                self.edit_pos = 0;
            }
            "down" => {
                self.edit_pos = self.search_string.len();
            }
            "backspace" => {
                if self.edit_pos > 0 {
                    self.edit_pos -= 1;
                    self.search_string.remove(self.edit_pos);
                } else if self.search_string.len() == 0 && self.edit_pos == 0 {
                    self.search_string.clear();
                    self.edit_pos = 0;
                    pane.execute_command("change_mode Normal");
                    pane.execute_command("clear_selection");
                }
                self.try_search(pane);
            }
            "delete" => {
                if self.edit_pos < self.search_string.len() {
                    self.search_string.remove(self.edit_pos);
                }
            }
            "copy" => {
                pane.execute_command(&format!("copy selection {}", self.number_buffer));
                pane.execute_command("change_mode Normal");
                pane.execute_command("clear_selection");
                self.search_string.clear();
                self.edit_pos = 0;
            }
            "delete_search" => {
                pane.execute_command("delete selection");
                pane.execute_command("change_mode Normal");
                pane.execute_command("clear_selection");
                self.search_string.clear();
                self.edit_pos = 0;
            }
            "cut" => {
                pane.execute_command(&format!("copy selection"));
                pane.execute_command("delete selection");
                pane.execute_command("change_mode Normal");
                pane.execute_command("clear_selection");
                self.search_string.clear();
                self.edit_pos = 0;
            }
            "paste" => {
                pane.execute_command(&format!("paste selection {}", self.number_buffer));
                pane.execute_command("change_mode Normal");
                pane.execute_command("clear_selection");
                self.search_string.clear();
                self.edit_pos = 0;
            }
            _ => {}
        }
        self.number_buffer.clear();
    }

    fn try_search(&mut self, pane: &mut dyn TextPane) {
        pane.execute_command("clear_selection");
        if self.search_string.len() > 0 {

            let (col, row) = pane.get_cursor();

            let file = pane.borrow_current_file_mut();

            let down = match self.search_type {
                SearchType::Forward => true,
                SearchType::Backward => false,
            };


            self.found_pos = file.find(col, row, &self.search_string, down);

        } else {
            self.found_pos.clear();
        }
    }
}

impl Mode for SearchMode {
    fn get_name(&self) -> String {
        "Search".to_string()
    }

    fn add_settings(&mut self, settings: Rc<RefCell<Settings>>) {
        self.settings = Some(settings);
    }

    fn refresh(&mut self) {

    }

    fn add_special(&mut self, something: &dyn Any) {
        if let Some(search_type) = something.downcast_ref::<SearchType>() {
            self.search_type = *search_type;
        }
    }

    fn get_special(&self) -> Option<&dyn Any> {
        None
    }
}

impl TextMode for SearchMode {
    fn process_keypress(&mut self, key: KeyEvent, pane: &mut dyn TextPane) {
        match key {
            key => {
                self.key_buffer.push(key);

                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow_mut();

                if let Some(command) = settings.mode_keybindings.get(&self.get_name(), &self.key_buffer) {
                    self.execute_command(command, pane);
                    self.key_buffer.clear();
                } else {
                    match key.key {
                        Key::Char(c) => {
                            self.search_string.insert(self.edit_pos, c);
                            //TODO: make sure that we move by the right byte size
                            self.edit_pos += 1;
                            self.key_buffer.clear();
                            self.try_search(pane);
                        }
                        _ => {}
                    }
                }

            }
        }
    }

    fn update_status(&self, _pane: &dyn TextPane) -> (String, String, String) {
        let first = format!("{}{} ", self.search_type ,self.search_string);
        let second = String::new();

        (self.get_name(), first, second)
    }

    fn start(&mut self, _pane: &mut dyn TextPane) {
    }
}