use std::fs;
use std::path::PathBuf;
use tree_sitter::Parser;
use crate::models::settings::Settings;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use either::Either;

use crate::models::style::{Style, StyledLine, StyledSpan, StyledText};
use crate::models::style::color::Color;

use self::buffer::Buffer;

pub mod buffer;



pub struct File {
    file: Option<Either<UnopenedFile, OpenedFile>>,
    saved: bool,
}

impl File {
    pub fn new(path: Option<PathBuf>, settings: Rc<RefCell<Settings>>) -> Self {
        Self {
            file: match path {
                Some(path) => Some(Either::Right(OpenedFile::new(path, settings.clone()))),
                None => Some(Either::Left(UnopenedFile::new(settings.clone()))),
            },
            saved: true,
        }
    }

    pub fn add_highlight(&mut self, start: usize, end: usize) {
        match self.file.as_mut().unwrap() {
            Either::Left(file) => file.add_highlight(start, end),
            Either::Right(file) => file.add_highlight(start, end),
        }
    }

    pub fn save(&mut self, file_path: Option<PathBuf>) {
        
        let file = self.file.take();

        self.file = match file.unwrap() {
            Either::Left(file) => {

                let file_path = file_path.expect("No file path given to save to");
                let string = file.buffer.to_string();

                fs::write(&file_path, string).unwrap();

                Some(Either::Right(OpenedFile::from((file, file_path))))
            },
            Either::Right(file) => {
                let string = file.buffer.to_string();

                fs::write(&file.path, string).unwrap();
                
                Some(Either::Right(file))
            },
        };
        self.saved = true;
    }

    pub fn get_path(&self) -> Option<PathBuf> {
        match self.file.as_ref().unwrap() {
            Either::Left(_) => None,
            Either::Right(file) => Some(file.path.clone()),
        }
    }

    pub fn has_saved(&self) -> bool {
        self.saved
    }


    pub fn get_byte(&self, byte_offset: usize) -> u8 {
        match self.file.as_ref().unwrap() {
            Either::Left(file) => file.buffer.get_byte(byte_offset),
            Either::Right(file) => file.buffer.get_byte(byte_offset),
        }
    }

    pub fn get_line_count(&self) -> usize {
        match self.file.as_ref().unwrap() {
            Either::Left(file) => file.buffer.get_line_count(),
            Either::Right(file) => file.buffer.get_line_count(),
        }
    }


    pub fn get_row_len(&self, row: usize) -> Option<usize> {
        match self.file.as_ref().unwrap() {
            Either::Left(file) => file.buffer.line_len(row),
            Either::Right(file) => file.buffer.line_len(row),
        }
    }



    pub fn get_byte_offset(&self, row: usize, col: usize) -> Option<usize> {
        match self.file.as_ref().unwrap() {
            Either::Left(file) => file.get_byte_offset(row, col),
            Either::Right(file) => file.get_byte_offset(row, col),
        }
    }

    pub fn insert_after_current<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        match self.file.as_mut().unwrap() {
            Either::Left(file) => file.insert_after_current(byte_offset, c),
            Either::Right(file) => file.insert_after_current(byte_offset, c),
        }
        self.saved = false;
    }
    pub fn insert_before_current<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        let byte_offset = byte_offset.saturating_sub(1);
        match self.file.as_mut().unwrap() {
            Either::Left(file) => file.insert_before_current(byte_offset, c),
            Either::Right(file) => file.insert_before_current(byte_offset, c),
        }
        self.saved = false;
    }

    pub fn insert_after<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        match self.file.as_mut().unwrap() {
            Either::Left(file) => file.insert_after(byte_offset, c),
            Either::Right(file) => file.insert_after(byte_offset, c),
        }
        self.saved = false;
    }

    pub fn insert_before<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        let byte_offset = byte_offset.saturating_sub(1);
        match self.file.as_mut().unwrap() {
            Either::Left(file) => file.insert_before(byte_offset, c),
            Either::Right(file) => file.insert_before(byte_offset, c),
        }
        self.saved = false;
    }

    pub fn delete_current<R>(&mut self, range: R) where R: std::ops::RangeBounds<usize> {
        match self.file.as_mut().unwrap() {
            Either::Left(file) => file.delete_current(range),
            Either::Right(file) => file.delete_current(range),
        }
        self.saved = false;
    }

    pub fn delete<R>(&mut self, range: R) where R: std::ops::RangeBounds<usize> {
        match self.file.as_mut().unwrap() {
            Either::Left(file) => file.delete(range),
            Either::Right(file) => file.delete(range),
        }
        self.saved = false;
    }

    pub fn replace_current<R, T>(&mut self, range: R, c: T) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        match self.file.as_mut().unwrap() {
            Either::Left(file) => file.replace_current(range, c),
            Either::Right(file) => file.replace_current(range, c),
        }
        self.saved = false;
    }

    pub fn replace<R, T>(&mut self, range: R, c: T) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        match self.file.as_mut().unwrap() {
            Either::Left(file) => file.replace(range, c),
            Either::Right(file) => file.replace(range, c),
        }
        self.saved = false;
    }

    pub fn get_name(&self) -> String {
        match self.file.as_ref().unwrap() {
            Either::Left(file) => file.get_name(),
            Either::Right(file) => file.get_name(),
        }
    }

    pub fn display(&self) -> StyledText {
        match self.file.as_ref().unwrap() {
            Either::Left(file) => file.display(),
            Either::Right(file) => file.display(),
        }
    }



}




pub struct UnopenedFile {
    buffer: buffer::Buffer,
    language: Option<String>,
    settings: Rc<RefCell<Settings>>,
    highlights: HashMap<usize, usize>,
}


impl UnopenedFile {
    pub fn new(settings: Rc<RefCell<Settings>>) -> Self {
        Self {
            buffer: buffer::Buffer::new(settings.clone()),
            language: None,
            settings,
            highlights: HashMap::new(),
        }
    }

    pub fn add_highlight(&mut self, start: usize, end: usize) {
        self.highlights.insert(start, end);
    }

    pub fn get_byte_offset(&self, row: usize, col: usize) -> Option<usize> {
        self.buffer.get_byte_offset(row, col)
    }

    pub fn insert_after_current<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        self.buffer.insert_current(byte_offset, c);
    }
    pub fn insert_before_current<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        let byte_offset = byte_offset.saturating_sub(1);
        self.buffer.insert_current(byte_offset, c);
    }

    pub fn insert_after<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        self.buffer.insert(byte_offset, c);
    }

    pub fn insert_before<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        let byte_offset = byte_offset.saturating_sub(1);
        self.buffer.insert(byte_offset, c);
    }

    pub fn delete_current<R>(&mut self, range: R) where R: std::ops::RangeBounds<usize> {
        self.buffer.delete_current(range);
    }

    pub fn delete<R>(&mut self, range: R) where R: std::ops::RangeBounds<usize> {
        self.buffer.delete(range);
    }

    pub fn replace_current<R, T>(&mut self, range: R, c: T) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        self.buffer.replace_current(range, c);
    }

    pub fn replace<R, T>(&mut self, range: R, c: T) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        self.buffer.replace(range, c);
    }

    pub fn get_name(&self) -> String {
        "".to_string()
    }

    pub fn display(&self) -> StyledText {
        let mut output = StyledText::new();
        output.lines.push(StyledLine::from(""));
        let mut line_index = 0;
        let mut prev = 0;
        for (start, end) in &self.highlights {
            let temp = self.buffer.get_slice(prev, *start)
                .expect("Positions were off")
                .to_string();
            if temp.contains('\n') {
                let mut lines = temp.split('\n');
                let first = lines.next().unwrap();
                output.lines[line_index].push(
                    StyledSpan::from(first.to_string())
                );
                for line in lines {
                    line_index += 1;
                    output.lines.push(
                        StyledLine::from(line.to_string())
                    );
                }
            } else {
                output.lines[line_index].push(
                    StyledSpan::from(temp)
                );
            }

            // TODO: style this with a particular style
            output.lines[line_index].push(
                StyledSpan::styled(self.buffer.get_slice(*start, *end)
                                       .expect("Positions were off")
                                       .to_string(),
                                   Style::new()
                                       .bg(Color::Magenta)
                )
            );


            prev = *end;
        }
        let temp = self.buffer.get_slice(prev, self.buffer.get_byte_count())
            .expect("Positions were off")
            .to_string();
        if temp.contains('\n') {
            let mut lines = temp.split('\n');
            let first = lines.next().unwrap();
            output.lines[line_index].push(
                StyledSpan::from(first.to_string())
            );
            for line in lines {
                line_index += 1;
                output.lines.push(
                    StyledLine::from(line.to_string())
                );
            }
        } else {
            output.lines[line_index].push(
                StyledSpan::from(temp)
            );
        }

        output
    }

}





pub struct OpenedFile {
    path: PathBuf,
    buffer: buffer::Buffer,
    lsp_info: Option<String>,// TODO: make this take lsp syntax highlighting info
    language: Option<String>,
    settings: Rc<RefCell<Settings>>,
    highlights: HashMap<usize, usize>,
}



impl OpenedFile {
    pub fn new(path: PathBuf, settings: Rc<RefCell<Settings>>) -> Self {
        let file = fs::File::open(&path).unwrap();
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

        let lsp_info = None;

        Self {
            path,
            buffer,
            lsp_info,
            language,
            settings,
            highlights: HashMap::new()
        }
    }

    pub fn add_highlight(&mut self, start: usize, end: usize) {
        self.highlights.insert(start, end);
    }

    pub fn get_byte_offset(&self, row: usize, col: usize) -> Option<usize> {
        self.buffer.get_byte_offset(row, col)
    }

    pub fn insert_after_current<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        self.buffer.insert_current(byte_offset, c);
    }
    pub fn insert_before_current<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        let byte_offset = byte_offset.saturating_sub(1);
        self.buffer.insert_current(byte_offset, c);
    }

    pub fn insert_after<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        self.buffer.insert(byte_offset, c);
    }

    pub fn insert_before<T>(&mut self, byte_offset: usize, c: T) where T: AsRef<str> {
        let byte_offset = byte_offset.saturating_sub(1);
        self.buffer.insert(byte_offset, c);
    }

    pub fn delete_current<R>(&mut self, range: R) where R: std::ops::RangeBounds<usize> {
        self.buffer.delete_current(range);
    }

    pub fn delete<R>(&mut self, range: R) where R: std::ops::RangeBounds<usize> {
        self.buffer.delete(range);
    }

    pub fn replace_current<R, T>(&mut self, range: R, c: T) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        self.buffer.replace_current(range, c);
    }

    pub fn replace<R, T>(&mut self, range: R, c: T) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        self.buffer.replace(range, c);
    }

    pub fn get_name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }


    pub fn display(&self) -> StyledText {
        let mut output = StyledText::new();
        output.lines.push(StyledLine::from(""));
        let mut line_index = 0;
        let mut prev = 0;
        for (start, end) in &self.highlights {
            let temp = self.buffer.get_slice(prev, *start)
                .expect("Positions were off")
                .to_string();
            if temp.contains('\n') {
                let mut lines = temp.split('\n');
                let first = lines.next().unwrap();
                output.lines[line_index].push(
                    StyledSpan::from(first.to_string())
                );
                for line in lines {
                    line_index += 1;
                    output.lines.push(
                        StyledLine::from(line.to_string())
                    );
                }
            } else {
                output.lines[line_index].push(
                    StyledSpan::from(temp)
                );
            }

            // TODO: style this with a particular style
            output.lines[line_index].push(
                StyledSpan::styled(self.buffer.get_slice(*start, *end)
                    .expect("Positions were off")
                    .to_string(),
                    Style::new()
                        .bg(Color::Magenta)
                )
            );


            prev = *end;
        }
        let temp = self.buffer.get_slice(prev, self.buffer.get_byte_count())
            .expect("Positions were off")
            .to_string();
        if temp.contains('\n') {
            let mut lines = temp.split('\n');
            let first = lines.next().unwrap();
            output.lines[line_index].push(
                StyledSpan::from(first.to_string())
            );
            for line in lines {
                line_index += 1;
                output.lines.push(
                    StyledLine::from(line.to_string())
                );
            }
        } else {
            output.lines[line_index].push(
                StyledSpan::from(temp)
            );
        }

        output
    }
}



impl From<(UnopenedFile, PathBuf)> for OpenedFile {
    fn from((file, path): (UnopenedFile, PathBuf)) -> Self {
        let mut buffer = file.buffer;
        let settings = file.settings;

        buffer.add_new_rope();

        Self {
            path,
            buffer,
            lsp_info: None,
            language: None,
            settings,
            highlights: file.highlights,
        }
    }
}














