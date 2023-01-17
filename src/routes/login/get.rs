use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut msg_html = String::new();

    for m in flash_messages.iter() {
        writeln!(msg_html, r#"<p><i>{}</i></p>"#, m.content()).unwrap();
    }

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
    {msg_html}
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
