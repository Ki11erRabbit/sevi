use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::{Sender, Receiver};
use crate::models::style::StyledText;
use crate::models::cursor::Cursor;
use crate::models::key::KeyEvent;
use crate::models::pane::TextPane;
use std::path::PathBuf;
use either::Either;


use crate::models::cursor::CursorMovement;
use crate::models::pane::Pane;
use crate::models::file::File;
use crate::models::{AppEvent, Message, Rect};
use crate::models::mode::command::CommandMode;
use crate::models::mode::insert::InsertMode;
use crate::models::settings::Settings;
use crate::models::mode::TextMode;
use crate::models::mode::normal::NormalMode;
use crate::models::mode::Mode;
use crate::models::mode::search::{SearchMode, SearchType};
use crate::models::mode::selection::{SelectionMode, SelectionType};
use crate::models::settings::editor_settings::NumberLineStyle;
use crate::threads::registers::RegisterMessage;


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
    register_channels: (Sender<RegisterMessage>, Rc<Receiver<RegisterMessage>>),
}


impl TextBuffer {
    pub fn new(file: File,
               sender: Sender<AppEvent>,
               settings: Rc<RefCell<Settings>>,
        register_channels: (Sender<RegisterMessage>, Rc<Receiver<RegisterMessage>>)) -> Self {
        //let file = File::new(path, settings.clone());

        let normal_mode = Rc::new(RefCell::new(NormalMode::new()));
        normal_mode.borrow_mut().add_settings(settings.clone());
        let command_mode = Rc::new(RefCell::new(CommandMode::new()));
        command_mode.borrow_mut().add_settings(settings.clone());
        let insert_mode = Rc::new(RefCell::new(InsertMode::new()));
        insert_mode.borrow_mut().add_settings(settings.clone());
        let selection_mode = Rc::new(RefCell::new(SelectionMode::new()));
        selection_mode.borrow_mut().add_settings(settings.clone());
        let search_mode = Rc::new(RefCell::new(SearchMode::new()));
        search_mode.borrow_mut().add_settings(settings.clone());

        let normal_mode: Rc<RefCell<dyn TextMode>> = normal_mode.clone();
        let command_mode: Rc<RefCell<dyn TextMode>> = command_mode.clone();
        let insert_mode: Rc<RefCell<dyn TextMode>> = insert_mode.clone();
        let selection_mode: Rc<RefCell<dyn TextMode>> = selection_mode.clone();
        let search_mode: Rc<RefCell<dyn TextMode>> = search_mode.clone();


        let mut modes = HashMap::new();
        modes.insert("Normal".to_string(), normal_mode.clone());
        modes.insert("Command".to_string(), command_mode);
        modes.insert("Insert".to_string(), insert_mode);
        modes.insert("Selection".to_string(), selection_mode);
        modes.insert("Search".to_string(), search_mode);

        Self {
            file,
            cursor: Cursor::new(),
            mode: normal_mode,
            modes,
            settings,
            sender,
            register_channels,
        }
    }

    fn get_number_line_width(&self) -> usize {
        let mut line_count = self.file.get_line_count();
        match self.settings.borrow().editor_settings.number_line {
            NumberLineStyle::None => 0,
            NumberLineStyle::Relative => {
                let mut places = 1;
                let mut num_width = 3;
                while places <= line_count {
                    places *= 10;
                    num_width += 1;
                }
                num_width
            }
            NumberLineStyle::Absolute => {
                let mut places = 1;
                let mut num_width = 0;
                while places <= line_count {
                    places *= 10;
                    num_width += 1;
                }
                num_width
            }
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
                    Some("start_of_file") => CursorMovement::FileStart,
                    Some("end_of_file") => CursorMovement::FileEnd,
                    Some("half_page_up") => CursorMovement::HalfPageUp,
                    Some("half_page_down") => CursorMovement::HalfPageDown,
                    Some("start_of_line") => CursorMovement::LineStart,
                    Some("end_of_line") => CursorMovement::LineEnd,
                    _ => panic!("Invalid direction"),
                };

                let amount = command_args.next().unwrap_or("1").parse::<usize>().unwrap_or(1);

                //todo: add to jump table when doing large movements

                self.cursor.move_cursor(direction, amount, &self.file);

            },
            "change_mode" => {
                let mode = command_args.next().unwrap_or("Normal");
                match mode {
                    "Normal" | "Insert" | "Command" => {
                        self.mode = self.modes.get(mode).unwrap().clone();
                    },
                    "insert_before" => {
                        self.mode = self.modes.get("Insert").unwrap().clone();
                        self.cursor.move_cursor(CursorMovement::Left, 1, &self.file);
                    },
                    "insert_after" => {
                        self.mode = self.modes.get("Insert").unwrap().clone();
                    },
                    "insert_start_of_line" => {
                        self.mode = self.modes.get("Insert").unwrap().clone();
                        //self.cursor.move_cursor(CursorMovement::StartOfLine, 1, &self.file);
                    },
                    "insert_end_of_line" => {
                        self.mode = self.modes.get("Insert").unwrap().clone();
                        //self.cursor.move_cursor(CursorMovement::EndOfLine, 1, &self.file);
                    },
                    "insert_below" => {
                        self.mode = self.modes.get("Insert").unwrap().clone();
                        self.cursor.move_cursor(CursorMovement::Down, 1, &self.file);
                    },
                    "insert_above" => {
                        self.mode = self.modes.get("Insert").unwrap().clone();
                        self.cursor.move_cursor(CursorMovement::Up, 1, &self.file);
                    },
                    "selection_normal" => {
                        let mode= self.modes.get("Selection").unwrap().clone();
                        let pos = self.get_cursor();
                        mode.borrow_mut().add_special(&pos);
                        mode.borrow_mut().add_special(&SelectionType::Normal);
                        self.mode = mode;
                    }
                    "selection_line" => {
                        let mode= self.modes.get("Selection").unwrap().clone();
                        let pos = self.get_cursor();
                        mode.borrow_mut().add_special(&pos);
                        mode.borrow_mut().add_special(&SelectionType::Line);
                        self.mode = mode;
                    }
                    "selection_block" => {
                        let mode= self.modes.get("Selection").unwrap().clone();
                        let pos = self.get_cursor();
                        mode.borrow_mut().add_special(&pos);
                        mode.borrow_mut().add_special(&SelectionType::Block);
                        self.mode = mode;
                    }
                    "search_down" => {
                        let mode = self.modes.get("Search").unwrap().clone();
                        mode.borrow_mut().add_special(&SearchType::Forward);
                        self.mode = mode;
                    }
                    "search_up" => {
                        let mode = self.modes.get("Search").unwrap().clone();
                        mode.borrow_mut().add_special(&SearchType::Backward);
                        self.mode = mode;
                    }
                    _ => panic!("Invalid mode"),

                }
                let mode = self.mode.clone();
                let mut mode = mode.borrow_mut();
                mode.start(self);
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
            "e" => {
                let path = command_args.next();
                if let Some(path) = path {
                    self.sender.send(AppEvent::OpenFile(path.to_owned().into()))
                        .expect("Failed to send open file event");
                }
            }
            "clear_selection" => {
                self.file.clear_highlights();
            }
            "select" => {
                let next = command_args.next();
                if let Some("row") = next {
                    if let Ok(row) = command_args.next().unwrap_or("0").parse::<usize>() {
                        self.file.select_row(row);
                    }
                } else {
                    if let Some(start) = next {
                        if let Some(end) = command_args.next() {
                            let start = start.split(',');
                            let end = end.split(',');
                            let start = start.map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                            let end = end.map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

                            let start = self.file.get_byte_offset(start[1], start[0]).unwrap();
                            let end = self.file.get_byte_offset(end[1], end[0]).unwrap();

                            self.file.add_highlight(start, end);

                        }
                    }
                }
            }
            "paste" => {
                if let Some(direction) = command_args.next() {

                }
            }
            "copy" => {
                if let Some(verb) = command_args.next() {
                    let next_arg = command_args.next();
                    let register = if let Ok(reg) = next_arg.unwrap_or("").parse::<usize>() {
                        Some(Either::Left(reg))
                    } else if let Some(reg) = next_arg {
                        Some(Either::Right(reg.to_string()))
                    } else {
                        None
                    };

                    let message = match verb {
                        "char" => {
                            let byte_offset = self.get_current_byte_position();

                            let c = self.file.get_char_at(byte_offset).expect("Invalid byte position in copy");

                            if let Some(Either::Left(reg)) = register {
                                RegisterMessage::AddNumbered(reg, c.to_string())
                            } else if let Some(Either::Right(reg)) = register {
                                RegisterMessage::AddNamed(reg, c.to_string())
                            } else {
                                RegisterMessage::SetClipboard(c.to_string())
                            }
                        }
                        "line" => {
                            let row = self.get_cursor().1;

                            let line = self.file.get_line(row).expect("Invalid row in copy");

                            if let Some(Either::Left(reg)) = register {
                                RegisterMessage::AddNumbered(reg, line)
                            } else if let Some(Either::Right(reg)) = register {
                                RegisterMessage::AddNamed(reg, line)
                            } else {
                                RegisterMessage::SetClipboard(line)
                            }
                        }
                        "word" => {
                            todo!()
                        }
                        "to_next_word" => {
                            todo!()
                        }
                        "to_prev_word" => {
                            todo!()
                        }
                        "to_end_of_word" => {
                            todo!()
                        }
                        "to_end_line" => {
                            todo!()
                        }
                        "to_start_line" => {
                            todo!()
                        }
                        _ => panic!("Invalid copy verb"),
                    };

                    self.register_channels.0.send(message).expect("Failed to send register message");


                }
            }
            "cut" => {
                if let Some(verb) = command_args.next() {

                }
            }
            "delete" => {
                if let Some(verb) = command_args.next() {

                }
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
        let number_line_width = self.get_number_line_width();
        self.cursor.set_number_line_width(number_line_width);
    }

    fn get_settings(&self) -> Rc<RefCell<Settings>> {
        self.settings.clone()
    }

    fn draw_section(&self, start_row: usize, end_row: usize) -> StyledText {
        self.file.display_section(start_row, end_row)
    }
}

impl TextPane for TextBuffer {
    fn get_cursor(&self) -> (usize, usize) {
        self.cursor.get_cursor()

    }

    fn change_file(&mut self, mut file: File) -> File {
        std::mem::swap(&mut self.file, &mut file);
        file
    }

    fn can_close(&self) -> bool {
        self.file.has_saved()
    }

    fn scroll(&mut self, rect: Rect) {
        self.cursor.scroll(rect);
    }

    fn backspace(&mut self) {
        let index = self.get_current_byte_position();

        if self.file.get_byte(index.saturating_sub(1)) == b'\n' {
            self.cursor.move_cursor(CursorMovement::Up, 1, &self.file);
            self.cursor.move_cursor(CursorMovement::LineEnd, 1, &self.file);
        } else {
            self.cursor.move_cursor(CursorMovement::Left, 1, &self.file);
        }
        self.file.delete(index.saturating_sub(1)..index);
    }

    fn delete(&mut self) {
        let index = self.get_current_byte_position();
        let (col, row) = self.get_cursor();
        if let Some(_) = self.file.get_byte_offset(col + 1, row) {
            self.file.delete(index..index + 1);
        }

    }

    fn newline(&mut self) {
        let index = self.get_current_byte_position();
        self.file.insert_after(index, '\n'.to_string());
        self.cursor.move_cursor(CursorMovement::Down, 1, &self.file);
        self.cursor.move_cursor(CursorMovement::LineStart, 1, &self.file);
    }

    fn insert_str_after(&mut self, index: usize, string: &str) {
        self.file.insert_after_current(index, string);
    }
    fn insert_str_before(&mut self, index: usize, string: &str) {
        self.file.insert_before_current(index, string);
    }


    fn get_byte_at(&self, byte_index: usize) -> u8 {
        self.file.get_byte(byte_index)
    }

    fn get_current_byte_position(&self) -> usize {
        let (col, row) = self.get_cursor();

        self.file.get_byte_offset(col, row).expect("Cursor was in an invalid position")
    }

    fn borrow_current_file(&self) -> &File {
        &self.file
    }

    fn borrow_current_file_mut(&mut self) -> &mut File {
        &mut self.file
    }
}





