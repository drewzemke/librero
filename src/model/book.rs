use super::open_library::OpenLibraryBook;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author_name: String,
    pub author_key: String,
    pub isbn: String,
}

impl TryFrom<OpenLibraryBook> for Book {
    type Error = String;

    fn try_from(book: OpenLibraryBook) -> Result<Self, Self::Error> {
        let extract_first_value = |list_opt: Option<Vec<_>>, error_msg: &str| {
            list_opt
                .and_then(|list| list.into_iter().next())
                .ok_or(String::from(error_msg))
        };

        let author_name = extract_first_value(book.author_name, "missing author_name")?;
        let author_key = extract_first_value(book.author_key, "missing author_key")?;
        let isbn = extract_first_value(book.isbn, "missing isbn")?;

        Ok(Self {
            title: book.title,
            author_name,
            author_key,
            isbn,
        })
    }
}
