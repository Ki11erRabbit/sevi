





#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumberLineStyle {
    /// No line numbers
    None,
    /// Line numbers relative to the current line
    Relative,
    /// Line numbers relative to the first line
    Absolute,
}



#[derive(Debug, Clone, PartialEq)]
pub struct EditorSettings {
    /// The style of the line numbers
    pub number_line: NumberLineStyle,
    /// The size of a tab
    pub tab_size: u8,
    /// Whether to use spaces instead of tabs
    pub use_spaces: bool,
    /// Whether to highlight matching brackets
    pub rainbow_delimiters: bool,
    /// The font settings
    /// This is only used in the GUI not the TUI.
    pub font_settings: Option<FontSettings>,
}


impl Default for EditorSettings {
    fn default() -> Self {
        EditorSettings {
            number_line: NumberLineStyle::Relative,
            tab_size: 4,
            use_spaces: true,
            rainbow_delimiters: true,
            font_settings: None,
        }
    }
}


impl EditorSettings {
    pub const fn new() -> Self {
        // TODO: try to open a config file and load the settings from there
        // if that fails, return the default settings
        Self::default()
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct FontSettings {
    pub size: u8,
    pub family: String,
}




