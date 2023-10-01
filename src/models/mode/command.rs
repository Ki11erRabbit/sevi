use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::models::key::{Key, KeyEvent};
use crate::models::mode::{Mode, TextMode};
use crate::models::pane::TextPane;
use crate::models::settings::Settings;

pub struct CommandMode {
    command_buffer: String,
    edit_pos: usize,
    key_buffer: Vec<KeyEvent>,
    settings: Option<Rc<RefCell<Settings>>>
}


impl CommandMode {
    pub fn new() -> Self {
        Self {
            command_buffer: String::new(),
            edit_pos: 0,
            settings: None,
            key_buffer: Vec::new(),
        }
    }

    fn execute_command(&mut self, command: &str, pane: &mut dyn TextPane) {
        let mut command_args = command.split_whitespace();
        let command_name = command_args.next().unwrap_or("");
        match command_name {
            "cancel" => {
                self.command_buffer.clear();
                self.edit_pos = 0;
                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
            }
            "left" => {
                //Todo: make sure that we move by the right byte size
                self.edit_pos = self.edit_pos.saturating_sub(1);
            }
            "right" => {
                if self.edit_pos < self.command_buffer.len() {
                    //Todo: make sure that we move by the right byte size
                    self.edit_pos += 1;
                }
            }
            "up" => {
                self.edit_pos = 0;
            }
            "down" => {

                self.edit_pos = self.command_buffer.len();
            }
            "backspace" => {
                if self.edit_pos > 0 {
                    self.edit_pos -= 1;
                    self.command_buffer.remove(self.edit_pos);
                }
            }
            "delete" => {
                if self.edit_pos < self.command_buffer.len() {
                    self.command_buffer.remove(self.edit_pos);
                }
            }
            "execute" => {
                pane.execute_command(&self.command_buffer);
                self.command_buffer.clear();
                self.edit_pos = 0;

                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
            }
            _ => {}
        }
    }
}

impl Mode for CommandMode {
    fn get_name(&self) -> String {
        "Command".to_string()
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
        let offset = self.get_name().chars().count() + 2 + self.edit_pos;
        Some(offset)
    }
}

impl TextMode for CommandMode {
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
                    drop(settings);
                    match key.key {
                        Key::Char(c) => {
                            self.command_buffer.insert(self.edit_pos, c);
                            //TODO: make sure that we move by the right byte size
                            self.edit_pos += 1;
                            self.key_buffer.clear();
                        }
                        _ => {}
                    }
                }

            }
        }
    }

    fn update_status(&self, _pane: &dyn TextPane) -> (String, String, String) {
        let first = format!(":{}", self.command_buffer);
        let second = String::new();

        (self.get_name(), first, second)
    }

    fn start(&mut self, _pane: &mut dyn TextPane) {
    }
}