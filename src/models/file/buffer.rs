use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
use crate::models::settings::Settings;

use tree_sitter;
use crop::{Rope, RopeSlice};





pub struct Buffer {
    current: usize,
    history: Vec<Rope>,
    tree_sitter_info: Option<(tree_sitter::Parser, Vec<tree_sitter::Tree>)>,
    settings: Rc<RefCell<Settings>>,
    version: usize,
}

impl Buffer {
    pub fn new(settings: Rc<RefCell<Settings>>) -> Self {
        Self {
            current: 0,
            history: vec![Rope::new()],
            tree_sitter_info: None,
            settings,
            version: 0,
        }
    }

    pub fn set_settings(&mut self, settings: Rc<RefCell<Settings>>) {
        self.settings = settings;
    }

    pub fn get_version(&self) -> usize {
        self.version
    }

    pub fn set_tree_sitter(&mut self, mut parser: tree_sitter::Parser) {
        let mut trees = Vec::new();
        trees.push(parser.parse(&self.history[self.current].to_string(), None).unwrap());
        self.tree_sitter_info = Some((parser, trees));
    }

    pub fn get_char_at(&self, byte_offset: usize) -> Option<char> {
        let current = &self.history[self.current];

        if byte_offset >= current.byte_len() {
            return None;
        }
        let mut total_bytes = 1;

        while byte_offset + total_bytes < current.byte_len() && current.is_char_boundary(byte_offset + total_bytes) {
            total_bytes += 1;
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
        let mut num_lines = self.history[self.current].line_len();
        if let Some('\n') = self.history[self.current].chars().last() {
            num_lines += 1;
        }
        num_lines
    }

    pub fn get_char_count(&self) -> usize {
        self.history[self.current].chars().count()
    }

    pub fn get_byte_count(&self) -> usize {
        self.history[self.current].bytes().count()
    }


    pub fn get_byte_offset(&self, x: usize, y: usize) -> Option<usize> {
        if y >= self.history[self.current].line_len() {
            return None;
        }
        let line_byte = self.history[self.current].byte_of_line(y);

        let line = self.history[self.current].line(y);
        let mut i = 0;
        let mut col_byte = 0;
        while i < line.byte_len() {
            if line.is_char_boundary(i) {
                if col_byte == x {
                    break;
                }
                col_byte += 1;
            }
            i += 1;
        }
        Some(line_byte + col_byte)
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

    pub fn delete_current<R>(&mut self, range: R) where R: std::ops::RangeBounds<usize> {
        
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

        self.version += 1;
    }

    pub fn replace_current<R, T>(&mut self, range: R, text: T) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        
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

            },
            None => {
                self.history[self.current].replace(range, text.as_ref());
            },

        }

        self.tree_sitter_info = tree_sitter_info;

        self.version += 1;
    }

    pub fn insert<T>(&mut self, byte_offset: usize, text: T) where T: AsRef<str> {
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

        let mut tree_sitter_info = self.tree_sitter_info.take();
            
        match tree_sitter_info.as_mut() {
            None => {
                match range.end_bound() {
                    std::ops::Bound::Included(n) => {
                        if *n >= self.history[self.current].byte_len() {
                            return;
                        }
                    },
                    std::ops::Bound::Excluded(n) => {
                        if *n >= self.history[self.current].byte_len() {
                            return;
                        }
                    },
                    std::ops::Bound::Unbounded => {
                        return;
                    },
                }
                self.history[self.current].delete(range);
            },
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
            }
        }


        self.tree_sitter_info = tree_sitter_info;

        self.version += 1;
    }

    pub fn replace<R, T>(&mut self, range: R, text: T) where R: std::ops::RangeBounds<usize>, T: AsRef<str> {
        self.get_new_rope();
        
        let mut tree_sitter_info = self.tree_sitter_info.take();    

        match tree_sitter_info.as_mut() {
            None => {
                self.history[self.current].replace(range, text.as_ref());
            },
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
            }
        }

        self.tree_sitter_info = tree_sitter_info;


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
        let buffer = self.get_new_rope();
        for (offset, text) in values.iter().rev() {
            buffer.insert(*offset, text.as_ref());
        }
    }

    pub fn delete_chain<R>(&mut self, values: Box<[R]>)
        where R: std::ops::RangeBounds<usize> + Copy
    {
        let buffer = self.get_new_rope();
        for range in values.iter().rev() {
            buffer.delete(*range);
        }
    }

    pub fn replace_chain<R, T>(&mut self, values: Box<[(R, T)]>)
        where R: std::ops::RangeBounds<usize> + Copy, T: AsRef<str>
    {
        let buffer = self.get_new_rope();
        for (range, text) in values.iter().rev() {
            buffer.replace(*range, text.as_ref());
        }
    }

    pub fn get_version_count(&self) -> usize {
        self.history.len()
    }


    pub fn get_row<'a>(&'a self, row: usize) -> Option<BufferSlice<'a>> {
        
        if row >= self.history[self.current].line_len() {
            return None;
        }
        


        let line = self.history[self.current].line(row);
        Some(BufferSlice::new(line, self.settings.clone()))

    }

    pub fn get_row_special<'a>(&'a self, row: usize, col_offset: usize, cols: usize) -> Option<BufferSlice<'a>> {
        
        if row >= self.history[self.current].line_len() {
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

    pub fn get_slice<'a>(&'a self, start: usize, end: usize) -> Option<BufferSlice<'a>> {
        if start > end {
            return None;
        }
        if end > self.history[self.current].bytes().count() {
            return None;
        }
        Some(BufferSlice::new(self.history[self.current].byte_slice(start..end), self.settings.clone()))
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





