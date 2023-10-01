use std::cell::RefCell;
use std::rc::Rc;
use tuirealm::{Attribute, AttrValue, Component, Event, Frame, MockComponent, State};
use tuirealm::command::{Cmd, CmdResult};

use tuirealm::tui::layout::Rect;
use tuirealm::tui::text::Text;
use tuirealm::tui::widgets::Paragraph;
use crate::models::{AppEvent, Message};
use crate::models::pane::Pane;
use crate::models::pane::text::TextBuffer;
use crate::models::status_bar::Status;

pub struct StatusBar {
    pane: Rc<RefCell<TextBuffer>>,
}

impl StatusBar {
    pub fn new(pane: Rc<RefCell<TextBuffer>>) -> Self {
        Self {
            pane,
        }
    }
}

impl MockComponent for StatusBar {
    fn view(&mut self, frame: &mut Frame, area: Rect) {

        let pane = self.pane.clone();
        let pane = pane.borrow();

        let (name, first, second) = pane.get_status();

        let bar = Status::new(area.into()).create_bar(name, first, second);

        let bar: Text = bar.into();
        frame.render_widget(
            Paragraph::new(bar)
            ,
            area,
        );

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

impl Component<Message, AppEvent> for StatusBar {
    fn on(&mut self, ev: Event<AppEvent>) -> Option<Message> {
        match ev {
            _ => None,
        }
    }
}