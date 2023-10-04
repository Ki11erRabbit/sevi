use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::models::file::file::InsertPairs;
use crate::models::key::{Key, KeyEvent};
use crate::models::mode::{Mode, TextMode};
use crate::models::pane::TextPane;
use crate::models::settings::Settings;

pub struct MirrorMode {
    settings: Option<Rc<RefCell<Settings>>>,
    key_buffer: Vec<KeyEvent>,
    text: String,
    edit_pos: usize,
    return_to: String
}


impl MirrorMode {
    pub fn new() -> Self {
        Self {
            settings: None,
            key_buffer: Vec::new(),
            text: String::new(),
            edit_pos: 0,
            return_to: String::from("Selection"),
        }
    }

    pub fn execute_command(&mut self, command: &str, pane: &mut dyn TextPane) {
        let mut command_args = command.split_whitespace();
        let command_name = command_args.next().unwrap_or("");
        match command_name {
            "cancel" => {
                self.text.clear();
                self.edit_pos = 0;
                eprintln!("returning to {}", self.return_to);
                pane.execute_command(&format!("change_mode {}", self.return_to));
                self.key_buffer.clear();
            }
            "left" => {
                //Todo: make sure that we move by the right byte size
                self.edit_pos = self.edit_pos.saturating_sub(1);
            }
            "right" => {
                if self.edit_pos < self.text.len() {
                    //Todo: make sure that we move by the right byte size
                    self.edit_pos += 1;
                }
            }
            "up" => {
                self.edit_pos = 0;
            }
            "down" => {
                self.edit_pos = self.text.len();
            }
            "backspace" => {
                if self.edit_pos > 0 {
                    self.edit_pos -= 1;
                    self.text.remove(self.edit_pos);
                }
            }
            "delete" => {
                if self.edit_pos < self.text.len() {
                    self.text.remove(self.edit_pos);
                }
            }
            "execute" => {
                // access the file and insert the reversed string then the non-reversed string
                let file = pane.borrow_current_file_mut();

                let mut mirror = Vec::new();
                for c in self.text.chars() {
                    mirror.push(c);
                }
                let mirror = mirror.iter().rev().map(|c| Self::get_pair(*c)).collect::<String>();

                file.insert_pairs((self.text.as_str(), mirror.as_str()));


                pane.execute_command("clear_selection");

                let settings = self.settings.clone().unwrap();
                let settings = settings.borrow();
                pane.execute_command(&format!("change_mode {}", settings.editor_settings.default_mode));
                self.key_buffer.clear();
            }
            _ => {}
        }
    }


    fn get_pair(c: char) -> char {
        match c {
            '(' => ')',
            ')' => '(',
            '[' => ']',
            ']' => '[',
            '{' => '}',
            '}' => '{',
            '<' => '>',
            '>' => '<',
            // Despite looking similar, they are different characters
            '｛' => '｝',
            '｝' => '｛',
            '（' => '）',
            '）' => '（',
            '［' => '］',
            '］' => '［',
            '＜' => '＞',
            '＞' => '＜',
            '【' => '】',
            '】' => '【',
            '〔' => '〕',
            '〕' => '〔',
            '〈' => '〉',
            '〉' => '〈',
            '《' => '》',
            '》' => '《',
            '「' => '」',
            '」' => '「',
            '『' => '』',
            '』' => '『',
            '〖' => '〗',
            '〗' => '〖',
            '〘' => '〙',
            '〙' => '〘',
            '〚' => '〛',
            '〛' => '〚',
            '〝' => '〞',
            '〞' => '〝',
            '‘' => '’',
            '’' => '‘',
            '“' => '”',
            '”' => '“',
            '‹' => '›',
            '›' => '‹',
            '⁅' => '⁆',
            '⁆' => '⁅',
            '⦃' => '⦄',
            '⦄' => '⦃',
            '⦅' => '⦆',
            '⦆' => '⦅',
            '⦇' => '⦈',
            '⦈' => '⦇',
            '⦉' => '⦊',
            '⦊' => '⦉',
            '⦋' => '⦌',
            '⦌' => '⦋',
            '⦍' => '⦎',
            '⦎' => '⦍',
            '⦏' => '⦐',
            '⦐' => '⦏',
            '⦑' => '⦒',
            '⦒' => '⦑',
            '⦓' => '⦔',
            '⦔' => '⦓',
            '⦕' => '⦖',
            '⦖' => '⦕',
            '⦗' => '⦘',
            '⦘' => '⦗',
            '⧘' => '⧙',
            '⧙' => '⧘',
            '⧚' => '⧛',
            '⧛' => '⧚',
            '⧼' => '⧽',
            '⧽' => '⧼',
            '⸂' => '⸃',
            '⸃' => '⸂',
            '⸄' => '⸅',
            '⸅' => '⸄',
            '⸉' => '⸊',
            '⸊' => '⸉',
            '⸌' => '⸍',
            '⸍' => '⸌',
            '⸜' => '⸝',
            '⸝' => '⸜',
            '⸠' => '⸡',
            '⸡' => '⸠',
            '⸢' => '⸣',
            '⸣' => '⸢',
            '⸤' => '⸥',
            '⸥' => '⸤',
            '⸦' => '⸧',
            '⸧' => '⸦',
            '⸨' => '⸩',
            '⸩' => '⸨',
            '〟' => '〟',
            '﴾' => '﴿',
            '﴿' => '﴾',
            '︗' => '︘',
            '︘' => '︗',
            '︵' => '︶',
            '︶' => '︵',
            '︷' => '︸',
            '︸' => '︷',
            '︹' => '︺',
            '︺' => '︹',
            '︻' => '︼',
            '︼' => '︻',
            '︽' => '︾',
            '︾' => '︽',
            '︿' => '﹀',
            '﹀' => '︿',
            '﹁' => '﹂',
            '﹂' => '﹁',
            '﹃' => '﹄',
            '﹄' => '﹃',
            '﹇' => '﹈',
            '﹈' => '﹇',
            '﹙' => '﹚',
            '﹚' => '﹙',
            '﹛' => '﹜',
            '﹜' => '﹛',
            '﹝' => '﹞',
            '﹞' => '﹝',
            '⎡' => '⎤',
            '⎤' => '⎡',
            '⎣' => '⎦',
            '⎦' => '⎣',
            '⎢' => '⎥',
            '⎥' => '⎢',
            '⎧' => '⎫',
            '⎫' => '⎧',
            '⎩' => '⎭',
            '⎭' => '⎩',
            '⎨' => '⎬',
            '⎬' => '⎨',
            '⎪' => '⎪',
            '⎰' => '⎱',
            '⎱' => '⎰',
            '❬' => '❭',
            '❭' => '❬',
            '❮' => '❯',
            '❯' => '❮',
            '❰' => '❱',
            '❱' => '❰',
            '❲' => '❳',
            '❳' => '❲',
            '❴' => '❵',
            '❵' => '❴',
            '⟅' => '⟆',
            '⟆' => '⟅',
            '⟦' => '⟧',
            '⟧' => '⟦',
            '⟨' => '⟩',
            '⟩' => '⟨',
            '⟪' => '⟫',
            '⟫' => '⟪',
            '⟬' => '⟭',
            '⟭' => '⟬',
            '⟮' => '⟯',
            '⟯' => '⟮',
            _ => c
        }
    }
}

impl Mode for MirrorMode {
    fn get_name(&self) -> String {
        "Mirror".to_string()
    }

    fn add_settings(&mut self, settings: Rc<RefCell<Settings>>) {
        self.settings = Some(settings);
    }

    fn refresh(&mut self) {
    }

    fn add_special(&mut self, something: &dyn Any) {
        if let Some(mode) = something.downcast_ref::<String>() {
            self.return_to = (*mode).clone();
        }
    }

    fn get_special(&self) -> Option<&dyn Any> {
        None
    }

    fn influence_cursor(&self) -> Option<usize> {
        let offset = self.get_name().chars().count() + 2 + self.edit_pos;
        Some(offset)
    }
}

impl TextMode for MirrorMode {
    fn process_keypress(&mut self, key: KeyEvent, pane: &mut dyn TextPane) {
        match key {
            key => {
                self.key_buffer.push(key);

                let settings = self.settings.clone().unwrap();
                let mut settings = settings.borrow_mut();

                if let Some(command) = settings.mode_keybindings.get(&self.get_name(), &self.key_buffer) {
                    let command = command.clone();
                    drop(settings);
                    self.execute_command(&command, pane);
                    self.key_buffer.clear();
                } else {
                    drop(settings);
                    match key.key {
                        Key::Char(c) => {
                            self.text.insert(self.edit_pos, c);
                            self.edit_pos += 1;
                        }
                        Key::Tab => {
                            self.text.insert(self.edit_pos, '\t');
                            self.edit_pos += 1;
                        }
                        _ => {}
                    }
                    self.key_buffer.clear();
                }
            }
        }
    }

    fn update_status(&self, _pane: &dyn TextPane) -> (String, String, String) {

        let mut mirror = Vec::new();
        for c in self.text.chars() {
            mirror.push(c);
        }
        let mirror = mirror.iter().rev().map(|c| Self::get_pair(*c)).collect::<String>();

        let first = format!(":{}  |  {}", self.text, mirror);

        let second = String::new();

        (self.get_name(), first, second)
    }

    fn start(&mut self, _pane: &mut dyn TextPane) {
        self.text.clear();
        self.edit_pos = 0;
        self.key_buffer.clear();
    }
}