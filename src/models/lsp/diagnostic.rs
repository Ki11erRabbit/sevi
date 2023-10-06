use serde::Deserialize;
use crate::models::lsp::LspRange;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Diagnostics {
    pub diagnostics: Vec<Diagnostic>,
    pub uri: String,
    pub version: Option<usize>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Diagnostics {
            diagnostics: Vec::new(),
            uri: String::new(),
            version: Some(0),
        }
    }
    pub fn diagnostics_on_line(&self, line: usize) -> Vec<&Diagnostic> {
        let mut result = Vec::new();
        for diagnostic in &self.diagnostics {
            let start_line = diagnostic.range.start.line;
            let end_line = diagnostic.range.end.line;
            if line >= start_line && line <= end_line {
                result.push(diagnostic);
            }
        }
        result
    }

    pub fn get_diagnostic(&self, line: usize, character: usize) -> Option<&Diagnostic> {
        ////eprintln!("{:?}", self.diagnostics);
        for diagnostic in &self.diagnostics {
            let start_line = diagnostic.range.start.line;
            let end_line = diagnostic.range.end.line;
            let start_character = diagnostic.range.start.character;
            let end_character = diagnostic.range.end.character;
            if line >= start_line && line <= end_line {
                if character >= start_character && character <= end_character {
                    return Some(diagnostic);
                }
            }
        }
        None
    }

    pub fn merge(&mut self, other: Diagnostics) {
        self.diagnostics.extend(other.diagnostics);
    }
}

#[derive(Debug, PartialEq, Deserialize, Hash, Eq, Clone)]
pub struct CodeDescription {
    pub href: String,
}

#[derive(Debug, Deserialize, PartialEq, Hash, Eq, Clone)]
pub struct Data {
    pub rendered: String,
}

#[derive(Debug, Deserialize, PartialEq, Hash, Eq, Clone)]
pub struct Diagnostic {
    /// The type of the diagnostic.
    pub code: Option<String>,
    /// The description of the diagnostic.
    pub code_description: Option<CodeDescription>,
    /// Additional metadata about the diagnostic.
    pub data: Option<Data>,
    /// The message to display to the user.
    pub message: String,
    /// The range where the error/warning is located in the source code.
    pub range: LspRange,
    /// The severity of the error/warning.
    pub severity: usize,
    /// The source of the error/warning which is the LSP
    pub source: Option<String>,
}
