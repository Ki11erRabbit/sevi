use std::cell::RefCell;
use std::rc::Rc;
use tuirealm::{Attribute, AttrValue, Component, Event, Frame, MockComponent, Props, State};
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::tui::layout::Rect;
use tuirealm::tui::widgets::Paragraph;
use crate::models::{AppEvent, Message};
use crate::models::pane::{Pane, TextPane};
use crate::models::pane::text::TextBuffer;
use crate::models::text_buffer::BufferText;

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

    fn update_scroll(&mut self, rect: Rect) {
        self.pane.borrow_mut().scroll(rect.into());
        if let Some((x, y)) = self.pane.borrow().get_scroll_amount() {
            self.scroll = (y as u16, x as u16);
        }
    }
}

impl MockComponent for Buffer {
    fn view(&mut self, frame: &mut Frame, area: Rect) {


        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            self.update_scroll(area);

            let pane = self.pane.clone();
            let pane = pane.borrow_mut();
            let (_, offset) = pane.get_scroll_amount().unwrap_or((0, 0));
            let start = offset;
            let end = offset + area.height as usize;
            let text = pane.draw_section(start, end);
            //let text = pane.draw();

            let (_, row) = pane.get_cursor();

            let settings = pane.get_settings();

            let number_line_type = {
                let settings = settings.borrow();
                settings.editor_settings.number_line
            };

            let text_buffer = BufferText::new(area.into(), settings.clone())
                .add_number_line_style(number_line_type)
                .add_current_row(row)
                .set_start_row(start);

            let text = text_buffer.draw(text);

            frame.render_widget(
                Paragraph::new(text),
                /*Editor::new(text)
                    .number_line_type(number_line_type, row),
                    //.scroll(self.scroll),*/
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

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
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
                Some(Message::Redraw)
            }
            _ => None,
        }
    }
    
    
    
}