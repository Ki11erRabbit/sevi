use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use std::str::SplitWhitespace;
use std::sync::mpsc::{Sender, Receiver};
use crate::models::style::{StyledLine, StyledSpan, StyledText};
use crate::models::cursor::Cursor;
use crate::models::key::KeyEvent;
use crate::models::pane::TextPane;
use either::Either;


use crate::models::cursor::CursorMovement;
use crate::models::pane::Pane;
use crate::models::file::File;
use crate::models::{AppEvent, Rect, settings};
use crate::models::file::file::ReplaceSelections;
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
        modes.insert("Normal".to_string(), normal_mode);
        modes.insert("Command".to_string(), command_mode);
        modes.insert("Insert".to_string(), insert_mode);
        modes.insert("Selection".to_string(), selection_mode);
        modes.insert("Search".to_string(), search_mode);


        let mode = {
            let settings = settings.clone();
            let settings = settings.borrow();
            let mode = &settings.editor_settings.default_mode;
            modes.get(mode).unwrap().clone()
        };


        Self {
            file,
            cursor: Cursor::new(),
            mode,
            modes,
            settings,
            sender,
            register_channels,
        }
    }

    fn get_number_line_width(&self) -> usize {
        let line_count = self.file.get_line_count();
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

    fn editor_commands(&mut self, command_name: &str, command_args: &mut SplitWhitespace) {
        match command_name {
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
            "w" => {
                if let Some(path) = command_args.next() {
                    let path = PathBuf::from(path);
                    match self.file.save(Some(path), false) {
                        Ok(_) => {
                            self.send_info_message("File saved");
                        }
                        Err(msg) => {
                            self.send_info_message(msg.as_str());
                        }
                    }
                } else {
                    match self.file.save(None, false) {
                        Ok(_) => {
                            self.send_info_message("File saved");
                        }
                        Err(msg) => {
                            self.send_info_message(msg.as_str());
                        }
                    }
                }
            }
            "w!" => {
                if let Some(path) = command_args.next() {
                    let path = PathBuf::from(path);
                    match self.file.save(Some(path), true) {
                        Ok(_) => {
                            self.send_info_message("File saved");
                        }
                        Err(msg) => {
                            self.send_info_message(msg.as_str());
                        }
                    }
                } else {
                    match self.file.save(None, true) {
                        Ok(_) => {
                            self.send_info_message("File saved");
                        }
                        Err(msg) => {
                            self.send_info_message(msg.as_str());
                        }
                    }
                }
            }
            "wq" => {
                if let Some(path) = command_args.next() {
                    let path = PathBuf::from(path);
                    match self.file.save(Some(path), false) {
                        Ok(_) => {
                            self.send_info_message("File saved");
                        }
                        Err(msg) => {
                            self.send_info_message(msg.as_str());
                            return;
                        }
                    }
                } else {
                    match self.file.save(None, false) {
                        Ok(_) => {
                            self.send_info_message("File saved");
                        }
                        Err(msg) => {
                            self.send_info_message(msg.as_str());
                            return;
                        }
                    }
                    self.sender.send(AppEvent::Close).expect("Failed to send quit event");
                }
            }
            "w!q" => {
                if let Some(path) = command_args.next() {
                    let path = PathBuf::from(path);
                    match self.file.save(Some(path), true) {
                        Ok(_) => {
                            self.send_info_message("File saved");
                        }
                        Err(msg) => {
                            self.send_info_message(msg.as_str());
                            return
                        }
                    }
                } else {
                    match self.file.save(None, true) {
                        Ok(_) => {
                            self.send_info_message("File saved");
                        }
                        Err(msg) => {
                            self.send_info_message(msg.as_str());
                            return
                        }
                    }
                    self.sender.send(AppEvent::Close).expect("Failed to send quit event");
                }
            }
            "w!q!" => {
                if let Some(path) = command_args.next() {
                    let path = PathBuf::from(path);
                    match self.file.save(Some(path), true) {
                        Ok(_) => {
                            self.send_info_message("File saved");
                        }
                        Err(msg) => {
                            self.send_info_message(msg.as_str());
                        }
                    }
                } else {
                    match self.file.save(None, true) {
                        Ok(_) => {
                            self.send_info_message("File saved");
                        }
                        Err(msg) => {
                            self.send_info_message(msg.as_str());
                        }
                    }
                    self.sender.send(AppEvent::ForceClose).expect("Failed to send force quit event");
                }
            }
            "wq!" => {
                if let Some(path) = command_args.next() {
                    let path = PathBuf::from(path);
                    match self.file.save(Some(path), false) {
                        Ok(_) => {
                            self.send_info_message("File saved");
                        }
                        Err(msg) => {
                            self.send_info_message(msg.as_str());
                        }
                    }
                } else {
                    match self.file.save(None, false) {
                        Ok(_) => {
                            self.send_info_message("File saved");
                        }
                        Err(msg) => {
                            self.send_info_message(msg.as_str());
                        }
                    }
                    self.sender.send(AppEvent::ForceClose).expect("Failed to send force quit event");
                }
            }
            _ => {}
        }
    }

    fn edit_commands(&mut self, command_name: &str, command_args: &mut SplitWhitespace) {
        match command_name {
            "paste" => {
                if let Some(direction) = command_args.next() {
                    let next_arg = command_args.next();
                    let register = if let Ok(reg) = next_arg.unwrap_or("").parse::<usize>() {
                        Some(Either::Left(reg))
                    } else if let Some(reg) = next_arg {
                        Some(Either::Right(reg.to_string()))
                    } else {
                        None
                    };

                    let message = if let Some(Either::Left(reg)) = register {
                        RegisterMessage::GetNumbered(reg)
                    } else if let Some(Either::Right(reg)) = register {
                        RegisterMessage::GetNamed(reg)
                    } else {
                        RegisterMessage::GetClipboard
                    };

                    self.register_channels.0.send(message).expect("Failed to send register message");

                    let message = self.register_channels.1.recv().expect("Failed to receive register message");

                    let multi;
                    let string = match message {
                        RegisterMessage::RegisterResult(Some(text),None) => {
                            multi = None;
                            text
                        },
                        RegisterMessage::RegisterResult(Some(text), Some(multii)) => {
                            multi = Some(multii);
                            text
                        },
                        _ => return,
                    };


                    let byte_offset = self.get_current_byte_position();
                    match direction {
                        "after" => {
                            self.file.insert_after_current(byte_offset, string);
                        },
                        "before" => {
                            self.file.insert_before_current(byte_offset, string);
                        },
                        "selection" => {
                            match multi {
                                Some(multi) => {
                                    self.file.replace_selections(multi);
                                }
                                None => {
                                    self.file.replace_selections(string.as_str());
                                }
                            }

                        }
                        _ => panic!("Invalid paste direction"),
                    }
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

                            let line = self.file.get_line(row).expect("Invalid row in copy").to_string();

                            if let Some(Either::Left(reg)) = register {
                                RegisterMessage::AddNumbered(reg, line)
                            } else if let Some(Either::Right(reg)) = register {
                                RegisterMessage::AddNamed(reg, line)
                            } else {
                                RegisterMessage::SetClipboard(line)
                            }
                        }
                        "word" => {
                            let byte_offset = self.get_current_byte_position();

                            let word = match self.file.get_word(byte_offset) {
                                Some(word) => word.to_string(),
                                None => {
                                    self.send_info_message("No word found at cursor position");

                                    return;
                                },
                            };

                            if let Some(Either::Left(reg)) = register {
                                RegisterMessage::AddNumbered(reg, word)
                            } else if let Some(Either::Right(reg)) = register {
                                RegisterMessage::AddNamed(reg, word)
                            } else {
                                RegisterMessage::SetClipboard(word)
                            }
                        }
                        "to_next_word" => {
                            let byte_offset = self.get_current_byte_position();

                            let text = self.file.get_until_next_word(byte_offset).expect("Invalid byte position in copy").to_string();

                            if let Some(Either::Left(reg)) = register {
                                RegisterMessage::AddNumbered(reg, text)
                            } else if let Some(Either::Right(reg)) = register {
                                RegisterMessage::AddNamed(reg, text)
                            } else {
                                RegisterMessage::SetClipboard(text)
                            }
                        }
                        "to_prev_word" => {
                            let byte_offset = self.get_current_byte_position();

                            let text = self.file.get_until_prev_word(byte_offset).expect("Invalid byte position in copy").to_string();

                            if let Some(Either::Left(reg)) = register {
                                RegisterMessage::AddNumbered(reg, text)
                            } else if let Some(Either::Right(reg)) = register {
                                RegisterMessage::AddNamed(reg, text)
                            } else {
                                RegisterMessage::SetClipboard(text)
                            }
                        }
                        "to_end_line" => {
                            let (col, row) = self.get_cursor();

                            let line = self.file.get_line(row).expect("Invalid row in copy").to_string();

                            let line = line.chars().skip(col - 1).collect::<String>();

                            if let Some(Either::Left(reg)) = register {
                                RegisterMessage::AddNumbered(reg, line)
                            } else if let Some(Either::Right(reg)) = register {
                                RegisterMessage::AddNamed(reg, line)
                            } else {
                                RegisterMessage::SetClipboard(line)
                            }
                        }
                        "to_start_line" => {
                            let (col, row) = self.get_cursor();

                            let line = self.file.get_line(row).expect("Invalid row in copy").to_string();

                            let line = line.chars().take(col).collect::<String>();

                            if let Some(Either::Left(reg)) = register {
                                RegisterMessage::AddNumbered(reg, line)
                            } else if let Some(Either::Right(reg)) = register {
                                RegisterMessage::AddNamed(reg, line)
                            } else {
                                RegisterMessage::SetClipboard(line)
                            }
                        }
                        "selection" => {
                            let selections = self.file.get_highlighted();

                            if let Some(selections) = selections {

                                if selections.len() == 1 {
                                    let text = selections[0].clone();

                                    if let Some(Either::Left(reg)) = register {
                                        RegisterMessage::AddNumbered(reg, text)
                                    } else if let Some(Either::Right(reg)) = register {
                                        RegisterMessage::AddNamed(reg, text)
                                    } else {
                                        RegisterMessage::SetClipboard(text)
                                    }
                                } else {
                                    RegisterMessage::SetMulti(selections)
                                }

                            } else {
                                return;
                            }
                        }
                        _ => panic!("Invalid copy verb"),
                    };

                    self.register_channels.0.send(message).expect("Failed to send register message");
                }
            }
            "delete" => {
                if let Some(verb) = command_args.next() {

                    match verb {
                        "char" => {
                            let byte_offset = self.get_current_byte_position();

                            self.file.delete(byte_offset..byte_offset + 1);
                        }
                        "line" => {
                            let row = self.get_cursor().1;

                            self.file.delete_line(row);
                            self.cursor.move_cursor(CursorMovement::LineStart, 1, &self.file);
                        }
                        "word" => {
                            let byte_offset = self.get_current_byte_position();

                            let byte_offset = self.file.delete_word(byte_offset);
                            self.set_cursor_to_byte_position(byte_offset);
                        }
                        "to_next_word" => {
                            let byte_offset = self.get_current_byte_position();

                            let text = self.file.get_until_next_word(byte_offset).expect("Invalid byte position in copy").to_string();

                            self.file.delete(byte_offset..byte_offset + text.len());
                        }
                        "to_prev_word" => {
                            let byte_offset = self.get_current_byte_position();

                            let text = self.file.get_until_prev_word(byte_offset).expect("Invalid byte position in copy").to_string();

                            self.file.delete(byte_offset - text.len()..byte_offset);
                            let byte_offset = byte_offset - text.len();
                            self.set_cursor_to_byte_position(byte_offset);
                        }
                        "to_end_line" => {
                            let (_, row) = self.get_cursor();

                            let line = self.file.get_line(row).expect("Invalid row in copy").to_string();

                            let byte_offset = self.get_current_byte_position();
                            self.file.delete(byte_offset..byte_offset + line.len());
                        }
                        "to_start_line" => {
                            let (col, row) = self.get_cursor();
                            let line = self.file.get_line(row).expect("Invalid row in copy").to_string();

                            let line = line.chars().take(col).collect::<String>();

                            let byte_offset = self.get_current_byte_position();
                            self.file.delete(byte_offset - line.len()..byte_offset);
                            let byte_offset = byte_offset - line.len();
                            self.set_cursor_to_byte_position(byte_offset);

                        }
                        "selection" => {
                            let byte_offset =self.file.delete_highlighted();
                            self.set_cursor_to_byte_position(byte_offset);
                        }
                        _ => panic!("Invalid delete verb"),
                    }

                }
            }
            "undo" => {
                self.file.undo();
            }
            "redo" => {
                self.file.redo();
            }
            _ => {}
        }
    }

    fn pane_commands(&mut self, command_name: &str, command_args: &mut SplitWhitespace) {
        match command_name {
            "change_mode" => {

                //let mode = command_args.next().unwrap_or("Normal");
                let mode = match command_args.next() {
                    Some(mode) => mode.to_string(),
                    None => self.settings.borrow().editor_settings.default_mode.clone(),
                };

                match mode.as_str() {
                    "Normal" | "Insert" | "Command" => {
                        self.mode = self.modes.get(&mode).unwrap().clone();
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

                            if let Some(start) = self.file.get_byte_offset(start[1], start[0]) {
                                if let Some(end) = self.file.get_byte_offset(end[1], end[0]) {
                                    self.file.add_highlight(start, end);
                                }
                            }


                        }
                    }
                }
            }
            _ => {}
        }
    }
    fn movement_commands(&mut self, command_name: &str, command_args: &mut SplitWhitespace) {
        match command_name {
            "move" => {
                let direction = command_args.next();
                if let Some("to") = direction {
                    if let Some(set) = command_args.next() {
                        let set = set.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                        self.cursor.set_cursor(set[0], set[1]);
                    }
                    return;
                } else if let Some("to_byte") = direction {
                    if let Some(byte) = command_args.next() {
                        if let Ok(byte) = byte.parse::<usize>() {
                            self.set_cursor_to_byte_position(byte);
                        }
                    }
                    return;
                }
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

            }
            "goto_line" => {
                if let Some(line) = command_args.next() {
                    if let Ok(line) = line.parse::<usize>() {
                        let (col, _) = self.get_cursor();
                        self.cursor.set_cursor(col, line);
                    }
                }
            }
            _ => {}
        }
    }

}


impl Pane for TextBuffer {
    fn execute_command(&mut self, command: &str) {
        let mut command_args = command.split_whitespace();
        let command = command_args.next().unwrap_or("");

        self.editor_commands(command, &mut command_args);
        self.edit_commands(command, &mut command_args);
        self.pane_commands(command, &mut command_args);
        self.movement_commands(command, &mut command_args);
    }

    fn get_cursor_position(&self) -> Option<(usize, usize)> {
        Some(self.cursor.get_relative_cursor(self))
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

        let settings = self.settings.clone();
        let settings = settings.borrow();

        let name = StyledText::from(vec![StyledLine::from(vec![StyledSpan::styled(name, settings.colors.status_bar.mode.get(&mode.get_name()).unwrap().clone())])]);

        let first = StyledText::from(vec![StyledLine::from(vec![StyledSpan::styled(first, settings.colors.status_bar.first)])]);
        let second = StyledText::from(vec![StyledLine::from(vec![StyledSpan::styled(second, settings.colors.status_bar.second)])]);


        (name,first, second)
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

    fn get_bottom_cursor_position(&self) -> Option<usize> {
        let mode = self.mode.clone();
        let mode = mode.borrow();
        if let Some(index) = mode.influence_cursor() {
            Some(index)
        } else {
            None
        }
    }

    fn send_info_message(&self, message: &str) {
        let message = AppEvent::Message(message.to_string().into_boxed_str());
        self.sender.send(message).expect("Failed to send info message");
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
        self.insert_char(index, '\n');
        self.cursor.move_cursor(CursorMovement::Down, 1, &self.file);
        self.cursor.move_cursor(CursorMovement::LineStart, 1, &self.file);
    }

    fn tab(&mut self) {
        let index = self.get_current_byte_position();

        let settings = self.settings.clone();
        let settings = settings.borrow();
        let tab_size = settings.editor_settings.tab_size;
        let use_spaces = settings.editor_settings.use_spaces;

        if use_spaces {
            self.insert_str_after(index, " ".repeat(tab_size as usize).as_str());
            self.cursor.move_cursor(CursorMovement::Right, tab_size as usize, &self.file)
        } else {
            self.insert_str_after(index, "\t");
            self.cursor.move_cursor(CursorMovement::Right, 1, &self.file)
        }
    }

    fn insert_char(&mut self, index: usize, c: char) {
        self.file.insert_char(index, c);
        self.cursor.move_cursor(CursorMovement::Right, 1, &self.file)
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
        eprintln!("Cursor: {}, {}", col, row);

        self.file.get_byte_offset(row, col).expect("Cursor was in an invalid position")
    }

    fn set_cursor_to_byte_position(&mut self, byte_index: usize) {
        let (col, row) = self.file.get_cursor(byte_index).expect("Invalid byte position");
        self.cursor.set_cursor(col, row);
    }

    fn borrow_current_file(&self) -> &File {
        &self.file
    }

    fn borrow_current_file_mut(&mut self) -> &mut File {
        &mut self.file
    }
}





