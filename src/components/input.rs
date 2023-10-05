use tuirealm::{Attribute, AttrValue, Component, Event, Frame, MockComponent, State};
use tuirealm::command::{Cmd, CmdResult};

use tuirealm::tui::prelude::Rect;
use crate::models::{AppEvent, Message};

/// This struct is for receiving input from the user
/// and passing it to the model.
pub struct InputLayer;


impl InputLayer {
    pub fn new() -> Self {
        Self {}
    }
}


impl MockComponent for InputLayer {
    fn view(&mut self, _frame: &mut Frame, _area: Rect) {

    }

    fn query(&self, _attr: Attribute) -> Option<AttrValue> {
        None
    }

    fn attr(&mut self, _attr: Attribute, _value: AttrValue) {
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}


impl Component<Message, AppEvent> for InputLayer {
    fn on(&mut self, ev: Event<AppEvent>) -> Option<Message> {
        match ev {
            Event::Keyboard(key_event) => {
                Some(Message::Key(key_event.into()))
            }
            Event::User(AppEvent::Close) => {
                Some(Message::Close)
            }
            Event::User(AppEvent::ForceQuit) => {
                Some(Message::AppClose)
            }
            Event::User(AppEvent::ForceClose) => {
                Some(Message::ForceClose)
            }
            Event::User(AppEvent::OpenFile(path)) => {
                Some(Message::OpenFile(path))
            }
            Event::WindowResize(_, _) => {
                Some(Message::Redraw)
            }
            Event::User(AppEvent::Message(msg)) => {
                Some(Message::InfoMessage(msg))
            }
            Event::User(AppEvent::CreateHelpFile) => {
                Some(Message::OpenHelpFile)
            }
            _ => None,
        }
    }
}