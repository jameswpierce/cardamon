use crate::cardamon::build;
use crate::cardamon::config::load_config;
use axum::Router;
use notify_debouncer_mini::notify::RecursiveMode;
use notify_debouncer_mini::{DebounceEventResult, new_debouncer};
use std::path::Path;
use std::time::Duration;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
pub async fn serve(dev_mode: bool) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    println!("starting server...");

    let mut debouncer = new_debouncer(
        Duration::from_secs(1),
        |res: DebounceEventResult| match res {
            Ok(_) => {
                println!("Change detected, triggering rebuild...");
                let _ = build::build();
            }
            Err(e) => println!("Error {:?}", e),
        },
    )
    .unwrap();

    debouncer.watcher().watch(
        Path::new(&config.directories.music),
        RecursiveMode::Recursive,
    )?;

    if dev_mode == true {
        debouncer.watcher().watch(
            Path::new(&config.directories.templates),
            RecursiveMode::Recursive,
        )?;
    }

    let app = Router::new()
        .route_service("/", ServeFile::new("output/index.html"))
        .route_service("/music", ServeDir::new(&config.directories.music))
        .fallback_service(
            ServeDir::new("output").not_found_service(ServeFile::new("output/index.html")),
        );
    // Start the server
    let addr = "127.0.0.1:3001";
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
