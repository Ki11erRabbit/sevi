use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::{Sender, Receiver};
use crate::models::style::StyledText;
use crate::models::cursor::Cursor;
use crate::models::key::KeyEvent;
use crate::models::pane::TextPane;
use std::path::PathBuf;


use crate::models::cursor::CursorMovement;
use crate::models::pane::Pane;
use crate::models::file::File;
use crate::models::{AppEvent, Message};
use crate::models::mode::command::CommandMode;
use crate::models::settings::Settings;
use crate::models::mode::TextMode;
use crate::models::mode::normal::NormalMode;
use crate::models::mode::Mode;



pub trait TextBufferObserver {
}







pub struct TextBuffer {
    file: File,
    cursor: Cursor,
    mode: Rc<RefCell<dyn TextMode>>,
    modes: HashMap<String, Rc<RefCell<dyn TextMode>>>,
    settings: Rc<RefCell<Settings>>,
    sender: Sender<AppEvent>,
    //lsp_channels: (Sender<LspMessage>, Receiver<LspMessage>),
    //register_channels: (Sender<RegisterMessage>, Receiver<RegisterMessage>),
}


impl TextBuffer {
    pub fn new(path: Option<PathBuf>, sender: Sender<AppEvent>, settings: Rc<RefCell<Settings>>) -> Self {
        let file = File::new(path, settings.clone());

        let normal_mode = Rc::new(RefCell::new(NormalMode::new()));
        normal_mode.borrow_mut().add_settings(settings.clone());
        let command_mode = Rc::new(RefCell::new(CommandMode::new()));
        command_mode.borrow_mut().add_settings(settings.clone());

        let normal_mode: Rc<RefCell<dyn TextMode>> = normal_mode.clone();
        let command_mode: Rc<RefCell<dyn TextMode>> = command_mode.clone();


        let mut modes = HashMap::new();
        modes.insert("Normal".to_string(), normal_mode.clone());
        modes.insert("Command".to_string(), command_mode);

        Self {
            file,
            cursor: Cursor::new(),
            mode: normal_mode,
            modes,
            settings,
            sender,
        }
    }

}


impl Pane for TextBuffer {
    fn execute_command(&mut self, command: &str) {
        let mut command_args = command.split_whitespace();
        let command = command_args.next().unwrap_or("");

        match command {
            "move" => {
                let direction = command_args.next();
                let direction = match direction {
                    Some("up") => CursorMovement::Up,
                    Some("down") => CursorMovement::Down,
                    Some("left") => CursorMovement::Left,
                    Some("right") => CursorMovement::Right,
                    Some("page_up") => CursorMovement::PageUp,
                    Some("page_down") => CursorMovement::PageDown,
                    /*Some("start_of_file") => CursorMovement::StartOfFile,
                    Some("end_of_file") => CursorMovement::EndOfFile,
                    Some("half_page_up") => CursorMovement::HalfPageUp,
                    Some("half_page_down") => CursorMovement::HalfPageDown,
                    Some("start_of_line") => CursorMovement::StartOfLine,
                    Some("end_of_line") => CursorMovement::EndOfLine,*/
                    _ => panic!("Invalid direction"),
                };

                let amount = command_args.next().unwrap_or("1").parse::<usize>().unwrap_or(1);

                //todo: add to jump table when doing large movements

                self.cursor.move_cursor(direction, amount, &self.file);

            },
            "change_mode" => {
                let mode = command_args.next().unwrap_or("Normal");
                self.mode = self.modes.get(mode).unwrap().clone();
            },
            "qa!" => {
                self.sender.send(AppEvent::ForceQuit).expect("Failed to send force quit event");
            }
            "q" => {
                self.sender.send(AppEvent::Close).expect("Failed to send quit event");
            }
            "q!" => {
                self.sender.send(AppEvent::ForceClose).expect("Failed to send force quit event");
            }
            _ => {},
        }
    }

    fn get_cursor_position(&self) -> Option<(usize, usize)> {
        Some(self.cursor.get_relative_cursor())
    }

    fn draw(&self) -> StyledText {
        self.file.display()
    }

    fn process_keypress(&mut self, key: KeyEvent) {
        let mode = self.mode.clone();
        let mut mode = mode.borrow_mut();
        mode.process_keypress(key, self);
    }

    fn get_status(&self) -> (StyledText, StyledText, StyledText) {
        let mode = self.mode.clone();
        let mode = mode.borrow();
        let (name, first, second) = mode.update_status(self);


        (StyledText::from(name),StyledText::from(first), StyledText::from(second))
    }

    fn get_scroll_amount(&self) -> Option<(usize, usize)> {
        Some(self.cursor.get_scroll_amount())
    }

    fn refresh(&mut self) {
    }
}

impl TextPane for TextBuffer {
    fn get_cursor(&self) -> (usize, usize) {
        self.cursor.get_cursor()

    }
}





