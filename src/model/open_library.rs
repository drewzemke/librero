use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenLibrarySearchResult {
    pub docs: Vec<OpenLibraryBook>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OpenLibraryBook {
    pub title: String,
    pub author_name: Option<Vec<String>>,
    pub author_key: Option<Vec<String>>,
    pub isbn: Option<Vec<String>>,
}
