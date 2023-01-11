use actix_web::{web, HttpResponse};
use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    password_confirmation: Secret<String>,
}

pub async fn change_password(data: web::Form<FormData>) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}
