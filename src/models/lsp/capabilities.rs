use serde::Deserialize;
use crate::models::lsp::semantic_tokens::SemanticTokensLegendRaw;
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Capabilities {
    pub callHierarchyProvider: Option<bool>,
    pub codeActionProvider: Option<bool>,
    pub codeLensProvider: Option<serde_json::Value>,
    pub completionProvider: Option<serde_json::Value>,
    pub declarationProvider: Option<bool>,
    pub definitionProvider: Option<bool>,
    pub documentFormattingProvider: Option<bool>,
    pub documentHighlightProvider: Option<bool>,
    pub documentOnTypeFormattingProvider: Option<serde_json::Value>,
    pub documentRangeFormattingProvider: Option<bool>,
    pub documentSymbolProvider: Option<bool>,
    pub experimental: Option<serde_json::Value>,
    pub foldingRangeProvider: Option<bool>,
    pub hoverProvider: Option<bool>,
    pub implementationProvider: Option<bool>,
    pub inlayHintsProvider: Option<serde_json::Value>,
    pub positionEncoding: Option<String>,
    pub referencesProvider: Option<bool>,
    pub renameProvider: Option<serde_json::Value>,
    pub selectionRangeProvider: Option<bool>,
    pub semanticTokensProvider: Option<SemanticTokensProvider>,
    pub signatureHelpProvider: Option<serde_json::Value>,
    pub textDocumentSync: Option<serde_json::Value>,
    pub typeDefinitionProvider: Option<bool>,
    pub workspace: Option<serde_json::Value>,
    pub serverInfo: Option<serde_json::Value>,
}


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct SemanticTokensProvider {
    pub legend: SemanticTokensLegendRaw,
    pub range: Option<serde_json::Value>,
    pub full: Option<serde_json::Value>,
}