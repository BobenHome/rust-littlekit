use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use sqlx::MySqlPool;

use crate::models::user::{User, UserQuery};

pub async fn get_users(
    State(db): State<MySqlPool>,
    Query(query): Query<UserQuery>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = match query.name {
        Some(name) => {
            let pattern = format!("%{}%", name);
            sqlx::query_as!(
                User,
                "SELECT id, name, email FROM users WHERE name LIKE ?",
                pattern
            )
            .fetch_all(&db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        }
        None => sqlx::query_as!(User, "SELECT id, name, email FROM users")
            .fetch_all(&db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
    };

    Ok(Json(users))
}
