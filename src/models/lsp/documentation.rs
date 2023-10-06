use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Hash, Eq, Clone)]
pub enum DocumentationType {
    String(String),
    MarkupContent(MarkupContent),
}

#[derive(Debug, Deserialize, PartialEq, Hash, Eq, Clone)]
pub struct MarkupContent {
    pub kind: String,
    pub value: String,
}