use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::models::key::{Key, KeyEvent};
use crate::models::mode::{Mode, TextMode};
use crate::models::pane::TextPane;
use crate::models::settings::Settings;

pub struct InsertMode {
    key_buffer: Vec<KeyEvent>,
    number_buffer: String,
    settings: Option<Rc<RefCell<Settings>>>,
}

impl InsertMode {
    pub fn new() -> InsertMode {
        InsertMode {
            key_buffer: Vec::new(),
            number_buffer: String::new(),
            settings: None,
        }
    }

    pub fn execute_command(&mut self, command: &str, pane: &mut dyn TextPane) {
        match command {
            "cancel" => {

                self.key_buffer.clear();

                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow_mut();
                if self.get_name() != settings.editor_settings.default_mode {
                    pane.execute_command("change_mode Normal");
                    pane.execute_command("move right 1");
                }
            }
            "tab" => {
                pane.tab();
            }
            "backspace" => {
                pane.backspace();
            }
            "delete" => {
                pane.delete();
            }
            "newline" => {
                pane.newline();
            }
            "left" => {
                pane.execute_command(&format!("move left 1"));
            },
            "right" => {
                pane.execute_command(&format!("move right 1"));
            },
            "up" => {
                pane.execute_command(&format!("move up 1"));
            },
            "down" => {
                pane.execute_command(&format!("move down 1"));
            },
            "start_of_file" => {
                pane.execute_command("move start_of_file");
            },
            "end_of_file" => {
                pane.execute_command("move end_of_file");
            },
            "page_up" => {
                pane.execute_command(&format!("move page_up 1"));
            },
            "page_down" => {
                pane.execute_command(&format!("move page_down 1"));
            },
            "half_page_up" => {
                pane.execute_command(&format!("move half_page_up 1"));
            },
            "half_page_down" => {
                pane.execute_command(&format!("move half_page_down 1"));
            },
            // These are not bound to any keybindings by default, these are just to allow for custom keybindings.
            // Say you want to have Emacs style keybindings, this allows it.
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
            _ => {

            }
        }
        self.key_buffer.clear();
        self.number_buffer.clear();
    }
}

impl Mode for InsertMode {
    fn get_name(&self) -> String {
        "Insert".to_string()
    }

    fn add_settings(&mut self, settings: Rc<RefCell<Settings>>) {
        self.settings = Some(settings);
    }

    fn refresh(&mut self) {

    }

    fn add_special(&mut self, _something: &dyn Any) {

    }

    fn get_special(&self) -> Option<&dyn Any> {
        None
    }

    fn influence_cursor(&self) -> Option<usize> {
        None
    }
}

impl TextMode for InsertMode {
    fn process_keypress(&mut self, key: KeyEvent, pane: &mut dyn TextPane) {
        match key {
            KeyEvent {
                key: Key::Esc,
                ..
            } => {
                self.key_buffer.clear();
                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow_mut();
                if self.get_name() != settings.editor_settings.default_mode {
                    pane.execute_command("change_mode Normal");
                    pane.execute_command("move right 1");
                }
            }
            key => {
                self.key_buffer.push(key);

                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow_mut();

                let command = if let Some(command) = settings.mode_keybindings.get(&self.get_name(), &self.key_buffer) {
                    let command = command.clone();
                    drop(settings);
                    Some(command)
                } else {
                    drop(settings);
                    match key.key {
                        Key::Char(c) => {
                            let index = pane.get_current_byte_position();
                            pane.insert_char(index, c);

                            self.key_buffer.clear();
                        }
                        _ => {}
                    }
                    None
                };
                match command {
                    Some(command) => {
                        self.execute_command(&command, pane);
                        self.key_buffer.clear();
                    }
                    None => {}
                }

            }
        }
    }

    fn update_status(&self, pane: &dyn TextPane) -> (String, String, String) {
        let (col, row) = pane.get_cursor();

        let first = format!("{}:{}", row + 1, col + 1);

        let mut second = String::new();
        if !self.key_buffer.is_empty() {
            for key in &self.key_buffer {
                second.push_str(&format!("{} ", key));
            }
        }


        (self.get_name(), first, second)
    }

    fn start(&mut self, _pane: &mut dyn TextPane) {
    }
}