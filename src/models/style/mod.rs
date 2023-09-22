use self::text_modifier::Modifier;



pub mod color;
pub mod text_modifier;



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
