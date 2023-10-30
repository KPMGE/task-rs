use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::{
    data::{dto::CreateUserDto, services::signup_service},
    domain::errors::ApiErrorType,
    infra::repositories::user::UserRepository,
};

#[post("/signup")]
pub async fn signup_controller(
    repo_data: Data<UserRepository>,
    user_json: Json<CreateUserDto>,
) -> impl Responder {
    let repo = repo_data.into_inner();
    let user = user_json.into_inner();

    signup_service(repo, user)
        .await
        .map(|token| HttpResponse::Ok().json(token))
        .unwrap_or_else(|e| match e.error_type {
            ApiErrorType::DatabaseError | ApiErrorType::UnexpectedError => {
                HttpResponse::InternalServerError().json(e.message)
            }
            ApiErrorType::ValidationError => HttpResponse::BadRequest().json(e.message),
        })
}
