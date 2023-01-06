use actix_web::{http::header::ContentType, web::Query, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

pub async fn login_form(query: Query<QueryParams>) -> HttpResponse {
    let error_html = match query.0.error {
        Some(error_message) => format!("<p><i>{error_message}</i></p>"),
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
