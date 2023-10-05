use core::fmt;

use bitflags::bitflags;



bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Modifier: u16 {
        const BOLD =         0b0000000000000001;
        const DIM =          0b0000000000000010;
        const ITALIC =       0b0000000000000100;
        const UNDERLINE =    0b0000000000001000;
        const SLOW_BLINK =   0b0000000000010000;
        const RAPID_BLINK =  0b0000000000100000;
        const REVERSED =     0b0000000001000000;
        const HIDDEN =       0b0000000010000000;
        const CROSSED_OUT =  0b0000000100000000;
    }
}


impl Modifier {
    pub fn config_file(&self) -> String {
        let mut modifiers = Vec::new();

        if self.contains(Modifier::BOLD) {
            modifiers.push("\"bold\"".to_string());
        }

        if self.contains(Modifier::DIM) {
            modifiers.push("\"dim\"".to_string());
        }

        if self.contains(Modifier::ITALIC) {
            modifiers.push("\"italic\"".to_string());
        }

        if self.contains(Modifier::UNDERLINE) {
            modifiers.push("\"underline\"".to_string());
        }

        if self.contains(Modifier::SLOW_BLINK) {
            modifiers.push("\"slow_blink\"".to_string());
        }

        if self.contains(Modifier::RAPID_BLINK) {
            modifiers.push("\"rapid_blink\"".to_string());
        }

        if self.contains(Modifier::REVERSED) {
            modifiers.push("\"reversed\"".to_string());
        }

        if self.contains(Modifier::HIDDEN) {
            modifiers.push("\"hidden\"".to_string());
        }

        if self.contains(Modifier::CROSSED_OUT) {
            modifiers.push("\"crossed_out\"".to_string());
        }

        format!("[{}]", modifiers.join(", "))
    }
}
impl fmt::Debug for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "NONE");
        }
        fmt::Debug::fmt(&self.0, f)
    }

}


//TODO: Add conditional Compilation for TUI
impl Into<tuirealm::tui::style::Modifier> for Modifier {
    fn into(self) -> tuirealm::tui::style::Modifier {
        let mut modifier = tuirealm::tui::style::Modifier::empty();
        if self.contains(Modifier::BOLD) {
            modifier = modifier | tuirealm::tui::style::Modifier::BOLD;
        }
        if self.contains(Modifier::DIM) {
            modifier = modifier | tuirealm::tui::style::Modifier::DIM;
        }
        if self.contains(Modifier::ITALIC) {
            modifier = modifier | tuirealm::tui::style::Modifier::ITALIC;
        }
        if self.contains(Modifier::UNDERLINE) {
            modifier = modifier | tuirealm::tui::style::Modifier::UNDERLINED;
        }
        if self.contains(Modifier::SLOW_BLINK) {
            modifier = modifier | tuirealm::tui::style::Modifier::SLOW_BLINK;
        }
        if self.contains(Modifier::RAPID_BLINK) {
            modifier = modifier | tuirealm::tui::style::Modifier::RAPID_BLINK;
        }
        if self.contains(Modifier::REVERSED) {
            modifier = modifier | tuirealm::tui::style::Modifier::REVERSED;
        }
        if self.contains(Modifier::HIDDEN) {
            modifier = modifier | tuirealm::tui::style::Modifier::HIDDEN;
        }
        if self.contains(Modifier::CROSSED_OUT) {
            modifier = modifier | tuirealm::tui::style::Modifier::CROSSED_OUT;
        }
        modifier
    }
}

pub fn parse_modifier(list: &toml::Value) -> Result<Modifier, String> {
    let mut modifier = Modifier::empty();

    if list.is_array() {
        let list = list.as_array().ok_or("modifier was not an array".to_string())?;

        for value in list {
            if value.is_str() {
                let value = value.as_str().ok_or("modifier was not a string".to_string())?;

                match value {
                    "bold" => modifier = modifier | Modifier::BOLD,
                    "dim" => modifier = modifier | Modifier::DIM,
                    "italic" => modifier = modifier | Modifier::ITALIC,
                    "underline" => modifier = modifier | Modifier::UNDERLINE,
                    "slow_blink" => modifier = modifier | Modifier::SLOW_BLINK,
                    "rapid_blink" => modifier = modifier | Modifier::RAPID_BLINK,
                    "reversed" => modifier = modifier | Modifier::REVERSED,
                    "hidden" => modifier = modifier | Modifier::HIDDEN,
                    "crossed_out" => modifier = modifier | Modifier::CROSSED_OUT,
                    _ => unreachable!(),
                }
            }
            else {
                return Err("modifier was not a string".to_string());
            }
        }
    } else {
        return Err("modifier was not an array".to_string());
    }

    Ok(modifier)
}