mod handlers;
mod models;
mod routes;

use axum::Router;
use dotenv::dotenv;
use routes::user_routes::user_routes;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

    let app = Router::new().merge(user_routes()).with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
