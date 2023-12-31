use std::str::FromStr;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// Reset the color.
    Reset,
    /// ANSI Color: Black. Foreground: 30, Backgound: 40
    Black,
    /// ANSI Color: Red. Foreground: 31, Backgound: 41
    Red,
    /// ANSI Color: Green. Foreground: 32, Backgound: 42
    Green,
    /// ANSI Color: Yellow. Foreground: 33, Backgound: 43
    Yellow,
    /// ANSI Color: Blue. Foreground: 34, Backgound: 44
    Blue,
    /// ANSI Color: Magenta. Foreground: 35, Backgound: 45
    Magenta,
    /// ANSI Color: Cyan. Foreground: 36, Backgound: 46
    Cyan,
    /// ANSI Color: White. Foreground: 37, Backgound: 47
    Gray,
    /// ANSI Color: Dark Gray. Foreground: 90, Backgound: 100
    DarkGray,
    /// ANSI Color: Light Red. Foreground: 91, Backgound: 101
    LightRed,
    /// ANSI Color: Light Green. Foreground: 92, Backgound: 102
    LightGreen,
    /// ANSI Color: Light Yellow. Foreground: 93, Backgound: 103
    LightYellow,
    /// ANSI Color: Light Blue. Foreground: 94, Backgound: 104
    LightBlue,
    /// ANSI Color: Light Magenta. Foreground: 95, Backgound: 105
    LightMagenta,
    /// ANSI Color: Light Cyan. Foreground: 96, Backgound: 106
    LightCyan,
    /// ANSI Color: White. Foreground: 97, Backgound: 107
    White,
    /// An RGB color.
    Rgb(u8, u8, u8),
    /// An 8-bit 256 color.
    Indexed(u8),
}

impl Color {
    pub fn config_file(&self) -> String {
        match self {
            Color::Reset => "\"reset\"".to_string(),
            Color::Black => "\"black\"".to_string(),
            Color::Red => "\"red\"".to_string(),
            Color::Green => "\"green\"".to_string(),
            Color::Yellow => "\"yellow\"".to_string(),
            Color::Blue => "\"blue\"".to_string(),
            Color::Magenta => "\"magenta\"".to_string(),
            Color::Cyan => "\"cyan\"".to_string(),
            Color::Gray => "\"gray\"".to_string(),
            Color::DarkGray => "\"dark-gray\"".to_string(),
            Color::LightRed => "\"light-red\"".to_string(),
            Color::LightGreen => "\"light-green\"".to_string(),
            Color::LightYellow => "\"light-yellow\"".to_string(),
            Color::LightBlue => "\"light-blue\"".to_string(),
            Color::LightMagenta => "\"light-magenta\"".to_string(),
            Color::LightCyan => "\"light-cyan\"".to_string(),
            Color::White => "\"white\"".to_string(),
            Color::Rgb(r, g, b) => format!("[{}, {}, {}]", r, g, b),
            Color::Indexed(color) => format!("{}", color),
        }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .to_lowercase()
            .replace([' ', '-', '_'], "")
            .replace("bright", "light")
            .replace("grey", "gray")
            .replace("silver", "gray")
            .replace("lightblack", "darkgray")
            .replace("lightwhite", "white")
            .replace("lightgray", "white")
            .as_ref() {
                "black" => Ok(Color::Black),
                "red" => Ok(Color::Red),
                "green" => Ok(Color::Green),
                "yellow" => Ok(Color::Yellow),
                "blue" => Ok(Color::Blue),
                "magenta" => Ok(Color::Magenta),
                "cyan" => Ok(Color::Cyan),
                "gray" => Ok(Color::Gray),
                "darkgray" => Ok(Color::DarkGray),
                "lightred" => Ok(Color::LightRed),
                "lightgreen" => Ok(Color::LightGreen),
                "lightyellow" => Ok(Color::LightYellow),
                "lightblue" => Ok(Color::LightBlue),
                "lightmagenta" => Ok(Color::LightMagenta),
                "lightcyan" => Ok(Color::LightCyan),
                "white" => Ok(Color::White),
                _ => {
                    if let Ok(index) = s.parse::<u8>() {
                        Ok(Color::Indexed(index))
                    } else if let (Ok(r), Ok(g), Ok(b)) = {
                        if !s.starts_with('#') || s.len() != 7 {
                            return Err(());
                        }
                        (
                            u8::from_str_radix(&s[1..3], 16),
                            u8::from_str_radix(&s[3..5], 16),
                            u8::from_str_radix(&s[5..7], 16),
                        )
                    } {
                        Ok(Color::Rgb(r, g, b))
                    } else {
                        Err(())
                    }

                }
        }
    }

}


impl Into<tuirealm::tui::style::Color> for Color {
    fn into(self) -> tuirealm::tui::style::Color {
        match self {
            Color::Reset => tuirealm::tui::style::Color::Reset,
            Color::Black => tuirealm::tui::style::Color::Black,
            Color::Red => tuirealm::tui::style::Color::Red,
            Color::Green => tuirealm::tui::style::Color::Green,
            Color::Yellow => tuirealm::tui::style::Color::Yellow,
            Color::Blue => tuirealm::tui::style::Color::Blue,
            Color::Magenta => tuirealm::tui::style::Color::Magenta,
            Color::Cyan => tuirealm::tui::style::Color::Cyan,
            Color::Gray => tuirealm::tui::style::Color::Gray,
            Color::DarkGray => tuirealm::tui::style::Color::DarkGray,
            Color::LightRed => tuirealm::tui::style::Color::LightRed,
            Color::LightGreen => tuirealm::tui::style::Color::LightGreen,
            Color::LightYellow => tuirealm::tui::style::Color::LightYellow,
            Color::LightBlue => tuirealm::tui::style::Color::LightBlue,
            Color::LightMagenta => tuirealm::tui::style::Color::LightMagenta,
            Color::LightCyan => tuirealm::tui::style::Color::LightCyan,
            Color::White => tuirealm::tui::style::Color::White,
            Color::Rgb(r, g, b) => tuirealm::tui::style::Color::Rgb(r, g, b),
            Color::Indexed(i) => tuirealm::tui::style::Color::Indexed(i),
        }
    }
}


pub fn parse_color(value: &toml::Value) -> Result<Color, String> {

    if value.is_str() {
        let value = value.as_str().expect("color was not a string");

        let color = match value {
            "reset" => Color::Reset,
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "dark-grey" => Color::DarkGray,
            "light-red" => Color::LightRed,
            "light-green" => Color::LightGreen,
            "light-yellow" => Color::LightYellow,
            "light-blue" => Color::LightBlue,
            "light-magenta" => Color::LightMagenta,
            "light-cyan" => Color::LightCyan,
            "grey" => Color::Gray,
            _ => unreachable!(),
        };
        Ok(color)
    }
    else if value.is_array() {
        let value = value.as_array().expect("color was not an array");

        if value.len() != 3 {
            return Err("color array was not of length 3".to_string());
        }

        Ok(Color::Rgb(
            value[0].as_integer().expect("color array value was not an integer") as u8,
            value[1].as_integer().expect("color array value was not an integer") as u8,
            value[2].as_integer().expect("color array value was not an integer") as u8,
        ))
    } else if value.is_integer() {
        let value = value.as_integer().ok_or("color was not an integer".to_string())? as u8;
        Ok(Color::Indexed(value))
    } else {
        Err("color was not a string or array".to_string())
    }

}

