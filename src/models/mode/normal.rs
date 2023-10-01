use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;


use crate::models::key::KeyEvent;
use crate::models::key::Key;
use crate::models::key::KeyModifiers;
use crate::models::pane::TextPane;
use crate::models::settings::Settings;

use super::{Mode, TextMode};



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

        match command_name {
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
                pane.execute_command("change_mode insert_before");
            },
            "insert_after" => {
                pane.execute_command("change_mode insert_after");
            },
            "insert_start_of_line" => {
                pane.execute_command("change_mode insert_start_of_line");
            },
            "insert_end_of_line" => {
                pane.execute_command("change_mode insert_end_of_line");
            },
            "insert_below" => {
                pane.execute_command("change_mode insert_below");
            },
            "insert_above" => {
                pane.execute_command("change_mode insert_above");
            },
            "command_mode" => {
                pane.execute_command("change_mode Command");
            },
            "selection_mode" => {
                pane.execute_command("change_mode selection_normal");
                return;
            },
            "selection_mode_line" => {
                pane.execute_command("change_mode selection_line");
                return;
            },
            "selection_mode_block" => {
                pane.execute_command("change_mode selection_block");
                return;
            },
            "search_mode_down" => {
                pane.execute_command("change_mode search_down");
                return;
            },
            "search_mode_up" => {
                pane.execute_command("change_mode search_up");
                return;
            },
            "replace_mode" => {
                pane.execute_command("change_mode replace");
            },
            "goto_line" => {
                if !self.number_buffer.is_empty() {
                    pane.execute_command(&format!("goto_line {}", self.number_buffer));
                    self.number_buffer.clear();
                } else {
                    return;
                }
            }
            "copy_char" => {
                pane.execute_command(&format!("copy char {}", self.number_buffer));
            }
            "copy_line" => {
                pane.execute_command(&format!("copy line {}", self.number_buffer));
            }
            "copy_word" => {
                pane.execute_command(&format!("copy word {}", self.number_buffer));
            }
            "copy_to_next_word" => {
                pane.execute_command(&format!("copy to_next_word {}", self.number_buffer));
            }
            "copy_to_prev_word" => {
                pane.execute_command(&format!("copy to_prev_word {}", self.number_buffer));
            }
            "copy_to_end_line" => {
                pane.execute_command(&format!("copy to_end_line {}", self.number_buffer));
            }
            "copy_to_start_line" => {
                pane.execute_command(&format!("copy to_start_line {}", self.number_buffer));
            }
            "delete_char" => {
                pane.execute_command("delete char");
            }
            "delete_line" => {
                pane.execute_command("delete line");
            }
            "delete_word" => {
                pane.execute_command("delete word");
            }
            "delete_to_next_word" => {
                pane.execute_command("delete to_next_word");
            }
            "delete_to_prev_word" => {
                pane.execute_command("delete to_prev_word");
            }
            "delete_to_end_line" => {
                pane.execute_command("delete to_end_line");
            }
            "delete_to_start_line" => {
                pane.execute_command("delete to_start_line");
            }
            "cut_char" => {
                pane.execute_command(&format!("copy char {}", self.number_buffer));
                pane.execute_command("delete char");
            }
            "cut_line" => {
                pane.execute_command(&format!("copy line {}", self.number_buffer));
                pane.execute_command("delete line");
            }
            "cut_word" => {
                pane.execute_command(&format!("copy word {}", self.number_buffer));
                pane.execute_command("delete word");
            }
            "cut_to_next_word" => {
                pane.execute_command(&format!("copy to_next_word {}", self.number_buffer));
                pane.execute_command("delete to_next_word");
            }
            "cut_to_prev_word" => {
                pane.execute_command(&format!("copy to_prev_word {}", self.number_buffer));
                pane.execute_command("delete to_prev_word");
            }
            "cut_to_end_line" => {
                pane.execute_command(&format!("copy to_end_line {}", self.number_buffer));
                pane.execute_command("delete to_end_line");
            }
            "cut_to_start_line" => {
                pane.execute_command(&format!("copy to_start_line {}", self.number_buffer));
                pane.execute_command("delete to_start_line");
            }
            "paste_before" => {
                pane.execute_command(&format!("paste before {}", self.number_buffer));
            }
            "paste_after" => {
                pane.execute_command(&format!("paste after {}", self.number_buffer));
            }
            "undo" => {
                pane.execute_command("undo");
            }
            "redo" => {
                pane.execute_command("redo");
            }
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

    fn add_special(&mut self, _something: &dyn Any) {

    }

    fn get_special(&self) -> Option<&dyn Any> {
        Some(&self.number_buffer)
    }

    fn influence_cursor(&self) -> Option<usize> {
        None
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
                                self.key_buffer.push(KeyEvent {
                                    key: Key::Char('0'),
                                    modifiers: KeyModifiers::NONE,
                                });
                                let settings = self.settings.clone().unwrap();
                                let mut settings = settings.borrow_mut();
                                if let Some(command) = settings.mode_keybindings.get(&self.get_name(), &self.key_buffer) {
                                    self.execute_command(command, pane);
                                }
                                return;
                            }

                            self.number_buffer.push_str(&digit.to_string());
                        }


                    }
                    _ => unreachable!(),

                }

            },
            KeyEvent {
                key: Key::Esc,
                ..
            } => {
                self.key_buffer.clear();
                self.number_buffer.clear();
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

    fn update_status(&self, pane: &dyn TextPane) -> (String, String, String) {
        let (col, row) = pane.get_cursor();


        let mut first = format!("{}:{}", row + 1, col + 1);

        if !self.number_buffer.is_empty() {
            first.push_str(&format!(" {}", self.number_buffer));
        }

        let mut second = String::new();
        if !self.key_buffer.is_empty() {
            for key in &self.key_buffer {
                second.push_str(&format!("{} ", key));
            }
        }

        let name = self.get_name();



        (name, first, second)
    }

    fn start(&mut self, _pane: &mut dyn TextPane) {
        self.key_buffer.clear();
        self.number_buffer.clear();
    }
}




