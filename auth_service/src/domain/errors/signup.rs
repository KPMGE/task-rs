pub enum SignupError {
    DatabaseError(sqlx::Error),
    InvalidUserError(validator::ValidationErrors),
    TokenError(jsonwebtoken::errors::Error),
} 
