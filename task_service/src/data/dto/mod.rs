use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskDto {
    pub title: String,
    pub description: String,
    pub due_date: NaiveDateTime,
}
