use crate::{
    modules::auth::{dto::sign_in::SignInDto, service},
    shared::{errors::CustomError, responses::CustomResponse},
};
use actix_web::{post, web, HttpResponse};
use sea_orm::DbConn;

#[post("/sign-in")]
pub async fn sign_in(
    db: web::Data<DbConn>,
    sign_in_dto: web::Json<SignInDto>,
) -> Result<HttpResponse, CustomError> {
    let _ = service::sign_in(&db, &sign_in_dto).await;
    Ok(HttpResponse::Ok().json(""))
}
