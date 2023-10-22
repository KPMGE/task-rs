use validator::Validate;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginUserDto {
    pub name: String,
    #[validate(email)]
    pub email: String,
}
