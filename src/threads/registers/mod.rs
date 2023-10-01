use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, RecvError, Sender, SendError, TryRecvError};
use either::Either;
use arboard::Clipboard;
use crate::threads::Mailbox;

pub trait RegisterUtils<T> {
    /// Get the value of a register.
    fn get(&self, name: T) -> Option<&String>;
    /// Set the value of a register.
    fn set(&mut self, name: T, value: String);
}


pub enum RegisterMessage {
    AddNamed(String, String),
    AddNumbered(usize, String),
    GetNamed(String),
    GetNumbered(usize),
    SetClipboard(String),
    SetMulti(Vec<String>),
    GetClipboard,
    RegisterResult(Option<String>, Option<Vec<String>>),
    Quit,
}
pub struct RegisterMailbox {
    // The receiver for messages to the register thread
    local_receiver: Receiver<RegisterMessage>,
    // The sender for messages to the register thread
    far_sender: Option<Sender<RegisterMessage>>,
    // The receiver for messages from the register thread
    // It is wrapped in an Rc so that it can be cloned
    far_receiver: Option<Rc<Receiver<RegisterMessage>>>,
    // The sender for messages from the register thread
    local_sender: Sender<RegisterMessage>,
}

impl RegisterMailbox {
    pub fn new() -> RegisterMailbox {
        let (local_sender, far_receiver) = std::sync::mpsc::channel();
        let (far_sender, local_receiver) = std::sync::mpsc::channel();

        RegisterMailbox {
            local_receiver,
            far_sender: Some(far_sender),
            far_receiver: Some(Rc::new(far_receiver)),
            local_sender,
        }
    }

    pub fn get_shared(&mut self) -> (Sender<RegisterMessage>, Rc<Receiver<RegisterMessage>>) {
        (self.far_sender.take().unwrap(), self.far_receiver.take().unwrap())
    }
}

impl Mailbox<RegisterMessage> for RegisterMailbox {
    fn send(&self, message: RegisterMessage) -> Result<(), SendError<RegisterMessage>> {
        self.local_sender.send(message)
    }

    fn recv(&self) -> Result<RegisterMessage, RecvError> {
        self.local_receiver.recv()
    }

    fn try_recv(&self) -> Result<RegisterMessage, TryRecvError> {
        self.local_receiver.try_recv()
    }
}

unsafe impl Send for RegisterMailbox {}
pub struct Registers {
    mailbox: RegisterMailbox,
    clipboard: Either<RefCell<Clipboard>, Option<String>>,
    named: HashMap<String, String>,
    numbered: HashMap<usize, String>,
    multi: Option<Vec<String>>,
    quit: bool,
}

impl Registers {
    pub fn new() -> Registers {

        let clipboard = match Clipboard::new() {
            Ok(clipboard) => {
                Either::Left(RefCell::new(clipboard))
            },
            Err(_) => {
                Either::Right(None)
            }
        };

        Registers {
            mailbox: RegisterMailbox::new(),
            clipboard,
            named: HashMap::new(),
            numbered: HashMap::new(),
            quit: false,
            multi: None,
        }
    }

    pub fn get_shared(&mut self) -> (Sender<RegisterMessage>, Rc<Receiver<RegisterMessage>>) {
        self.mailbox.get_shared()
    }

    fn handle_message(&mut self, message: RegisterMessage) {
        match message {
            RegisterMessage::AddNamed(name, value) => {
                self.set(name, value);
            },
            RegisterMessage::AddNumbered(name, value) => {
                self.set(name, value);
            },
            RegisterMessage::GetNamed(name) => {
                let message = RegisterMessage::RegisterResult(self.get(name).cloned(), None);
                self.mailbox.send(message).expect("Failed to send message");
            },
            RegisterMessage::GetNumbered(name) => {
                let message = RegisterMessage::RegisterResult(self.get(name).cloned(), None);
                self.mailbox.send(message).expect("Failed to send message");
            },
            RegisterMessage::SetClipboard(value) => {
                self.set_clipboard(value);
            },
            RegisterMessage::GetClipboard => {
                let message= if self.multi.is_some() {
                    let message = RegisterMessage::RegisterResult(self.get_clipboard(), self.multi.take());
                    message
                } else {
                    let message = RegisterMessage::RegisterResult(self.get_clipboard(), None);
                    message
                };
                self.mailbox.send(message).expect("Failed to send message");
            },
            RegisterMessage::RegisterResult(_, _) => {
                // This should never happen
            }
            RegisterMessage::Quit => {
                eprintln!("Register thread received quit message");
                self.quit = true;
            }
            RegisterMessage::SetMulti(multi) => {
                self.set_clipboard(multi.join("\n"));
                self.multi = Some(multi);
            }
        }
    }


    fn get_clipboard(&self) -> Option<String> {
        match &self.clipboard {
            Either::Left(clipboard) => {
                let mut clipboard = clipboard.borrow_mut();
                match clipboard.get_text() {
                    Ok(contents) => Some(contents),
                    Err(_) => None
                }
            },
            Either::Right(contents) => {
                contents.clone()
            }
        }
    }

    fn set_clipboard(&mut self, value: String) {
        match &mut self.clipboard {
            Either::Left(clipboard) => {
                let mut clipboard = clipboard.borrow_mut();
                match clipboard.set_text(value.clone()) {
                    Ok(_) => {},
                    Err(_) => {}
                }
            },
            Either::Right(contents) => {
                *contents = Some(value);
            }
        }
    }

    pub fn run(&mut self) {
        while !self.quit {
            match self.mailbox.recv() {
                Ok(message) => {
                    self.handle_message(message);
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
}

impl RegisterUtils<usize> for Registers {
    fn get(&self, name: usize) -> Option<&String> {
        self.numbered.get(&name)
    }

    fn set(&mut self, name: usize, value: String) {
        self.numbered.insert(name, value);
    }
}

impl RegisterUtils<String> for Registers {
    fn get(&self, name: String) -> Option<&String> {
        self.named.get(&name)
    }

    fn set(&mut self, name: String, value: String) {
        self.named.insert(name, value);
    }
}