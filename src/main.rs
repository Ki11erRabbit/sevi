use std::rc::Rc;
use clap::Parser;
use model::Model;
use tuirealm::{PollStrategy, Update};


use crate::models::Message;
use crate::threads::lsp::LspController;

pub mod model;



pub mod models;
pub mod components;
pub mod widgets;
pub mod threads;
mod arg_parser;
pub mod lsp;


fn main() {

    let args = arg_parser::Args::parse();

    args.perform_commands();

    let mut register = threads::registers::Registers::new();

    let shared = register.get_shared();

    let registers_handle = std::thread::spawn(move || {
        register.run();
    });

    let mut controller = LspController::new();

    let (lsp_sender, lsp_receiver) = std::sync::mpsc::channel();
    let (lsp_controller_sender, lsp_controller_receiver) = std::sync::mpsc::channel();

    controller.set_listen(lsp_receiver);
    controller.set_response(lsp_controller_sender);

    let lsp_listener = Rc::new(lsp_controller_receiver);

    let lsp_channels = (lsp_sender, lsp_listener);

    let lsp_handle = std::thread::spawn(move || {
        let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
        let tokio_handle =tokio_runtime.spawn_blocking(move || {
            match controller.run() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }
            drop(controller);
        });
        tokio_runtime.block_on(tokio_handle).unwrap();
    });


    let path = args.get_path();

    let mut model = Model::new(path,shared, lsp_channels);

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
    lsp_handle.join().unwrap();
}
