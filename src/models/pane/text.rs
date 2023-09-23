use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::{Sender, Receiver};
use crate::models::style::StyledText;

use std::path::PathBuf;


use crate::models::pane::Pane;
use crate::models::file::File;
use crate::models::settings::Settings;
use crate::models::mode::TextMode;
use crate::models::mode::normal::NormalMode;



pub trait TextBufferObserver {
}







pub struct TextBuffer {
    file: File,
    mode: Rc<RefCell<dyn TextMode>>,
    modes: HashMap<String, Rc<RefCell<dyn TextMode>>>,
    settings: Rc<RefCell<Settings>>,
    //lsp_channels: (Sender<LspMessage>, Receiver<LspMessage>),
    //register_channels: (Sender<RegisterMessage>, Receiver<RegisterMessage>),
}


impl TextBuffer {
    pub fn new(path: Option<PathBuf>, settings: Rc<RefCell<Settings>>) -> Self {
        let file = File::new(path, settings.clone());

        let normal_mode: Rc<RefCell<dyn TextMode>> = Rc::new(RefCell::new(NormalMode::new()));

        let mut modes = HashMap::new();
        modes.insert("Normal".to_string(), normal_mode.clone());

        Self {
            file,
            mode: normal_mode,
            modes,
            settings,
        }
    }

}


impl Pane for TextBuffer {
    fn execute_command(&mut self, command: &str) {
    }

    fn get_cursor_position(&self) -> Option<(usize, usize)> {
        None
    }

    fn draw(&self) -> StyledText {
        self.file.display()
    }

}





