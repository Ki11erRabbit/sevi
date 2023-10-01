use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::models::key::{Key, KeyEvent};
use crate::models::mode::{Mode, TextMode};
use crate::models::pane::TextPane;
use crate::models::settings::Settings;

pub struct InsertMode {
    key_buffer: Vec<KeyEvent>,
    settings: Option<Rc<RefCell<Settings>>>,
}

impl InsertMode {
    pub fn new() -> InsertMode {
        InsertMode {
            key_buffer: Vec::new(),
            settings: None,
        }
    }

    pub fn execute_command(&mut self, command: &str, pane: &mut dyn TextPane) {
        match command {
            "cancel" => {

                self.key_buffer.clear();
                pane.execute_command("change_mode Normal");
                pane.execute_command("move right 1");

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
            _ => {
                self.key_buffer.clear();
            }
        }
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
                pane.execute_command("change_mode Normal");
                pane.execute_command("move right 1");
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