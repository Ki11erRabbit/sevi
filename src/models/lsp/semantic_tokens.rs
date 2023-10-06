use std::collections::HashMap;

pub struct SemanticTokensRaw {
    pub result_id: Option<String>,
    pub data: Vec<u32>,
}

impl SemanticTokensRaw {

    pub fn to_semantic_tokens(self, token_types: HashMap<u32, String>, modifiers: Vec<(u32, String)>) -> SemanticTokens {
        let mut counter = 0;
        let mut line = 0;
        let mut start_character = 0;
        let mut length = 0;
        let mut token_type = 0;
        let mut token_modifiers = 0;

        let mut tokens = Vec::new();

        for int in &self.data {
            match counter {
                0 => line = *int as usize,
                1 => start_character = *int as usize,
                2 => length = *int as usize,
                3 => token_type = *int,
                4 => {
                    token_modifiers = *int;
                    let mut tok_modifiers = Vec::new();

                    for modifier in &modifiers {
                        if token_modifiers & modifier.0 == modifier.0 {
                            tok_modifiers.push(modifier.1.clone());
                        }
                    }
                    let token_modifiers = tok_modifiers;


                    let token = SemanticToken {
                        line,
                        start_character,
                        length,
                        token_type: token_types.get(&token_type).unwrap().clone(),
                        token_modifiers,
                    };
                    tokens.push(token);
                    counter = 0;
                },
                _ => unreachable!("SemanticTokensRaw: counter is out of bounds"),
            }
        }

        SemanticTokens {
            result_id: self.result_id,
            data: tokens,
        }


    }
}

pub struct SemanticTokensPartialResultRaw {
    data: Vec<u32>,
}


pub struct SemanticToken {
    pub line: usize,
    pub start_character: usize,
    pub length: usize,
    pub token_type: String,
    pub token_modifiers: Vec<String>,
}

pub struct SemanticTokens {
    pub result_id: Option<String>,
    pub data: Vec<SemanticToken>,
}