
use std::collections::HashMap;
use std::fs::File;
use std::{fmt, io};
use std::fmt::Formatter;
use std::io::{Read, Write};

use crate::models::key::{Key, key_event_to_string, KeyEvent, KeyModifiers};


impl fmt::Display for ModeKeybindings {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[Universal]\n")?;

        {
            let mut grouped_bindings: HashMap<&String, Vec<&Vec<KeyEvent>>> = HashMap::new();
            for (keys, command) in &self.universal_bindings {
                match grouped_bindings.get_mut(command) {
                    Some(keys_vec) => {
                        keys_vec.push(keys);
                    },
                    None => {
                        grouped_bindings.insert(command, vec![keys]);
                    },
                }
            }

            for (command, keys_vec) in grouped_bindings {
                write!(f, "{} = ", command)?;

                if keys_vec.len() == 1 {
                    write!(f, "{}\n", keys_to_string(&keys_vec[0]))?;
                } else if keys_vec.len() > 1 {
                    write!(f, "[")?;

                    let output = keys_vec.iter().map(|keys| keys_to_string(keys)).collect::<Vec<String>>().join(", ");

                    write!(f, "{}]\n", output)?;
                }
            }


        }


        for (mode, bindings) in &self.bindings {
            write!(f, "\n[{}]\n", mode)?;

            let mut grouped_bindings: HashMap<&String, Vec<&Vec<KeyEvent>>> = HashMap::new();

            for (keys, command) in bindings {
                match grouped_bindings.get_mut(command) {
                    Some(keys_vec) => {
                        keys_vec.push(keys);
                    },
                    None => {
                        grouped_bindings.insert(command, vec![keys]);
                    },
                }

                //write!(f, "{} = {}\n", command, keys_to_string(keys))?;
            }

            for (command, keys_vec) in grouped_bindings {
                write!(f, "{} = ", command)?;

                if keys_vec.len() == 1 {
                    write!(f, "{}\n", keys_to_string(&keys_vec[0]))?;
                } else if keys_vec.len() > 1 {
                    write!(f, "[")?;

                    let output = keys_vec.iter().map(|keys| keys_to_string(keys)).collect::<Vec<String>>().join(", ");


                    write!(f, "{}]\n", output)?;
                }
            }
        }
        Ok(())

    }
}


pub type Keys= Vec<KeyEvent>;

#[derive(Debug)]
pub struct ModeKeybindings {
    universal_bindings: HashMap<Vec<KeyEvent>, String>,
    bindings: HashMap<String, HashMap<Vec<KeyEvent>, String>>,
}


impl Default for ModeKeybindings {
    fn default() -> ModeKeybindings {
        let mut bindings = HashMap::new();

        bindings.insert("Normal".to_string(), ModeKeybindings::generate_normal_keybindings());
        bindings.insert("Insert".to_string(), ModeKeybindings::generate_insert_keybindings());
        bindings.insert("Command".to_string(), ModeKeybindings::generate_command_keybindings());
        bindings.insert("Selection".to_string(), ModeKeybindings::generate_selection_keybindings());
        bindings.insert("Search".to_string(), ModeKeybindings::generate_search_keybindings());

        ModeKeybindings {
            universal_bindings: ModeKeybindings::generate_universal_keybindings(),
            bindings,
        }
    }
}



impl ModeKeybindings {

    pub fn new() -> ModeKeybindings {
        let mut bindings = ModeKeybindings::default();

        if crate::arg_parser::IGNORE_USER_SETTINGS.load(std::sync::atomic::Ordering::Relaxed) {
            return bindings;
        }

        let xdg_dirs = xdg::BaseDirectories::with_prefix("sevi").unwrap();
        let user_bindings_path = xdg_dirs.place_config_file("keybindings.toml").expect("Failed to create user keybindings file");
        match File::open(user_bindings_path) {
            Err(_) => {},
            Ok(mut user_bindings) => {
                let mut string = String::new();
                user_bindings.read_to_string(&mut string).expect("Failed to read user keybindings file");

                let user_bindings = ModeKeybindings::load_user_bindings(&string);

                bindings.merge_bindings(user_bindings);
            },
        }
        bindings
    }

    pub fn create_default_config_file() -> io::Result<()> {
        let bindings = ModeKeybindings::default();

        let xdg_dirs = xdg::BaseDirectories::with_prefix("sevi").unwrap();
        let user_bindings_path = xdg_dirs.place_config_file("keybindings.toml").expect("Failed to create user keybindings file");

        let mut file = File::create(user_bindings_path)?;
        file.write_all(bindings.to_string().as_bytes())
    }


    pub fn get(&mut self, mode: &String, keys: &Vec<KeyEvent>) -> Option<&String> {


        match self.bindings.get(mode) {
            Some(mode_bindings) => {
                match mode_bindings.get(keys) {
                    Some(command) => Some(command),
                    None => {
                        match self.universal_bindings.get(keys) {
                            Some(command) => Some(command),
                            None => None,
                        }
                    }
                }
            },
            None => {
                match self.universal_bindings.get(keys) {
                    Some(command) => Some(command),
                    None => None,
                }
            },
        }
    }

    pub fn get_ignore_universal(&mut self, mode: &String, keys: &Vec<KeyEvent>) -> Option<&String> {
        match self.bindings.get(mode) {
            Some(mode_bindings) => {
                mode_bindings.get(keys)
            },
            None => None,
        }
    }

    pub fn set_universal(&mut self, keys: Vec<KeyEvent>, command: &str) {
        self.universal_bindings.insert(keys, command.to_string());
    }

    pub fn set(&mut self, mode: &str, keys: Vec<KeyEvent>, command: &str) {
        match self.bindings.get_mut(mode) {
            Some(mode_bindings) => {
                mode_bindings.insert(keys, command.to_string());
            },
            None => {
                let mut mode_bindings = HashMap::new();
                mode_bindings.insert(keys, command.to_string());
                self.bindings.insert(mode.to_string(), mode_bindings);
            },
        }
    }

    fn merge_bindings(&mut self, other: Self) {
        for (keys, command) in other.universal_bindings {
            self.universal_bindings.insert(keys, command);
        }

        for (mode, bindings) in other.bindings {
            match self.bindings.get_mut(&mode) {
                Some(mode_bindings) => {
                    for (keys, command) in bindings {
                        mode_bindings.insert(keys, command);
                    }
                },
                None => {
                    self.bindings.insert(mode, bindings);
                },
            }
        }
    }


    fn generate_universal_keybindings() -> HashMap<Vec<KeyEvent>, String> {
        let mut bindings = HashMap::new();

        // Movement
        {
            // Axial Movement
            {
                // Right
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Right,
                        modifiers: KeyModifiers::NONE,
                    }], "right".to_string());
                }
                // Left
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Left,
                        modifiers: KeyModifiers::NONE,
                    }], "left".to_string());
                }
                // Down
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Down,
                        modifiers: KeyModifiers::NONE,
                    }], "down".to_string());
                }
                // Up
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Up,
                        modifiers: KeyModifiers::NONE,
                    }], "up".to_string());
                }
            }
            {
                // Start of File
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Home,
                        modifiers: KeyModifiers::NONE,
                    }], "start_of_file".to_string());
                }
                // End of File
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::End,
                        modifiers: KeyModifiers::NONE,
                    }], "end_of_file".to_string());
                }
                // Page Up
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::PageUp,
                        modifiers: KeyModifiers::NONE,
                    }], "page_up".to_string());
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('b'),
                        modifiers: KeyModifiers::CTRL,
                    }], "page_up".to_string());
                }
                // Page Down
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::PageDown,
                        modifiers: KeyModifiers::NONE,
                    }], "page_down".to_string());
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('f'),
                        modifiers: KeyModifiers::CTRL,
                    }], "page_down".to_string());
                }
                // Half Page Up
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('u'),
                        modifiers: KeyModifiers::CTRL,
                    }], "half_page_up".to_string());
                }
                // Half Page Down
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('d'),
                        modifiers: KeyModifiers::CTRL,
                    }], "half_page_down".to_string());
                }

            }

        }
        // Pane Management
        {
            // Splitting
            {
                // Split Horizontal
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Char('s'),
                    modifiers: KeyModifiers::NONE,

                }], "split_horizontal".to_string());

                // Split Vertical
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Char('v'),
                    modifiers: KeyModifiers::NONE,

                }], "split_vertical".to_string());

            }

            // Pane Left
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Char('h'),
                    modifiers: KeyModifiers::NONE,
                }], "pane_left".to_string());
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Left,
                    modifiers: KeyModifiers::NONE,
                }], "pane_left".to_string());
            }
            // Pane Right
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Char('l'),
                    modifiers: KeyModifiers::NONE,
                }], "pane_right".to_string());
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Right,
                    modifiers: KeyModifiers::NONE,
                }], "pane_right".to_string());
            }
            // Pane Up
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Char('k'),
                    modifiers: KeyModifiers::NONE,
                }], "pane_up".to_string());
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Up,
                    modifiers: KeyModifiers::NONE,
                }], "pane_up".to_string());
            }
            // Pane Down
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Char('j'),
                    modifiers: KeyModifiers::NONE,
                }], "pane_down".to_string());
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Down,
                    modifiers: KeyModifiers::NONE,
                }], "pane_down".to_string());
            }


        }

        // Tab Management
        {
            // New Tab
            {
                // New Tab
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('t'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Char('t'),
                    modifiers: KeyModifiers::NONE,
                }], "new_tab".to_string());

                // New Tab with Current Pane
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('t'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Char('T'),
                    modifiers: KeyModifiers::NONE,
                }], "new_tab_current_pane".to_string());
            }

            // Tab Left
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('t'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Char('h'),
                    modifiers: KeyModifiers::NONE,
                }], "tab_left".to_string());

                bindings.insert(vec![KeyEvent {
                    key: Key::Char('t'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Left,
                    modifiers: KeyModifiers::NONE,
                }], "tab_left".to_string());
            }

            // Tab Right
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('t'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Char('l'),
                    modifiers: KeyModifiers::NONE,
                }], "tab_right".to_string());

                bindings.insert(vec![KeyEvent {
                    key: Key::Char('t'),
                    modifiers: KeyModifiers::CTRL,
                }, KeyEvent {
                    key: Key::Right,
                    modifiers: KeyModifiers::NONE,
                }], "tab_right".to_string());
            }
        }


        // Jump Management
        {
            // Jump Forwards 
            bindings.insert(vec![KeyEvent {
                key: Key::Char('i'),
                modifiers: KeyModifiers::CTRL,
            }], "jump_forwards".to_string());

            // Jump Backwards
            bindings.insert(vec![KeyEvent {
                key: Key::Char('o'),
                modifiers: KeyModifiers::CTRL,
            }], "jump_backwards".to_string());
        }


        // Misc Commands
        {
            // Cancel
            bindings.insert(vec![KeyEvent {
                key: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }], "cancel".to_string());
        }



        bindings
    }

    fn generate_normal_keybindings() -> HashMap<Vec<KeyEvent>, String> {
        let mut bindings = HashMap::new();

        // Movement
        {
            // Axial Movement
            {
                // Right
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('l'),
                        modifiers: KeyModifiers::NONE,
                    }], "right".to_string());
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char(' '),
                        modifiers: KeyModifiers::NONE,
                    }], "right".to_string());
                }
                // Left
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('h'),
                        modifiers: KeyModifiers::NONE,
                    }], "left".to_string());
                    bindings.insert(vec![KeyEvent {
                        key: Key::Backspace,
                        modifiers: KeyModifiers::NONE,
                    }], "left".to_string());
                }
                // Down
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('j'),
                        modifiers: KeyModifiers::NONE,
                    }], "down".to_string());
                    bindings.insert(vec![KeyEvent {
                        key: Key::Enter,
                        modifiers: KeyModifiers::NONE,
                    }], "down".to_string());
                }
                // Up
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('k'),
                        modifiers: KeyModifiers::NONE,
                    }], "up".to_string());
                }
            }
            // Line Movement
            {
                // Start of Line
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('0'),
                    modifiers: KeyModifiers::NONE,
                }], "start_of_line".to_string());
                // End of Line
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('$'),
                    modifiers: KeyModifiers::NONE,
                }], "end_of_line".to_string());

                // Up one Line at Start
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('-'),
                    modifiers: KeyModifiers::NONE,
                }], "up_line_start".to_string());
                // Down one Line at Start
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('+'),
                    modifiers: KeyModifiers::NONE,
                }], "down_line_start".to_string());
            }

            // File Movement
            {
                // Start of File
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('g'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('g'),
                        modifiers: KeyModifiers::NONE,
                    }], "start_of_file".to_string());
                }
                // End of File
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('G'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('G'),
                        modifiers: KeyModifiers::NONE,
                    }], "end_of_file".to_string());
                }
                // Goto Line
                // Requires a number to be entered otherwise it should get ignored
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('G'),
                        modifiers: KeyModifiers::NONE,
                    }], "goto_line".to_string());
                }
                // Page Up
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('b'),
                        modifiers: KeyModifiers::CTRL,
                    }], "page_up".to_string());
                }
                // Page Down
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('f'),
                        modifiers: KeyModifiers::CTRL,
                    }], "page_down".to_string());
                }
                // Half Page Up
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('u'),
                        modifiers: KeyModifiers::CTRL,
                    }], "half_page_up".to_string());
                }
                // Half Page Down
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('d'),
                        modifiers: KeyModifiers::CTRL,
                    }], "half_page_down".to_string());
                }

            }
            // Word Movement
            {
                // Next Word
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('w'),
                        modifiers: KeyModifiers::NONE,
                    }], "next_word_front".to_string());

                    // Next word back
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('B'),
                        modifiers: KeyModifiers::NONE,
                    }], "next_word_back".to_string());
                }
                // Previous Word
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('W'),
                        modifiers: KeyModifiers::NONE,
                    }], "previous_word_front".to_string());

                    // Previous word back
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('b'),
                        modifiers: KeyModifiers::NONE,
                    }], "previous_word_back".to_string());
                }
            }
            // Special Movement
            {
                // Goto Other Pair
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('%'),
                    modifiers: KeyModifiers::NONE,
                }], "goto_pair".to_string());

                // Jump Paragraph
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('}'),
                    modifiers: KeyModifiers::NONE,
                }], "jump_paragraph".to_string());

                // Jump Paragraph Back
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('{'),
                    modifiers: KeyModifiers::NONE,
                }], "jump_paragraph_back".to_string());

            }
        }
        // Mode Change
        {
            // Insert Mode
            {
                // Insert Before Cursor
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('i'),
                    modifiers: KeyModifiers::NONE,
                }], "insert_before".to_string());
                // Insert After Cursor
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('a'),
                    modifiers: KeyModifiers::NONE,
                }], "insert_after".to_string());

                // Insert at Beginning of Line
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('I'),
                    modifiers: KeyModifiers::NONE,
                }], "insert_start".to_string());
                // Insert End of Line
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('A'),
                    modifiers: KeyModifiers::NONE,
                }], "insert_end".to_string());

                // Insert Below
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('o'),
                    modifiers: KeyModifiers::NONE,
                }], "insert_below".to_string());
                // Insert Above
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('O'),
                    modifiers: KeyModifiers::NONE,
                }], "insert_above".to_string());

            }

            // Command Mode
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char(':'),
                    modifiers: KeyModifiers::NONE,
                }], "command_mode".to_string());
            }
            
            // Selection (Visual) Mode
            {
                // Selection Mode
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('v'),
                    modifiers: KeyModifiers::NONE,
                }], "selection_mode".to_string());
                // Selection Block
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('V'),
                    modifiers: KeyModifiers::NONE,
                }], "selection_mode_line".to_string());
                // Selection Mode Block
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('v'),
                    modifiers: KeyModifiers::CTRL,
                }], "selection_mode_block".to_string());
            }
            
            // Search Mode
            {
                // Search Mode
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('/'),
                    modifiers: KeyModifiers::NONE,
                }], "search_mode_down".to_string());

                // Search Mode
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('?'),
                    modifiers: KeyModifiers::NONE,
                }], "search_mode_up".to_string());
            }

            // Replace Mode
            {
                // Replace Mode
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('R'),
                    modifiers: KeyModifiers::NONE,
                }], "replace_mode".to_string());
            }
        }

        // Copy, Cut, Paste
        {
            // Paste
            {
                // Paste Before Cursor
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('P'),
                    modifiers: KeyModifiers::NONE,
                }], "paste_before".to_string());
                // Paste After Cursor
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('p'),
                    modifiers: KeyModifiers::NONE,
                }], "paste_after".to_string());
            }

            // Copy
            {

                // Copy Char
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('y'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('y'),
                        modifiers: KeyModifiers::NONE,
                    }], "copy_char".to_string());

                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('Y'),
                        modifiers: KeyModifiers::NONE,
                    }], "copy_char".to_string());
                }

                // Copy Line
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('y'),
                    modifiers: KeyModifiers::NONE,
                }, KeyEvent {
                    key: Key::Char('l'),
                    modifiers: KeyModifiers::NONE,
                }], "copy_line".to_string());

                // Copy Word
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('y'),
                    modifiers: KeyModifiers::NONE,
                }, KeyEvent {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::NONE,
                }], "copy_word".to_string());

                // Copy to Next Word
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('y'),
                    modifiers: KeyModifiers::NONE,
                }, KeyEvent {
                    key: Key::Char('n'),
                    modifiers: KeyModifiers::NONE, 
                }, KeyEvent {
                    key: Key::Char('w'), 
                    modifiers: KeyModifiers::NONE,
                }], "copy_to_next_word".to_string());

                // Copy to Previous Word
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('y'),
                    modifiers: KeyModifiers::NONE,
                }, KeyEvent {
                    key: Key::Char('p'),
                    modifiers: KeyModifiers::NONE, 
                }, KeyEvent {
                    key: Key::Char('w'), 
                    modifiers: KeyModifiers::NONE,
                }], "copy_to_prev_word".to_string());

                // Copy to End of Line
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('y'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('$'),
                        modifiers: KeyModifiers::NONE,
                    }], "copy_to_end_line".to_string());
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('y'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('e'),
                        modifiers: KeyModifiers::NONE,
                    }], "copy_to_end_line".to_string());

                }
                // Copy to Start of Line
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('y'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('s'),
                        modifiers: KeyModifiers::NONE,
                    }], "copy_to_start_line".to_string());

                }

            }

            // Cut
            {
                // Cut Char
                {
                    bindings.insert(vec![KeyEvent { 
                        key: Key::Char('x'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('x'),
                        modifiers: KeyModifiers::NONE,
                    }], "cut_char".to_string());

                    bindings.insert(vec![KeyEvent { 
                        key: Key::Char('X'),
                        modifiers: KeyModifiers::NONE,
                    }], "cut_char".to_string());
                }
                // Cut Line
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('x'),
                    modifiers: KeyModifiers::NONE,
                }, KeyEvent {
                    key: Key::Char('l'),
                    modifiers: KeyModifiers::NONE,
                }], "cut_line".to_string());

                // Cut Word
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('x'),
                    modifiers: KeyModifiers::NONE,
                }, KeyEvent {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::NONE,
                }], "cut_word".to_string());

                // Cut to Next Word
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('x'),
                    modifiers: KeyModifiers::NONE,
                }, KeyEvent {
                    key: Key::Char('n'),
                    modifiers: KeyModifiers::NONE, 
                }, KeyEvent {
                    key: Key::Char('w'), 
                    modifiers: KeyModifiers::NONE,
                }], "cut_to_next_word".to_string());

                // Cut to Previous Word
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('x'),
                    modifiers: KeyModifiers::NONE,
                }, KeyEvent {
                    key: Key::Char('p'),
                    modifiers: KeyModifiers::NONE, 
                }, KeyEvent {
                    key: Key::Char('w'), 
                    modifiers: KeyModifiers::NONE,
                }], "cut_to_prev_word".to_string());

                // Cut to End of Line
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('x'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('$'),
                        modifiers: KeyModifiers::NONE,
                    }], "cut_to_end_line".to_string());
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('d'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('e'),
                        modifiers: KeyModifiers::NONE,
                    }], "cut_to_end".to_string());

                }
                // Cut to Start of Line
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('x'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('s'),
                        modifiers: KeyModifiers::NONE,
                    }], "cut_to_end".to_string());

                }

            }

            // Delete
            {
                // Delete Char
                {
                    bindings.insert(vec![KeyEvent { 
                        key: Key::Char('d'),
                        modifiers: KeyModifiers::NONE, 
                    }, KeyEvent {
                        key: Key::Char('d'),
                        modifiers: KeyModifiers::NONE,
                    }], "delete_char".to_string());

                    bindings.insert(vec![KeyEvent { 
                        key: Key::Char('D'),
                        modifiers: KeyModifiers::NONE, 
                    }], "delete_char".to_string());
                }
                // Delete Line
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('d'),
                    modifiers: KeyModifiers::NONE,
                }, KeyEvent {
                    key: Key::Char('l'),
                    modifiers: KeyModifiers::NONE,
                }], "delete_line".to_string());

                // Delete Word
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('d'),
                    modifiers: KeyModifiers::NONE,
                }, KeyEvent {
                    key: Key::Char('w'),
                    modifiers: KeyModifiers::NONE,
                }], "delete_word".to_string());

                // Delete to Next Word
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('d'),
                    modifiers: KeyModifiers::NONE,
                }, KeyEvent {
                    key: Key::Char('n'),
                    modifiers: KeyModifiers::NONE, 
                }, KeyEvent {
                    key: Key::Char('w'), 
                    modifiers: KeyModifiers::NONE,
                }], "delete_to_next_word".to_string());

                // Delete to Previous Word
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('d'),
                    modifiers: KeyModifiers::NONE,
                }, KeyEvent {
                    key: Key::Char('p'),
                    modifiers: KeyModifiers::NONE, 
                }, KeyEvent {
                    key: Key::Char('w'), 
                    modifiers: KeyModifiers::NONE,
                }], "delete_to_prev_word".to_string());

                // Delete to End of Line
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('d'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('$'),
                        modifiers: KeyModifiers::NONE,
                    }], "delete_to_end_line".to_string());
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('d'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('e'),
                        modifiers: KeyModifiers::NONE,
                    }], "delete_to_end".to_string());

                }
                // Delete to Start of Line
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('d'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('s'),
                        modifiers: KeyModifiers::NONE,
                    }], "delete_to_start".to_string());

                }

            }

        }
        
        
        // Register Management
        {
        }
        
        // Jump Management
        {
        }

        // Completion
        {
            bindings.insert(vec![KeyEvent {
                key: Key::Char('n'),
                modifiers: KeyModifiers::CTRL,
            }], "open_completion".to_string());
        }

        // Misc Commands
        {
            // Undo
            bindings.insert(vec![KeyEvent {
                key: Key::Char('u'),
                modifiers: KeyModifiers::NONE,
            }], "undo".to_string());
            // Redo
            bindings.insert(vec![KeyEvent {
                key: Key::Char('r'),
                modifiers: KeyModifiers::CTRL,
            }], "redo".to_string());

            // Replace
            bindings.insert(vec![KeyEvent {
                key: Key::Char('r'),
                modifiers: KeyModifiers::NONE,
            }], "replace".to_string());
        }

        

        bindings
    }

    fn generate_insert_keybindings() -> HashMap<Vec<KeyEvent>, String> {
        let mut bindings = HashMap::new();

        //todo: add conditional compilation for gui tab completion
        // Completion
        {
            bindings.insert(vec![KeyEvent {
                key: Key::Char('n'),
                modifiers: KeyModifiers::CTRL,
            }], "open_completion".to_string());
        }
        // Backspace and Delete
        {
            // Backspace
            bindings.insert(vec![KeyEvent {
                key: Key::Backspace,
                modifiers: KeyModifiers::NONE,
            }], "backspace".to_string());

            // Delete
            bindings.insert(vec![KeyEvent {
                key: Key::Delete,
                modifiers: KeyModifiers::NONE,
            }], "delete".to_string());
        }
        // cancel
        {
            bindings.insert(vec![KeyEvent {
                key: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }], "cancel".to_string());
        }
        // Enter Newline
        {
            bindings.insert(vec![KeyEvent {
                key: Key::Enter,
                modifiers: KeyModifiers::NONE,
            }], "newline".to_string());
        }
        // Tab
        {
            bindings.insert(vec![KeyEvent {
                key: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }], "tab".to_string());
        }

        bindings
    }

    fn generate_command_keybindings() -> HashMap<Vec<KeyEvent>, String> {
        let mut bindings = HashMap::new();

        // Move to ends
        {
            // Start of Line
            bindings.insert(vec![KeyEvent {
                key: Key::Up,
                modifiers: KeyModifiers::NONE,
            }], "start".to_string());

            // End of Line
            bindings.insert(vec![KeyEvent {
                key: Key::Down,
                modifiers: KeyModifiers::NONE,
            }], "end".to_string());
        }
        // Execute
        {
            bindings.insert(vec![KeyEvent {
                key: Key::Enter,
                modifiers: KeyModifiers::NONE,
            }], "execute".to_string());
        }
        // Backspace and delete
        {
            bindings.insert(vec![KeyEvent {
                key: Key::Backspace,
                modifiers: KeyModifiers::NONE,
            }], "backspace".to_string());

            bindings.insert(vec![KeyEvent {
                key: Key::Delete,
                modifiers: KeyModifiers::NONE,
            }], "delete".to_string());
        }
    
        bindings
    }

    fn generate_selection_keybindings() -> HashMap<Vec<KeyEvent>, String> {
        let mut bindings = HashMap::new();

        // Copy, Cut, Delete, Paste
        {
            // Paste
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('p'),
                    modifiers: KeyModifiers::NONE,
                }], "paste".to_string());
            }

            // Copy
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('y'),
                    modifiers: KeyModifiers::NONE,
                }], "copy".to_string());
            }

            // Cut
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('x'),
                    modifiers: KeyModifiers::NONE,
                }], "cut".to_string());
            }

            // Delete
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('d'),
                    modifiers: KeyModifiers::NONE,
                }], "delete".to_string());
            }
        }
        // Movement
        {
            // Axial Movement
            {
                // Right
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('l'),
                        modifiers: KeyModifiers::NONE,
                    }], "right".to_string());
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char(' '),
                        modifiers: KeyModifiers::NONE,
                    }], "right".to_string());
                }
                // Left
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('h'),
                        modifiers: KeyModifiers::NONE,
                    }], "left".to_string());
                    bindings.insert(vec![KeyEvent {
                        key: Key::Backspace,
                        modifiers: KeyModifiers::NONE,
                    }], "left".to_string());
                }
                // Down
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('j'),
                        modifiers: KeyModifiers::NONE,
                    }], "down".to_string());
                    bindings.insert(vec![KeyEvent {
                        key: Key::Enter,
                        modifiers: KeyModifiers::NONE,
                    }], "down".to_string());
                }
                // Up
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('k'),
                        modifiers: KeyModifiers::NONE,
                    }], "up".to_string());
                }
            }
            // Line Movement
            {
                // Start of Line
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('0'),
                    modifiers: KeyModifiers::NONE,
                }], "start_of_line".to_string());
                // End of Line
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('$'),
                    modifiers: KeyModifiers::NONE,
                }], "end_of_line".to_string());

                // Up one Line at Start
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('+'),
                    modifiers: KeyModifiers::NONE,
                }], "up_line_start".to_string());
                // Down one Line at Start
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('-'),
                    modifiers: KeyModifiers::NONE,
                }], "down_line_start".to_string());
            }

            // File Movement
            {
                // Start of File
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('g'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('g'),
                        modifiers: KeyModifiers::NONE,
                    }], "start_of_file".to_string());
                }
                // End of File
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('G'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('G'),
                        modifiers: KeyModifiers::NONE,
                    }], "end_of_file".to_string());
                }
                // Goto Line
                // Requires a number to be entered otherwise it should get ignored
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('G'),
                        modifiers: KeyModifiers::NONE,
                    }], "goto_line".to_string());
                }
                // Page Up
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('b'),
                        modifiers: KeyModifiers::CTRL,
                    }], "page_up".to_string());
                }
                // Page Down
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('f'),
                        modifiers: KeyModifiers::CTRL,
                    }], "page_down".to_string());
                }
                // Half Page Up
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('u'),
                        modifiers: KeyModifiers::CTRL,
                    }], "half_page_up".to_string());
                }
                // Half Page Down
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('d'),
                        modifiers: KeyModifiers::CTRL,
                    }], "half_page_down".to_string());
                }

            }
            // Word Movement
            {
                // Next Word
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('w'),
                        modifiers: KeyModifiers::NONE,
                    }], "next_word_front".to_string());

                    // Next word back
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('B'),
                        modifiers: KeyModifiers::NONE,
                    }], "next_word_back".to_string());
                }
                // Previous Word
                {
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('W'),
                        modifiers: KeyModifiers::NONE,
                    }], "previous_word_front".to_string());

                    // Previous word back
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('b'),
                        modifiers: KeyModifiers::NONE,
                    }], "previous_word_back".to_string());
                }
            }
            // Special Movement
            {
                // Goto Other Pair
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('%'),
                    modifiers: KeyModifiers::NONE,
                }], "goto_pair".to_string());

                // Jump Paragraph
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('}'),
                    modifiers: KeyModifiers::NONE,
                }], "jump_paragraph".to_string());

                // Jump Paragraph Back
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('{'),
                    modifiers: KeyModifiers::NONE,
                }], "jump_paragraph_back".to_string());

            }
        }

        bindings
    }

    fn generate_search_keybindings() -> HashMap<Vec<KeyEvent>, String>  {
        let mut bindings = HashMap::new();

        // Movement between matches
        {
            // Next Match
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('n'),
                    modifiers: KeyModifiers::CTRL,
                }], "next_match".to_string());

                bindings.insert(vec![KeyEvent {
                    key: Key::Enter,
                    modifiers: KeyModifiers::NONE,
                }], "next_match".to_string());
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('l'),
                    modifiers: KeyModifiers::CTRL,
                }], "next_match".to_string());
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('j'),
                    modifiers: KeyModifiers::CTRL,
                }], "next_match".to_string());
            }
            // Previous Match
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('N'),
                    modifiers: KeyModifiers::CTRL,
                }], "previous_match".to_string());

                bindings.insert(vec![KeyEvent {
                    key: Key::Backspace,
                    modifiers: KeyModifiers::NONE,
                }], "previous_match".to_string());
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('k'),
                    modifiers: KeyModifiers::CTRL,
                }], "previous_match".to_string());
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('h'),
                    modifiers: KeyModifiers::CTRL,
                }], "previous_match".to_string());
            }



        }



        // Escape
        {
            bindings.insert(vec![KeyEvent {
                key: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }], "cancel".to_string());
        }
        // Backspace and Delete
        {
            bindings.insert(vec![KeyEvent {
                key: Key::Backspace,
                modifiers: KeyModifiers::NONE,
            }], "backspace".to_string());
            bindings.insert(vec![KeyEvent {
                key: Key::Delete,
                modifiers: KeyModifiers::NONE,
            }], "delete".to_string());
        }
        // Copy, Cut, Delete, Paste
        {
            // Paste
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('p'),
                    modifiers: KeyModifiers::CTRL,
                }], "paste".to_string());
            }

            // Copy
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('y'),
                    modifiers: KeyModifiers::CTRL,
                }], "copy".to_string());
            }

            // Cut
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('x'),
                    modifiers: KeyModifiers::CTRL,
                }], "cut".to_string());
            }

            // Delete
            {
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('d'),
                    modifiers: KeyModifiers::CTRL,
                }], "delete_search".to_string());
            }
        }

        bindings
    }

    fn load_user_bindings(file_string: &str) -> ModeKeybindings {
        let table: toml::Value = toml::from_str(file_string).expect("failed to parse user keybindings");

        let possible_bindings = [
            "right",
            "left",
            "up",
            "down",
            "start_of_file",
            "end_of_file",
            "page_up",
            "page_down",
            "half_page_up",
            "half_page_down",
            "split_horizontal",
            "split_vertical",
            "pane_left",
            "pane_right",
            "pane_up",
            "pane_down",
            "new_tab",
            "new_tab_current_pane",
            "tab_left",
            "tab_right",
            "jump_forwards",
            "jump_backwards",
            "cancel",
            "start_of_line",
            "end_of_line",
            "up_line_start",
            "down_line_start",
            "goto_line",
            "next_word_front",
            "next_word_back",
            "previous_word_front",
            "previous_word_back",
            "goto_pair",
            "jump_paragraph",
            "jump_paragraph_back",
            "insert_before",
            "insert_after",
            "insert_start",
            "insert_end",
            "insert_below",
            "insert_above",
            "command_mode",
            "selection_mode",
            "selection_mode_line",
            "selection_mode_block",
            "search_mode_down",
            "search_mode_up",
            "replace_mode",
            "paste_before",
            "paste_after",
            "copy_char",
            "copy_line",
            "copy_word",
            "copy_to_next_word",
            "copy_to_prev_word",
            "copy_to_end_line",
            "copy_to_start_line",
            "cut_char",
            "cut_line",
            "cut_word",
            "cut_to_next_word",
            "cut_to_prev_word",
            "cut_to_end_line",
            "cut_to_start_line",
            "delete_char",
            "delete_line",
            "delete_word",
            "delete_to_next_word",
            "delete_to_prev_word",
            "delete_to_end_line",
            "delete_to_start_line",
            "open_completion",
            "undo",
            "redo",
            "replace",
            "backspace",
            "delete",
            "newline",
            "tab",
            "start",
            "end",
            "execute",
            "paste",
            "copy",
            "cut",
            "next_match",
            "previous_match",
            "delete_search",
        ];

        let universal_bindings = match table.get("Universal") {
            Some(value) => {
                parse_keybindings(value, &possible_bindings)
            },
            None => {
                HashMap::new()
            },
        };

        let normal_bindings= match table.get("Normal") {
            Some(value) => {
                parse_keybindings(value, &possible_bindings)
            },
            None => {
                HashMap::new()
            },
        };

        let insert_bindings = match table.get("Insert") {
            Some(value) => {
                parse_keybindings(value, &possible_bindings)
            },
            None => {
                HashMap::new()
            },
        };

        let command_bindings = match table.get("Command") {
            Some(value) => {
                parse_keybindings(value, &possible_bindings)
            },
            None => {
                HashMap::new()
            },
        };

        let selection_bindings = match table.get("Selection") {
            Some(value) => {
                parse_keybindings(value, &possible_bindings)
            },
            None => {
                HashMap::new()
            },
        };

        let search_bindings = match table.get("Search") {
            Some(value) => {
                parse_keybindings(value, &possible_bindings)
            },
            None => {
                HashMap::new()
            },
        };

        let replace_bindings = match table.get("Replace") {
            Some(value) => {
                parse_keybindings(value, &possible_bindings)
            },
            None => {
                HashMap::new()
            },
        };

        let mut bindings = HashMap::new();

        bindings.insert("Normal".to_string(), normal_bindings);
        bindings.insert("Insert".to_string(), insert_bindings);
        bindings.insert("Command".to_string(), command_bindings);
        bindings.insert("Selection".to_string(), selection_bindings);
        bindings.insert("Search".to_string(), search_bindings);
        bindings.insert("Replace".to_string(), replace_bindings);

        ModeKeybindings {
            universal_bindings,
            bindings,
        }
    }

}

fn keys_to_string(keys: &Keys) -> String {
    let mut string = String::new();


    if keys.len() == 1 {
        let key = &keys[0];
        string.push_str(&key_event_to_string(key));
    } else {

        string.push_str("{ keys = [");

        string.push_str(&keys.iter().map(|key| {
            key_event_to_string(key)
        }).collect::<Vec<String>>().join(", "));

        string.push_str("] }");
    }


    string
}


fn parse_key(value: &toml::Value) -> Vec<KeyEvent> {
    match value {
        toml::Value::String(string) => {
            if string.len() == 1 {
                let key = string.chars().next().unwrap();
                vec![KeyEvent {
                    key: Key::Char(key),
                    modifiers: KeyModifiers::NONE,
                }]
            } else {
                let key = match string.as_str() {
                    "Space" => Key::Char(' '),
                    "Backspace" => Key::Backspace,
                    "Enter" => Key::Enter,
                    "Left" => Key::Left,
                    "Right" => Key::Right,
                    "Up" => Key::Up,
                    "Down" => Key::Down,
                    "Home" => Key::Home,
                    "End" => Key::End,
                    "PageUp" => Key::PageUp,
                    "PageDown" => Key::PageDown,
                    "Tab" => Key::Tab,
                    "BackTab" => Key::BackTab,
                    "Delete" => Key::Delete,
                    "Insert" => Key::Insert,
                    "Esc" => Key::Esc,
                    "CapsLock" => Key::CapsLock,
                    "ScrollLock" => Key::ScrollLock,
                    "NumLock" => Key::NumLock,
                    "PrintScreen" => Key::PrintScreen,
                    "Pause" => Key::Pause,
                    "Menu" => Key::Menu,
                    "KeypadBegin" => Key::KeypadBegin,
                    "F1" => Key::F(1),
                    "F2" => Key::F(2),
                    "F3" => Key::F(3),
                    "F4" => Key::F(4),
                    "F5" => Key::F(5),
                    "F6" => Key::F(6),
                    "F7" => Key::F(7),
                    "F8" => Key::F(8),
                    "F9" => Key::F(9),
                    "F10" => Key::F(10),
                    "F11" => Key::F(11),
                    "F12" => Key::F(12),
                    "F13" => Key::F(13),
                    "F14" => Key::F(14),
                    "F15" => Key::F(15),
                    "F16" => Key::F(16),
                    "F17" => Key::F(17),
                    "F18" => Key::F(18),
                    "F19" => Key::F(19),
                    "F20" => Key::F(20),
                    "F21" => Key::F(21),
                    "F22" => Key::F(22),
                    "F23" => Key::F(23),
                    "F24" => Key::F(24),
                    x => {
                        println!("unimplemented key: {}", x);
                        unimplemented!()},
                };

                vec![KeyEvent {
                    key,
                    modifiers: KeyModifiers::NONE,
                }]
            }
        },
        toml::Value::Table(table) => {

            if let Some(key) = table.get("key") {
                let key = parse_key(key);
                let mod_keys = table["mod"].as_array().expect("modifier keys were not an array").iter().map(|value| {
                    match value.as_str().unwrap() {
                        "Ctrl" => KeyModifiers::CTRL,
                        "Alt" => KeyModifiers::ALT,
                        "Shift" => KeyModifiers::SHIFT,
                        _ => unimplemented!(),
                    }
                }).fold(KeyModifiers::NONE, |acc, modifier| {
                    acc | modifier
                });

                vec![KeyEvent {
                    key: key[0].key,
                    modifiers: mod_keys,
                }]
            }
            else if let Some(keys) = table.get("keys") {
                let keys = keys.as_array().expect("keys were not an array").iter().map(|value| {
                    parse_key(value).remove(0)
                }).collect();

                keys
            }
            else {
                unreachable!()
            }

        },
        _ => unreachable!(),
    }
}

fn parse_keys(value: &toml::Value) -> Vec<Keys> {
    match value {
        toml::Value::String(_) => {
            let key = parse_key(value);
            vec![key]
        },
        toml::Value::Table(_) => {
            let key = parse_key(value);
            vec![key]
        },
        toml::Value::Array(array) => {
            array.iter().map(|value| {
                parse_key(value)
            }).collect()
        },
        _ => unreachable!(),
    }
}

fn parse_custom_binding(value: &toml::Value) -> (Keys, String) {
    let keys = parse_keys(&value["binding"]);

    let command = value["command"].as_str().unwrap().to_string();

    (keys[0].clone(), command)
}

fn parse_custom(value: &toml::Value) -> Vec<(Keys, String)> {
    let mut custom = Vec::new();

    let array = value.as_array().expect("custom keybindings were not an array");

    for value in array {
        custom.push(parse_custom_binding(value));
    }

    custom
}

fn parse_keybindings(table: &toml::Value, commands: &[&str]) -> HashMap<Keys, String> {
    let mut keybindings = HashMap::new();

    for command in commands.iter() {
        if let Some(value) = table.get(command) {
            let keys = parse_keys(value);

            for key in keys {
                keybindings.insert(key, command.to_string());
            }
        }
    }
    if let Some(value) = table.get("custom") {
        let custom = parse_custom(value);

        for (keys, command) in custom {
            keybindings.insert(keys, command);
        }
    }

    keybindings
}