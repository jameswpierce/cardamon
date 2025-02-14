use crate::cardamon::build;
use crate::cardamon::config::load_config;
use axum::Router;
use notify::{RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::broadcast;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    println!("starting server...");
    let (tx, _) = broadcast::channel::<()>(10);

    let watch_tx = tx.clone();
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(_) => {
            println!("Change detected, triggering rebuild...");
            let _ = build::build();
            let _ = watch_tx.send(());
        }
        Err(e) => eprintln!("Watch error: {}", e),
    })?;

    watcher.watch(
        Path::new(&config.directories.music),
        RecursiveMode::Recursive,
    )?;

    let app = Router::new()
        .route_service("/", ServeFile::new("output/index.html"))
        .fallback_service(
            ServeDir::new("output").not_found_service(ServeFile::new("output/index.html")),
        );
    // Start the server
    let addr = "127.0.0.1:3000";
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
