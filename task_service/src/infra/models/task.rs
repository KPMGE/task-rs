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

impl From<TaskDb> for Task {
    fn from(value: TaskDb) -> Self {
        Task {
            id: value.id,
            description: value.description,
            title: value.title,
            due_date: value.due_date,
        }
    }
}

impl From<&TaskDb> for Task {
    fn from(value: &TaskDb) -> Self {
        Task {
            id: value.id,
            description: value.description.clone(),
            title: value.title.clone(),
            due_date: value.due_date,
        }
    }
}
