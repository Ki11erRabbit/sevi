use std::fs::File;
use std::path::PathBuf;
use tree_sitter::Parser;
use crate::models::settings::Settings;
use std::rc::Rc;
use std::cell::RefCell;

use crate::models::style::StyledText;

use self::buffer::Buffer;

pub mod buffer;






pub struct OpenedFile {
    path: PathBuf,
    buffer: buffer::Buffer,
    lsp_info: Option<String>,// TODO: make this take lsp syntax highlighting info
    language: Option<String>,
    settings: Rc<RefCell<Settings>>,
}



impl OpenedFile {
    pub fn new(path: PathBuf, settings: Rc<RefCell<Settings>>) -> Self {
        let file = File::open(&path).unwrap();
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
        }
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

        StyledText::from(self.buffer.to_string())
    }
}














