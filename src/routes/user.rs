use axum::{
    routing::{get, post},
    Router
};
use crate::handlers::user_handlers::{create_user, get_users};

pub fn get_user_routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users", post(create_user))
}