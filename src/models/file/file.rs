use std::cell::RefCell;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::rc::Rc;
use tree_sitter::Parser;
use crate::models::file::buffer::Buffer;
use crate::models::settings::Settings;
use crate::models::style::{Style, StyledLine, StyledSpan, StyledText};
use crate::models::style::color::Color;


pub trait ReplaceSelections<S> {
    fn replace_selections(&mut self, selections: S);
}

pub struct LSPInfo {

}
pub struct File {
    path: Option<PathBuf>,
    language: Option<String>,
    buffer: Buffer,
    lsp_info: Option<LSPInfo>,
    settings: Rc<RefCell<Settings>>,
    highlights: BTreeSet<usize>,
    saved: bool,
}

impl File {
    pub fn new(path: Option<PathBuf>, settings: Rc<RefCell<Settings>>) -> Self {
        match path {
            Some(path) => {
                let string = std::fs::read_to_string(&path).unwrap();

                let file_type = path.extension().and_then(|ext| ext.to_str()).unwrap_or("txt").to_string();

                let (language, mut buffer) = match file_type.as_str() {
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

                        (Some("rust".to_string()), buffer)
                    },
                    "c" | "h" => {
                        let language = tree_sitter_c::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        (Some("c".to_string()), buffer)
                    },
                    "cpp" | "hpp" => {
                        let language = tree_sitter_cpp::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        (Some("cpp".to_string()), buffer)
                    },
                    "py" => {
                        let language = tree_sitter_python::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        (Some("python".to_string()), buffer)
                    },
                    "lsp" => {
                        let language = tree_sitter_commonlisp::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        (Some("commonlisp".to_string()), buffer)
                    },
                    "swift" => {
                        let language = tree_sitter_swift::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        (Some("swift".to_string()), buffer)
                    },
                    "go" => {
                        let language = tree_sitter_go::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        (Some("go".to_string()), buffer)
                    },
                    "sh" => {
                        let language = tree_sitter_bash::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        (Some("bash".to_string()), buffer)
                    },
                    "js" => {
                        let language = tree_sitter_javascript::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        (Some("javascript".to_string()), buffer)
                    },
                    "cs" => {
                        let language = tree_sitter_c_sharp::language();

                        let mut parser = Parser::new();

                        parser.set_language(language).unwrap();

                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        buffer.set_tree_sitter(parser);

                        (Some("csharp".to_string()), buffer)
                    },
                    "txt" | _ => {
                        let mut buffer = Buffer::from(string);
                        buffer.set_settings(settings.clone());

                        (None, buffer)
                    }
                };

                buffer.add_new_rope();
                buffer.add_new_rope();

                let lsp_info = None;

                Self {
                    path: Some(path),
                    buffer,
                    lsp_info,
                    language,
                    settings,
                    highlights: BTreeSet::new(),
                    saved: true,
                }
            }
            None => {
                let mut buffer = Buffer::new(settings.clone());
                buffer.add_new_rope();
                buffer.add_new_rope();

                Self {
                    path: None,
                    buffer,
                    lsp_info: None,
                    language: None,
                    settings,
                    highlights: BTreeSet::new(),
                    saved: true,
                }
            }
        }
    }


    pub fn save(&mut self, file_path: Option<PathBuf>) {
        match file_path {
            Some(path) => {
                match &mut self.path {
                    Some(_) => {
                        self.buffer.save(&path);
                        self.saved = true;
                    }
                    None => {
                        self.buffer.save(&path);
                        self.saved = true;
                    }
                }
                self.path = Some(path);
            }
            None => {
                if let Some(path) = &self.path {
                    self.buffer.save(path);
                    self.saved = true;
                }
                //todo: put in message for not having a bound file path
            }
        }
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

    pub fn display(&self) -> StyledText {
        let string = self.buffer.to_string();
        let mut acc = String::with_capacity(string.len());
        let mut output = StyledText::new();
        let mut line = StyledLine::new();
        let mut highlight = false;
        for (i, c) in string.bytes().enumerate() {
            if self.highlights.contains(&i) {
                if !highlight {
                    line.push(StyledSpan::from(acc.clone()));
                    acc.clear();
                }
                highlight = true;
            } else if highlight {

                if highlight {
                    //TODO: put in a particular style
                    line.push(StyledSpan::styled(acc.clone(),
                                                 Style::default().bg(Color::Magenta)
                    ));
                    acc.clear();
                }
                highlight = false;
            }
            if c == b'\n' {
                if highlight {
                    line.push(StyledSpan::styled(acc.clone(),Style::default()
                        .bg(Color::Magenta)
                    ));
                } else {
                    line.push(StyledSpan::from(acc.clone()));
                }
                output.lines.push(line);
                line = StyledLine::new();
                acc.clear();
            } else {
                acc.push(c as char);
            }
        }
        output
    }

    pub fn display_section(&self, start_row: usize, end_row: usize) -> StyledText {
        let mut string = String::new();

        for i in start_row..=end_row {
            if let Some(line) = self.buffer.get_row(i) {
                string.push_str(&line.to_string());
            }
        }
        let mut acc = String::with_capacity(string.len());
        let mut output = StyledText::new();
        let mut line = StyledLine::new();
        let mut highlight = false;
        for (i, c) in string.chars().enumerate() {
            if self.highlights.contains(&i) {
                highlight = true;
            } else if highlight {
                highlight = false;
                //TODO: put in a particular style
                line.push(StyledSpan::styled(acc.clone(),
                                             Style::default().bg(Color::Magenta)
                ));
                acc.clear();
            }
            if c == '\n' {
                if highlight {
                    line.push(StyledSpan::styled(acc.clone(),Style::default()
                        .bg(Color::Magenta)
                    ));
                } else {
                    line.push(StyledSpan::from(acc.clone()));
                }
                output.lines.push(line);
                line = StyledLine::new();
                acc.clear();
            } else {
                acc.push(c);
            }
        }
        output

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