use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct TodoEntity {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub done_at: Option<DateTime<Utc>>,
}

impl TodoEntity {
    pub fn to_response(&self) -> TodoResponse {
        TodoResponse {
            id: self.id,
            title: self.title.clone(),
            content: self.content.clone(),
            created_at: self.created_at.clone(),
            done_at: self.done_at.clone(),
        }
    }
}

#[derive(Debug, FromRow, Serialize)]
pub struct TodoResponse {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub done_at: Option<DateTime<Utc>>,
}
