use std::cell::RefCell;
use std::rc::Rc;
use tuirealm::{Attribute, AttrValue, Component, Event, Frame, MockComponent, State};
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::Alignment;
use tuirealm::tui::layout::Rect;
use tuirealm::tui::text::Text;
use tuirealm::tui::widgets::{Cell, Paragraph};
use crate::models::{AppEvent, Message};
use crate::models::pane::Pane;
use crate::models::pane::text::TextBuffer;
use crate::models::style::StyledText;

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
        let name: Text = name.into();
        frame.render_widget(
            Paragraph::new(name)
                .alignment(Alignment::Left)
            ,
            area,
        );


        frame.render_widget(
            Paragraph::new(String::from(' '))
                .alignment(Alignment::Left)
            ,
            area,
        );

        let first: Text = first.into();
        frame.render_widget(
            Paragraph::new(first)
                .alignment(Alignment::Left)
            ,
            area,
        );

        let second: Text = second.into();

        frame.render_widget(
            Paragraph::new(second)
                .alignment(Alignment::Right)
            ,
            area,
        );

    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        None
    }

    fn attr(&mut self, _attr: Attribute, _value: AttrValue) {

    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Message, AppEvent> for StatusBar {
    fn on(&mut self, ev: Event<AppEvent>) -> Option<Message> {
        match ev {
            Event::User(AppEvent::StatusChanged) => {
                Some(Message::Redraw)
            }
            _ => None,
        }
    }
}