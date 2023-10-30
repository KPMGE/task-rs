#[derive(Debug)]
pub enum ApiErrorType {
    DatabaseError,
    ValidationError,
    UnexpectedError,
}

#[derive(Debug)]
pub struct ApiError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: ApiErrorType,
}
