use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::models::settings::mode_keybindings::ModeKeybindings;


use crate::models::key::KeyEvent;
use crate::models::key::Key;
use crate::models::key::KeyModifiers;
use crate::models::pane::TextPane;
use crate::models::settings::Settings;

use super::{Mode, ModeObserver, TextMode};



pub struct NormalMode {
    number_buffer: String,
    settings: Option<Rc<RefCell<Settings>>>,
    key_buffer: Vec<KeyEvent>,
}



impl NormalMode {
    pub fn new() -> NormalMode {
        NormalMode {
            number_buffer: String::new(),
            settings: None,
            key_buffer: Vec::new(),
        }
    }

    pub fn execute_command(&mut self, command: &str, pane: &mut dyn TextPane) {
        let mut command_args = command.split_whitespace();
        let command_name = command_args.next().unwrap_or("");


        match command {
            "cancel" => {
                self.number_buffer.clear();
                self.key_buffer.clear();
            },
            "left" => {
                pane.execute_command(&format!("move left {}", self.number_buffer));

                self.number_buffer.clear();
            },
            "right" => {
                pane.execute_command(&format!("move right {}", self.number_buffer));

                self.number_buffer.clear();
            },
            "up" => {
                pane.execute_command(&format!("move up {}", self.number_buffer));

                self.number_buffer.clear();
            },
            "down" => {
                pane.execute_command(&format!("move down {}", self.number_buffer));

                self.number_buffer.clear();
            },
            "start_of_file" => {
                pane.execute_command("move start_of_file");
            },
            "end_of_file" => {
                pane.execute_command("move end_of_file");
            },
            "page_up" => {
                pane.execute_command(&format!("move page_up {}", self.number_buffer));
            },
            "page_down" => {
                pane.execute_command(&format!("move page_down {}", self.number_buffer));
            },
            "half_page_up" => {
                pane.execute_command(&format!("move half_page_up {}", self.number_buffer));
            },
            "half_page_down" => {
                pane.execute_command(&format!("move half_page_down {}", self.number_buffer));
            },
            "start_of_line" => {
                pane.execute_command("move start_of_line");
            },
            "end_of_line" => {
                pane.execute_command("move end_of_line");
            },
            "up_line_start" => {
                pane.execute_command("move up_line_start");
            },
            "down_line_end" => {
                pane.execute_command("move down_line_end");
            },
            "next_word_front" => {
                pane.execute_command("move next_word_front");
            },
            "next_word_back" => {
                pane.execute_command("move next_word_back");
            },
            "previous_word_front" => {
                pane.execute_command("move previous_word_front");
            },
            "previous_word_back" => {
                pane.execute_command("move previous_word_back");
            },
            "insert_before" => {
                pane.execute_command("insert before");
            },
            "insert_after" => {
                pane.execute_command("insert after");
            },
            "insert_start_of_line" => {
                pane.execute_command("insert start_of_line");
            },
            "insert_end_of_line" => {
                pane.execute_command("insert end_of_line");
            },
            "insert_below" => {
                pane.execute_command("insert below");
            },
            "insert_above" => {
                pane.execute_command("insert above");
            },
            "command_mode" => {
                pane.execute_command("command_mode");
            },
            "selection_mode" => {
                pane.execute_command("selection_mode normal");
            },
            "selection_mode_line" => {
                pane.execute_command("selection_mode line");
            },
            "selection_mode_block" => {
                pane.execute_command("selection_mode block");
            },
            "search_mode_down" => {
                pane.execute_command("search_mode down");
            },
            "search_mode_up" => {
                pane.execute_command("search_mode up");
            },
            "replace_mode" => {
                pane.execute_command("replace_mode");
            },
            _ => {},
        }

        self.key_buffer.clear();
        self.number_buffer.clear();

    }
}


impl Mode for NormalMode {
    fn get_name(&self) -> String {
        String::from("Normal")
    }

    fn add_settings(&mut self, settings: Rc<RefCell<Settings>>) {
        self.settings = Some(settings);
    }

    fn refresh(&mut self) {
    }
}


impl TextMode for NormalMode {
    fn process_keypress(&mut self, key: KeyEvent, pane: &mut dyn TextPane) {
        
        match key {
            KeyEvent {
                key: code @ (Key::Char('1') | Key::Char('2') | Key::Char('3') |
                             Key::Char('4') | Key::Char('5') | Key::Char('6') |
                             Key::Char('7') | Key::Char('8') | Key::Char('9') | Key::Char('0')),
                modifiers: KeyModifiers::NONE,
            } => {
                match code {
                    Key::Char(code) => {
                    
                        if let Some(digit) = code.to_digit(10) {
                            if digit == 0 && self.number_buffer.is_empty() {
                                return;
                            }

                            self.number_buffer.push_str(&digit.to_string());
                        }


                    }
                    _ => unreachable!(),

                }

            },
            key => {

                self.key_buffer.push(key);

                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow_mut();
                
                if let Some(command) = settings.mode_keybindings.get(&self.get_name(), &self.key_buffer) {
                    self.execute_command(command, pane);
                }
            }
        }
    }

}




