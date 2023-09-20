
use tuirealm::{Props, props::{TextSpan, Alignment, Style}, Attribute, AttrValue, MockComponent, tui::{prelude::Rect, widgets::Paragraph}, State, command::{Cmd, CmdResult}, NoUserEvent, event::{KeyEvent, Key}, Event, Component};

use tuirealm::adapter::Frame;
use crate::Msg;





pub struct HelloWorld {
    props: Props,
}


impl Default for HelloWorld {
    fn default() -> Self {


        let mut props = Props::default();

        props.set(Attribute::Text, AttrValue::String("Hello World!".to_string()));
        props.set(Attribute::Alignment, AttrValue::Alignment(Alignment::Center));
        props.set(Attribute::Display, AttrValue::Flag(true));
        
        Self {
            props,
        }
    }
}



impl MockComponent for HelloWorld {
    fn view(&mut self, frame: &mut Frame, area: Rect) {

        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {

            let text = self
                .props
                .get_or(Attribute::Text, AttrValue::String("".to_string()))
                .unwrap_string();

            let alignment = self
                .props
                .get_or(Attribute::Alignment, AttrValue::Alignment(Alignment::Left))
                .unwrap_alignment();

            frame.render_widget(
                Paragraph::new(text)
                    .style(
                        Style::default()
                    )
                    .alignment(alignment),
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


impl Component<Msg, NoUserEvent> for HelloWorld {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {

        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                ..
            }) => return Some(Msg::AppClose),
            _ => {}
        }

        None
    }
}
       
