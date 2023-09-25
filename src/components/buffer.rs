use std::cell::RefCell;
use std::rc::Rc;
use tuirealm::{Attribute, AttrValue, Component, Event, Frame, MockComponent, NoUserEvent, Props, State};
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::tui::layout::Rect;
use tuirealm::tui::text::Text;
use tuirealm::tui::widgets::Paragraph;
use crate::models::{AppEvent, Message};
use crate::models::pane::Pane;
use crate::models::pane::text::TextBuffer;
use crate::models::style::StyledText;

pub struct Buffer {
    pane: Rc<RefCell<TextBuffer>>,
    props: Props,
    scroll: (u16, u16),
}


impl Buffer{
    pub fn new(pane: Rc<RefCell<TextBuffer>>) -> Self {
        let mut props = Props::default();

        props.set(Attribute::Display, AttrValue::Flag(true));

        Self {
            props,
            pane,
            scroll: (0, 0),
        }
    }

    fn update_scroll(&mut self) {
        if let Some((x, y)) = self.pane.borrow().get_scroll_amount() {
            self.scroll = (x as u16, y as u16);
        }
    }
}

impl MockComponent for Buffer {
    fn view(&mut self, frame: &mut Frame, area: Rect) {


        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let pane = self.pane.clone();
            let pane = pane.borrow();
            let text = pane.draw();

            frame.render_widget(
                Paragraph::new(text)
                    .scroll(self.scroll),
                area,
            );
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Message, AppEvent> for Buffer {
    fn on(&mut self, ev: Event<AppEvent>) -> Option<Message> {
        match ev {
            Event::User(AppEvent::Edit) => {
                Some(Message::Redraw)
            }
            Event::User(AppEvent::Scroll) => {
                self.update_scroll();
                Some(Message::Redraw)
            }
            _ => None,
        }
    }
    
    
    
}