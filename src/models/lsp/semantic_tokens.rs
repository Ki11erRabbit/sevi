use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{Range, RangeInclusive};
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct SemanticTokensLegendRaw {
    pub tokenTypes: Vec<String>,
    pub tokenModifiers: Vec<String>,
}

impl Into<SemanticTokensLegend> for SemanticTokensLegendRaw {
    fn into(self) -> SemanticTokensLegend {
        let mut token_types = HashMap::new();
        let mut token_modifiers = Vec::new();

        for (index, token_type) in self.tokenTypes.iter().enumerate() {
            token_types.insert(index as u32, token_type.clone());
        }

        let mut bit = 1;

        for token_modifier in self.tokenModifiers.iter() {
            token_modifiers.push((bit, token_modifier.clone()));
            bit <<= 1;
        }

        SemanticTokensLegend {
            token_types,
            token_modifiers,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SemanticTokensLegend {
    pub token_types: HashMap<u32, String>,
    pub token_modifiers: Vec<(u32, String)>,
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct SemanticTokensRaw {
    pub resultId: Option<String>,
    pub data: Vec<u32>,
}

impl SemanticTokensRaw {

    pub fn to_semantic_tokens(self, token_types: &HashMap<u32, String>, modifiers: &Vec<(u32, String)>) -> SemanticTokens {
        let mut counter = 0;
        let mut line = 0;
        let mut start_character = 0;
        let mut length = 0;
        let mut token_type = 0;
        let mut token_modifiers = 0;

        let mut tokens = Vec::new();

        for int in &self.data {
            match counter {
                0 => {
                    line = *int as usize;
                    counter += 1;
                },
                1 => {
                    start_character = *int as usize;
                    counter += 1;
                },
                2 => {
                    length = *int as usize;
                    counter += 1;
                },
                3 => {
                    token_type = *int;
                    counter += 1;
                },
                4 => {
                    token_modifiers = *int;
                    let mut tok_modifiers = Vec::new();

                    for modifier in modifiers.iter() {
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
            result_id: self.resultId,
            data: tokens,
        }
    }
}

pub struct SemanticTokensPartialResultRaw {
    data: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SemanticToken {
    pub line: usize,
    pub start_character: usize,
    pub length: usize,
    pub token_type: String,
    pub token_modifiers: Vec<String>,
}

impl SemanticToken {
    pub fn generate_range(&self) -> (RangeInclusive<usize>, usize) {
        let range = self.start_character..=self.start_character + self.length;

        (range, self.line)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SemanticTokens {
    pub result_id: Option<String>,
    pub data: Vec<SemanticToken>,
}

impl SemanticTokens {
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}