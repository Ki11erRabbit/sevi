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
}

impl TextMode for InsertMode {
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
                            let index = pane.get_current_byte_position();
                            pane.insert_str_after(index, &c.to_string());

                            self.key_buffer.clear();
                        }
                        _ => {}
                    }
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
}