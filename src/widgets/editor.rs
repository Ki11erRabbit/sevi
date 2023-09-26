use std::fmt::format;
use tuirealm::props::{Alignment, Style};
use tuirealm::tui::buffer::Buffer;
use tuirealm::tui::prelude::Rect;
use tuirealm::tui::text::{Span, StyledGrapheme, Text};
use tuirealm::tui::widgets::{Block, Widget, Wrap};
use unicode_width::UnicodeWidthStr;
use crate::models::settings::editor_settings::NumberLineStyle;
use crate::widgets::reflow::{LineComposer, LineTruncator, WordWrapper};

fn get_line_offset(line_width: u16, text_area_width: u16, alignment: Alignment) -> u16 {
    match alignment {
        Alignment::Center => (text_area_width / 2).saturating_sub(line_width / 2),
        Alignment::Right => text_area_width.saturating_sub(line_width),
        Alignment::Left => 0,
    }
}

pub struct Editor<'a> {
    block: Option<Block<'a>>,
    style: Style,
    wrap: Option<Wrap>,
    text: Text<'a>,
    scroll: (u16, u16),
    alignment: Alignment,
    number_line_type: NumberLineStyle,
    cursor_row: Option<usize>,
}

impl<'a> Editor<'a> {
    pub fn new<T>(text: T) -> Editor<'a> where T: Into<Text<'a>> {
        Editor {
            block: None,
            style: Style::default(),
            wrap: None,
            text: text.into(),
            scroll: (0, 0),
            alignment: Alignment::Left,
            number_line_type: NumberLineStyle::None,
            cursor_row: None
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = Some(wrap);
        self
    }

    pub fn scroll(mut self, scroll: (u16, u16)) -> Self {
        self.scroll = scroll;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn number_line_type(mut self, number_line_type: NumberLineStyle, cursor_row: usize) -> Self {
        self.number_line_type = number_line_type;
        self.cursor_row = Some(cursor_row);
        self
    }
}

impl<'a> Widget for Editor<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let text_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if text_area.height < 1 {
            return;
        }
        match self.number_line_type {
            NumberLineStyle::None => {}
            NumberLineStyle::Relative => {
                let mut places = 1;
                let mut num_width = 3;
                while places <= self.text.lines.len() {
                    places *= 10;
                    num_width += 1;
                }

                for (i, line) in self.text.lines.iter_mut().enumerate() {
                    if let Some(row) = self.cursor_row {
                        let line_number = if i == row {
                            format!("{:<width$}", i + 1, width = num_width)
                        } else {
                            format!("{:width$}", (i as isize - row as isize).abs() as usize , width = num_width)
                        };
                        line.spans.insert(0, Span {
                            content: line_number.into(),
                            style: self.style,
                        });
                    }
                }
            }
            NumberLineStyle::Absolute => {
                let mut places = 1;
                let mut num_width = 0;
                while places <= self.text.lines.len() {
                    places *= 10;
                    num_width += 1;
                }

                for (i, line) in self.text.lines.iter_mut().enumerate() {
                    let line_number = format!("{:width$}", i + 1, width = num_width);
                    line.spans.insert(0, Span {
                        content: line_number.into(),
                        style: self.style,
                    });
                }
            }
        }

        let style = self.style;
        let styled = self.text.lines.iter().map(|line| {
            (
                line.spans
                    .iter()
                    .flat_map(|span| span.styled_graphemes(style)),
                line.alignment.unwrap_or(self.alignment),
            )
        });

        let mut line_composer: Box<dyn LineComposer> = if let Some(Wrap { trim }) = self.wrap {
            Box::new(WordWrapper::new(styled, text_area.width, trim))
        } else {
            let mut line_composer = Box::new(LineTruncator::new(styled, text_area.width));
            line_composer.set_horizontal_offset(self.scroll.1);
            line_composer
        };
        let mut y = 0;
        while let Some((current_line, current_line_width, current_line_alignment)) =
            line_composer.next_line()
        {
            if y >= self.scroll.0 {
                let mut x =
                    get_line_offset(current_line_width, text_area.width, current_line_alignment.into());
                for StyledGrapheme { symbol, style } in current_line {
                    let width = symbol.width();
                    if width == 0 {
                        continue;
                    }
                    buf.get_mut(text_area.left() + x, text_area.top() + y - self.scroll.0)
                        .set_symbol(if symbol.is_empty() {
                            // If the symbol is empty, the last char which rendered last time will
                            // leave on the line. It's a quick fix.
                            " "
                        } else {
                            symbol
                        })
                        .set_style(*style);
                    x += width as u16;
                }
            }
            y += 1;
            if y >= text_area.height + self.scroll.0 {
                break;
            }
        }
    }
}