use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::handlers::user_handler::get_users;

pub fn user_routes() -> Router<PgPool> {
    Router::new().route("/users", get(get_users))
}
