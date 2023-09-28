use model::Model;
use tuirealm::{PollStrategy, Update, Attribute, AttrValue};
use tuirealm::Sub;
use tuirealm::SubEventClause;
use tuirealm::SubClause;

use std::env;
use crate::models::Message;

pub mod model;



pub mod models;
pub mod components;
pub mod widgets;
pub mod threads;


fn main() {

    let mut register = threads::registers::Registers::new();

    let shared = register.get_shared();

    let registers_handle = std::thread::spawn(move || {
        register.run();
    });

    let mut model = Model::new(shared);

    model.initialize();


    while !model.quit {

        match model.app.tick(PollStrategy::Once) {
            Ok(messages) if messages.len() > 0 => {
                model.redraw = true;

                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            Ok(_) => {
                model.update(Some(Message::Tick));
            },
            _ => {},
        }

        if model.redraw {
            model.view();
            model.redraw = false;
        }
            
    }
    registers_handle.join().unwrap();
    
}
