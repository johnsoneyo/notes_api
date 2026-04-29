use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateNote {
   pub title: String,
   pub content: String,
}

#[derive(Serialize)]
pub struct Note {
   pub id: u64,
   pub title: String,
   pub content: String,
}
