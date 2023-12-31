use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::models::key::{Key, KeyEvent};
use crate::models::mode::{Mode, TextMode};
use crate::models::pane::TextPane;
use crate::models::settings::Settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionType {
    Normal,
    Line,
    Block,
}


pub struct SelectionMode {
    selection_type: SelectionType,
    start: (usize, usize),
    settings: Option<Rc<RefCell<Settings>>>,
    key_buffer: Vec<KeyEvent>,
    number_buffer: String,
}

impl SelectionMode {
    pub fn new() -> SelectionMode {
        SelectionMode {
            selection_type: SelectionType::Normal,
            start: (0, 0),
            settings: None,
            key_buffer: Vec::new(),
            number_buffer: String::new(),
        }
    }

    pub fn set_selection_type(&mut self, selection_type: SelectionType) {
        self.selection_type = selection_type;
    }

    pub fn set_start(&mut self, start: (usize, usize)) {
        self.start = start;
    }

    pub fn add_selection(&mut self, pane: &mut dyn TextPane) {
        let (col, row) = pane.get_cursor();
        let (start_col, start_row) = self.start;

        pane.execute_command("clear_selection");
        match self.selection_type {
            SelectionType::Normal => {
                if row < start_row {
                    pane.execute_command(&format!("select {},{} {},{}", col, row, start_col, start_row));
                } else if row > start_row {
                    pane.execute_command(&format!("select {},{} {},{}", start_col, start_row, col, row));
                } else {
                    if col < start_col {
                        pane.execute_command(&format!("select {},{} {},{}", col, row, start_col, start_row));
                    } else {
                        pane.execute_command(&format!("select {},{} {},{}", start_col, start_row, col, row));
                    }
                }
                //pane.execute_command(&format!("select {},{} {},{}", start_col, start_row, col, row));
            },
            SelectionType::Line => {
                if row == start_row {
                    pane.execute_command(&format!("select row {}", row));
                } else if row < start_row {
                    for i in row..=start_row {
                        pane.execute_command(&format!("select row {}", i));
                    }
                } else {
                    for i in start_row..=row {
                        pane.execute_command(&format!("select row {}", i));
                    }
                }
            },
            SelectionType::Block => {
                if row < start_row {
                    for i in row..=start_row {
                        if col < start_col {
                            pane.execute_command(&format!("select {},{} {},{}", col, i, start_col, i));
                        } else {
                            pane.execute_command(&format!("select {},{} {},{}", start_col, i, col, i));
                        }
                    }
                } else {
                    for i in start_row..=row {
                        if col < start_col {
                            pane.execute_command(&format!("select {},{} {},{}", col, i, start_col, i));
                        } else {
                            pane.execute_command(&format!("select {},{} {},{}", start_col, i, col, i));
                        }
                    }
                }

            }
        }
    }

    pub fn execute_command(&mut self, command: &str, pane: &mut dyn TextPane) {
        let mut command_args = command.split_whitespace();
        let command_name = command_args.next().unwrap_or("");

        match command_name {
            "cancel" => {
                self.key_buffer.clear();
                let settings = self.settings.clone().unwrap();
                let settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
                pane.execute_command("clear_selection");
            },
            "left" => {
                pane.execute_command(&format!("move left {}", self.number_buffer));
                self.add_selection(pane);
            },
            "right" => {
                pane.execute_command(&format!("move right {}", self.number_buffer));
                self.add_selection(pane);
            },
            "up" => {
                pane.execute_command(&format!("move up {}", self.number_buffer));
                self.add_selection(pane);
            },
            "down" => {
                pane.execute_command(&format!("move down {}", self.number_buffer));
                self.add_selection(pane);
            },
            "start_of_file" => {
                pane.execute_command("move start_of_file");
                self.add_selection(pane);
            },
            "end_of_file" => {
                pane.execute_command("move end_of_file");
                self.add_selection(pane);
            },
            "page_up" => {
                pane.execute_command(&format!("move page_up {}", self.number_buffer));
                self.number_buffer.clear();
                self.add_selection(pane);
            },
            "page_down" => {
                pane.execute_command(&format!("move page_down {}", self.number_buffer));
                self.number_buffer.clear();
                self.add_selection(pane);
            },"half_page_up" => {
                pane.execute_command(&format!("move half_page_up {}", self.number_buffer));
                self.number_buffer.clear();
                self.add_selection(pane);
            },
            "half_page_down" => {
                pane.execute_command(&format!("move half_page_down {}", self.number_buffer));
                self.number_buffer.clear();
                self.add_selection(pane);
            },
            "start_of_line" => {
                pane.execute_command("move start_of_line");
                self.add_selection(pane);
            },
            "end_of_line" => {
                pane.execute_command("move end_of_line");
                self.add_selection(pane);
            },
            "up_line_start" => {
                pane.execute_command("move up_line_start");
                self.add_selection(pane);
            },
            "down_line_start" => {
                pane.execute_command("move down_line_start");
                self.add_selection(pane);
            },
            "next_word_front" => {
                pane.execute_command(format!("move next_word_front {}", self.number_buffer).as_str());
                self.number_buffer.clear();
                self.add_selection(pane);
            },
            "next_word_back" => {
                pane.execute_command(format!("move next_word_back {}", self.number_buffer).as_str());
                self.number_buffer.clear();
                self.add_selection(pane);
            },
            "previous_word_front" => {
                pane.execute_command(format!("move prev_word_front {}", self.number_buffer).as_str());
                self.number_buffer.clear();
                self.add_selection(pane);
            },
            "previous_word_back" => {
                pane.execute_command(format!("move prev_word_back {}", self.number_buffer).as_str());
                self.number_buffer.clear();
                self.add_selection(pane);
            },
            "copy" => {
                pane.execute_command(&format!("copy selection"));
                let settings = self.settings.clone().unwrap();
                let settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
                pane.execute_command("clear_selection");
            }
            "delete" => {
                pane.execute_command("delete selection");
                let settings = self.settings.clone().unwrap();
                let settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
                pane.execute_command("clear_selection");    }
            "cut" => {
                pane.execute_command(&format!("copy selection"));
                pane.execute_command("delete selection");
                let settings = self.settings.clone().unwrap();
                let settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
                pane.execute_command("clear_selection");
            }
            "paste" => {
                pane.execute_command("paste selection");
                let settings = self.settings.clone().unwrap();
                let settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
                pane.execute_command("clear_selection");
            }
            "mirror_mode" => {
                let command = match self.selection_type {
                    SelectionType::Normal => "selection_normal_mirror",
                    SelectionType::Line => "selection_line_mirror",
                    SelectionType::Block => "selection_block_mirror",
                };

                pane.execute_command(&format!("change_mode mirror {}", command));
            }
            "pair_mode" => {
                let command = match self.selection_type {
                    SelectionType::Normal => "selection_normal_pair",
                    SelectionType::Line => "selection_line_pair",
                    SelectionType::Block => "selection_block_pair",
                };

                pane.execute_command(&format!("change_mode pair {}", command));
            }
            _ => {}
        }

        self.key_buffer.clear();
        self.number_buffer.clear();
    }
}

impl Mode for SelectionMode {
    fn get_name(&self) -> String {
        "Selection".to_string()
    }

    fn add_settings(&mut self, settings: Rc<RefCell<Settings>>) {
        self.settings = Some(settings);
    }

    fn refresh(&mut self) {

    }

    //todo: also get the number buffer from normal mode
    fn add_special(&mut self, something: &dyn Any) {
        if let Some((col, row)) = something.downcast_ref::<(usize, usize)>() {
            self.set_start((*col, *row));
        }
        if let Some(selection_type) = something.downcast_ref::<SelectionType>() {
            self.set_selection_type(*selection_type);
        }
        if let Some(number_buffer) = something.downcast_ref::<String>() {
            self.number_buffer = number_buffer.clone();
        }
    }

    fn get_special(&self) -> Option<&dyn Any> {
        None
    }

    fn influence_cursor(&self) -> Option<usize> {
        None
    }
}

impl TextMode for SelectionMode {
    fn process_keypress(&mut self, key: KeyEvent, pane: &mut dyn TextPane) {

        match key {
            KeyEvent {
                key: Key::Esc,
                ..
            } => {
                self.key_buffer.clear();
                let settings = self.settings.clone().unwrap();
                let settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
                pane.execute_command("clear_selection");
            },
            key => {

                self.key_buffer.push(key);

                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow_mut();
                if let Some(command) = settings.mode_keybindings.get(&self.get_name(), &self.key_buffer) {
                    let command = command.clone();
                    drop(settings);
                    self.execute_command(&command, pane);
                }
            }
        }
    }

    fn update_status(&self, pane: &dyn TextPane) -> (String, String, String) {

        let (col, row) = pane.get_cursor();

        let first = if self.number_buffer.is_empty() {
            format!("{}:{}", row, col)
        } else {
            format!("{}:{} {}", row, col, self.number_buffer)
        };

        let second = match self.selection_type {
            SelectionType::Normal => String::from("Normal"),
            SelectionType::Line => String::from("Line"),
            SelectionType::Block => String::from("Block"),
        };

        (self.get_name(), first, second)
    }

    fn start(&mut self, pane: &mut dyn TextPane) {

        match self.selection_type {
            SelectionType::Line => {
                let (_, row) = pane.get_cursor();
                pane.execute_command(&format!("select row {}", row));
            }
            SelectionType::Normal | SelectionType::Block => {
                let (col, row) = pane.get_cursor();
                pane.execute_command(&format!("select {},{} {},{}", col, row, col, row));
            }
        }


    }
}