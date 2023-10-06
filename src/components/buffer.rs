use std::cell::RefCell;
use std::rc::Rc;
use tuirealm::{Attribute, AttrValue, Component, Event, Frame, MockComponent, Props, State};
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::tui::layout::Rect;
use tuirealm::tui::prelude::Text;
use tuirealm::tui::widgets::Paragraph;
use crate::models::{AppEvent, Message};
use crate::models::pane::{Pane, TextPane};
use crate::models::pane::text::TextBuffer;
use crate::models::text_buffer::BufferText;

pub struct Buffer {
    pane: Rc<RefCell<TextBuffer>>,
    props: Props,
    scroll: (u16, u16),
    display_info: bool,
}


impl Buffer{
    pub fn new(pane: Rc<RefCell<TextBuffer>>) -> Self {
        let mut props = Props::default();

        props.set(Attribute::Display, AttrValue::Flag(true));

        let text = format!("SEVI - Structural Editor VIsual\n\nversion {}\nby Alec Davis.\n\
            Sevi is open source and freely distributable\n\n\
            type  :q<Enter>              to exit\n\
            type  :help<Enter>           for help", env!("CARGO_PKG_VERSION"));

        props.set(Attribute::Text, AttrValue::String(text));

        Self {
            props,
            pane,
            scroll: (0, 0),
            display_info: true,
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
            if self.display_info {
                let text = self.props.get_or(Attribute::Text, AttrValue::Text(String::new().into()));

                let text = match text {
                    AttrValue::String(text) => text,
                    _ => String::new().into(),
                };

                let text = format!("{}{}", "\n".repeat((area.height as usize - 7) / 2), text);

                frame.render_widget(
                    Paragraph::new(Text::from(text))
                        .alignment(tuirealm::tui::layout::Alignment::Center),
                    area,
                );

            } else {
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

                let text_buffer = BufferText::new(settings.clone())
                    .add_number_line_style(number_line_type)
                    .add_current_row(row)
                    .set_start_row(start)
                    .set_scroll_cols(self.scroll.1 as usize);

                let text = text_buffer.draw(text);
                frame.render_widget(
                    Paragraph::new(text),
                    area,
                );
            }



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
            Event::User(AppEvent::RemoveInfoDisplay) => {
                self.display_info = false;
                Some(Message::Redraw)
            }
            _ => None,
        }
    }
    
    
    
}