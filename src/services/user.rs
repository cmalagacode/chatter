use axum::Json;
use axum::http::StatusCode;
use crate::models::user::{CreateUser, User};
use crate::models::message::Message;
use crate::db::db::{insert_user, get_users};

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<Message>) {
    let status = insert_user(Json(payload));
    (status, Json(Message{ status_code: status.as_u16(), message: String::from("success") }))
}

pub async fn get_all_users() -> (StatusCode, Json<Vec<User>>) {
    let (status, users) = get_users();
    (status, users)
}
