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
}

impl SelectionMode {
    pub fn new() -> SelectionMode {
        SelectionMode {
            selection_type: SelectionType::Normal,
            start: (0, 0),
            settings: None,
            key_buffer: Vec::new(),
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
                pane.execute_command("change_mode Normal");
                pane.execute_command("clear_selection");
            },
            "left" => {
                pane.execute_command(&format!("move left 1"));
                self.add_selection(pane);
            },
            "right" => {
                pane.execute_command(&format!("move right 1"));
                self.add_selection(pane);
            },
            "up" => {
                pane.execute_command(&format!("move up 1"));
                self.add_selection(pane);
            },
            "down" => {
                pane.execute_command(&format!("move down 1"));
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
                pane.execute_command(&format!("move page_up 1"));
                self.add_selection(pane);
            },
            "page_down" => {
                pane.execute_command(&format!("move page_down 1"));
                self.add_selection(pane);
            },
            _ => {}
        }

        self.key_buffer.clear();
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

    fn add_special(&mut self, something: &dyn Any) {
        if let Some((col, row)) = something.downcast_ref::<(usize, usize)>() {
            self.set_start((*col, *row));
        }
        if let Some(selection_type) = something.downcast_ref::<SelectionType>() {
            self.set_selection_type(*selection_type);
        }
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
                pane.execute_command("change_mode Normal");
                pane.execute_command("clear_selection");
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

        let first = format!("{}:{}", row, col);

        let second = match self.selection_type {
            SelectionType::Normal => String::from("Normal"),
            SelectionType::Line => String::from("Line"),
            SelectionType::Block => String::from("Block"),
        };

        (self.get_name(), first, second)
    }

    fn start(&mut self, pane: &mut dyn TextPane) {
        pane.execute_command("clear_selection");
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