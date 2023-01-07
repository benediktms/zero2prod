use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
pub async fn an_error_flash_message_is_set_on_failure() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": "john_doe",
        "password": "password"
    });

    let res = app.post_login(&login_body).await;

    let flash_cookie = res.cookies().find(|c| c.name() == "_flash").unwrap();
    assert_eq!(flash_cookie.value(), "Authentication failed");

    assert_eq!(res.status().as_u16(), 303);
    assert_is_redirect_to(&res, "/login");
}
