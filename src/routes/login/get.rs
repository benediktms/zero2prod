use crate::startup::HmacSecret;
use actix_web::{
    http::header::ContentType,
    web::{Data, Query},
    HttpResponse,
};
use hmac::{Hmac, Mac};
use secrecy::ExposeSecret;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    error: String,
    tag: String,
}

impl QueryParams {
    fn verify(self, secret: &HmacSecret) -> Result<String, anyhow::Error> {
        let tag = hex::decode(self.tag)?;
        let query_string = format!("error={}", urlencoding::Encoded::new(&self.error));

        let mut mac =
            Hmac::<sha2::Sha256>::new_from_slice(secret.0.expose_secret().as_bytes()).unwrap();
        mac.update(query_string.as_bytes());
        mac.verify_slice(&tag)?;

        Ok(self.error)
    }
}

pub async fn login_form(
    query: Option<Query<QueryParams>>,
    secret: Data<HmacSecret>,
) -> HttpResponse {
    let error_html = match query {
        Some(query) => match query.0.verify(&secret) {
            Ok(error) => {
                format!("<p><i>{}</i></p>", htmlescape::encode_minimal(&error))
            }
            Err(e) => {
                tracing::warn!(
                    error.message = %e,
                    error.cause_chain = ?e,
                    "Failed to verify query parameters using the HMAC tag."
                );

                "".into()
            }
        },

        None => "".into(),
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Login</title>
  </head>
  <body>
    {error_html}
    <h1>Login</h1>
    <form action="/login" method="post">
      <label>
        Username
        <input type="text" placeholder="Enter username" name="username" />
      </label>

      <label>
        Password
        <input type="password" name="password" />
      </label>

      <button type="submit">Submit</button>
    </form>
  </body>
</html>
"#
        ))
}
