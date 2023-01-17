use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::{
    authentication::{validate_credentails, AuthError, Credentials},
    routes::get_username,
    session_state::TypedSession,
    utils::{e500, see_other},
};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    password_confirmation: Secret<String>,
}

pub async fn change_password(
    data: web::Form<FormData>,
    session: TypedSession,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = session.get_user_id().map_err(e500)?;
    if user_id.is_none() {
        return Ok(see_other("/login"));
    }
    let user_id = user_id.unwrap();

    if data.new_password.expose_secret() != data.password_confirmation.expose_secret() {
        FlashMessage::error("Passwords do not match").send();
        return Ok(see_other("/admin/password"));
    }

    if data.new_password.expose_secret().len() < 8 {
        FlashMessage::error("Password must be at least 8 characters long").send();
        return Ok(see_other("/admin/password"));
    }
    if data.new_password.expose_secret().len() > 128 {
        FlashMessage::error("Password must be at most 128 characters long").send();
        return Ok(see_other("/admin/password"));
    }

    let username = get_username(user_id, &pool).await.map_err(e500)?;

    let credentials = Credentials {
        username,
        password: data.0.current_password,
    };

    if let Err(e) = validate_credentails(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("Current password is incorrect").send();
                return Ok(see_other("/admin/password"));
            }
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }

    Ok(HttpResponse::Ok().finish())
}
