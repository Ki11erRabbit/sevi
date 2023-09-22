
use std::collections::HashMap;

use crate::models::key::{Key, KeyEvent, KeyModifiers};





pub struct ModeKeybindings {
    bindings: HashMap<String, HashMap<Vec<KeyEvent>, String>>,
}


impl ModeKeybindings {



    pub fn get(&mut self, mode: &str, keys: &Vec<KeyEvent>) -> Option<String> {
        match self.bindings.get(mode) {
            Some(mode_bindings) => {
                match mode_bindings.get(keys) {
                    Some(command) => Some(command.to_string()),
                    None => None,
                }
            },
            None => None,
        }
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
                        key: Key::Right,
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
                        key: Key::Left,
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
                        key: Key::Down,
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
                    bindings.insert(vec![KeyEvent {
                        key: Key::Up,
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
                }], "up_start".to_string());
                // Down one Line at Start
                bindings.insert(vec![KeyEvent {
                    key: Key::Char('-'),
                    modifiers: KeyModifiers::NONE,
                }], "down_start".to_string());
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
                    bindings.insert(vec![KeyEvent {
                        key: Key::Home,
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
                    bindings.insert(vec![KeyEvent {
                        key: Key::End,
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
                    bindings.insert(vec![KeyEvent {
                        key: Key::End,
                        modifiers: KeyModifiers::NONE,
                    }], "goto_line".to_string());
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

                    // Insert Bellow
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('o'),
                        modifiers: KeyModifiers::NONE,
                    }], "insert_bellow".to_string());
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
                        }], "copy_to_end".to_string());

                    }

                }

                // Cut
                {
                    // Cut Char
                    {
                        bindings.insert(vec![KeyEvent { 
                            key: Key::Char('d'), 
                            modifiers: KeyModifiers::NONE,
                        }, KeyEvent {
                            key: Key::Char('d'),
                            modifiers: KeyModifiers::NONE,
                        }], "cut_char".to_string());

                        bindings.insert(vec![KeyEvent { 
                            key: Key::Char('D'), 
                            modifiers: KeyModifiers::NONE,
                        }], "cut_char".to_string());
                    }
                    // Cut Line
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('d'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('l'),
                        modifiers: KeyModifiers::NONE,
                    }], "cut_line".to_string());

                    // Cut Word
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('d'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('w'),
                        modifiers: KeyModifiers::NONE,
                    }], "cut_word".to_string());

                    // Cut to Next Word
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('d'),
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
                        key: Key::Char('d'),
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
                            key: Key::Char('d'),
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

                }

                // Delete
                {
                    // Delete Char
                    {
                        bindings.insert(vec![KeyEvent { 
                            key: Key::Char('x'),  
                            modifiers: KeyModifiers::NONE, 
                        }, KeyEvent {
                            key: Key::Char('x'),
                            modifiers: KeyModifiers::NONE,
                        }], "delete_char".to_string());

                        bindings.insert(vec![KeyEvent { 
                            key: Key::Char('X'),  
                            modifiers: KeyModifiers::NONE, 
                        }], "delete_char".to_string());
                    }
                    // Delete Line
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('x'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('l'),
                        modifiers: KeyModifiers::NONE,
                    }], "delete_line".to_string());

                    // Delete Word
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('x'),
                        modifiers: KeyModifiers::NONE,
                    }, KeyEvent {
                        key: Key::Char('w'),
                        modifiers: KeyModifiers::NONE,
                    }], "delete_word".to_string());

                    // Delete to Next Word
                    bindings.insert(vec![KeyEvent {
                        key: Key::Char('x'),
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
                        key: Key::Char('x'),
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
                            key: Key::Char('x'),
                            modifiers: KeyModifiers::NONE,
                        }, KeyEvent {
                            key: Key::Char('$'),
                            modifiers: KeyModifiers::NONE,
                        }], "delete_to_end_line".to_string());
                        bindings.insert(vec![KeyEvent {
                            key: Key::Char('x'),
                            modifiers: KeyModifiers::NONE,
                        }, KeyEvent {
                            key: Key::Char('e'),
                            modifiers: KeyModifiers::NONE,
                        }], "delete_to_end".to_string());

                    }

                }

            }
            
            // Pane Management
            {
            }

            // Tab Management
            {
            }
            
            // Register Management
            {
            }
            
            // Jump Management
            {
            }


            // Misc Commands
            {
                // Cancel
                bindings.insert(vec![KeyEvent {
                    key: Key::Esc,
                    modifiers: KeyModifiers::NONE,
                }], "cancel".to_string());
                
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

        }

        bindings
    }
}


