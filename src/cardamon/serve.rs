use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting server...");
    let app = Router::new()
        .route_service("/", ServeFile::new("output/index.html"))
        .fallback_service(
            ServeDir::new("output").not_found_service(ServeFile::new("output/index.html")),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
