
use tuirealm::event::{KeyEvent, Key};
use tuirealm::{State, NoUserEvent, Component, Event};
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::Style;
use tuirealm::{Props, Attribute, AttrValue, MockComponent, Frame, tui::{prelude::Rect, widgets::Paragraph}};

use crate::Msg;




pub struct Pane {
    props: Props,
}


impl Default for Pane {
    fn default() -> Self {
        let mut props = Props::default();

        props.set(Attribute::Text, AttrValue::String("".to_string()));
        props.set(Attribute::Display, AttrValue::Flag(true));
        
        Self {
            props,
        }
    }

}


impl MockComponent for Pane {
    fn view(&mut self, frame: &mut Frame, area: Rect) {

        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {

            let text = self
                .props
                .get_or(Attribute::Text, AttrValue::String("".to_string()))
                .unwrap_string();

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
                let text = std::fs::read_to_string(file_name).unwrap();
                self.props.set(Attribute::Text, AttrValue::String(text));
                CmdResult::None
            }
            _ => CmdResult::None,

        }
    }
}


impl Component<Msg, NoUserEvent> for Pane {
    fn on(&mut self, ev: tuirealm::Event<NoUserEvent>) -> Option<Msg> {
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
            _ => {}
        }
        None
    }
}
