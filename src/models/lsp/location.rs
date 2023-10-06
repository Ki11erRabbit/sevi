use serde::Deserialize;
use crate::models::lsp::LspRange;

#[derive(Debug, Deserialize, PartialEq, Hash, Eq, Clone)]
pub enum LocationResponse {
    Location(Location),
    LocationLink(LocationLink),
    Locations(Vec<Location>),
    Null,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq, Hash, Eq, Clone)]
pub struct Location {
    pub uri: String,
    pub range: LspRange,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq, Hash, Eq, Clone)]
pub struct LocationLink {
    pub originSelectionRange: Option<LspRange>,
    pub targetUri: String,
    pub targetRange: LspRange,
    pub targetSelectionRange: LspRange,
}