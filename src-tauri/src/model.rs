use serde::Serialize;

#[derive(Serialize)]
pub struct Memo {
    pub id: String,
    pub topic_id: String,
    pub timestamp: i64,
    pub latest: bool,
    pub content: String,
}

#[derive(Serialize)]
pub struct Topic {
    pub id: String,
    pub title: String,
    pub timestamp: i64,
}
