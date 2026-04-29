use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
}
