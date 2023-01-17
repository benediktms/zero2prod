use uuid::Uuid;

use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    let app = spawn_app().await;

    let res = app.get_change_password().await;

    assert_is_redirect_to(&res, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    let res = app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "password_confirmation": &new_password,
        }))
        .await;

    assert_is_redirect_to(&res, "/login")
}

#[tokio::test]
async fn new_passwords_must_match() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_password = Uuid::new_v4().to_string();

    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    }))
    .await;

    let res = app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "password_confirmation": &another_password,
        }))
        .await;

    assert_is_redirect_to(&res, "/admin/password");

    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains(r#"<p><i>Passwords do not match</i></p>"#));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    }))
    .await;

    let res = app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "password_confirmation": &new_password,
        }))
        .await;

    assert_is_redirect_to(&res, "/admin/password");

    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains(r#"<p><i>Current password is incorrect</i></p>"#));
}

#[tokio::test]
async fn new_password_must_be_longer_than_8_characters() {
    let app = spawn_app().await;
    let new_password = "short";

    app.post_login(&serde_json::json!(
        {
            "username": &app.test_user.username,
            "password": &app.test_user.password
        }
    ))
    .await;

    let res = app
        .post_change_password(&serde_json::json!(
            {
                "current_password": &app.test_user.password,
                "new_password": &new_password,
                "password_confirmation": &new_password,
            }
        ))
        .await;

    assert_is_redirect_to(&res, "/admin/password");

    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains(r#"<p><i>Password must be at least 8 characters long</i></p>"#));
}

#[tokio::test]
async fn new_password_must_be_shorter_than_128_characters() {
    let app = spawn_app().await;
    let new_password = "a".repeat(129);

    app.post_login(&serde_json::json!(
        {
            "username": &app.test_user.username,
            "password": &app.test_user.password
        }
    ))
    .await;

    let res = app
        .post_change_password(&serde_json::json!(
            {
                "current_password": &app.test_user.password,
                "new_password": &new_password,
                "password_confirmation": &new_password,
            }
        ))
        .await;

    assert_is_redirect_to(&res, "/admin/password");

    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains(r#"<p><i>Password must be at most 128 characters long</i></p>"#));
}
