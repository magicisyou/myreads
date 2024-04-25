use serde::{Deserialize, Serialize};
use std::fmt;

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

impl ReadState {
    pub fn from(state: &str) -> Self {
        match state {
            "Completed" => Self::Completed,
            "Reading" => Self::Reading,
            "NotCompleted" => Self::NotCompleted,
            _ => Self::WishToRead,
        }
    }
}

impl fmt::Display for ReadState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadState::WishToRead => write!(f, "WishToRead"),
            ReadState::Reading => write!(f, "Reading"),
            ReadState::NotCompleted => write!(f, "NotCompleted"),
            ReadState::Completed => write!(f, "Completed"),
        }
    }
}
