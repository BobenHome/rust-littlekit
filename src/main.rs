use axum::{
    extract::{Json, Query},
    routing::get,
    Router,
};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建路由
    let app: Router = Router::new()
        .route("/", get(hello_world))
        .route("/json", get(hello_json))
        .route("/query", get(hello_query));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    
    println!("Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

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
