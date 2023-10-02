use std::fs::File;
use std::{fmt, io};
use std::fmt::{Formatter, write};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumberLineStyle {
    /// No line numbers
    None,
    /// Line numbers relative to the current line
    Relative,
    /// Line numbers relative to the first line
    Absolute,
}

impl fmt::Display for NumberLineStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NumberLineStyle::None => write!(f, "None"),
            NumberLineStyle::Relative => write!(f, "Relative"),
            NumberLineStyle::Absolute => write!(f, "Absolute"),
        }
    }
}


impl fmt::Display for EditorSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[EditorSettings]")?;

        write!(f, "\nnumber_line = \"{}\"", self.number_line)?;
        write!(f, "\ntab_size = {}", self.tab_size)?;
        write!(f, "\nuse_spaces = {}", self.use_spaces)?;
        write!(f, "\nrainbow_delimiters = {}", self.rainbow_delimiters)?;
        write!(f, "\ndefault_mode = \"{}\"", self.default_mode)
    }
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
    /// The default mode
    /// This is the mode that the editor will start in.
    /// The value should be either "Normal" or "Insert".
    pub default_mode: String,
}


impl Default for EditorSettings {
    fn default() -> Self {
        EditorSettings {
            number_line: NumberLineStyle::None,
            tab_size: 4,
            use_spaces: true,
            rainbow_delimiters: false,
            font_settings: None,
            default_mode: String::from("Normal"),
        }
    }
}


impl EditorSettings {
    pub fn new() -> Self {
        let mut settings = EditorSettings::default();

        if crate::arg_parser::IGNORE_USER_SETTINGS.load(std::sync::atomic::Ordering::Relaxed) {
            return settings;
        }

        let xdg_dirs = xdg::BaseDirectories::with_prefix("sevi").unwrap();
        let config_path = xdg_dirs.place_config_file("config.toml").expect("Could not create config file");
        match File::open(config_path) {
            Err(_) => {},
            Ok(mut user_config) => {
                let mut string = String::new();
                user_config.read_to_string(&mut string).expect("Could not read config file");

                let user_settings = EditorSettings::load_user_settings(&string);

                settings.merge_settings(user_settings);
            }
        }

        settings
    }
    pub fn create_default_config_file() -> io::Result<()> {
        let settings = EditorSettings::default();

        let xdg_dirs = xdg::BaseDirectories::with_prefix("sevi").unwrap();
        let user_config_path = xdg_dirs.place_config_file("config.toml").expect("Could not create config file");

        let mut file = File::create(user_config_path)?;
        file.write_all(settings.to_string().as_bytes())
    }

    fn load_user_settings(config: &str) -> EditorSettings {
        let table = config.parse::<toml::Value>().expect("Could not parse config file");

        let values = [
            "number_line",
            "tab_size",
            "use_spaces",
            "rainbow_delimiters",
            "font_settings",
            "default_mode",
        ];

        match table.get("EditorSettings") {
            None => EditorSettings::default(),
            Some(editor_settings) => parse_settings(editor_settings, &values),
        }
    }

    fn merge_settings(&mut self, user_settings: EditorSettings) {
        if user_settings.number_line != NumberLineStyle::None {
            self.number_line = user_settings.number_line;
        }
        if user_settings.tab_size != 0 {
            self.tab_size = user_settings.tab_size;
        }
        if user_settings.use_spaces {
            self.use_spaces = user_settings.use_spaces;
        }
        if user_settings.rainbow_delimiters {
            self.rainbow_delimiters = user_settings.rainbow_delimiters;
        }
        if user_settings.font_settings.is_some() {
            self.font_settings = user_settings.font_settings;
        }
        if user_settings.default_mode != "Normal" {
            self.default_mode = user_settings.default_mode;
        }
    }

}


#[derive(Debug, Clone, PartialEq)]
pub struct FontSettings {
    pub size: u8,
    pub family: String,
}





fn parse_settings(table: &toml::Value, values: &[&str]) -> EditorSettings {
    let number_line: NumberLineStyle;
    let tab_size: u8;
    let use_spaces: bool;
    let rainbow_delimiters: bool;
    let font_settings: Option<FontSettings>;
    let default_mode: String;

    if let Some(number_line_str) = table.get(values[0]) {
        number_line = match number_line_str.as_str().unwrap() {
            "None" => NumberLineStyle::None,
            "Relative" => NumberLineStyle::Relative,
            "Absolute" => NumberLineStyle::Absolute,
            _ => panic!("Invalid number line style"),
        };
    } else {
        number_line = NumberLineStyle::None;
    }

    if let Some(tab_size_str) = table.get(values[1]) {
        tab_size = tab_size_str.as_integer().unwrap() as u8;
    } else {
        tab_size = 4;
    }

    if let Some(use_spaces_str) = table.get(values[2]) {
        use_spaces = use_spaces_str.as_bool().unwrap();
    } else {
        use_spaces = true;
    }

    if let Some(rainbow_delimiters_str) = table.get(values[3]) {
        rainbow_delimiters = rainbow_delimiters_str.as_bool().unwrap();
    } else {
        rainbow_delimiters = false;
    }

    if let Some(font_settings_table) = table.get(values[4]) {
        let size: u8;
        let family: String;

        if let Some(size_str) = font_settings_table.get("size") {
            size = size_str.as_integer().unwrap() as u8;
        } else {
            size = 12;
        }

        if let Some(family_str) = font_settings_table.get("family") {
            family = family_str.as_str().unwrap().to_string();
        } else {
            family = "monospace".to_string();
        }

        font_settings = Some(FontSettings {
            size,
            family,
        });
    } else {
        font_settings = None;
    }

    if let Some(default_mode_str) = table.get(values[5]) {
        default_mode = default_mode_str.as_str().unwrap().to_string();
    } else {
        default_mode = "Normal".to_string();
    }

    EditorSettings {
        number_line,
        tab_size,
        use_spaces,
        rainbow_delimiters,
        font_settings,
        default_mode,
    }
}



