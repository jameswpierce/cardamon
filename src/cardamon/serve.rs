use crate::cardamon::build;
use crate::cardamon::config::load_config;
use axum::Router;
use notify_debouncer_mini::notify::{RecursiveMode};
use notify_debouncer_mini::{DebounceEventResult, DebouncedEventKind, new_debouncer};
use std::path::Path;
use std::time::Duration;
use tower_http::services::{ServeDir, ServeFile};
use notify_debouncer_mini::notify::{
    event::{
        CreateKind,
        ModifyKind,
        RemoveKind,
        AccessKind,
    }
};

fn is_relevant_event(event_kind: &DebouncedEventKind) -> bool {
    match event_kind {
        DebouncedEventKind::Create(create_kind) => match create_kind {
            CreateKind::File => true,      // New file created
            CreateKind::Folder => true,    // New directory created
            CreateKind::Any => true,       // Any creation
            _ => false,
        },
        DebouncedEventKind::Modify(modify_kind) => match modify_kind {
            ModifyKind::Name(_) => true,   // Rename or move
            ModifyKind::Data(_) => true,   // Content changes
            ModifyKind::Metadata(_) => false, // Metadata changes (timestamps, permissions)
            ModifyKind::Any => true,       // Any modification
            _ => false,
        },
        DebouncedEventKind::Remove(remove_kind) => match remove_kind {
            RemoveKind::File => true,      // File removed
            RemoveKind::Folder => true,    // Directory removed
            RemoveKind::Any => true,       // Any removal
            _ => false,
        },
        _ => false,
    }
}

#[tokio::main]
pub async fn serve(dev_mode: bool) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    println!("starting server...");

    let mut debouncer = new_debouncer(
        Duration::from_secs(1),
        |res: DebounceEventResult| match res {
            Ok(events) => {
                let relevant_events: Vec<_> = events
                    .into_iter()
                    .filter(|event| {
                        is_relevant_event(&event.kind)
                    })
                    .collect();

                if !relevant_events.is_empty() {
                    println!("Change detected, triggering rebuild...");
                    let _ = build::build();
                }
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
    let addr = format!("{}:{}", &config.server.domain, &config.server.port);
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
