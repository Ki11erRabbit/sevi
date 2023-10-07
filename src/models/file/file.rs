use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::fmt::Formatter;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use tree_sitter::Parser;
use crate::models::file::buffer::Buffer;
use crate::models::settings::Settings;
use crate::models::style::{StyledLine, StyledSpan, StyledText};
use crate::threads::lsp::{LspControllerMessage, LspNotification, LspRequest, LspResponse};

#[derive(Debug)]
pub enum FileError {
    FileDoesNotExist,
    Directory,
    RecoverFileFound(File),
}
pub trait ReplaceSelections<S> {
    fn replace_selections(&mut self, selections: S);
}

pub trait InsertPairs<P> {
    fn insert_pairs(&mut self, pairs: P);
}


#[derive(Debug)]
pub struct LspInfo {
    pub lsp_channels: (Sender<LspControllerMessage>, Rc<Receiver<LspControllerMessage>>),
    pub lsp_client: Option<Arc<Receiver<LspControllerMessage>>>,
    pub first_pass: bool,
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("File")
            .field("path", &self.path)
            .field("buffer", &self.buffer)
            .field("lsp_info", &self.lsp_info)
            .field("language", &self.language)
            .field("highlights", &self.highlights)
            .field("saved", &self.saved)
            .finish()
    }
}

pub struct File {
    path: Option<PathBuf>,
    language: Option<String>,
    buffer: Buffer,
    lsp_info: LspInfo,
    settings: Rc<RefCell<Settings>>,
    highlights: BTreeSet<usize>,
    saved: bool,
    safe_close: bool,
    /// Syntax highlights are stored in a hashmap with the key being column and row
    syntax_highlights: HashMap<(usize, usize), String>
}

impl File {
    pub fn new(path: Option<PathBuf>,
               settings: Rc<RefCell<Settings>>,
               lsp_channels:(Sender<LspControllerMessage>, Rc<Receiver<LspControllerMessage>>)) -> Result<Self,FileError> {
        match path {
            Some(path) => {
                if path.is_dir() {
                    return Err(FileError::Directory);
                }
                if !path.is_file() {
                    return Err(FileError::FileDoesNotExist);
                }

                let parent = path.parent();
                let file_name = path.file_name().unwrap();
                let file_name = file_name.to_str().unwrap();

                let recover_file_path = match parent {
                    Some(parent) => {
                        PathBuf::from(format!("{}##{}", parent.display(), file_name))
                    }
                    None => {
                        PathBuf::from(format!("##{}", file_name))
                    }
                };

                let recovered_file;

                if recover_file_path.exists() {
                    recovered_file = true;
                } else {
                    recovered_file = false;
                }
                let string = std::fs::read_to_string(&path).unwrap();

                let file_type = path.extension().and_then(|ext| ext.to_str()).unwrap_or("txt").to_string();

                let (language, mut buffer, lsp_info) = match file_type.as_str() {
                    /*"scm" => {
                        let language = unsafe { tree_sitter_scheme() };
                        let mut buffer = Buffer::new(string);

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        buffer.set_tree_sitter(parser);

                        (Some("scheme".to_string()), buffer)
                    }*/
                    "rs" => {
                        let language = tree_sitter_rust::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        lsp_channels.0.send(LspControllerMessage::CreateClient(String::from("rust").into())).unwrap();

                        let lsp_client;

                        loop {
                            match lsp_channels.1.try_recv() {
                                Ok(LspControllerMessage::ClientCreated(client)) => {
                                    lsp_client = Some(client);
                                    break;
                                },
                                Ok(LspControllerMessage::NoClient) => {
                                    lsp_client = None;
                                    break;
                                },
                                Ok(_) => {},
                                Err(TryRecvError::Empty) => {
                                    continue;
                                },
                                Err(TryRecvError::Disconnected) => {
                                    unreachable!();
                                }
                            }
                        }

                        let lsp_info = LspInfo {
                            lsp_channels,
                            lsp_client,
                            first_pass: true,
                        };



                        (Some("rust".to_string()), buffer, lsp_info)
                    },
                    "c" | "h" => {
                        let language = tree_sitter_c::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        lsp_channels.0.send(LspControllerMessage::CreateClient(String::from("c").into())).unwrap();

                        let lsp_client;

                        loop {
                            match lsp_channels.1.try_recv() {
                                Ok(LspControllerMessage::ClientCreated(client)) => {
                                    lsp_client = Some(client);
                                    break;
                                },
                                Ok(LspControllerMessage::NoClient) => {
                                    lsp_client = None;
                                    break;
                                },
                                Ok(_) => {},
                                Err(TryRecvError::Empty) => {
                                    continue;
                                },
                                Err(TryRecvError::Disconnected) => {
                                    unreachable!();
                                }
                            }
                        }

                        let lsp_info = LspInfo {
                            lsp_channels,
                            lsp_client,
                            first_pass: true,
                        };

                        (Some("c".to_string()), buffer, lsp_info)
                    },
                    "cpp" | "hpp" => {
                        let language = tree_sitter_cpp::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);


                        lsp_channels.0.send(LspControllerMessage::CreateClient(String::from("cpp").into())).unwrap();

                        let lsp_client;

                        loop {
                            match lsp_channels.1.try_recv() {
                                Ok(LspControllerMessage::ClientCreated(client)) => {
                                    lsp_client = Some(client);
                                    break;
                                },
                                Ok(LspControllerMessage::NoClient) => {
                                    lsp_client = None;
                                    break;
                                },
                                Ok(_) => {},
                                Err(TryRecvError::Empty) => {
                                    continue;
                                },
                                Err(TryRecvError::Disconnected) => {
                                    unreachable!();
                                }
                            }
                        }

                        let lsp_info = LspInfo {
                            lsp_channels,
                            lsp_client,
                            first_pass: true,
                        };

                        (Some("cpp".to_string()), buffer, lsp_info)
                    },
                    "py" => {
                        let language = tree_sitter_python::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        lsp_channels.0.send(LspControllerMessage::CreateClient(String::from("python").into())).unwrap();

                        let lsp_client;

                        loop {
                            match lsp_channels.1.try_recv() {
                                Ok(LspControllerMessage::ClientCreated(client)) => {
                                    lsp_client = Some(client);
                                    break;
                                },
                                Ok(LspControllerMessage::NoClient) => {
                                    lsp_client = None;
                                    break;
                                },
                                Ok(_) => {},
                                Err(TryRecvError::Empty) => {
                                    continue;
                                },
                                Err(TryRecvError::Disconnected) => {
                                    unreachable!();
                                }
                            }
                        }

                        let lsp_info = LspInfo {
                            lsp_channels,
                            lsp_client,
                            first_pass: true,
                        };

                        (Some("python".to_string()), buffer, lsp_info)
                    },
                    "lsp" => {
                        let language = tree_sitter_commonlisp::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        lsp_channels.0.send(LspControllerMessage::CreateClient(String::from("commonlisp").into())).unwrap();

                        let lsp_client;

                        loop {
                            match lsp_channels.1.try_recv() {
                                Ok(LspControllerMessage::ClientCreated(client)) => {
                                    lsp_client = Some(client);
                                    break;
                                },
                                Ok(LspControllerMessage::NoClient) => {
                                    lsp_client = None;
                                    break;
                                },
                                Ok(_) => {},
                                Err(TryRecvError::Empty) => {
                                    continue;
                                },
                                Err(TryRecvError::Disconnected) => {
                                    unreachable!();
                                }
                            }
                        }

                        let lsp_info = LspInfo {
                            lsp_channels,
                            lsp_client,
                            first_pass: true,
                        };

                        (Some("commonlisp".to_string()), buffer, lsp_info)
                    },
                    "swift" => {
                        let language = tree_sitter_swift::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        lsp_channels.0.send(LspControllerMessage::CreateClient(String::from("swift").into())).unwrap();

                        let lsp_client;

                        loop {
                            match lsp_channels.1.try_recv() {
                                Ok(LspControllerMessage::ClientCreated(client)) => {
                                    lsp_client = Some(client);
                                    break;
                                },
                                Ok(LspControllerMessage::NoClient) => {
                                    lsp_client = None;
                                    break;
                                },
                                Ok(_) => {},
                                Err(TryRecvError::Empty) => {
                                    continue;
                                },
                                Err(TryRecvError::Disconnected) => {
                                    unreachable!();
                                }
                            }
                        }

                        let lsp_info = LspInfo {
                            lsp_channels,
                            lsp_client,
                            first_pass: true,
                        };


                        (Some("swift".to_string()), buffer, lsp_info)
                    },
                    "go" => {
                        let language = tree_sitter_go::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        lsp_channels.0.send(LspControllerMessage::CreateClient(String::from("go").into())).unwrap();

                        let lsp_client;

                        loop {
                            match lsp_channels.1.try_recv() {
                                Ok(LspControllerMessage::ClientCreated(client)) => {
                                    lsp_client = Some(client);
                                    break;
                                },
                                Ok(LspControllerMessage::NoClient) => {
                                    lsp_client = None;
                                    break;
                                },
                                Ok(_) => {},
                                Err(TryRecvError::Empty) => {
                                    continue;
                                },
                                Err(TryRecvError::Disconnected) => {
                                    unreachable!();
                                }
                            }
                        }

                        let lsp_info = LspInfo {
                            lsp_channels,
                            lsp_client,
                            first_pass: true,
                        };


                        (Some("go".to_string()), buffer, lsp_info)
                    },
                    "sh" => {
                        let language = tree_sitter_bash::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        lsp_channels.0.send(LspControllerMessage::CreateClient(String::from("bash").into())).unwrap();

                        let lsp_client;

                        loop {
                            match lsp_channels.1.try_recv() {
                                Ok(LspControllerMessage::ClientCreated(client)) => {
                                    lsp_client = Some(client);
                                    break;
                                },
                                Ok(LspControllerMessage::NoClient) => {
                                    lsp_client = None;
                                    break;
                                },
                                Ok(_) => {},
                                Err(TryRecvError::Empty) => {
                                    continue;
                                },
                                Err(TryRecvError::Disconnected) => {
                                    unreachable!();
                                }
                            }
                        }

                        let lsp_info = LspInfo {
                            lsp_channels,
                            lsp_client,
                            first_pass: true,
                        };


                        (Some("bash".to_string()), buffer, lsp_info)
                    },
                    "js" => {
                        let language = tree_sitter_javascript::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        lsp_channels.0.send(LspControllerMessage::CreateClient(String::from("javascript").into())).unwrap();

                        let lsp_client;

                        loop {
                            match lsp_channels.1.try_recv() {
                                Ok(LspControllerMessage::ClientCreated(client)) => {
                                    lsp_client = Some(client);
                                    break;
                                },
                                Ok(LspControllerMessage::NoClient) => {
                                    lsp_client = None;
                                    break;
                                },
                                Ok(_) => {},
                                Err(TryRecvError::Empty) => {
                                    continue;
                                },
                                Err(TryRecvError::Disconnected) => {
                                    unreachable!();
                                }
                            }
                        }

                        let lsp_info = LspInfo {
                            lsp_channels,
                            lsp_client,
                            first_pass: true,
                        };


                        (Some("javascript".to_string()), buffer, lsp_info)
                    },
                    "cs" => {
                        let language = tree_sitter_c_sharp::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        lsp_channels.0.send(LspControllerMessage::CreateClient(String::from("c#").into())).unwrap();

                        let lsp_client;

                        loop {
                            match lsp_channels.1.try_recv() {
                                Ok(LspControllerMessage::ClientCreated(client)) => {
                                    lsp_client = Some(client);
                                    break;
                                },
                                Ok(LspControllerMessage::NoClient) => {
                                    lsp_client = None;
                                    break;
                                },
                                Ok(_) => {},
                                Err(TryRecvError::Empty) => {
                                    continue;
                                },
                                Err(TryRecvError::Disconnected) => {
                                    unreachable!();
                                }
                            }
                        }

                        let lsp_info = LspInfo {
                            lsp_channels,
                            lsp_client,
                            first_pass: true,
                        };

                        (Some("csharp".to_string()), buffer, lsp_info)
                    },
                    "txt" | _ => {
                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        let lsp_info = LspInfo {
                            lsp_channels,
                            lsp_client: None,
                            first_pass: true,
                        };

                        (None, buffer, lsp_info)
                    }
                };

                buffer.add_new_rope();
                buffer.add_new_rope();



                let file = Self {
                    path: Some(path),
                    buffer,
                    lsp_info,
                    language,
                    settings,
                    highlights: BTreeSet::new(),
                    saved: true,
                    safe_close: false,
                    syntax_highlights: HashMap::new(),
                };



                match file.lsp_info.lsp_client {
                    Some(_) => {
                        let uri = file.generate_uri();
                        let text = file.buffer.to_string().into_boxed_str();

                        let notification = LspNotification::Open(uri.clone().into(), text);

                        let language = file.language.clone().unwrap().into_boxed_str();
                        let message = LspControllerMessage::Notification(language, notification);

                        file.lsp_info.lsp_channels.0.send(message).unwrap();

                        let request = LspRequest::SemanticTokens(uri.into());

                        let message = LspControllerMessage::Request(file.language.clone().unwrap().into(), request);

                        file.lsp_info.lsp_channels.0.send(message).unwrap();
                    },
                    None => {},
                }

                if recovered_file {
                    Err(FileError::RecoverFileFound(file))
                } else {
                    return Ok(file);
                }
            }
            None => {
                let mut buffer = Buffer::new(settings.clone());
                buffer.add_new_rope();
                buffer.add_new_rope();

                Ok(Self {
                    path: None,
                    buffer,
                    lsp_info: LspInfo {
                        lsp_channels,
                        lsp_client: None,
                        first_pass: true,
                    },
                    language: None,
                    settings,
                    highlights: BTreeSet::new(),
                    saved: true,
                    safe_close: false,
                    syntax_highlights: HashMap::new(),
                })
            }
        }
    }

    pub fn set_safe_close(&mut self) {
        self.safe_close = true;
    }
    pub fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    pub fn save(&mut self, file_path: Option<PathBuf>, force: bool) -> Result<(), String> {
        match file_path {
            Some(path) => {
                match &mut self.path {
                    Some(path) => {
                        if path.is_dir() {
                            return Err("Cannot save over a directory".to_string());
                        }
                        if path.is_file() && !force {
                            return Err("File already exists".to_string());
                        }

                        self.buffer.save(&path);
                        self.saved = true;
                    }
                    None => {
                        if path.is_dir() {
                            return Err("Cannot save over a directory".to_string());
                        }
                        if path.is_file() && !force {
                            return Err("File already exists".to_string());
                        }

                        self.buffer.save(&path);
                        self.saved = true;
                    }
                }
                self.path = Some(path);
                return Ok(());
            }
            None => {
                if let Some(path) = &self.path {
                    self.buffer.save(path);
                    self.saved = true;
                    return Ok(());
                } else {
                    return Err("No file path bound to file".to_string());
                }
            }
        }
    }

    pub fn get_settings(&self) -> Rc<RefCell<Settings>> {
        self.settings.clone()
    }

    pub fn get_path(&self) -> Option<PathBuf> {
        self.path.clone()
    }

    pub fn has_saved(&self) -> bool {
        self.saved
    }

    pub fn get_byte(&self, index: usize) -> u8 {
        self.buffer.get_nth_byte(index).expect("Invalid byte index")
    }
    pub fn get_char_at(&self, byte_offset: usize) -> Option<char> {
        self.buffer.get_char_at(byte_offset)
    }

    pub fn get_line(&self, row: usize) -> Option<String> {
        self.buffer.get_row(row).map(|line| line.to_string())
    }

    pub fn get_word(&self, byte_offset: usize) -> Option<String> {
        self.buffer.get_word(byte_offset).map(|word| word.to_string())
    }
    pub fn get_until_next_word(&self, byte_offset: usize) -> Option<String> {
        self.buffer.get_until_next_word(byte_offset).map(|word| word.to_string())
    }

    pub fn get_until_prev_word(&self, byte_offset: usize) -> Option<String> {
        self.buffer.get_until_prev_word(byte_offset).map(|word| word.to_string())
    }

    pub fn get_highlights(&self) -> BTreeSet<usize> {
        self.highlights.clone()
    }

    pub fn get_highlighted(&self) -> Option<Vec<String>> {
        if self.highlights.is_empty() {
            return None;
        }


        let mut output = Vec::new();
        let mut string = String::new();

        let mut iter = self.highlights.iter();
        let byte = iter.next();

        let mut start = *byte.unwrap();
        let mut end = *byte.unwrap();
        let mut last_end = *byte.unwrap();

        while let Some(byte) = iter.next() {
            if *byte == last_end + 1 {
                end = *byte;
                last_end = *byte;
            } else {

                for i in start..=end {
                    string.push(self.buffer.get_nth_byte(i).unwrap() as char);
                }
                output.push(string.clone());
                string.clear();

                start = *byte;
                end = *byte;
                last_end = *byte;
            }

            while let Some(byte) = iter.next() {
                if *byte == last_end + 1 {
                    end = *byte;
                    last_end = *byte;
                } else {

                    for i in start..=end {
                        string.push(self.buffer.get_nth_byte(i).unwrap() as char);
                    }
                    output.push(string.clone());
                    string.clear();

                    start = *byte;
                    end = *byte;
                    last_end = *byte;
                    break
                }

            }
        }
        if start != end {

            for i in start..=end {
                string.push(self.buffer.get_nth_byte(i).unwrap() as char);
            }
            output.push(string.clone());
            string.clear();
        }
        Some(output)
    }

    pub fn delete_highlighted(&mut self) -> usize {
        let mut iter = self.highlights.iter().rev();
        let byte = iter.next();

        let mut ranges = Vec::new();

        let mut start = *byte.unwrap();
        let mut end = *byte.unwrap();
        let mut start_end = *byte.unwrap();

        while let Some(byte) = iter.next() {
            if *byte == start_end - 1 {
                start = *byte;
                start_end = *byte;
            } else {
                //self.buffer.delete(start..=end);
                ranges.push(start..=end);

                start = *byte;
                end = *byte;
                start_end = *byte;
            }

            while let Some(byte) = iter.next() {
                if *byte == start_end - 1 {
                    start = *byte;
                    start_end = *byte;
                } else {
                    //self.buffer.delete(start..=end);
                    ranges.push(start..=end);

                    start = *byte;
                    end = *byte;
                    start_end = *byte;
                }

            }
        }
        if start != end {
            //self.buffer.delete(start..=end);
            ranges.push(start..=end);
        }

        self.buffer.bulk_delete(ranges);

        start
    }

    pub fn get_line_count(&self) -> usize {
        self.buffer.get_line_count()
    }

    pub fn get_row_len(&self, row: usize) -> Option<usize> {
        self.buffer.line_len(row)
    }

    pub fn clear_highlights(&mut self) {
        self.highlights.clear();
    }


    pub fn add_highlight(&mut self, start: usize, end: usize) {
        for i in start..=end {
            self.highlights.insert(i);
        }
    }

    pub fn select_row(&mut self, row: usize) {
        if let Some (line) = self.buffer.get_row(row) {
            let len = line.len();
            let start = self.buffer.get_byte_offset(0, row).unwrap();
            let byte_offset = self.buffer.get_byte_offset(len, row).unwrap();

            self.add_highlight(start, byte_offset);
        }
    }

    pub fn find(&mut self, _col: usize, row: usize, string: &str, down: bool) -> BTreeSet<usize> {
        let range = if down {
            row..self.buffer.get_line_count()
        } else {
            0..row
        };
        for y in range {
            if let Some(line) = self.buffer.get_row(y) {
                let mut line = line.to_string();
                let mut split_bits = 0;
                while let Some(index) = line.find(string) {

                    let index = index + split_bits;


                    let range =  self.buffer.get_byte_offset(index, y)
                        .expect("Invalid byte offset")
                        ..
                        self.buffer.get_byte_offset(index, y)
                            .expect("Invalid byte offset") +
                            string.len();
                    for b in range {
                        self.highlights.insert(b);
                    }
                    split_bits += string.len();
                    line = line.split_off(string.len());
                }
            }
        }
        self.highlights.clone()
    }

    pub fn get_byte_offset(&self, row: usize, col: usize) -> Option<usize> {
        self.buffer.get_byte_offset(col, row)
    }

    pub fn get_cursor(&self, byte_offset: usize) -> Option<(usize, usize)> {
        self.buffer.get_cursor_from_byte_offset(byte_offset)
    }

    pub fn insert_char(&mut self, byte_offset: usize, c: char) {
        self.buffer.insert_current(byte_offset, c.to_string());
        self.saved = false;
    }

    pub fn insert_after_current<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        self.buffer.insert(byte_offset, c);
        self.saved = false;
    }
    pub fn insert_before_current<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        let byte_offset = byte_offset.saturating_sub(1);
        self.buffer.insert(byte_offset, c);
        self.saved = false;
    }

    pub fn insert_after<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        self.buffer.insert(byte_offset, c);
        self.saved = false;
    }

    pub fn insert_before<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        let byte_offset = byte_offset.saturating_sub(1);
        self.buffer.insert(byte_offset, c);
        self.saved = false;
    }

    pub fn delete_current<R>(&mut self, range: R) where R: std::ops::RangeBounds<usize> {
        self.buffer.delete_current(range);
        self.saved = false;
    }
    pub fn delete<R>(&mut self, range: R) where R: std::ops::RangeBounds<usize> {
        self.buffer.delete(range);
        self.saved = false;
    }

    pub fn delete_word(&mut self, byte_offset: usize) -> usize {
        let x = self.buffer.delete_word(byte_offset);
        self.saved = false;
        x
    }
    pub fn delete_line(&mut self, row: usize) {
        self.buffer.delete_line(row);
        self.saved = false;
    }

    pub fn replace_current<R, T>(&mut self, range: R, c: T) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        self.buffer.replace_current(range, c);
        self.saved = false;
    }

    pub fn replace<R, T>(&mut self, range: R, c: T) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        self.buffer.replace(range, c);
        self.saved = false;
    }


    pub fn undo(&mut self) {
        self.buffer.undo();
        self.saved = false;
    }

    pub fn redo(&mut self) {
        self.buffer.redo();
        self.saved = false;
    }

    fn generate_uri(& self) -> String {
        let working_dir = std::env::current_dir().unwrap();
        match &self.path {
            None => format!("untitled://{}", working_dir.display()),
            Some(file_name) => {
                let uri = format!("file://{}/{}", working_dir.display(), file_name.display());
                uri
            },
        }
    }

    pub fn next_word_front(&self, mut byte_position: usize, mut amount: usize) -> usize {
        while amount > 0 {
            byte_position = self.buffer.next_word_front(byte_position);
            amount -= 1;
        }
        byte_position
    }

    pub fn next_word_back(&self, mut byte_position: usize, mut amount: usize) -> usize {


        while amount > 0 {
            byte_position = self.buffer.next_word_back(byte_position);
            amount -= 1;
        }
        byte_position
    }

    pub fn prev_word_front(&self, mut byte_position: usize, mut amount: usize) -> usize {

        while amount > 0 {
            byte_position = self.buffer.prev_word_front(byte_position);
            amount -= 1;
        }
        byte_position
    }

    pub fn prev_word_back(&self, mut byte_position: usize, mut amount: usize) -> usize {

        while amount > 0 {
            byte_position = self.buffer.prev_word_back(byte_position);
            amount -= 1;
        }
        byte_position
    }

    /// This function is where we query our Lsp Channels to see what information we can get
    pub fn refresh(&mut self) -> Result<Option<String>, String> {

        let lsp_client = self.lsp_info.lsp_client.take();

        match &lsp_client {
            None => {},
            Some(client) => {

                if !self.saved || self.lsp_info.first_pass || self.syntax_highlights.is_empty() {
                    self.lsp_info.first_pass = false;
                    let uri = self.generate_uri();
                    let uri = uri.into_boxed_str();

                    let request = LspRequest::SemanticTokens(uri);

                    let message = LspControllerMessage::Request(self.language.clone().unwrap().into(), request);

                    self.lsp_info.lsp_channels.0.send(message).unwrap();
                }

                let message = match client.try_recv() {
                    Ok(message) => message,
                    Err(TryRecvError::Empty) => {
                        self.lsp_info.lsp_client = lsp_client;
                        return Ok(None);
                    },
                    Err(TryRecvError::Disconnected) => {
                        return Err("LSP Client Disconnected".to_string());
                    }
                };

                match message {
                    LspControllerMessage::Response(LspResponse::SemanticTokens(tokens)) => {

                        eprintln!("{:?}", tokens);

                        if !tokens.is_empty() {
                            eprintln!("Got tokens");
                            self.syntax_highlights.clear();
                            for token in tokens.data {
                                let (range, line) = token.generate_range();
                                for i in range {
                                    self.syntax_highlights.insert((i, line), token.token_type.clone());
                                }

                            }

                            return Ok(Some("Syntax Highlighting initialization successful".to_string()));
                        } else {
                            self.lsp_info.first_pass = true;
                        }
                    },
                    _ => {},
                }

            }
        }

        self.lsp_info.lsp_client = lsp_client;

        Ok(None)
    }

    fn is_delimiter(&self, b: usize) -> bool {
        if let Some(c) = self.buffer.get_char_at(b) {
            match c {
                '(' => true,
                ')' => true,
                '[' => true,
                ']' => true,
                '{' => true,
                '}' => true,
                // Despite looking similar, they are different characters
                '｛' => true,
                '｝' => true,
                '（' => true,
                '）' => true,
                '［' => true,
                '］' => true,
                '【' => true,
                '】' => true,
                '「' => true,
                '」' => true,
                '『' => true,
                '』' => true,
                '〝' => true,
                '〞' => true,
                '〈' => true,
                '〉' => true,
                '《' => true,
                '》' => true,
                '〔' => true,
                '〕' => true,
                '〖' => true,
                '〗' => true,
                '〘' => true,
                '〙' => true,
                '〚' => true,
                '〛' => true,
                '«' => true,
                '»' => true,
                '‹' => true,
                '›' => true,
                '"' => true,
                '\'' => true,
                '‘' => true,
                '’' => true,
                '“' => true,
                '”' => true,
                '⁅' => true,
                '⁆' => true,
                '〈' => true,
                '〉' => true,
                '⎡' => true,
                '⎤' => true,
                '⎣' => true,
                '⎦' => true,
                '⎢' => true,
                '⎥' => true,
                '⎧' => true,
                '⎨' => true,
                '⎩' => true,
                '⎫' => true,
                '⎬' => true,
                '⎭' => true,
                '⎪' => true,
                '⎰' => true,
                '⎱' => true,
                '❬' => true,
                '❭' => true,
                '❮' => true,
                '❯' => true,
                '❰' => true,
                '❱' => true,
                '❴' => true,
                '❵' => true,
                '⟦' => true,
                '⟧' => true,
                '⟨' => true,
                '⟩' => true,
                '❲' => true,
                '❳' => true,
                '⦃' => true,
                '⦄' => true,
                '⦅' => true,
                '⦆' => true,
                '⦇' => true,
                '⦈' => true,
                '⦉' => true,
                '⦊' => true,
                '⦋' => true,
                '⦌' => true,
                '⦍' => true,
                '⦎' => true,
                '⦏' => true,
                '⦐' => true,
                '⦑' => true,
                '⦒' => true,
                '⦗' => true,
                '⦘' => true,
                '⧘' => true,
                '⧙' => true,
                '⧚' => true,
                '⧛' => true,
                '⧼' => true,
                '⧽' => true,
                '⸂' => true,
                '⸃' => true,
                '⸄' => true,
                '⸅' => true,
                '⸉' => true,
                '⸊' => true,
                '⸌' => true,
                '⸍' => true,
                '⸜' => true,
                '⸝' => true,
                '⸠' => true,
                '⸡' => true,
                '⸢' => true,
                '⸣' => true,
                '⸤' => true,
                '⸥' => true,
                '⸦' => true,
                '⸧' => true,
                '⸨' => true,
                '⸩' => true,
                //todo: add a way to have the user add more of these
                _ => false,
            }
        } else {
            false
        }
    }
    fn is_pair(left: char, right: char) -> bool {
        match (left, right) {
            ('(', ')') => true,
            ('{', '}') => true,
            ('[', ']') => true,
            ('｛', '｝') => true,
            ('（', '）') => true,
            ('［', '］') => true,
            ('【', '】') => true,
            ('「', '」') => true,
            ('『', '』') => true,
            ('〝', '〞') => true,
            ('〈', '〉') => true,
            ('《', '》') => true,
            ('〔', '〕') => true,
            ('〖', '〗') => true,
            ('〘', '〙') => true,
            ('〚', '〛') => true,
            ('«', '»') => true,
            ('‹', '›') => true,
            ('"', '"') => true,
            ('\'', '\'') => true,
            ('‘', '’') => true,
            ('“', '”') => true,
            ('⁅', '⁆') => true,
            ('〈','〉') => true,
            ('⎡', '⎤') => true,
            ('⎢', '⎥') => true,
            ('⎣', '⎦') => true,
            ('⎧', '⎫') => true,
            ('⎨', '⎬') => true,
            ('⎩', '⎭') => true,
            ('⎪', '⎪') => true,
            ('⎰', '⎱') => true,
            ('⎱', '⎰') => true,
            ('❬','❭') => true,
            ('❮', '❯') => true,
            ('❰', '❱') => true,
            ('❴', '❵') => true,
            ('⟦', '⟧') => true,
            ('⟨', '⟩') => true,
            ('❲', '❳') => true,
            ('⦃', '⦄') => true,
            ('⦅', '⦆') => true,
            ('⦇', '⦈') => true,
            ('⦉', '⦊') => true,
            ('⦋', '⦌') => true,
            ('⦍', '⦎') => true,
            ('⦏', '⦐') => true,
            ('⦑', '⦒') => true,
            ('⦗', '⦘') => true,
            ('⧘', '⧙') => true,
            ('⧚', '⧛') => true,
            ('⧼', '⧽') => true,
            ('⸂', '⸃') => true,
            ('⸄', '⸅') => true,
            ('⸉', '⸊') => true,
            ('⸌', '⸍') => true,
            ('⸜', '⸝') => true,
            ('⸠', '⸡') => true,
            ('⸢', '⸣') => true,
            ('⸤', '⸥') => true,
            ('⸦', '⸧') => true,
            ('⸨', '⸩') => true,
            _ => false,
        }
    }


    fn internal_display(&self, text: String, offset: usize, mut row: usize) -> StyledText {

        let mut rainbow_delimiters = Vec::new();

        let mut skip_counter = 0;

        let string = text;
        let mut acc = String::new();
        let mut output = StyledText::new();
        let mut line = StyledLine::new();
        let mut highlight = false;

        let mut column = 0;

        for (i, _) in string.bytes().enumerate() {
            if skip_counter > 0 {
                skip_counter -= 1;
                continue;
            }
            let i = i + offset;
            column += 1;

            let chr = self.buffer.get_char_at(i).unwrap();

            skip_counter = chr.len_utf8() - 1;

            if self.highlights.contains(&i) && !(self.is_delimiter(i) && self.settings.borrow().editor_settings.rainbow_delimiters){
                if !highlight {
                    line.push(StyledSpan::from(acc.clone()));
                    acc.clear();
                }

                if chr == '\n' {
                    row += 1;
                    column = 0;
                    acc.push(' ');
                    if highlight {
                        let settings = self.settings.borrow();
                        let selection_color = settings.colors.selected;

                        line.push(StyledSpan::styled(acc.clone(),
                                                     selection_color
                        ));
                    } else {
                        line.push(StyledSpan::from(acc.clone()));
                    }
                    output.lines.push(line);
                    line = StyledLine::new();
                    acc.clear();
                } else {
                    if chr == '\t' {
                        let settings = self.settings.borrow();
                        let tab_size = settings.editor_settings.tab_size;
                        for _ in 0..tab_size {
                            acc.push(' ');
                        }
                    } else if chr == '\r' {
                        acc.push(' ');
                    } else {
                        acc.push(chr);
                    }
                }
                highlight = true;

            } else if self.highlights.contains(&i) && self.is_delimiter(i) && self.settings.borrow().editor_settings.rainbow_delimiters {
                let settings = self.settings.clone();
                let settings = settings.borrow();

                if highlight {
                    let selection_color = settings.colors.selected;

                    line.push(StyledSpan::styled(acc.clone(),
                                                 selection_color
                    ));
                    acc.clear();
                } else {
                    line.push(StyledSpan::from(acc.clone()));
                    acc.clear();
                }
                highlight = true;

                let color = settings.colors.rainbow_delimiters[rainbow_delimiters.len() % settings.colors.rainbow_delimiters.len()];

                let color = if rainbow_delimiters.is_empty() {
                    rainbow_delimiters.push((chr, color));
                    acc.push(chr);
                    color

                } else {
                    let last = rainbow_delimiters.last().unwrap();
                    if Self::is_pair(last.0, chr) {
                        let color = rainbow_delimiters.pop().unwrap().1;
                        acc.push(chr);
                        color
                    } else {
                        rainbow_delimiters.push((chr, color));
                        acc.push(chr);
                        color
                    }
                };

                let selection_color = settings.colors.selected;
                let selection_color = selection_color.patch(color);

                line.push(StyledSpan::styled(acc.clone(),
                                             selection_color
                ));
                acc.clear();
            } else if self.is_delimiter(i) && self.settings.borrow().editor_settings.rainbow_delimiters {
                let settings = self.settings.clone();
                let settings = settings.borrow();

                if highlight {
                    let selection_color = settings.colors.selected;

                    line.push(StyledSpan::styled(acc.clone(),
                                                 selection_color
                    ));
                    acc.clear();
                } else {
                    line.push(StyledSpan::from(acc.clone()));
                    acc.clear();
                }
                highlight = false;
                let color = settings.colors.rainbow_delimiters[rainbow_delimiters.len() % settings.colors.rainbow_delimiters.len()];

                let color = if rainbow_delimiters.is_empty() {
                    rainbow_delimiters.push((chr, color));
                    acc.push(chr);
                    color

                } else {
                    let last = rainbow_delimiters.last().unwrap();
                    if Self::is_pair(last.0, chr) {
                        let color = rainbow_delimiters.pop().unwrap().1;
                        acc.push(chr);
                        color
                    } else {
                        rainbow_delimiters.push((chr, color));
                        acc.push(chr);
                        color
                    }
                };

                line.push(StyledSpan::styled(acc.clone(),
                                             color
                ));
                acc.clear();
            } else if self.syntax_highlights.contains_key(&(column, row)) {
                let settings = self.settings.clone();
                let settings = settings.borrow();

                if highlight {
                    let selection_color = settings.colors.selected;

                    line.push(StyledSpan::styled(acc.clone(),
                                                 selection_color
                    ));
                    acc.clear();
                } else {
                    line.push(StyledSpan::from(acc.clone()));
                    acc.clear();
                }

                let token_type = self.syntax_highlights.get(&(column, row)).unwrap();
                acc.push(chr);

                let color = settings.colors.syntax_highlighting[token_type];
                if highlight {
                    let selection_color = settings.colors.selected;

                    let selection_color = selection_color.patch(color);

                    line.push(StyledSpan::styled(acc.clone(),
                                                 selection_color
                    ));
                } else {
                    line.push(StyledSpan::styled(acc.clone(),
                         color
                    ));
                }
                acc.clear();
            } else if chr == '\n' {
                row += 1;
                column = 0;
                if highlight {
                    let settings = self.settings.borrow();
                    let selection_color = settings.colors.selected;

                    line.push(StyledSpan::styled(acc.clone(),
                                                 selection_color
                    ));
                } else {
                    line.push(StyledSpan::from(acc.clone()));
                }
                acc.clear();
                acc.push(' ');
                line.push(StyledSpan::from(acc.clone()));

                output.lines.push(line);
                line = StyledLine::new();
                acc.clear();
            } else {
                if highlight {
                    let settings = self.settings.borrow();
                    let selection_color = settings.colors.selected;

                    line.push(StyledSpan::styled(acc.clone(),
                                                 selection_color
                    ));
                    acc.clear();
                }
                highlight = false;

                if chr == '\t' {
                    let settings = self.settings.borrow();
                    let tab_size = settings.editor_settings.tab_size;
                    for _ in 0..tab_size {
                        acc.push(' ');
                    }
                } else if chr == '\r' {
                    acc.push(' ');
                } else {
                    acc.push(chr);
                }
            }
        }
        if !acc.is_empty() {
            if !highlight {
                line.push(StyledSpan::from(acc.clone()));
                acc.clear();
            } else {
                let settings = self.settings.borrow();
                let selection_color = settings.colors.selected;

                line.push(StyledSpan::styled(acc.clone(),
                                             selection_color
                ));
                acc.clear();
            }
            output.lines.push(line);
        }


        output
    }
    pub fn display(&self) -> StyledText {
        self.internal_display(self.buffer.to_string(), 0, 0)
    }
    /*pub fn display(&self) -> StyledText {
        // TODO: make this use less heap allocations
        let mut rainbow_delimiters = Vec::new();

        let mut skip_counter = 0;

        let string = self.buffer.to_string();
        let mut acc = String::with_capacity(string.len());
        let mut output = StyledText::new();
        let mut line = StyledLine::new();
        //let mut highlight = false;

        for (i, _) in string.bytes().enumerate() {
            if skip_counter > 0 {
                skip_counter -= 1;
                continue;
            }

            let mut rainbow= false;

            let base_color = if self.is_delimiter(i) && self.settings.borrow().editor_settings.rainbow_delimiters {
                let settings = self.settings.clone();
                let settings = settings.borrow();

                let color = settings.colors.rainbow_delimiters[rainbow_delimiters.len() % settings.colors.rainbow_delimiters.len()];

                let chr = self.buffer.get_char_at(i).unwrap();
                rainbow = true;
                if rainbow_delimiters.is_empty() {
                    rainbow_delimiters.push((chr, color));
                    color
                } else {
                    let last = rainbow_delimiters.last().unwrap();
                    if Self::is_pair(last.0, chr) {
                        let color = rainbow_delimiters.pop().unwrap().1;
                        color
                    } else {
                        rainbow_delimiters.push((chr, color));
                        color
                    }
                }
            } else {
                let settings = self.settings.borrow();
                let color = settings.colors.buffer_color;

                color
            };

            let chr = self.buffer.get_char_at(i).unwrap();

            skip_counter = chr.len_utf8() - 1;

            acc.push(chr);

            let color = if self.highlights.contains(&i) {
                let settings = self.settings.clone();
                let settings = settings.borrow();

                let selection_color = settings.colors.selected;


                if rainbow {
                    selection_color.patch(base_color)
                } else {
                    base_color.patch(selection_color)
                }
            } else {
                base_color
            };
            line.push(StyledSpan::styled(acc.clone(),
                                         color
            ));
            acc.clear();
            if chr == '\n' {
                output.lines.push(line);
                line = StyledLine::new();
            }
        }

        output
    }*/

    /*pub fn display(&self) -> StyledText {

        let mut rainbow_delimiters = Vec::new();

        let mut skip_counter = 0;

        let string = self.buffer.to_string();
        let mut acc = String::with_capacity(string.len());
        let mut output = StyledText::new();
        let mut line = StyledLine::new();
        let mut highlight = false;
        for (i, c) in string.bytes().enumerate() {
            if skip_counter > 0 {
                skip_counter -= 1;
                continue;
            }



            if self.is_delimiter(i) {
                if !highlight {
                    line.push(StyledSpan::from(acc.clone()));
                    acc.clear();
                } else {
                    let settings = self.settings.borrow();
                    let selection_color = settings.colors.selected;

                    line.push(StyledSpan::styled(acc.clone(),
                                                 selection_color
                    ));
                    acc.clear();
                }

                let char = self.buffer.get_char_at(i).unwrap();

                let settings = self.settings.clone();
                let settings = settings.borrow();

                let color = settings.colors.rainbow_delimiters[rainbow_delimiters.len() % settings.colors.rainbow_delimiters.len()];


                let mut color = if rainbow_delimiters.is_empty() {
                    rainbow_delimiters.push((char, color));
                    color
                } else {
                    let last = rainbow_delimiters.last().unwrap();
                    if Self::is_pair(last.0, char) {
                        let color = rainbow_delimiters.pop().unwrap().1;
                        color
                    } else {
                        rainbow_delimiters.push((char, color));
                        color
                    }
                };

                acc.push(char);

                let color = if highlight {
                    let mut selection_color = settings.colors.selected;
                    selection_color.patch(color);

                    selection_color
                } else {
                    color
                };

                line.push(StyledSpan::styled(acc.clone(),
                                             color
                ));
                acc.clear();



                skip_counter = char.len_utf8();


            }

            if self.highlights.contains(&i) {
                if !highlight {
                    line.push(StyledSpan::from(acc.clone()));
                    acc.clear();
                }
                highlight = true;
            } else if highlight {

                if highlight {
                    let settings = self.settings.borrow();
                    let selection_color = settings.colors.selected;

                    line.push(StyledSpan::styled(acc.clone(),
                                                 selection_color
                    ));
                    acc.clear();
                }
                highlight = false;
            }
            if c == b'\n' {
                acc.push(' ');
                if highlight {
                    let settings = self.settings.borrow();
                    let selection_color = settings.colors.selected;

                    line.push(StyledSpan::styled(acc.clone(),
                                                 selection_color
                    ));
                } else {
                    line.push(StyledSpan::from(acc.clone()));
                }
                output.lines.push(line);
                line = StyledLine::new();
                acc.clear();
            } else {
                if c == b'\t' {
                    let settings = self.settings.borrow();
                    let tab_size = settings.editor_settings.tab_size;
                    for _ in 0..tab_size {
                        acc.push(' ');
                    }
                } else if c == b'\r' {
                    acc.push(' ');
                } else {
                    acc.push(c as char);
                }
            }
        }
        if !acc.is_empty() {
            if highlight {
                let settings = self.settings.borrow();
                let selection_color = settings.colors.selected;

                line.push(StyledSpan::styled(acc.clone(),
                                             selection_color
                ));
            } else {
                line.push(StyledSpan::from(acc.clone()));
            }
            output.lines.push(line);
        }
        output
    }*/

    pub fn display_section(&self, start_row: usize, end_row: usize) -> StyledText {
        let mut string = String::new();

        for i in start_row..=end_row {
            if let Some(line) = self.buffer.get_row(i) {
                string.push_str(&line.to_string());
            }
        }

        self.internal_display(string, self.buffer.get_byte_offset(0, start_row).unwrap(), start_row)
    }

    pub fn recover(&mut self) -> Result<(), String>{

        let path = match self.path {
            Some(ref path) => path,
            None => return Err(String::from("No path to recover from")),
        };


        let parent = path.parent();
        let file_name = path.file_name().unwrap();
        let file_name = file_name.to_str().unwrap();

        let recover_file_path = match parent {
            Some(parent) => {
                PathBuf::from(format!("{}##{}", parent.display(), file_name))
            }
            None => {
                PathBuf::from(format!("##{}##", file_name))
            }
        };

        let mut string = String::new();
        let mut file = match std::fs::File::open(&recover_file_path) {
            Ok(file) => file,
            Err(_) => return Err(String::from("An error occurred while opening the file")),
        };

        match file.read_to_string(&mut string) {
            Ok(_) => (),
            Err(_) => return Err(String::from("An error occured while reading the file")),
        };

        self.buffer.replace(.., string);


        Ok(())
    }

}

impl ReplaceSelections<&str> for File {
    fn replace_selections(&mut self, selection: &str) {

        let mut ranges = Vec::new();

        let mut iter = self.highlights.iter();
        let byte = iter.next();

        let mut start = *byte.unwrap();
        let mut end = *byte.unwrap();
        let mut last_end = *byte.unwrap();

        while let Some(byte) = iter.next() {
            if *byte == last_end + 1 {
                end = *byte;
                last_end = *byte;
            } else {

                ranges.push(start..=end);

                start = *byte;
                end = *byte;
                last_end = *byte;
            }

            while let Some(byte) = iter.next() {
                if *byte == last_end + 1 {
                    end = *byte;
                    last_end = *byte;
                } else {

                    ranges.push(start..=end);

                    start = *byte;
                    end = *byte;
                    last_end = *byte;
                    break
                }

            }
        }
        if start != end {
            ranges.push(start..=end);
        }

        let strings = vec![selection.to_string(); ranges.len()];

        self.buffer.replace_bulk(ranges, strings);

    }
}

impl ReplaceSelections<Vec<String>> for File {
    fn replace_selections(&mut self, selection: Vec<String>) {

        let mut ranges = Vec::new();

        let mut iter = self.highlights.iter();
        let byte = iter.next();

        let mut start = *byte.unwrap();
        let mut end = *byte.unwrap();
        let mut last_end = *byte.unwrap();

        while let Some(byte) = iter.next() {
            if *byte == last_end + 1 {
                end = *byte;
                last_end = *byte;
            } else {

                ranges.push(start..=end);

                start = *byte;
                end = *byte;
                last_end = *byte;
            }

            while let Some(byte) = iter.next() {
                if *byte == last_end + 1 {
                    end = *byte;
                    last_end = *byte;
                } else {

                    ranges.push(start..=end);

                    start = *byte;
                    end = *byte;
                    last_end = *byte;
                    break
                }

            }
        }
        if start != end {
            ranges.push(start..=end);
        }


        self.buffer.replace_bulk(ranges, selection);

    }
}

impl InsertPairs<(&str, &str)> for File {
    fn insert_pairs(&mut self, pair: (&str, &str)) {
        let mut ranges = Vec::new();

        let mut iter = self.highlights.iter();
        let byte = iter.next();

        let mut start = *byte.unwrap();
        let mut end = *byte.unwrap();
        let mut last_end = *byte.unwrap();

        while let Some(byte) = iter.next() {
            if *byte == last_end + 1 {
                end = *byte;
                last_end = *byte;
            } else {

                ranges.push((start, end));

                start = *byte;
                end = *byte;
                last_end = *byte;
            }

            while let Some(byte) = iter.next() {
                if *byte == last_end + 1 {
                    end = *byte;
                    last_end = *byte;
                } else {

                    ranges.push((start,end));

                    start = *byte;
                    end = *byte;
                    last_end = *byte;
                    break
                }

            }
        }
        if start != end {
            ranges.push((start, end));
        }

        let mut pairs = Vec::new();
        for _ in ranges.iter() {
            pairs.push(pair);
        }

        self.buffer.insert_bulk_pair(ranges, pairs);
    }
}

impl InsertPairs<Vec<(&str, &str)>> for File {
    fn insert_pairs(&mut self, mut pairs: Vec<(&str, &str)>) {

        let mut ranges = Vec::new();

        let mut iter = self.highlights.iter();
        let byte = iter.next();

        let mut start = *byte.unwrap();
        let mut end = *byte.unwrap();
        let mut last_end = *byte.unwrap();

        while let Some(byte) = iter.next() {
            if *byte == last_end + 1 {
                end = *byte;
                last_end = *byte;
            } else {
                ranges.push((start, end));

                start = *byte;
                end = *byte;
                last_end = *byte;
            }

            while let Some(byte) = iter.next() {
                if *byte == last_end + 1 {
                    end = *byte;
                    last_end = *byte;
                } else {
                    ranges.push((start, end));

                    start = *byte;
                    end = *byte;
                    last_end = *byte;
                    break
                }
            }
        }
        if start != end {
            ranges.push((start, end));
        }

        if pairs.len() < ranges.len() {
            pairs.truncate(ranges.len());
        } else if pairs.len() > ranges.len() {
            ranges.truncate(pairs.len());
        }

        self.buffer.insert_bulk_pair(ranges, pairs);
    }
}

impl Drop for File {
    fn drop(&mut self) {
        if !self.saved && !self.safe_close {
            match self.path {
                Some(ref path) => {
                    let filename = path.file_name().unwrap().to_str().unwrap();
                    let parent = path.parent().unwrap().to_str().unwrap();
                    let path = PathBuf::from(format!("{}##{}",parent, filename));
                    let mut file = std::fs::File::create(path).unwrap();
                    file.write_all(self.buffer.to_string().as_bytes()).unwrap();
                }
                None => {
                    // TODO: move this to its own function
                    let file_ext = match self.language {
                        None => String::from("txt"),
                        Some(ref language) => match language.as_str() {
                            "rust" => String::from("rs"),
                            "c" => String::from("c"),
                            "cpp" => String::from("cpp"),
                            "java" => String::from("java"),
                            "python" => String::from("py"),
                            "javascript" => String::from("js"),
                            "html" => String::from("html"),
                            "css" => String::from("css"),
                            "markdown" => String::from("md"),
                            "latex" => String::from("tex"),
                            "toml" => String::from("toml"),
                            "yaml" => String::from("yaml"),
                            "json" => String::from("json"),
                            "csv" => String::from("csv"),
                            "csharp" => String::from("cs"),
                            "haskell" => String::from("hs"),
                            "go" => String::from("go"),
                            "php" => String::from("php"),
                            "kotlin" => String::from("kt"),
                            _ => String::from("txt"),
                        }
                    };

                    let mut path = PathBuf::from("##untitled");
                    let mut number = 1;
                    while path.exists() {
                        path = PathBuf::from(format!("##untitled{}.{}", number, file_ext));
                        number += 1;
                    }
                    let mut file = std::fs::File::create(path).unwrap();
                    file.write_all(self.buffer.to_string().as_bytes()).unwrap();
                }
            }

        }

    }
}