use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
use std::io::Write;
use std::ops::{RangeBounds};
use std::path::PathBuf;
use crate::models::settings::Settings;

use tree_sitter;
use crop::{Rope, RopeSlice};



impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Buffer {{ current: {}, history: {:?},  version: {} }}", self.current, self.history,  self.version)
    }
}


pub struct Buffer {
    current: usize,
    history: Vec<Rope>,
    tree_sitter_info: Option<(tree_sitter::Parser, Vec<tree_sitter::Tree>)>,
    settings: Rc<RefCell<Settings>>,
    version: usize,
    num_lines: RefCell<Option<usize>>,
}


impl Buffer {
    pub fn new(settings: Rc<RefCell<Settings>>) -> Self {
        Self {
            current: 0,
            history: vec![Rope::new()],
            tree_sitter_info: None,
            settings,
            version: 0,
            num_lines: RefCell::new(None),
        }
    }

    pub fn set_settings(&mut self, settings: Rc<RefCell<Settings>>) {
        self.settings = settings;
    }

    pub fn get_version(&self) -> usize {
        self.version
    }

    pub fn save(&mut self, file_path: &PathBuf) {
        let file = std::fs::File::create(file_path).unwrap();
        let mut writer = std::io::BufWriter::new(file);
        let buffer = &self.history[self.current];
        writer.write_all(&buffer.bytes().collect::<Vec<_>>().as_slice()).unwrap();
        self.add_new_rope();
    }

    pub fn set_tree_sitter(&mut self, mut parser: tree_sitter::Parser) {
        let mut trees = Vec::new();
        for buffer in self.history.iter() {
            trees.push(parser.parse(&buffer.to_string(), None).unwrap());
        }
        //trees.push(parser.parse(&self.history[self.current].to_string(), None).unwrap());
        //eprintln!("{}", trees[0].root_node().to_sexp());
        self.tree_sitter_info = Some((parser, trees));
    }

    pub fn get_char_at(&self, mut byte_offset: usize) -> Option<char> {
        let current = &self.history[self.current];

        if byte_offset >= current.byte_len() {
            return None;
        }
        let mut total_bytes = 1;

        if current.is_char_boundary(byte_offset) {
            while byte_offset + total_bytes < current.byte_len() && !current.is_char_boundary(byte_offset + total_bytes) {
                total_bytes += 1;
            }
        } else if !current.is_char_boundary(byte_offset) {
            while !current.is_char_boundary(byte_offset) && byte_offset > 0 {
                byte_offset -= 1;
            }
            while byte_offset + total_bytes < current.byte_len() && !current.is_char_boundary(byte_offset + total_bytes) {
                total_bytes += 1;
            }
        }


        let bytes = current.byte_slice(byte_offset..byte_offset + total_bytes);

        let c = bytes.chars().next().unwrap();
        Some(c)
    }

    pub fn get_byte(&self, byte_offset: usize) -> u8 {
        self.history[self.current].byte(byte_offset)
    }

    pub fn undo(&mut self) {
        self.current = self.current.saturating_sub(1);

        self.version += 1;
    }

    pub fn redo(&mut self) {
        if self.current < self.history.len() - 1 {
            self.current += 1;
        }

        self.version += 1;
    }

    pub fn line_len(&self, row: usize) -> Option<usize> {
        self.history[self.current].lines().nth(row).map(|line| line.chars().map(|c| if c == '\t' {
            self.settings.borrow().editor_settings.tab_size as usize
        } else {
            1
        }).sum())
    }

    pub fn get_line_count(&self) -> usize {
        /*let mut num_lines = self.get_line_count() - 1;
        if let Some('\n') = self.history[self.current].chars().last() {
            num_lines += 1;
        }*/

        let num_lines = *self.num_lines.borrow();
        match num_lines {
            None => {
                let num_lines = self.history[self.current].raw_lines().count();
                *self.num_lines.borrow_mut() = Some(num_lines);
                num_lines
            },
            Some(num_lines) => {
                num_lines
            }
        }
    }

    pub fn get_char_count(&self) -> usize {
        self.history[self.current].chars().count()
    }

    pub fn get_byte_count(&self) -> usize {
        self.history[self.current].bytes().count()
    }


    pub fn get_byte_offset(&self, x: usize, y: usize) -> Option<usize> {
        if y >= self.get_line_count() - 1 {
            return Some(self.history[self.current].byte_len());
        }


        let line_byte = self.history[self.current].byte_of_line(y);

        let line = self.history[self.current].line(y);

        let line = line.chars().take(x).collect::<String>();

        let mut x = line.len();
        while x > 0 && !self.history[self.current].is_char_boundary(line_byte + x) {
            x -= 1;
        }

        return Some(line_byte + x);
    }

    fn get_new_rope(&mut self) -> &mut Rope {
        let buffer = self.history[self.current].clone();
        if self.current < self.history.len() - 1 {
            self.history.truncate(self.current + 1);
        }
        
        match self.tree_sitter_info.as_mut() {
            None => {},
            Some((_, trees)) => {
                let tree = trees[self.current].clone();

                if self.current < trees.len() - 1 {
                    trees.truncate(self.current + 1);
                }
                trees.push(tree);
            }
        }

        self.history.push(buffer);
        self.current += 1;
        &mut self.history[self.current]
    }


    pub fn add_new_rope(&mut self) {
        if self.current > 0 {
            if self.history[self.current - 1] == self.history[self.current] {
                return;
            }
        }
        let buffer = self.history[self.current].clone();
        if self.current < self.history.len() - 1 {
            self.history.truncate(self.current + 1);
        }
        
        match self.tree_sitter_info.as_mut() {
            None => {},
            Some((_, trees)) => {
                let tree = trees[self.current].clone();

                if self.current < trees.len() - 1 {
                    trees.truncate(self.current + 1);
                }
                trees.push(tree);
            }
        }


        self.history.push(buffer);
        self.current += 1;
    }

    pub fn insert_current<T>(&mut self, byte_offset: usize, text: T) where T: AsRef<str> {
        if text.as_ref().contains('\n') {
            *self.num_lines.borrow_mut() = None;
        }

        if self.current == 0 {
            self.get_new_rope();
        }
    
        let mut tree_sitter_info = self.tree_sitter_info.take();

        match tree_sitter_info.as_mut() {
            Some((parser, trees)) => {
                
                let line_num = self.history[self.current].line_of_byte(byte_offset);

                let y = line_num;
                let x = byte_offset - self.history[self.current].byte_of_line(y);
                

                self.history[self.current].insert(byte_offset, text.as_ref());

                let end_x = x + text.as_ref().bytes().count();
                let end_y = self.history[self.current].line_of_byte(byte_offset + text.as_ref().bytes().count());


                let edit = tree_sitter::InputEdit {
                    start_byte: byte_offset,
                    old_end_byte: byte_offset,
                    new_end_byte: byte_offset + text.as_ref().len(),
                    start_position: tree_sitter::Point::new(y, x),
                    old_end_position: tree_sitter::Point::new(y, x),
                    new_end_position: tree_sitter::Point::new(end_y, end_x),
                };
                
                trees[self.current].edit(&edit);
                trees[self.current] = parser.parse(&self.history[self.current].to_string(), Some(&trees[self.current])).unwrap();

            }
            None => {
            
                self.history[self.current].insert(byte_offset, text.as_ref());

            },
        }

        self.tree_sitter_info = tree_sitter_info;


        self.version += 1;
    }

    pub fn bulk_delete<R>(&mut self, ranges: Vec<R>) where R: std::ops::RangeBounds<usize> {
        if self.current == 0 {
            self.get_new_rope();
        }

        for range in ranges {
            self.delete_internal(range);
        }
        self.version += 1;
    }

    fn delete_internal<R>(&mut self, range: R) where R: std::ops::RangeBounds<usize> {
        *self.num_lines.borrow_mut() = None;

        let mut tree_sitter_info = self.tree_sitter_info.take();

        match tree_sitter_info.as_mut() {
            Some((parser, trees)) => {

                let start;
                match range.start_bound() {
                    std::ops::Bound::Included(n) => {
                        start = *n;
                    },
                    std::ops::Bound::Excluded(n) => {
                        start = *n + 1;
                    },
                    std::ops::Bound::Unbounded => {
                        start = 0;
                    },
                }

                let end = match range.end_bound() {
                    std::ops::Bound::Included(n) => {
                        *n
                    },
                    std::ops::Bound::Excluded(n) => {
                        *n - 1
                    },
                    std::ops::Bound::Unbounded => {
                        self.history[self.current].byte_len()
                    },
                };

                if self.history[self.current].byte_slice(start..=end).chars().collect::<String>().contains('\n') {
                    *self.num_lines.borrow_mut() = None;
                }

                let line_num = self.history[self.current].line_of_byte(start);

                let y = line_num;
                let x = start - self.history[self.current].byte_of_line(y);

                self.history[self.current].delete(range);

                let end_x = x;
                let end_y = y;

                let edit = tree_sitter::InputEdit {
                    start_byte: start,
                    old_end_byte: start + 1,
                    new_end_byte: start,
                    start_position: tree_sitter::Point::new(y, x),
                    old_end_position: tree_sitter::Point::new(y, x + 1),
                    new_end_position: tree_sitter::Point::new(end_y, end_x),
                };

                trees[self.current].edit(&edit);
                trees[self.current] = parser.parse(&self.history[self.current].to_string(), Some(&trees[self.current])).unwrap();

            },
            None => {
                self.history[self.current].delete(range);
            }
        }

        self.tree_sitter_info = tree_sitter_info;

    }
    pub fn delete_current<R>(&mut self, range: R) where R: std::ops::RangeBounds<usize> {
        if self.current == 0 {
            self.get_new_rope();
        }

        self.delete_internal(range);


        self.version += 1;
    }

    fn replace_internal<R, T>(&mut self, range:R, text: T) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        if text.as_ref().contains('\n') {
            *self.num_lines.borrow_mut() = None;
        }

        let mut tree_sitter_info = self.tree_sitter_info.take();

        match tree_sitter_info.as_mut() {
            Some((parser, trees)) => {

                let start;
                match range.start_bound() {
                    std::ops::Bound::Included(n) => {
                        start = *n;
                    },
                    std::ops::Bound::Excluded(n) => {
                        start = *n + 1;
                    },
                    std::ops::Bound::Unbounded => {
                        start = 0;
                    },
                }
                let end = match range.end_bound() {
                    std::ops::Bound::Included(n) => {
                        *n
                    },
                    std::ops::Bound::Excluded(n) => {
                        *n - 1
                    },
                    std::ops::Bound::Unbounded => {
                        self.history[self.current].byte_len()
                    },
                };

                if self.history[self.current].byte_slice(start..=end).chars().collect::<String>().contains('\n') {
                    *self.num_lines.borrow_mut() = None;
                }


                let line_num = self.history[self.current].line_of_byte(start);

                let y = line_num;
                let x = start - self.history[self.current].byte_of_line(y);


                let end_x = x + text.as_ref().bytes().count();
                let end_y = self.history[self.current].line_of_byte(start + text.as_ref().bytes().count());


                let edit = tree_sitter::InputEdit {
                    start_byte: start,
                    old_end_byte: start + text.as_ref().len(),
                    new_end_byte: start + text.as_ref().len(),
                    start_position: tree_sitter::Point::new(y, x),
                    old_end_position: tree_sitter::Point::new(y, x),
                    new_end_position: tree_sitter::Point::new(end_y, end_x),
                };

                self.history[self.current].replace(range, text.as_ref());

                trees[self.current].edit(&edit);

                trees[self.current] = parser.parse(&self.history[self.current].to_string(), Some(&trees[self.current])).unwrap();

            },
            None => {
                self.history[self.current].replace(range, text.as_ref());
            },

        }

        self.tree_sitter_info = tree_sitter_info;
    }

    pub fn replace_bulk<R, T>(&mut self, mut ranges: Vec<R>, mut texts: Vec<T>) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        self.get_new_rope();

        if ranges.len() < texts.len() {
            texts.truncate(ranges.len());
        } else if ranges.len() > texts.len() {
            ranges.truncate(texts.len());
        }


        for (range, text) in ranges.iter().rev().zip(texts.iter().rev()) {
            let start;
            match range.start_bound() {
                std::ops::Bound::Included(n) => {
                    start = *n;
                },
                std::ops::Bound::Excluded(n) => {
                    start = *n + 1;
                },
                std::ops::Bound::Unbounded => {
                    start = 0;
                },
            }

            let end;
            match range.end_bound() {
                std::ops::Bound::Included(n) => {
                    end = *n;
                },
                std::ops::Bound::Excluded(n) => {
                    end = *n - 1;
                },
                std::ops::Bound::Unbounded => {
                    end = self.history[self.current].byte_len();
                },
            }
            if end - start < text.as_ref().len() {
                let range = start..=(start + end - start);
                self.replace_internal(range.clone(), text.as_ref()[..(end - start)].to_string());

                match range.end_bound() {
                    std::ops::Bound::Included(n) => {
                        self.insert_current(*n, &text.as_ref()[(end - start)..]);
                    },
                    std::ops::Bound::Excluded(n) => {
                        self.insert_current(*n, &text.as_ref()[(end - start)..]);
                    },
                    std::ops::Bound::Unbounded => {
                        self.insert_current(self.history[self.current].byte_len(), &text.as_ref()[(end - start)..]);
                    },
                }
            } else {
                let range = start..=end;
                self.delete_internal(range.clone());
                match range.start_bound() {
                    std::ops::Bound::Included(n) => {
                        self.insert_current(*n, text.as_ref());
                    },
                    std::ops::Bound::Excluded(n) => {
                        self.insert_current(*n, text.as_ref());
                    },
                    std::ops::Bound::Unbounded => {
                        self.insert_current(self.history[self.current].byte_len(), text.as_ref());
                    },
                }
            }
        }

        self.version += 1;
    }

    pub fn replace_current<R, T>(&mut self, range: R, text: T) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        if self.current == 0 {
            self.get_new_rope();
        }

        let start;
        match range.start_bound() {
            std::ops::Bound::Included(n) => {
                start = *n;
            },
            std::ops::Bound::Excluded(n) => {
                start = *n + 1;
            },
            std::ops::Bound::Unbounded => {
                start = 0;
            },
        }

        let end;
        match range.end_bound() {
            std::ops::Bound::Included(n) => {
                end = *n;
            },
            std::ops::Bound::Excluded(n) => {
                end = *n - 1;
            },
            std::ops::Bound::Unbounded => {
                end = self.history[self.current].byte_len();
            },
        }
        if end - start < text.as_ref().len() {
            let range = start..(start + end - start);
            self.replace_internal(range.clone(), text.as_ref()[..(end - start)].to_string());

            match range.end_bound() {
                std::ops::Bound::Included(n) => {
                    self.insert_current(*n, &text.as_ref()[(end - start)..]);
                },
                std::ops::Bound::Excluded(n) => {
                    self.insert_current(*n, &text.as_ref()[(end - start)..]);
                },
                std::ops::Bound::Unbounded => {
                    self.insert_current(self.history[self.current].byte_len(), &text.as_ref()[(end - start)..]);
                },
            }
        }



        self.version += 1;
    }

    pub fn insert<T>(&mut self, byte_offset: usize, text: T) where T: AsRef<str> {
        if text.as_ref().contains('\n') {
            *self.num_lines.borrow_mut() = None;
        }
        self.get_new_rope();
        
        let mut tree_sitter_info = self.tree_sitter_info.take();

        match tree_sitter_info.as_mut() {
            None => {
                self.history[self.current].insert(byte_offset, text.as_ref());
            },
            Some((parser, trees)) => {
                
                let line_num = self.history[self.current].line_of_byte(byte_offset);

                let y = line_num;
                let x = byte_offset - self.history[self.current].byte_of_line(y);
                

                self.history[self.current].insert(byte_offset, text.as_ref());

                let end_x = x + text.as_ref().bytes().count();
                let end_y = self.history[self.current].line_of_byte(byte_offset + text.as_ref().bytes().count());

                let edit = tree_sitter::InputEdit {
                    start_byte: byte_offset,
                    old_end_byte: byte_offset,
                    new_end_byte: byte_offset + text.as_ref().len(),
                    start_position: tree_sitter::Point::new(y, x),
                    old_end_position: tree_sitter::Point::new(y, x),
                    new_end_position: tree_sitter::Point::new(end_y, end_x),
                };

                trees[self.current].edit(&edit);
                trees[self.current] = parser.parse(&self.history[self.current].to_string(), Some(&trees[self.current])).unwrap();
            }
        }

        
        self.tree_sitter_info = tree_sitter_info;

        self.version += 1;
    }

    pub fn delete<R>(&mut self, range: R) where R: std::ops::RangeBounds<usize> {
        self.get_new_rope();

        self.delete_internal(range);

        self.version += 1;
    }

    /// The return value is the byte offset of the start of the deleted word
    pub fn delete_word(&mut self, byte_offset: usize) -> usize {
        self.get_new_rope();

        let mut tree_sitter_info = self.tree_sitter_info.take();


        let out = match tree_sitter_info.as_mut() {
            None => {
                let mut current = self.get_char_at(byte_offset);
                let mut start = byte_offset;
                while let Some(c) = current {
                    if c.is_alphanumeric() || c == '_' {
                        start = start.saturating_sub(1);
                    } else {
                        start += 1;
                        break;
                    }
                    current = self.get_char_at(start);
                }

                current = self.get_char_at(byte_offset);
                let mut end = byte_offset;
                while let Some(c) = current {
                    if c.is_alphanumeric() || c == '_' {
                        end += 1;
                    } else {
                        break;
                    }
                    current = self.get_char_at(end);
                }
                self.history[self.current].delete(start..end);
                start
            },
            Some((parser, trees)) => {

                let mut current = self.get_char_at(byte_offset);
                let mut start = byte_offset;
                while let Some(c) = current {
                    if c.is_alphanumeric() || c == '_' {
                        start = start.saturating_sub(1);
                    } else {
                        start += 1;
                        break;
                    }
                    current = self.get_char_at(start);
                }

                current = self.get_char_at(byte_offset);
                let mut end = byte_offset;
                while let Some(c) = current {
                    if c.is_alphanumeric() || c == '_' {
                        end += 1;
                    } else {
                        break;
                    }
                    current = self.get_char_at(end);
                }
                self.history[self.current].delete(start..end);

                let line_num = self.history[self.current].line_of_byte(byte_offset);

                let y = line_num;
                let x = byte_offset - self.history[self.current].byte_of_line(y);

                let end_x = x;
                let end_y = y;

                let edit = tree_sitter::InputEdit {
                    start_byte: start,
                    old_end_byte: start,
                    new_end_byte: end,
                    start_position: tree_sitter::Point::new(y, x),
                    old_end_position: tree_sitter::Point::new(y, x),
                    new_end_position: tree_sitter::Point::new(end_y, end_x),
                };

                trees[self.current].edit(&edit);
                trees[self.current] = parser.parse(&self.history[self.current].to_string(), Some(&trees[self.current])).unwrap();
                start
            }
        };

        self.tree_sitter_info = tree_sitter_info;

        self.version += 1;
        out
    }
    pub fn delete_line(&mut self, row: usize) {
        *self.num_lines.borrow_mut() = None;
        self.get_new_rope();

        let mut tree_sitter_info = self.tree_sitter_info.take();

        match tree_sitter_info.as_mut() {
            None => {
                let line_byte = self.history[self.current].byte_of_line(row);
                let line_len = self.get_line_count() - 1;
                let next_line_byte = if row + 1 < line_len {
                    self.history[self.current].byte_of_line(row + 1)
                } else {
                    self.history[self.current].byte_len()
                };
                self.history[self.current].delete(line_byte..next_line_byte);
            },
            Some((parser, trees)) => {

                let line_byte = self.history[self.current].byte_of_line(row);
                let line_len = self.get_line_count() - 1;
                let next_line_byte = if row + 1 < line_len {
                    self.history[self.current].byte_of_line(row + 1)
                } else {
                    self.history[self.current].byte_len()
                };
                self.history[self.current].delete(line_byte..next_line_byte);

                let y = row;
                let x = 0;

                let end_x = x;
                let end_y = y;

                let edit = tree_sitter::InputEdit {
                    start_byte: line_byte,
                    old_end_byte: next_line_byte,
                    new_end_byte: line_byte,
                    start_position: tree_sitter::Point::new(y, x),
                    old_end_position: tree_sitter::Point::new(y, x),
                    new_end_position: tree_sitter::Point::new(end_y, end_x),
                };

                trees[self.current].edit(&edit);
                trees[self.current] = parser.parse(&self.history[self.current].to_string(), Some(&trees[self.current])).unwrap();
            }
        }

        self.tree_sitter_info = tree_sitter_info;

        self.version += 1;
    }

    pub fn replace<R, T>(&mut self, range: R, text: T) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        self.get_new_rope();

        let start;
        match range.start_bound() {
            std::ops::Bound::Included(n) => {
                start = *n;
            },
            std::ops::Bound::Excluded(n) => {
                start = *n + 1;
            },
            std::ops::Bound::Unbounded => {
                start = 0;
            },
        }

        let end;
        match range.end_bound() {
            std::ops::Bound::Included(n) => {
                end = *n;
            },
            std::ops::Bound::Excluded(n) => {
                end = *n - 1;
            },
            std::ops::Bound::Unbounded => {
                end = self.history[self.current].byte_len();
            },
        }
        if end - start < text.as_ref().len() {
            let range = start..(start + end - start);
            self.replace_internal(range.clone(), text.as_ref()[..(end - start)].to_string());

            match range.end_bound() {
                std::ops::Bound::Included(n) => {
                    self.insert_current(*n, &text.as_ref()[(end - start)..]);
                },
                std::ops::Bound::Excluded(n) => {
                    self.insert_current(*n, &text.as_ref()[(end - start)..]);
                },
                std::ops::Bound::Unbounded => {
                    self.insert_current(self.history[self.current].byte_len(), &text.as_ref()[(end - start)..]);
                },
            }
        }

        self.version += 1;
    }

    pub fn get_nth_byte(&self, n: usize) -> Option<u8> {
        self.history[self.current].bytes().nth(n)
    }

    pub fn get_nth_char(&self, n: usize) -> Option<char> {
        self.history[self.current].chars().nth(n)
    }

    pub fn insert_chain<T>(&mut self, values: Vec<(usize, T)>)
        where T: AsRef<str>
    {
        *self.num_lines.borrow_mut() = None;
        let buffer = self.get_new_rope();
        for (offset, text) in values.iter().rev() {
            buffer.insert(*offset, text.as_ref());
        }
    }

    pub fn delete_chain<R>(&mut self, values: Box<[R]>)
        where R: std::ops::RangeBounds<usize> + Copy
    {
        *self.num_lines.borrow_mut() = None;
        let buffer = self.get_new_rope();
        for range in values.iter().rev() {
            buffer.delete(*range);
        }
    }

    pub fn replace_chain<R, T>(&mut self, values: Box<[(R, T)]>)
        where R: std::ops::RangeBounds<usize> + Copy, T: AsRef<str>
    {
        *self.num_lines.borrow_mut() = None;
        let buffer = self.get_new_rope();
        for (range, text) in values.iter().rev() {
            buffer.replace(*range, text.as_ref());
        }
    }

    pub fn insert_pair<T>(&mut self, start: usize, end: usize, text: (T, T)) where T: AsRef<str> {
        self.get_new_rope();
        self.history[self.current].insert(end + 1, text.1);
        self.history[self.current].insert(start, text.0);
    }

    pub fn insert_bulk_pair<T>(&mut self, ranges: Vec<(usize, usize)>, texts: Vec<(T, T)>) where T: AsRef<str> {
        self.get_new_rope();
        for (range, text) in ranges.iter().rev().zip(texts.iter().rev()) {
            self.history[self.current].insert(range.1 + 1, &text.1);
            self.history[self.current].insert(range.0, &text.0);
        }
    }

    pub fn get_version_count(&self) -> usize {
        self.history.len()
    }


    pub fn get_row(&self, row: usize) -> Option<BufferSlice> {
        if row >= self.get_line_count() {
            return None;
        }

        let line = self.history[self.current].line_slice(row..row + 1);
        Some(BufferSlice::new(line, self.settings.clone()))
    }

    pub fn get_row_special(&self, row: usize, col_offset: usize, cols: usize) -> Option<BufferSlice> {
        
        if row >= self.get_line_count() - 1 {
            return None;
        }
        let line = self.history[self.current].line(row);

        let len = if cols + col_offset > line.chars().count() {
            line.bytes().count()
        } else if cols + col_offset == line.chars().count() {
            line.bytes().count() - 1
        } else {
            cols + col_offset
        };
            
        //cmp::min(cols + col_offset, line.bytes().count());

        if col_offset > len {
            return None;
        }
        Some(BufferSlice::new(line.byte_slice(col_offset..len), self.settings.clone()))

    }

    pub fn get_slice(&self, start: usize, end: usize) -> Option<BufferSlice> {
        if start > end {
            return None;
        }
        if end > self.history[self.current].bytes().count() {
            return None;
        }
        Some(BufferSlice::new(self.history[self.current].byte_slice(start..end), self.settings.clone()))
    }

    pub fn get_word(&self, byte_offset: usize) -> Option<BufferSlice> {

        if byte_offset >= self.history[self.current].bytes().count() {
            return None;
        }

        if !self.get_char_at(byte_offset).unwrap().is_alphanumeric() {
            return None;
        }

        let mut start = byte_offset;
        let mut end = byte_offset;

        let mut current = self.get_char_at(byte_offset);
        while let Some(c) = current {
            if c.is_alphanumeric() || c == '_' {
                start = start.saturating_sub(1);
            } else {
                break;
            }
            current = self.get_char_at(start);
        }

        current = self.get_char_at(byte_offset);
        while let Some(c) = current {
            if c.is_alphanumeric() || c == '_' {
                end += 1;
            } else {
                break;
            }
            current = self.get_char_at(end);
        }

        if start == end {
            return None;
        }

        Some(BufferSlice::new(self.history[self.current].byte_slice(start..end), self.settings.clone()))
    }

    pub fn get_until_next_word(&self, byte_offset: usize) -> Option<BufferSlice> {
        if byte_offset >= self.history[self.current].bytes().count() {
            return None;
        }

        let mut end = byte_offset;


        let mut current = self.get_char_at(byte_offset);

        let mut found_break = false;
        while let Some(c) = current {
            if !c.is_alphanumeric() && c != '_' {
                end += 1;
                if found_break {
                    break;
                }
            } else {
                found_break = true;
                end += 1;
            }
            current = self.get_char_at(end);
        }
        Some(BufferSlice::new(self.history[self.current].byte_slice(byte_offset..end), self.settings.clone()))
    }

    pub fn get_until_prev_word(&self, byte_offset: usize) -> Option<BufferSlice> {
        if byte_offset >= self.history[self.current].bytes().count() {
            return None;
        }

        let mut start = byte_offset;

        let mut current = self.get_char_at(byte_offset);

        let mut found_break = false;

        while let Some(c) = current {
            if !c.is_alphanumeric() && c != '_' {
                if found_break {
                    break;
                }
                start = start.saturating_sub(1);
            } else {
                found_break = true;
                start = start.saturating_sub(1);
            }
            current = self.get_char_at(start);
        }

        Some(BufferSlice::new(self.history[self.current].byte_slice(start..byte_offset), self.settings.clone()))
    }

    pub fn get_cursor_from_byte_offset(&self, byte_offset: usize) -> Option<(usize, usize)> {
        if byte_offset >= self.history[self.current].bytes().count() {
            return None;
        }

        let line_num = self.history[self.current].line_of_byte(byte_offset);

        let y = line_num;
        let mut x = byte_offset - self.history[self.current].byte_of_line(y);

        while !self.history[self.current].is_char_boundary(x) {
            x -= 1;
        }

        Some((x, y))
    }

    pub fn next_word_front(&self, mut byte_position: usize) -> usize {

        while !self.history[self.current].is_char_boundary(byte_position) {
            byte_position -= 1;
        }

        let mut current = self.get_char_at(byte_position);
        let mut start = byte_position;
        if let Some(c) = current {
            if !(c.is_alphanumeric() || c == '_') {
                start += c.len_utf8();
                current = self.get_char_at(start);
            }
        }

        while let Some(c) = current {
            if !(c.is_alphanumeric() || c == '_') {
                start += c.len_utf8();
                //current = self.get_char_at(start);
                break
            } else {
                start += c.len_utf8();
            }
            current = self.get_char_at(start);
        }
        start
    }

    pub fn prev_word_front(&self, byte_position: usize) -> usize {

        let mut current = self.get_char_at(byte_position);
        let mut start = byte_position;
        if let Some(c) = current {
            if !(c.is_alphanumeric() || c == '_') {
                start = start.saturating_sub(c.len_utf8());
                current = self.get_char_at(start);
            }
        }

        while let Some(c) = current {
            if !(c.is_alphanumeric() || c == '_') {
                start = start.saturating_sub(c.len_utf8());
                current = self.get_char_at(start);
                break
            } else {
                start = start.saturating_sub(c.len_utf8());
            }
            current = self.get_char_at(start);
        }
        while let Some(c) = current {
            if c.is_alphanumeric() || c == '_' {
                start = start.saturating_sub(c.len_utf8());
            } else {
                start = start.saturating_add(c.len_utf8());
                break;
            }
            current = self.get_char_at(start);
        }

        start
    }

    pub fn next_word_back(&self, byte_position: usize) -> usize {
        let mut current = self.get_char_at(byte_position);
        let mut start = byte_position;

        if let Some(c) = current {
            if !(c.is_alphanumeric() || c == '_') {
                start += c.len_utf8();
                current = self.get_char_at(start);
            }
        }

        while let Some(c) = current {
            if !(c.is_alphanumeric() || c == '_') {
                start += c.len_utf8();
                current = self.get_char_at(start);
                break
            } else {
                start += c.len_utf8();
            }
            current = self.get_char_at(start);
        }
        while let Some(c) = current {
            if c.is_alphanumeric() || c == '_' {
                start += c.len_utf8();
            } else {
                start = start.saturating_sub(c.len_utf8());
                break;
            }
            current = self.get_char_at(start);
        }

        start
    }

    pub fn prev_word_back(&self, byte_position: usize) -> usize {
        let mut current = self.get_char_at(byte_position);
        let mut start = byte_position;

        if let Some(c) = current {
            if !(c.is_alphanumeric() || c == '_') {
                start = start.saturating_sub(c.len_utf8());
                current = self.get_char_at(start);
            }
        }

        while let Some(c) = current {
            if !(c.is_alphanumeric() || c == '_') {
                start = start.saturating_sub(c.len_utf8());
                current = self.get_char_at(start);
                break
            } else {
                start = start.saturating_sub(c.len_utf8());
            }
            current = self.get_char_at(start);
        }
        while let Some(c) = current {
            if c.is_alphanumeric() || c == '_' {
                start = start.saturating_sub(c.len_utf8());
            } else {
                start = start.saturating_add(c.len_utf8());
                break;
            }
            current = self.get_char_at(start);
        }

        start
    }

}





impl From<&str> for Buffer {
    fn from(s: &str) -> Self {
        Self {
            current: 0,
            history: vec![Rope::from(s)],
            settings: Rc::new(RefCell::new(Settings::default())),
            tree_sitter_info: None,
            version: 0,
            num_lines: RefCell::new(None),

        }
    }
}

impl From<String> for Buffer {
    fn from(s: String) -> Self {
        Self {
            current: 0,
            history: vec![Rope::from(s)],
            settings: Rc::new(RefCell::new(Settings::default())),
            tree_sitter_info: None,
            version: 0,
            num_lines: RefCell::new(None),
        }
    }
}

impl From<&String> for Buffer {
    fn from(s: &String) -> Self {
        Self {
            current: 0,
            history: vec![Rope::from(s.as_str())],
            settings: Rc::new(RefCell::new(Settings::default())),
            tree_sitter_info: None,
            version: 0,
            num_lines: RefCell::new(None),
        }
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.history[self.current])
    }
}



impl fmt::Display for BufferSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.slice)
    }
}

pub struct BufferSlice<'a> {
    pub slice: RopeSlice<'a>,
    pub settings: Rc<RefCell<Settings>>,
}

impl<'a> BufferSlice<'a> {
    pub fn new(slice: RopeSlice<'a>, settings: Rc<RefCell<Settings>>) -> Self {
        Self {
            slice,
            settings,
        }
    }

    pub fn get_line_count(&self) -> usize {
        let mut num_lines = self.slice.line_len();
        if let Some('\n') = self.slice.chars().last() {
            num_lines += 1;
        }
        num_lines
    }

    pub fn len(&self) -> usize {
        self.slice.chars().map(|c| if c == '\t' {
            self.settings.borrow().editor_settings.tab_size as usize
        } else {
            1
        }).sum()
    }

    pub fn byte_len(&self) -> usize {
        self.slice.bytes().count()
    }

    pub fn get_byte_start(&self) -> usize {
        self.slice.byte_of_line(0)
    }

}





