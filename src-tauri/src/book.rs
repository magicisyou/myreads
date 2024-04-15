use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub name: String,
    pub author: String,
    pub read_state: ReadState,
    pub starred: bool,
}

#[derive(Serialize, Deserialize)]
pub enum ReadState {
    Completed,
    WishToRead,
    Reading,
    NotCompleted,
}

impl Book {
    pub fn from(name: String, author: String, read_state: ReadState, starred: bool) -> Self {
        Self {
            name,
            author,
            read_state,
            starred,
        }
    }
}
