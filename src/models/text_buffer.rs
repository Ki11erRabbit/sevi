use std::cell::RefCell;
use std::rc::Rc;
use crate::models::settings::editor_settings::NumberLineStyle;
use crate::models::settings::Settings;
use crate::models::style::{StyledSpan, StyledText};


pub struct BufferText {
    settings: Rc<RefCell<Settings>>,
    number_line_type: NumberLineStyle,
    cursor_row: Option<usize>,
    start_row: usize,
    scroll_cols: Option<usize>,
}

impl BufferText {
    pub fn new(settings: Rc<RefCell<Settings>>) -> Self {
        Self {
            settings,
            number_line_type: NumberLineStyle::None,
            cursor_row: None,
            start_row: 0,
            scroll_cols: None,
        }
    }

    pub fn add_number_line_style(mut self, number_line_type: NumberLineStyle) -> Self {
        self.number_line_type = number_line_type;
        self
    }

    pub fn add_current_row(mut self, cursor_row: usize) -> Self {
        self.cursor_row = Some(cursor_row);
        self
    }

    pub fn set_start_row(mut self, start_row: usize) -> Self {
        self.start_row = start_row;
        self
    }
    pub fn set_scroll_cols(mut self, scroll_cols: usize) -> Self {
        self.scroll_cols = Some(scroll_cols);
        self
    }

    pub fn draw<'a>(self, mut text: StyledText<'a>) -> StyledText<'a> {
        let text = match self.number_line_type {
            NumberLineStyle::None => {
                for line in text.iter_mut() {
                    match self.scroll_cols {
                        None => {},
                        Some(cols) => {
                            line.drop(cols)
                        }
                    }
                }
                text
            },
            NumberLineStyle::Relative => {
                let mut places = 1;
                let mut num_width = 3;
                while places <= self.start_row + text.rows() {
                    num_width += 1;
                    places *= 10;
                }
                let settings = self.settings.clone();
                let settings = settings.borrow();

                for (i, line) in text.iter_mut().enumerate() {
                    let i = i + self.start_row;
                    match self.scroll_cols {
                        None => {},
                        Some(cols) => {
                            line.drop(cols)
                        }
                    }

                    if let Some(row) = self.cursor_row {
                        if i == row {
                            let line_number = format!("{:<width$}", i + 1, width = num_width);
                            let color = settings.colors.number_bar.current_line;
                            line.insert(0, StyledSpan::styled(line_number, color));
                        } else {
                            let line_number = format!("{:width$}", (i as isize - row as isize).abs() as usize , width = num_width);
                            let color = settings.colors.number_bar.other_lines;
                            line.insert(0, StyledSpan::styled(line_number, color));
                        };
                    }
                }
                text
            }
            NumberLineStyle::Absolute => {
                let mut places = 1;
                let mut num_width = 0;
                while places <= self.start_row + text.rows() {
                    places *= 10;
                    num_width += 1;
                }

                for (i, line) in text.iter_mut().enumerate() {
                    let i = i + self.start_row;

                    match self.scroll_cols {
                        None => {},
                        Some(cols) => {
                            line.drop(cols)
                        }
                    }

                    let line_number = format!("{:width$}", i + 1, width = num_width);
                    let settings = self.settings.clone();
                    let settings = settings.borrow();
                    let color = if let Some(row) = self.cursor_row {
                        if i == row {
                            settings.colors.number_bar.current_line
                        } else {
                            settings.colors.number_bar.other_lines
                        }
                    } else {
                        settings.colors.number_bar.other_lines
                    };
                    line.insert(0, StyledSpan::styled(line_number, color));
                }
                text
            }
        };

        text
    }


}


