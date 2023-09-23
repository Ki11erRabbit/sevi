
use tuirealm::event::{KeyEvent, Key};
use tuirealm::{State, NoUserEvent, Component, Event};
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::Style;
use tuirealm::{Props, Attribute, AttrValue, MockComponent, Frame, tui::{prelude::Rect, widgets::Paragraph}};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use std::env;

use crate::models::pane::text::TextBuffer;
use crate::models::pane::Pane;

use crate::Msg;
use crate::UserEvent;




pub struct PaneContainer {
    props: Props,
    text_buffer: TextBuffer,
}


impl Default for PaneContainer {
    fn default() -> Self {
        
        let args: Vec<String> = env::args().collect();
        let path = if args.len() > 1 {
            let path = PathBuf::from(args[1].clone());
            Some(PathBuf::from(path))
        } else {
            None
        };

        let mut props = Props::default();

        props.set(Attribute::Text, AttrValue::String("".to_string()));
        props.set(Attribute::Display, AttrValue::Flag(true));

        let settings = crate::models::settings::Settings::default();

        let settings = Rc::new(RefCell::new(settings));

        let text_buffer = TextBuffer::new(path, settings);
        
        Self {
            props,
            text_buffer,
        }
    }
}

impl PaneContainer {
    pub fn new(path: PathBuf) -> Self {
        let mut props = Props::default();

        props.set(Attribute::Text, AttrValue::String("".to_string()));
        props.set(Attribute::Display, AttrValue::Flag(true));

        let settings = crate::models::settings::Settings::default();

        let settings = Rc::new(RefCell::new(settings));

        let text_buffer = TextBuffer::new(Some(path), settings);
        
        Self {
            props,
            text_buffer,
        }
    }
}


impl MockComponent for PaneContainer {
    fn view(&mut self, frame: &mut Frame, area: Rect) {

        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {

            let text = self.text_buffer.draw();

            let text = text.to_string();

            frame.render_widget(
                Paragraph::new(text)
                    .style(
                        Style::default()
                    ),
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

    fn state(&self) -> tuirealm::State {
        State::None
    }

    fn perform(&mut self, cmd: tuirealm::command::Cmd) -> tuirealm::command::CmdResult {
        match cmd {
            Cmd::Custom(file_name) => {
                
                let settings = crate::models::settings::Settings::default();

                let settings = Rc::new(RefCell::new(settings));

                self.text_buffer = TextBuffer::new(Some(PathBuf::from(file_name)), settings);


                CmdResult::None
            }
            _ => CmdResult::None,

        }
    }
}


impl Component<Msg, UserEvent> for PaneContainer {
    fn on(&mut self, ev: tuirealm::Event<UserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                ..
            }) => return Some(Msg::AppClose),
            Event::Keyboard(KeyEvent {
                code: Key::Char('m'),
                ..
            }) => {

                self.perform(Cmd::Custom("src/model.rs"));

                return Some(Msg::Redraw);
            },
            Event::User(UserEvent::OpenFile(file_name)) => {
                
                let settings = crate::models::settings::Settings::default();

                let settings = Rc::new(RefCell::new(settings));

                self.text_buffer = TextBuffer::new(Some(PathBuf::from(file_name)), settings);

                

                return Some(Msg::Redraw);
            },
            _ => {}
        }
        None
    }
}
