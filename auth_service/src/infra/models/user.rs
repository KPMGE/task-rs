use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct UserDb {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}
