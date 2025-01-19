use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    routing::get,
    Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashMap;

// 用户结构体
#[derive(Serialize)]
struct User {
    id: i64,
    name: String,
    email: String,
}

// 查询参数
#[derive(Deserialize)]
struct UserQuery {
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载 .env 文件
    dotenv().ok();

    // 连接 PostgreSQL
    let db = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

    // 创建路由
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/json", get(hello_json))
        .route("/query", get(hello_query))
        .route("/users", get(get_users))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?; // 启动服务器，返回值 → 表示服务器运行状态

    Ok(())
}
// 路由处理函数的返回值 → 发送给 HTTP 客户端
// 处理函数
async fn hello_world() -> &'static str {
    "Hello, World!"
}

// 处理 JSON 响应
async fn hello_json() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Hello, Rust Http!"
    }))
}

// 处理查询参数
async fn hello_query(Query(params): Query<HashMap<String, String>>) -> String {
    format!(
        "Hello, {}!",
        params.get("name").unwrap_or(&"World".to_string())
    )
}

// 查询用户
async fn get_users(
    State(db): State<PgPool>,
    Query(query): Query<UserQuery>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = match query.name {
        Some(name) => {
            let pattern = format!("%{}%", name);
            sqlx::query_as!(
                User,
                "SELECT id, name, email FROM users WHERE name LIKE $1",
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
