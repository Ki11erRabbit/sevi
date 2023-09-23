use model::Model;
use tuirealm::{PollStrategy, Update, Attribute, AttrValue};
use tuirealm::Sub;
use tuirealm::SubEventClause;
use tuirealm::SubClause;

use std::env;

pub mod model;
pub mod hello_world;
pub mod pane;

pub mod models;











#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    AppClose,
    Redraw,
    OpenFile(String),
}


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Id {
    HelloWorld,
    Pane,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum UserEvent {
    OpenFile(String)
}



fn main() {
    let mut model = Model::default();

    model.initialize();

    let args: Vec<String> = env::args().collect();

    let _ = model.app.subscribe(&Id::Pane, Sub::new(SubEventClause::User(UserEvent::OpenFile("".to_string())), SubClause::Always));


    if args.len() > 1 {
        model.update(Some(Msg::OpenFile(args[1].clone())));
    }


    while !model.quit {

        match model.app.tick(PollStrategy::Once) {
            Err(err) => {
                assert!(model
                        .app
                        .attr(
                            &Id::HelloWorld,
                            Attribute::Text,
                            AttrValue::String(format!("Error: {}", err)),
                        )
                        .is_ok());

            },
            Ok(messages) if messages.len() > 0 => {
                model.redraw = true;

                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            _ => {},
        }

        if model.redraw {
            model.view();
            model.redraw = false;
        }
            
    }
    
}
