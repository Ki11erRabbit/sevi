
use std::time::Duration;

use tuirealm::{Application, NoUserEvent, terminal::TerminalBridge, tui::prelude::{Layout, Direction, Constraint}, EventListenerCfg, Update, Event};

use std::rc::Rc;
use std::cell::RefCell;
use std::env;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use tuirealm::listener::{ListenerResult, Poll};
use crate::components::status_bar::StatusBar;
use crate::components::buffer::Buffer;


use crate::models::{AppEvent, Id, Message};
use crate::models::pane::Pane;
use crate::models::pane::text::TextBuffer;

pub struct AppEventPort{
    pub receiver: Receiver<AppEvent>,
}

impl Poll<AppEvent> for AppEventPort {
    fn poll(&mut self) -> ListenerResult<Option<Event<AppEvent>>> {
        match self.receiver.try_recv() {
            Ok(event) => Ok(Some(Event::User(event))),
            Err(_) => Ok(None),
        }
    }
}

pub struct Model {
    pub app: Application<Id, Message, AppEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: Rc<RefCell<TerminalBridge>>,
    pub pane: Rc<RefCell<TextBuffer>>,
    pub sender: Sender<AppEvent>,
}

impl Default for Model {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let path = if args.len() > 1 {
            let path = PathBuf::from(args[1].clone());
            Some(PathBuf::from(path))
        } else {
            None
        };

        let settings = crate::models::settings::Settings::default();

        let settings = Rc::new(RefCell::new(settings));

        let pane = TextBuffer::new(path, settings);
        let pane = Rc::new(RefCell::new(pane));

        let (sender, receiver) = std::sync::mpsc::channel();

        let mut app = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1))
                .port(Box::new(AppEventPort {
                    receiver,
                }),
                Duration::from_millis(10)
                ),
        );

        assert!(app.mount(
            Id::Buffer,
            Box::new(
                Buffer::new(pane.clone())
            ),
            Vec::default(),
            ).is_ok());

        assert!(app.active(&Id::Buffer).is_ok());

        assert!(app.mount(
            Id::Status,
            Box::new(
                StatusBar::new(pane.clone())
            ),
            Vec::default(),
            ).is_ok());


        Self {
            app,
            quit: false,
            redraw: true,
            terminal: Rc::new(RefCell::new(TerminalBridge::new().expect("Failed to create terminal bridge"))),
            pane,
            sender,
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
                                Constraint::Min(0),
                                Constraint::Length(1),
                            ]
                                .as_ref(),
                        )
                        .split(f.size());

                    self.app.view(&Id::Buffer, f, chunks[0]);
                    self.app.view(&Id::Status, f, chunks[1])
                })
                .is_ok());


        match self.pane.borrow().get_cursor_position() {
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
                let _ = term.set_cursor(x as u16, y as u16);
                let _ = term.show_cursor();
            },
        }
        match self.pane.borrow().get_scroll_amount() {
            None => {},
            Some((x, y)) => {
                if x == 0 && y == 0 {
                    return;
                }
                self.sender.send(AppEvent::Scroll(x as u16, y as u16)).unwrap();
            }
        }


    }

    /*fn init_app() -> Application<Id, Message, NoUserEvent> {

        let mut app: Application<Id, Message, NoUserEvent> = Application::init(
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
    }*/

    pub fn initialize(&mut self) {
        let terminal = self.terminal.clone();
        let mut terminal = terminal.borrow_mut();
        let _ = terminal.enter_alternate_screen();
        let _ = terminal.enable_raw_mode();
    }
    
}

impl Update<Message> for Model {
    fn update(&mut self, msg: Option<Message>) -> Option<Message> {

        if let Some(msg) = msg {




            match msg {
                Message::AppClose => {
                    self.quit = true;
                    None
                },
                Message::Redraw => {
                    self.redraw = true;

                    None
                },
                Message::OpenFile(file) => {
                    None
                },
                Message::Key(key) => {
                    self.pane.borrow_mut().process_keypress(key);
                    None
                }
                _ => None,
            }

        } else {
            None
        }

    }


}
    
