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
    found_pos: BTreeSet<usize>,
    number_buffer: String,
    searched_whole_file: bool,
    moving_cursor: bool,
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
            searched_whole_file: false,
            moving_cursor: false,
        }
    }

    fn get_search_query(&self) -> String {
        // Remove the escape characters
        let mut out = self.search_string.clone();
        out = out.replace("\\n", "\n");
        out = out.replace("\\t", "\t");
        out = out.replace("\\r", "\r");
        out = out.replace("\\\\", "\\");

        out
    }

    fn execute_command(&mut self, command: &str, pane: &mut dyn TextPane) {
        let mut command_args = command.split_whitespace();
        let command_name = command_args.next().unwrap_or("");

        match command_name {
            "cancel" => {
                self.search_string.clear();
                self.edit_pos = 0;
                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
                pane.execute_command("clear_selection");
            }
            "left" => {
                //Todo: make sure that we move by the right byte size
                self.edit_pos = self.edit_pos.saturating_sub(1);
                self.moving_cursor = false;
            }
            "right" => {
                if self.edit_pos < self.search_string.len() {
                    //Todo: make sure that we move by the right byte size
                    self.edit_pos += 1;
                }
                self.moving_cursor = false;
            }
            "up" => {
                self.edit_pos = 0;
                self.moving_cursor = false;
            }
            "down" => {
                self.edit_pos = self.search_string.len();
                self.moving_cursor = false;
            }
            "backspace" => {
                if self.edit_pos > 0 {
                    self.edit_pos -= 1;
                    self.search_string.remove(self.edit_pos);
                } else if self.search_string.len() == 0 && self.edit_pos == 0 {
                    self.search_string.clear();
                    self.edit_pos = 0;
                    let settings = self.settings.clone().unwrap();
                    let mut settings = settings.borrow();
                    pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
                    pane.execute_command("clear_selection");
                }
                self.try_search(pane);
                self.moving_cursor = false;
            }
            "delete" => {
                if self.edit_pos < self.search_string.len() {
                    self.search_string.remove(self.edit_pos);
                }
                self.moving_cursor = false;
            }
            "copy" => {
                pane.execute_command(&format!("copy selection {}", self.number_buffer));
                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
                pane.execute_command("clear_selection");
                self.search_string.clear();
                self.edit_pos = 0;
            }
            "delete_search" => {
                pane.execute_command("delete selection");
                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
                pane.execute_command("clear_selection");
                self.search_string.clear();
                self.edit_pos = 0;
            }
            "cut" => {
                pane.execute_command(&format!("copy selection"));
                pane.execute_command("delete selection");
                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
                pane.execute_command("clear_selection");
                self.search_string.clear();
                self.edit_pos = 0;
            }
            "paste" => {
                pane.execute_command(&format!("paste selection {}", self.number_buffer));
                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
                pane.execute_command("clear_selection");
                self.search_string.clear();
                self.edit_pos = 0;
            }
            "next_match" => {
                self.next_match(pane);
                self.key_buffer.clear();
                self.moving_cursor = true;
            }
            "previous_match" => {
                self.previous_match(pane);
                self.key_buffer.clear();
                self.moving_cursor = true;
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

            let search_string = self.get_search_query();

            self.found_pos = file.find(col, row, &search_string, down);

        } else {
            self.found_pos.clear();
        }
    }

    fn search_rest(&mut self, pane: &mut dyn TextPane) -> bool {
        if self.search_string.len() > 0 {

            let (col, row) = pane.get_cursor();

            let file = pane.borrow_current_file_mut();

            let down = match self.search_type {
                SearchType::Forward => false,
                SearchType::Backward => true,
            };

            let search_string = self.get_search_query();

            let found = file.find(col, row, &search_string, down);

            self.found_pos.extend(found);

        } else {
            self.found_pos.clear();
        }
        let out = self.searched_whole_file;

        self.searched_whole_file = true;

        out
    }

    fn next_match(&mut self, pane: &mut dyn TextPane) {
        if self.found_pos.len() > 0 {

            let mut iter = self.found_pos.iter();


            let mut counter = 0;
            while let Some(byte) = iter.next() {
                eprintln!("{} {}", *byte, pane.get_current_byte_position());

                if *byte < pane.get_current_byte_position() {
                    continue
                } else if *byte >= pane.get_current_byte_position() && *byte - counter == pane.get_current_byte_position() {
                    counter += 1;
                } else {
                    pane.execute_command(&format!("move to_byte {}", *byte));
                    return;
                }
            }
            self.search_rest(pane);

            let mut iter = self.found_pos.iter();
            let byte = iter.next().unwrap();
            pane.execute_command(&format!("move to_byte {}", *byte));

        }
    }

    fn previous_match(&mut self, pane: &mut dyn TextPane) {
        if self.found_pos.len() > 0 {

            loop {
                let mut iter = self.found_pos.iter().rev();

                let mut counter = 0;
                while let Some(byte) = iter.next() {
                    eprintln!("{} {}", *byte, pane.get_current_byte_position());

                    if *byte > pane.get_current_byte_position() {
                        continue
                    } else if *byte <= pane.get_current_byte_position() && *byte + counter == pane.get_current_byte_position() {
                        counter += 1;
                    } else {
                        pane.execute_command(&format!("move to_byte {}", *byte));
                        return;
                    }
                }

                if self.search_rest(pane) {
                    let mut iter = self.found_pos.iter().rev();
                    let byte = iter.next().unwrap();
                    pane.execute_command(&format!("move to_byte {}", *byte));
                    return;
                }
            }


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

    fn influence_cursor(&self) -> Option<usize> {
        if self.moving_cursor {
            return None;
        }
        let offset = self.get_name().chars().count() + 2 + self.edit_pos;
        Some(offset)
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
                    let command = command.clone();
                    drop(settings);
                    self.execute_command(&command, pane);
                    self.key_buffer.clear();
                } else {
                    self.moving_cursor = false;
                    drop(settings);
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
        self.moving_cursor = false;
        self.searched_whole_file = false;
        self.found_pos.clear();
    }
}