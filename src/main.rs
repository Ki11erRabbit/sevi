use model::Model;
use tuirealm::{PollStrategy, Update, Attribute, AttrValue};
use tuirealm::Sub;
use tuirealm::SubEventClause;
use tuirealm::SubClause;

use std::env;

pub mod model;



pub mod models;
pub mod components;


fn main() {
    let mut model = Model::default();

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
            _ => {},
        }

        if model.redraw {
            model.view();
            model.redraw = false;
        }
            
    }
    
}
