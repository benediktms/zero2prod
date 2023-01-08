use actix_web::{cookie::Cookie, http::header::ContentType, HttpRequest, HttpResponse};

pub async fn login_form(request: HttpRequest) -> HttpResponse {
    let error_html = match request.cookie("_flash") {
        None => "".into(),
        Some(cookie) => {
            format!(r#"<p><i>{}</i></p>"#, cookie.value())
        }
    };

    let mut res = HttpResponse::Ok()
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
        ));

    res.add_removal_cookie(&Cookie::new("_flash", "")).unwrap();

    res
}
