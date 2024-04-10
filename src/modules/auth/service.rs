use std::time::SystemTime;

use super::dto::sign_in::SignInDto;
use crate::{
    core::config::get_config,
    shared::{errors::CustomError, responses::CustomResponse},
};
use actix_web::web;
use bcrypt::verify;
use entity::user;
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: u64,
}

pub async fn sign_in(
    db: &DbConn,
    dto: &web::Json<SignInDto>,
) -> Result<CustomResponse, CustomError> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(dto.username.clone()))
        .one(db)
        .await
        .map_err(|_| CustomError::ServerError)?;

    if user.is_none() {
        return Err(CustomError::NotFound);
    }

    let mut user: user::Model = user.unwrap().into();

    if !verify(dto.password.clone().unwrap(), &user.password).unwrap() {
        return Err(CustomError::Unauthorized);
    }

    let claims = Claims {
        sub: user.id,
        exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 4 * 60 * 60,
    };

    let config = get_config().expect("Failed to read configuration.");

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.expose_secret().as_bytes()),
    )
    .unwrap();

    Ok(CustomResponse::Created { id: 1 as usize })
}
