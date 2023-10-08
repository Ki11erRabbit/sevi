use std::collections::HashMap;
use std::fmt::Display;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use futures::executor::block_on;
use futures::FutureExt;
use serde_json::Value;
use tokio::io;
use tokio::process::Command;
use crate::lsp::Client;
use crate::models::lsp::{LspMessage, process_json};
use crate::models::lsp::completion::CompletionList;
use crate::models::lsp::diagnostic::Diagnostics;
use crate::models::lsp::location::LocationResponse;
use crate::models::lsp::semantic_tokens::{SemanticTokens, SemanticTokensLegend};

pub enum LspRequest {
    /// Tells the server to shutdown
    Shutdown,
    /// Tells the server to exit
    Exit,
    /// Requires a URI
    RequestDiagnostic(Box<str>),
    /// Requires a URI, a position, and a way a completion was triggered
    RequestCompletion(Box<str>, (usize, usize), Box<str>),
    /// Requires a URI and a position
    GotoDeclaration(Box<str>, (usize, usize)),
    /// Requires a URI and a position
    GotoDefinition(Box<str>, (usize, usize)),
    /// Requires a URI and a position
    GotoTypeDefinition(Box<str>, (usize, usize)),
    /// Requires a URI and a position
    GotoImplementation(Box<str>, (usize, usize)),
    /// Semantic tokens are for syntax highlighting, and take a uri
    SemanticTokens(Box<str>),

}

unsafe impl Send for LspResponse {}

pub enum LspResponse {
    PublishDiagnostics(Diagnostics),
    Completion(CompletionList),
    Location(LocationResponse),
    SemanticTokens(SemanticTokens),

}

unsafe impl Send for LspNotification {}
pub enum LspNotification {
    /// 0 is the uri
    /// 1 is the version
    /// 2 is the text
    ChangeText(Box<str>, usize, Box<str>),
    /// 0 is the uri
    /// 1 is the text
    Open(Box<str>, Box<str>),
    /// 0 is the uri
    Close(Box<str>),
    /// 0 is the uri
    /// 1 is the text
    Save(Box<str>, Box<str>),
    /// 0 is the uri
    /// 1 is the reason
    WillSave(Box<str>, Box<str>),


}


unsafe impl Send for LspControllerMessage {}

pub enum LspControllerMessage {
    /// String is the language id
    Request(Box<str>, LspRequest),
    /// Response to a request
    Response(LspResponse),
    /// Box<str> is the language id
    Notification(Box<str>, LspNotification),
    /// String is the language id
    CreateClient(Box<str>),
    /// Notification to tell the caller how to receive responses
    /// The receiver is for the language server side
    ClientCreated(Arc<Receiver<LspControllerMessage>>),
    /// Notification to tell the caller that there is no client for the language
    NoClient,
    Resend(Box<str>, LspResponse),
    Exit,
}


impl Drop for LspController {
    fn drop(&mut self) {
        //eprintln!("Dropping lsp controller");
        for (_, client) in self.clients.iter_mut() {
            match client.send_exit() {
                Ok(_) => {},
                Err(_) => {
                    //eprintln!("Error: {:?}", e);
                }
            }
        }
    }
}

unsafe impl Send for LspController {}

pub struct LspController {
    clients: HashMap<String, Client>,
    semantic_tokens: HashMap<String, SemanticTokensLegend>,
    //channels: (Sender<ControllerMessage>, Receiver<ControllerMessage>),
    listen: Option<Receiver<LspControllerMessage>>,
    response: Option<Sender<LspControllerMessage>>,
    server_channels: HashMap<String, (Sender<LspControllerMessage>, Arc<Receiver<LspControllerMessage>>)>,
    exit: bool,
}

impl LspController {

    pub fn new() -> Self {
        LspController {
            clients: HashMap::new(),
            semantic_tokens: HashMap::new(),
            //channels: std::sync::mpsc::channel(),
            listen: None,
            response: None,
            server_channels: HashMap::new(),
            exit: false,

        }
    }

    pub fn set_listen(&mut self, listen: Receiver<LspControllerMessage>) {
        self.listen = Some(listen);
    }

    pub fn set_response(&mut self, response: Sender<LspControllerMessage>) {
        self.response = Some(response);
    }



    pub fn run(&mut self) -> io::Result<()> {
        //eprintln!("Running lsp controller");
        while !self.exit {
            self.check_messages()?;


            let future = self.check_clients();
            let _ = block_on(future);


        }
        Ok(())
    }

    async fn check_client(client: &mut Client) -> io::Result<Value> {
        let future = client.process_messages();
        let val = future.await;
        let json = val?;
        Ok(json)
    }

    async fn check_clients(&mut self) -> io::Result<()> {

        for (language, client) in self.clients.iter_mut() {

            let json;
            let mut future = Self::check_client(client).boxed();

            if let Some(value) = (&mut future).now_or_never() {
                json = value?;
                ////eprintln!("Got json");
            } else {
                continue;
                //json = future.await?;
            }

            //eprintln!("Json for: {} \n{:#?}", language, json);

            match process_json(json).expect("Failed to process json") {
                LspMessage::Diagnostics(diagnostics) => {
                    ////eprintln!("Got diagnostics");
                    let sender = self.server_channels.get(language).unwrap().0.clone();

                    let message = LspControllerMessage::Response(
                        LspResponse::PublishDiagnostics(diagnostics)
                    );

                    sender.send(message).expect("Failed to send diagnostics");
                },
                LspMessage::Completions(completion) => {
                    //eprintln!("Got completion");
                    let sender = self.server_channels.get(language).unwrap().0.clone();

                    let message = LspControllerMessage::Response(
                        LspResponse::Completion(completion)
                    );

                    sender.send(message).expect("Failed to send completions");
                },
                LspMessage::Location(location) => {
                    //eprintln!("Got location");

                    let sender = self.server_channels.get(language).unwrap().0.clone();

                    let message = LspControllerMessage::Response(
                        LspResponse::Location(location)
                    );

                    sender.send(message).expect("Failed to send location");
                },
                LspMessage::None => {
                    ////eprintln!("Got none");
                    continue;
                }
                LspMessage::Capabilities(capabilities) => {
                    let semantic_token_legend = match capabilities.semanticTokensProvider {
                        None => continue,
                        Some(provider) => {
                            provider.legend.into()
                        }
                    };
                    //eprintln!("{:#?}", semantic_token_legend);

                    self.semantic_tokens.insert(language.clone(), semantic_token_legend);

                   //eprintln!("{:#?}", self.semantic_tokens);
                }
                LspMessage::SemanticTokens(tokens) => {
                    //eprintln!("Got semantic tokens");
                    let sender = self.server_channels.get(language).unwrap().0.clone();

                    match self.semantic_tokens.get(language) {
                        None => {
                            continue;
                        },
                        Some(token_legend) => {
                            let semantic_tokens = tokens.to_semantic_tokens(&token_legend.token_types, &token_legend.token_modifiers);

                            let message = LspControllerMessage::Response(
                                LspResponse::SemanticTokens(semantic_tokens));

                            sender.send(message).expect("Failed to send semantic tokens");
                        }
                    }
                }
            }
        }

        Ok(())
    }


    fn check_messages(&mut self) -> io::Result<()> {

        match self.listen.as_ref().unwrap().try_recv() {
            Ok(LspControllerMessage::CreateClient(lang)) => {
                self.create_client(lang)
            },
            Ok(LspControllerMessage::Request(lang, req)) => {
                self.check_request(lang, req)
            },
            Ok(LspControllerMessage::Notification(lang, notif)) => {
                self.check_notification(lang, notif)
            },
            Ok(LspControllerMessage::Exit) => {
                self.exit = true;
                return Ok(());
            },
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                return Err(io::Error::new(io::ErrorKind::Other, "Channel disconnected"));
            }
            Err(_) => {
                return Ok(());
            },
            _ => {
                return Ok(());
            }
        }

    }

    fn check_notification<R>(&mut self, lang: R, notif: LspNotification) -> io::Result<()> where R: AsRef<str> + Display {
        let mut clients_to_remove = Vec::new();
        match self.clients.get_mut(&lang.to_string()) {
            Some(client) => {
                match notif {
                    LspNotification::ChangeText(uri, version, text) => {
                        match client.did_change_text(uri.as_ref(), version, text.as_ref()) {
                            Ok(_) => {},
                            Err(_) => {
                                //eprintln!("Error: {:?}", e);
                                clients_to_remove.push(lang.to_string());
                            }
                        }
                    },
                    LspNotification::Open(uri, text) => {
                        match client.send_did_open(&lang.to_string(),uri.as_ref(), text.as_ref()) {
                            Ok(_) => {},
                            Err(_) => {
                                //eprintln!("Error: {:?}", e);
                                clients_to_remove.push(lang.to_string());
                            }
                        }
                    },
                    LspNotification::Close(uri) => {
                        match client.did_close(uri.as_ref()) {
                            Ok(_) => {},
                            Err(_) => {
                                //eprintln!("Error: {:?}", e);
                                clients_to_remove.push(lang.to_string());
                            }
                        }
                    },
                    LspNotification::Save(uri, text) => {
                        match client.did_save_text(uri.as_ref(), text.as_ref()) {
                            Ok(_) => {},
                            Err(_) => {
                                //eprintln!("Error: {:?}", e);
                                clients_to_remove.push(lang.to_string());
                            }
                        }
                    },
                    LspNotification::WillSave(uri, reason) => {
                        let reason = match reason.as_ref() {
                            "manual" => 1,
                            "afterDelay" => 2,
                            "focusOut" => 3,
                            _ => {
                                return Err(io::Error::new(io::ErrorKind::Other, "Invalid reason"));
                            }
                        };
                        match client.will_save_text(uri.as_ref(), reason) {
                            Ok(_) => {},
                            Err(_) => {
                                //eprintln!("Error: {:?}", e);
                                clients_to_remove.push(lang.to_string());
                            }
                        }
                    },
                }
            },
            None => {
                //return Err(io::Error::new(io::ErrorKind::Other, "No client for language"));
            }
        }

        if !clients_to_remove.is_empty() {
            for client in clients_to_remove {
                self.clients.remove(&client);
            }
        }

        Ok(())
    }

    fn check_request(&mut self, lang: Box<str>, req: LspRequest) -> io::Result<()> {
        match self.clients.get_mut(&*lang) {
            Some(client) => {
                match req {
                    LspRequest::Shutdown => {
                        client.send_shutdown()?;
                    },
                    LspRequest::Exit => {
                        client.send_exit()?;
                    },
                    LspRequest::RequestDiagnostic(uri) => {
                        client.request_diagnostic(uri.as_ref())?;
                    },
                    LspRequest::RequestCompletion(uri, pos, trigger) => {
                        let trigger = match trigger.as_ref() {
                            "invoked" => 1,
                            "triggerCharacter" => 2,
                            "triggerForIncompleteCompletions" => 3,
                            _ => {
                                return Err(io::Error::new(io::ErrorKind::Other, "Invalid trigger"));
                            }
                        };

                        client.request_completion(uri, pos, trigger)?;
                    },
                    LspRequest::GotoDeclaration(uri, pos) => {
                        client.goto_declaration(uri, pos)?;
                    },
                    LspRequest::GotoDefinition(uri, pos) => {
                        client.goto_definition(uri, pos)?;
                    },
                    LspRequest::GotoTypeDefinition(uri, pos) => {
                        client.goto_type_definition(uri, pos)?;
                    },
                    LspRequest::GotoImplementation(uri, pos) => {
                        client.goto_implementation(uri, pos)?;
                    },
                    LspRequest::SemanticTokens(uri) => {
                        client.request_semantic_tokens(uri)?;
                    },
                }
            },
            None => {
                return Err(io::Error::new(io::ErrorKind::Other, "No client for language"));
            }
        }

        Ok(())
    }

    fn create_client<R>(&mut self, lang: R) -> io::Result<()> where R: AsRef<str> {
        let client = match lang.as_ref() {
            "rust" => {
                if let Some((_, recv)) = self.server_channels.get("rust") {
                    self.response.as_ref().unwrap().send(LspControllerMessage::ClientCreated(recv.clone())).unwrap();
                    return Ok(());
                }
                let rust_analyzer = Command::new("rust-analyzer")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;

                let mut lsp_client = Client::new(rust_analyzer);

                lsp_client.initialize()?;

                lsp_client
            },
            "c" | "cpp" => {

                if let Some((_, recv)) = self.server_channels.get(lang.as_ref()) {
                    self.response.as_ref().unwrap().send(LspControllerMessage::ClientCreated(recv.clone())).unwrap();
                    return Ok(());
                }

                /*let clangd = Command::new("clangd")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;

                let mut lsp_client = Client::new(clangd);*/

                let c_lsp =  match Command::new("ccls")
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn() {
                    Ok(ccls) => ccls,
                    Err(_) => {
                        let clangd = match Command::new("clangd")
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .spawn() {
                            Ok(clangd) => clangd,
                            Err(_) => {
                                self.response.as_ref().unwrap().send(LspControllerMessage::NoClient).unwrap();
                                return Ok(());
                            }
                        };
                        clangd
                    }
                };

                let mut lsp_client = Client::new(c_lsp);

                lsp_client.initialize()?;

                lsp_client
            },
            "python" => {
                if let Some((_, recv)) = self.server_channels.get(lang.as_ref()) {
                    self.response.as_ref().unwrap().send(LspControllerMessage::ClientCreated(recv.clone())).unwrap();
                    return Ok(());
                }
                let python_lsp = Command::new("python-lsp-server")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;

                let mut lsp_client = Client::new(python_lsp);

                lsp_client.initialize()?;

                lsp_client
            },
            "swift" => {
                if let Some((_, recv)) = self.server_channels.get(lang.as_ref()) {
                    self.response.as_ref().unwrap().send(LspControllerMessage::ClientCreated(recv.clone())).unwrap();
                    return Ok(());
                }
                let apple_swift = Command::new("sourcekit-lsp")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;

                let mut lsp_client = Client::new(apple_swift);

                lsp_client.initialize()?;

                lsp_client
            },
            "go" => {
                if let Some((_, recv)) = self.server_channels.get(lang.as_ref()) {
                    self.response.as_ref().unwrap().send(LspControllerMessage::ClientCreated(recv.clone())).unwrap();
                    return Ok(());
                }
                let gopls = Command::new("gopls")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;

                let mut lsp_client = Client::new(gopls);

                lsp_client.initialize()?;

                lsp_client
            },
            "bash" => {
                if let Some((_, recv)) = self.server_channels.get(lang.as_ref()) {
                    self.response.as_ref().unwrap().send(LspControllerMessage::ClientCreated(recv.clone())).unwrap();
                    return Ok(());
                }
                let bash_lsp = Command::new("bash-language-server")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;

                let mut lsp_client = Client::new(bash_lsp);

                lsp_client.initialize()?;

                lsp_client
            },
            "haskell" => {
                if let Some((_, recv)) = self.server_channels.get(lang.as_ref()) {
                    self.response.as_ref().unwrap().send(LspControllerMessage::ClientCreated(recv.clone())).unwrap();
                    return Ok(());
                }
                let haskell_lsp = Command::new("haskell-language-server")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;

                let mut lsp_client = Client::new(haskell_lsp);

                lsp_client.initialize()?;

                lsp_client
            },
            _ => {
                self.response.as_ref().unwrap().send(LspControllerMessage::NoClient).unwrap();
                return Ok(());
            }
        };

        let (tx, rx) = std::sync::mpsc::channel();

        let rx = Arc::new(rx);

        self.server_channels.insert(lang.as_ref().to_string(), (tx, rx.clone()));

        //let client = Arc::new(Mutex::new(client));

        self.clients.insert(lang.as_ref().to_string(), client);

        self.response.as_ref().unwrap().send(LspControllerMessage::ClientCreated(rx)).unwrap();

        Ok(())
    }




}