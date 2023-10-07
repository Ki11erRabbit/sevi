pub mod diagnostic;
pub mod completion;
pub mod location;
pub mod documentation;
pub mod semantic_tokens;
pub mod capabilities;

use serde::Deserialize;
use serde_json::Value;
use tokio::io;
use crate::models::lsp::capabilities::Capabilities;
use crate::models::lsp::completion::{CompletionList, PartialCompletionList};
use crate::models::lsp::diagnostic::Diagnostics;
use crate::models::lsp::location::{Location, LocationLink, LocationResponse};
use crate::models::lsp::semantic_tokens::{SemanticTokens, SemanticTokensRaw};


#[derive(Debug, Deserialize, PartialEq, Hash, Eq, Clone, Copy)]
pub struct LspRange {
    pub start: Position,
    pub end: Position,
}

impl LspRange {
    pub fn get_positions(&self) -> ((usize, usize), (usize, usize)) {
        let start = (self.start.character, self.start.line);
        let end = (self.end.character, self.end.line);
        (start, end)
    }
}

#[derive(Debug, Deserialize, PartialEq, Hash, Eq, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}


#[derive(Debug, PartialEq)]
pub enum LspMessage {
    None,
    Capabilities(Capabilities),
    Diagnostics(Diagnostics),
    Completions(CompletionList),
    Location(LocationResponse),
    SemanticTokens(SemanticTokensRaw),

}

pub fn process_json(json: Value) -> io::Result<LspMessage> {

    //eprintln!("json: {:#?}", json);

    if json["method"] != Value::Null {

        let method = json["method"].as_str().unwrap();
        match method {
            "textDocument/publishDiagnostics" => {
                let obj = json["params"].clone();
                //eprintln!("diagnostics");

                let diagnostics: Diagnostics = match serde_json::from_value(obj) {
                    Ok(value) => value,
                    Err(e) => {
                        //eprintln!("Error: {:?}", e);
                        return Ok(LspMessage::None);
                    }
                };
                Ok(LspMessage::Diagnostics(diagnostics))
            },

            _ => {
                println!("Unknown method: {}", method);
                Ok(LspMessage::None)
            }
        }
    }
    else if json["id"] != Value::Null {
        let id: usize = match serde_json::from_value(json["id"].clone()) {
            Ok(value) => value,
            Err(e) => {
                //eprintln!("Id Error: {:?}", e);
                return Ok(LspMessage::None);
            }
        };
        match id {
            1 => {
                let obj = json["result"]["capabilities"].clone();


                //eprintln!("capabilities: {:#?}", obj);

                let capabilities: Capabilities = match serde_json::from_value(obj) {
                    Ok(value) => value,
                    Err(_) => {
                        //eprintln!("Capabilities Error: {:?}", e);
                        return Ok(LspMessage::None);
                    }
                };
                //eprintln!("capabilities: {:#?}", capabilities);

                Ok(LspMessage::Capabilities(capabilities))
            }
            2 => {
                let obj = json["result"].clone();
                //eprintln!("completion");

                let completion_list: PartialCompletionList = match serde_json::from_value(obj) {
                    Ok(value) => value,
                    Err(e) => {
                        //eprintln!("Completion Error: {:?}", e);
                        return Ok(LspMessage::None);
                    }
                };
                Ok(LspMessage::Completions(completion_list.into()))
            },
            3 | 4 | 5 | 6 => {
                let obj = json["result"].clone();

                if obj.is_array() {
                    let locations: Vec<Location> = match serde_json::from_value(obj) {
                        Ok(value) => value,
                        Err(e) => {
                            //eprintln!("Location Error: {:?}", e);
                            return Ok(LspMessage::None);
                        }
                    };

                    let locations = LocationResponse::Locations(locations);

                    Ok(LspMessage::Location(locations))
                }
                else if obj.is_object() {
                    if json.get("uri").is_some() {
                        let location: Location = match serde_json::from_value(obj) {
                            Ok(value) => value,
                            Err(e) => {
                                //eprintln!("Location Error: {:?}", e);
                                return Ok(LspMessage::None);
                            }
                        };

                        let location = LocationResponse::Location(location);

                        Ok(LspMessage::Location(location))
                    }
                    else if json.get("targetUri").is_some() {
                        let location_link: LocationLink = match serde_json::from_value(obj) {
                            Ok(value) => value,
                            Err(e) => {
                                //eprintln!("Location Error: {:?}", e);
                                return Ok(LspMessage::None);
                            }
                        };

                        let location = LocationResponse::LocationLink(location_link);

                        Ok(LspMessage::Location(location))
                    }
                    else {
                        let location = LocationResponse::Null;

                        Ok(LspMessage::Location(location))
                    }

                }
                else {
                    Ok(LspMessage::None)
                }


            },
            7 => {
                let obj = json["result"].clone();
                //eprintln!("semantic tokens");

                let semantic_tokens: SemanticTokensRaw = match serde_json::from_value(obj) {
                    Ok(value) => value,
                    Err(e) => {
                        //eprintln!("Semantic Tokens Error: {:?}", e);
                        return Ok(LspMessage::None);
                    }
                };
                Ok(LspMessage::SemanticTokens(semantic_tokens))
            }
            _ => {
                //eprintln!("Unknown id: {}", id);
                Ok(LspMessage::None)
            }
        }

    }
    else {
        //eprintln!("Error: no method or result");
        Ok(LspMessage::None)
    }

}
