
use std::time::Duration;

use tuirealm::{Application, NoUserEvent, terminal::TerminalBridge, tui::prelude::{Layout, Direction, Constraint}, EventListenerCfg, Update};

use crate::{Msg, Id, pane::PaneContainer, UserEvent};

use crate::hello_world::HelloWorld;




pub struct Model {
    pub app: Application<Id, Msg, UserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Failed to create terminal"),
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        let _ = self.terminal.leave_alternate_screen();
        let _ = self.terminal.disable_raw_mode();
        let _ = self.terminal.clear_screen();
    }
}


impl Model {
    pub fn view(&mut self) {
        assert!(self
                .terminal
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

    }

    fn init_app() -> Application<Id, Msg, UserEvent> {

        let mut app: Application<Id, Msg, UserEvent> = Application::init(
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
        let _ = self.terminal.enter_alternate_screen();
        let _ = self.terminal.enable_raw_mode();
    }
    
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {

            self.redraw = true;


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
                }
            }

        } else {
            None
        }

    }


}
    
