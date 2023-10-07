use serde::Deserialize;
use serde_json::Value;
use crate::models::lsp::documentation::DocumentationType;
use crate::models::lsp::LspRange;


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PartialCompletionList {
    pub isIncomplete: bool,
    pub items: Vec<PartialCompletionItem>,
}

impl Into<CompletionList> for PartialCompletionList {
fn into(self) -> CompletionList {
        let mut items = Vec::new();
        for item in self.items {
            items.push(item.into());
        }
        CompletionList {
            isIncomplete: self.isIncomplete,
            items,
        }
    }
}
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CompletionList {
    pub isIncomplete: bool,
    pub items: Vec<CompletionItem>,
}

impl CompletionList {
    pub fn generate_text(&self) -> Vec<(String, String)> {
        let mut result = Vec::new();
        for item in &self.items {
            let (insert_text, info) = item.generate_text();
            result.push((insert_text, info));
        }
        result
    }

    pub fn get_completion(&self, index: usize) -> Option<&CompletionItem> {
        self.items.get(index)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq,   Clone)]
pub struct PartialCompletionItem {
    pub label: String,
    pub labelDetails: Option<CompletionItemLabelDetails>,
    pub kind: usize,
    pub score: Option<f64>,
    pub tags: Option<Vec<usize>>,
    pub detail: Option<String>,
    pub documentation: Option<Value>,
    pub deprecated: Option<bool>,
    pub preselect: Option<bool>,
    pub sortText: Option<String>,
    pub filterText: Option<String>,
    pub insertText: Option<String>,
    pub insertTextFormat: Option<usize>,
    pub insertTextMode: Option<usize>,
    pub textEdit: Option<Value>,
    pub textEditText: Option<String>,
    pub additionalTextEdits: Option<Vec<TextEdit>>,
    pub commitCharacters: Option<Vec<String>>,
    pub command: Option<Command>,
    pub data: Option<Value>,
}

impl PartialCompletionItem {
    pub fn get_edit_text(&self) -> Option<TextEditType> {
        if let Some(text_edit) = &self.textEdit {
            if let Ok(edit_text) = serde_json::from_value::<TextEdit>(text_edit.clone()) {
                return Some(TextEditType::TextEdit(edit_text));
            }
            else if let Ok(edit_text) = serde_json::from_value::<InsertReplaceEdit>(text_edit.clone()) {
                return Some(TextEditType::InsertReplaceEdit(edit_text));
            }
            else {
                return None;
            }
        }
        None
    }
}

impl Into<CompletionItem> for PartialCompletionItem {
    fn into(self) -> CompletionItem {
        CompletionItem {
            textEdit: self.get_edit_text(),
            label: self.label,
            labelDetails: self.labelDetails,
            kind: self.kind,
            score: self.score,
            tags: self.tags,
            detail: self.detail,
            documentation: match self.documentation {
                Some(value) => {
                    match serde_json::from_value::<DocumentationType>(value) {
                        Ok(value) => Some(value),
                        Err(_) => None,
                    }
                },
                None => None,
            },
            deprecated: self.deprecated,
            preselect: self.preselect,
            sortText: self.sortText,
            filterText: self.filterText,
            insertText: self.insertText,
            insertTextFormat: self.insertTextFormat,
            insertTextMode: self.insertTextMode,
            textEditText: self.textEditText,
            additionalTextEdits: self.additionalTextEdits,
            commitCharacters: self.commitCharacters,
            command: self.command,
            data: self.data,
        }
    }
}
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq,   Clone)]
pub struct CompletionItem {
    pub label: String,
    pub labelDetails: Option<CompletionItemLabelDetails>,
    pub kind: usize,
    pub score: Option<f64>,
    pub tags: Option<Vec<usize>>,
    pub detail: Option<String>,
    pub documentation: Option<DocumentationType>,
    pub deprecated: Option<bool>,
    pub preselect: Option<bool>,
    pub sortText: Option<String>,
    pub filterText: Option<String>,
    pub insertText: Option<String>,
    pub insertTextFormat: Option<usize>,
    pub insertTextMode: Option<usize>,
    pub textEdit: Option<TextEditType>,
    pub textEditText: Option<String>,
    pub additionalTextEdits: Option<Vec<TextEdit>>,
    pub commitCharacters: Option<Vec<String>>,
    pub command: Option<Command>,
    pub data: Option<Value>,
}

impl CompletionItem {
    pub fn generate_text(&self) -> (String, String) {

        let kind = match self.kind {
            1 => "Text",
            2 => "Method",
            3 => "Function",
            4 => "Constructor",
            5 => "Field",
            6 => "Variable",
            7 => "Class",
            8 => "Interface",
            9 => "Module",
            10 => "Property",
            11 => "Unit",
            12 => "Value",
            13 => "Enum",
            14 => "Keyword",
            15 => "Snippet",
            16 => "Color",
            17 => "File",
            18 => "Reference",
            19 => "Folder",
            20 => "EnumMember",
            21 => "Constant",
            22 => "Struct",
            23 => "Event",
            24 => "Operator",
            25 => "TypeParameter",
            _ => "Unknown",
        };

        let info = format!("({}) {}", kind, self.label);


        let mut insert_text = String::new();
        if let Some(text_edit) = &self.textEdit {
            insert_text = match text_edit {
                TextEditType::TextEdit(text_edit) => text_edit.newText.clone(),
                TextEditType::InsertReplaceEdit(text_edit) => {
                    text_edit.new_text.clone()
                }
            };
        } else if let Some(text_edit) = &self.textEditText {
            insert_text = text_edit.clone();
        } else if let Some(text_edit) = &self.insertText {
            insert_text = text_edit.clone();
        }
        (insert_text, info)
    }
}

#[derive(Debug, Deserialize, PartialEq, Hash, Eq, Clone)]
pub struct CompletionItemLabelDetails {
    detail: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Hash, Eq, Clone)]
pub enum TextEditType {
    TextEdit(TextEdit),
    InsertReplaceEdit(InsertReplaceEdit),
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq, Hash, Eq, Clone)]
pub struct TextEdit {
    pub range: LspRange,
    pub newText: String,
}

impl TextEdit {
    pub fn get_range(&self) -> ((usize, usize), (usize, usize)) {
        let start = self.range.start;
        let end = self.range.end;
        ((start.character, start.line), (end.character, end.line))
    }
}

#[derive(Debug, Deserialize, PartialEq, Hash, Eq, Clone)]
pub struct InsertReplaceEdit {
    pub insert: LspRange,
    pub replace: LspRange,
    pub new_text: String,
}

#[derive(Debug, Deserialize, PartialEq,  Eq, Clone)]
pub struct Command {
    pub title: String,
    pub command: String,
    pub arguments: Option<Vec<Value>>,
}

