use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    pub name: String,
    #[validate(email)]
    pub email: String
}
