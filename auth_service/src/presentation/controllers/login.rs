use actix_web::{
    get,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::{
    data::{dto::LoginUserDto, services::login_service},
    infra::repositories::user::UserRepository,
};

#[get("/login")]
pub async fn login_controller(
    user_repo: Data<UserRepository>,
    json_user_dto: Json<LoginUserDto>,
) -> impl Responder {
    let repo = user_repo.into_inner();
    let user_dto = json_user_dto.into_inner();

    login_service(repo, user_dto)
        .await
        .map(|token| HttpResponse::Ok().json(token))
        .unwrap_or_else(|e| {
            eprintln!("ERROR: {}", e);
            HttpResponse::InternalServerError().finish()
        })
}
