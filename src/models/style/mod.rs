use self::text_modifier::Modifier;

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
}

impl Default for Style {
    fn default() -> Style {
        Style::new()
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


    pub fn patch_style(&mut self, style: Style) {
        self.style = self.style.patch(style);
    }

    pub fn reset_style(&mut self) {
        self.style = Style::reset();
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
        StyledSpan::raw(text)
    }
}


pub struct StyledText<'a> {
    pub spans: Vec<StyledSpan<'a>>,
}

impl<'a> StyledText<'a> {
    pub fn new() -> StyledText<'a> {
        StyledText {
            spans: Vec::new(),
        }
    }


    pub fn push(&mut self, span: StyledSpan<'a>) {
        self.spans.push(span);
    }

    pub fn push_raw<T>(&mut self, content: T) where T: Into<Cow<'a, str>> {
        self.spans.push(StyledSpan::raw(content));
    }

    pub fn push_styled<T>(&mut self, content: T, style: Style) where T: Into<Cow<'a, str>> {
        self.spans.push(StyledSpan::styled(content, style));
    }

    pub fn patch_style(&mut self, style: Style) {
        for span in &mut self.spans {
            span.patch_style(style);
        }
    }

    pub fn reset_style(&mut self) {
        for span in &mut self.spans {
            span.reset_style();
        }
    }

    pub fn clear(&mut self) {
        self.spans.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }

    pub fn len(&self) -> usize {
        self.spans.len()
    }

    pub fn iter(&self) -> std::slice::Iter<StyledSpan<'a>> {
        self.spans.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<StyledSpan<'a>> {
        self.spans.iter_mut()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<StyledSpan<'a>> {
        self.spans.into_iter()
    }
}

impl<'a> Default for StyledText<'a> {
    fn default() -> StyledText<'a> {
        StyledText::new()
    }
}

impl<'a> From<StyledSpan<'a>> for StyledText<'a> {
    fn from(span: StyledSpan<'a>) -> StyledText<'a> {
        let mut text = StyledText::new();
        text.push(span);
        text
    }
}

impl<'a> From<std::slice::Iter<'a, StyledSpan<'a>>> for StyledText<'a> {
    fn from(iter: std::slice::Iter<'a, StyledSpan<'a>>) -> StyledText<'a> {
        let mut text = StyledText::new();
        for span in iter {
            text.push(span.clone());
        }
        text
    }
}

impl<'a> From<std::vec::IntoIter<StyledSpan<'a>>> for StyledText<'a> {
    fn from(iter: std::vec::IntoIter<StyledSpan<'a>>) -> StyledText<'a> {
        let mut text = StyledText::new();
        for span in iter {
            text.push(span);
        }
        text
    }
}

impl<'a> From<std::slice::IterMut<'a, StyledSpan<'a>>> for StyledText<'a> {
    fn from(iter: std::slice::IterMut<'a, StyledSpan<'a>>) -> StyledText<'a> {
        let mut text = StyledText::new();
        for span in iter {
            text.push(span.clone());
        }
        text
    }
}

