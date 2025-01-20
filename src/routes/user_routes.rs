use axum::{routing::get, Router};
use sqlx::MySqlPool;

use crate::handlers::user_handler::get_users;

pub fn user_routes() -> Router<MySqlPool> {
    Router::new().route("/users", get(get_users))
}
