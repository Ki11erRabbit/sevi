use self::text_modifier::Modifier;

use std::fmt;
use std::borrow::Cow;


pub mod color;
pub mod text_modifier;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Style {
    pub fg: Option<color::Color>,
    pub bg: Option<color::Color>,
    pub underline_color: Option<color::Color>,
    pub add_modifier: Modifier,
    pub sub_modifier: Modifier,
}


impl Style {
    pub const fn new() -> Style {
        Style {
            fg: None,
            bg: None,
            underline_color: None,
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        }
    }

    pub const fn reset() -> Style {
        Style {
            fg: None,
            bg: None,
            underline_color: None,
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        }
    }

    pub const fn fg(mut self, color: color::Color) -> Style {
        self.fg = Some(color);
        self
    }

    pub const fn bg(mut self, color: color::Color) -> Style {
        self.bg = Some(color);
        self
    }

    pub const fn underline_color(mut self, color: color::Color) -> Style {
        self.underline_color = Some(color);
        self
    }

    pub const fn add_modifier(mut self, modifier: Modifier) -> Style {
        self.sub_modifier = self.sub_modifier.difference(modifier);
        self.add_modifier = self.add_modifier.union(modifier);
        self
    }

    pub const fn sub_modifier(mut self, modifier: Modifier) -> Style {
        self.add_modifier = self.add_modifier.difference(modifier);
        self.sub_modifier = self.sub_modifier.union(modifier);
        self
    }

    pub fn patch(mut self, other: Style) -> Style {
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);

        self.underline_color = other.underline_color.or(self.underline_color);

        self.add_modifier.remove(other.sub_modifier);
        self.add_modifier.insert(other.add_modifier);
        self.sub_modifier.remove(other.add_modifier);
        self.sub_modifier.insert(other.sub_modifier);

        self
    }

    pub fn config_file(&self) -> String {
        let mut output = String::new();

        if let Some(fg) = self.fg {
            output.push_str(&format!("fg = {}\n", fg.config_file()));
        }

        if let Some(bg) = self.bg {
            output.push_str(&format!("bg = {}\n", bg.config_file()));
        }

        if let Some(underline_color) = self.underline_color {
            output.push_str(&format!("underline_color = {}\n", underline_color.config_file()));
        }

        if !self.add_modifier.is_empty() {
            output.push_str(&format!("modifiers = {}\n", self.add_modifier.config_file()));
        }


        output
    }
}

impl Default for Style {
    fn default() -> Style {
        Style::new()
    }
}

pub fn parse_style(table: &toml::Value) -> Result<Style,String> {
    let table = table.as_table().ok_or("style was not a table".to_string())?;
    let mut style = Style::new();

    match table.get("fg") {
        Some(value) => {
            let value = color::parse_color(value)?;
            style = style.fg(value);
        },
        None => {},
    }

    match table.get("bg") {
        Some(value) => {
            let value = color::parse_color(value)?;
            style = style.bg(value);
        },
        None => {},
    }

    match table.get("underline_color") {
        Some(value) => {
            let value = color::parse_color(value)?;
            style = style.underline_color(value);
        },
        None => {},
    }

    match table.get("modifiers") {
        Some(value) => {
            let value = text_modifier::parse_modifier(value)?;
            style = style.add_modifier(value);
        },
        None => {},
    }

    Ok(style)
}


//TODO: Add conditional Compilation for TUI
impl Into<tuirealm::tui::style::Style> for Style {
    fn into(self) -> tuirealm::tui::style::Style {
        let mut style = tuirealm::tui::style::Style::default();
        if let Some(fg) = self.fg {
            style = style.fg(fg.into());
        }
        if let Some(bg) = self.bg {
            style = style.bg(bg.into());
        }
        if let Some(underline_color) = self.underline_color {
            style = style.underline_color(underline_color.into());
        }
        style = style.add_modifier(self.add_modifier.into());
        style = style.remove_modifier(self.sub_modifier.into());
        style
    }
}





#[derive(Debug, Clone)]
pub struct StyledSpan<'a> {
    pub text: Cow<'a, str>,
    pub style: Style,
}

impl<'a> StyledSpan<'a> {


    /// Create a new StyledSpan with no style.
    pub fn raw<T>(content: T) -> StyledSpan<'a> where T: Into<Cow<'a, str>> {
        StyledSpan {
            text: content.into(),
            style: Style::new(),
        }
    }

    /// Create a new StyledSpan with the given style.
    pub fn styled<T>(content: T, style: Style) -> StyledSpan<'a> where T: Into<Cow<'a, str>> {
        StyledSpan {
            text: content.into(),
            style,
        }
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }


    pub fn patch_style(&mut self, style: Style) {
        self.style = self.style.patch(style);
    }

    pub fn reset_style(&mut self) {
        self.style = Style::reset();
    }


}

impl fmt::Display for StyledSpan<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)?;
        Ok(())
    }
}


impl<'a> From<&'a str> for StyledSpan<'a> {
    fn from(text: &'a str) -> StyledSpan<'a> {
        StyledSpan::raw(text)
    }
}

impl<'a> From<String> for StyledSpan<'a> {
    fn from(text: String) -> StyledSpan<'a> {
        StyledSpan::raw(text)
    }
}

impl<'a> From<&String> for StyledSpan<'a> {
    fn from(text: &String) -> StyledSpan<'a> {
        StyledSpan::raw(text.clone())
    }
}


impl<'a> Into<tuirealm::tui::prelude::Span<'a>> for StyledSpan<'a> {
    fn into(self) -> tuirealm::tui::prelude::Span<'a> {
        tuirealm::tui::prelude::Span::styled(self.text, self.style.into())
    }
}

impl<'a> Into<tuirealm::tui::prelude::Line<'a>> for StyledSpan<'a> {
    fn into(self) -> tuirealm::tui::prelude::Line<'a> {
        tuirealm::tui::prelude::Line::styled(self.text, self.style.into())
    }
}

#[derive(Debug, Clone)]
pub struct StyledLine<'a> {
    pub spans: Vec<StyledSpan<'a>>,
}

impl<'a> StyledLine<'a> {
    pub fn new() -> StyledLine<'a> {
        StyledLine {
            spans: Vec::new(),
        }
    }

    pub fn push(&mut self, span: StyledSpan<'a>) {
        self.spans.push(span);
    }

    pub fn extend(&mut self, spans: Vec<StyledSpan<'a>>) {
        self.spans.extend(spans);
    }

    pub fn len(&self) -> usize {
        self.spans.iter().map(|s| s.len()).sum()
    }

    pub fn insert(&mut self, index: usize, span: StyledSpan<'a>) {
        self.spans.insert(index, span);
    }
}

impl fmt::Display for StyledLine<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for span in &self.spans {
            write!(f, "{}", span)?;
        }
        Ok(())
    }
}

impl<'a> From<&'a str> for StyledLine<'a> {
    fn from(text: &'a str) -> StyledLine<'a> {
        StyledLine::from(vec![StyledSpan::from(text)])
    }
}

impl<'a> From<String> for StyledLine<'a> {
    fn from(text: String) -> StyledLine<'a> {
        StyledLine::from(vec![StyledSpan::from(text)])
    }
}

impl<'a> From<Vec<StyledSpan<'a>>> for StyledLine<'a> {
    fn from(spans: Vec<StyledSpan<'a>>) -> StyledLine<'a> {
        StyledLine {
            spans,
        }
    }
}

impl<'a> Into<StyledText<'a>> for StyledLine<'a> {
    fn into(self) -> StyledText<'a> {
        StyledText::from(vec![self])
    }
}

impl<'a> Into<tuirealm::tui::prelude::Line<'a>> for StyledLine<'a> {
    fn into(self) -> tuirealm::tui::prelude::Line<'a> {
        tuirealm::tui::prelude::Line::from(self.spans.into_iter().map(|span| span.into()).collect::<Vec<_>>())
    }
}

#[derive(Debug, Clone)]
pub struct StyledText<'a> {
    pub lines: Vec<StyledLine<'a>>,
}

impl<'a> StyledText<'a> {
    pub fn new() -> StyledText<'a> {
        StyledText {
            lines: Vec::new(),
        }
    }

    pub fn raw<T>(content: T) -> Self where T: Into<Cow<'a, str>> {
        let lines: Vec<_> = match content.into() {
            Cow::Borrowed("") => vec![StyledLine::from("")],
            Cow::Borrowed(s) => s.lines().map(StyledLine::from).collect(),
            Cow::Owned(s) if s.is_empty() => vec![StyledLine::from("")],
            Cow::Owned(s) => s.lines().map(|l| StyledLine::from(l.to_owned())).collect(),
        };

        StyledText::from(lines)
    }


    pub fn clear(&mut self) {
        self.lines.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn len(&self) -> usize {
        self.lines.iter().map(|l| l.len()).sum()
    }

    pub fn rows(&self) -> usize {
        self.lines.len()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut StyledLine<'a>> {
        self.lines.iter_mut()
    }
}

impl fmt::Display for StyledText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.lines {
            write!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl<'a> Default for StyledText<'a> {
    fn default() -> StyledText<'a> {
        StyledText::new()
    }
}

impl<'a> From<Vec<StyledLine<'a>>> for StyledText<'a> {
    fn from(lines: Vec<StyledLine<'a>>) -> StyledText<'a> {
        StyledText {
            lines,
        }
    }
}


impl<'a> From<&'a str> for StyledText<'a> {
    fn from(text: &'a str) -> StyledText<'a> {
        StyledText::raw(text)
    }
}

impl<'a> From<String> for StyledText<'a> {
    fn from(text: String) -> StyledText<'a> {
        StyledText::raw(text)
    }
}

impl<'a> From<&'a String> for StyledText<'a> {
    fn from(text: &'a String) -> StyledText<'a> {
        StyledText::raw(text)
    }
}

impl<'a> Into<Vec<StyledSpan<'a>>> for StyledText<'a> {
    fn into(self) -> Vec<StyledSpan<'a>> {
        self.lines.into_iter().flat_map(|line| line.spans).collect::<Vec<_>>()
    }
}


//TODO: Add conditional Compilation for TUI

impl<'a> Into<tuirealm::tui::prelude::Text<'a>> for StyledText<'a> {
    fn into(self) -> tuirealm::tui::prelude::Text<'a> {

        let temp: Vec<tuirealm::tui::prelude::Line> = self.lines.into_iter().map(|line| line.into()).collect::<Vec<_>>();
        tuirealm::tui::prelude::Text::from(temp)
    }
}
