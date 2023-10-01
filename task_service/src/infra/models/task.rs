use chrono::NaiveDateTime;

use crate::domain::entities::Task;

pub struct TaskDb {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub due_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Into<Task> for TaskDb {
    fn into(self) -> Task {
        Task {
            id: self.id,
            description: self.description,
            title: self.title,
            due_date: self.due_date,
        }
    }
}
