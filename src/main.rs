mod handlers;
mod models;
mod routes;
mod utils;

use axum::Router;
use dotenv::dotenv;
use routes::user_routes::user_routes;
use sqlx::MySqlPool;
use tracing::info;
use utils::logger::init_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    init_logger();

    dotenv().ok();
    info!("Environment loaded");

    let db = MySqlPool::connect(&std::env::var("DATABASE_URL")?).await?;
    info!("Database connected");

    let app = Router::new().merge(user_routes()).with_state(db);
    info!("Router initialized");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
