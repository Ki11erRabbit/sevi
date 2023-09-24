
use std::time::Duration;

use tuirealm::{Application, NoUserEvent, terminal::TerminalBridge, tui::prelude::{Layout, Direction, Constraint}, EventListenerCfg, Update};
use tuirealm::tui::backend::CrosstermBackend;
use std::io::Stdout;
use std::rc::Rc;
use std::cell::RefCell;

use crate::{Msg, Id, pane::PaneContainer, UserEvent};

use crate::hello_world::HelloWorld;
use crate::models::Message;
use crate::models::pane::text::TextBuffer;


pub struct Model<'a> {
    pub app: Application<Id, Message<'a>, UserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: Rc<RefCell<TerminalBridge>>,
    pub cursor: Option<(u16, u16)>,
    pub pane: TextBuffer,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: Rc::new(RefCell::new(TerminalBridge::new().expect("Failed to create terminal bridge"))),
            cursor: None,
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        let terminal = self.terminal.clone();
        let mut terminal = terminal.borrow_mut();
        let _ = terminal.leave_alternate_screen();
        let _ = terminal.disable_raw_mode();
        let _ = terminal.clear_screen();
    }
}


impl Model {
    pub fn view(&mut self) {
        assert!(self
                .terminal.borrow_mut()
                .raw_mut()
                .draw(|f| {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(0)
                        .constraints(
                            [
                                Constraint::Length(3),
                            ]
                                .as_ref(),
                        )
                        .split(f.size());

                    self.app.view(&Id::Pane, f, chunks[0]);
                })
                .is_ok());


        match self.cursor {
            None => {
                let terminal = self.terminal.clone();
                let mut terminal = terminal.borrow_mut();
                let term = terminal.raw_mut();
                let _ = term.hide_cursor();
            },
            Some((x, y)) => {
                let terminal = self.terminal.clone();
                let mut terminal = terminal.borrow_mut();
                let term = terminal.raw_mut();
                let _ = term.set_cursor(x, y);
                let _ = term.show_cursor();
            },
        }

    }

    fn init_app() -> Application<Id, Message, UserEvent> {

        let mut app: Application<Id, Message, UserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        assert!(app
                .mount(
                    Id::Pane,
                    Box::new(
                        PaneContainer::default()
                    ),
                    Vec::default(),
                )
                .is_ok());

        assert!(app.active(&Id::Pane).is_ok());
        app
    }

    pub fn initialize(&mut self) {
        let terminal = self.terminal.clone();
        let mut terminal = terminal.borrow_mut();
        let _ = terminal.enter_alternate_screen();
        let _ = terminal.enable_raw_mode();
    }
    
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {

        if let Some(msg) = msg {




            match msg {
                Msg::AppClose => {
                    self.quit = true;
                    None
                },
                Msg::Redraw => {
                    self.redraw = true;

                    None
                },
                Msg::OpenFile(file) => {
                    Some(Msg::OpenFile(file))
                },
                Msg::MoveCursor(Some((x, y))) => {
                    match &mut self.cursor {
                        None => {
                            self.cursor = Some((x, y));
                        },
                        Some(cursor) => {
                            cursor.0 = x;
                            cursor.1 = y;
                        },
                    }
                    self.redraw = true;

                    None
                },
                Msg::MoveCursor(None) => {
                    self.cursor = None;

                    None
                },
            }

        } else {
            None
        }

    }


}
    
