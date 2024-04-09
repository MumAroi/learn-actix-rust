use validator::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct SignInDto{
    #[validate(required)]
    pub username: Option<String>,
    #[validate(required)]
    pub password: Option<String>,
}