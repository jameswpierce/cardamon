use crate::cardamon::build;
use crate::cardamon::config::load_config;
use axum::Router;
use notify_debouncer_full::notify::RecursiveMode;
use notify_debouncer_full::notify::event::{CreateKind, EventKind, ModifyKind, RemoveKind};
use notify_debouncer_full::{DebounceEventResult, new_debouncer};
use std::path::Path;
use std::time::Duration;
use tower_http::services::{ServeDir, ServeFile};

fn is_relevant_event(event_kind: &EventKind) -> bool {
    match event_kind {
        EventKind::Create(create_kind) => match create_kind {
            CreateKind::File => true,   // New file created
            CreateKind::Folder => true, // New directory created
            CreateKind::Any => true,    // Any creation
            _ => false,
        },
        EventKind::Modify(modify_kind) => match modify_kind {
            ModifyKind::Name(_) => true,      // Rename or move
            ModifyKind::Data(_) => true,      // Content changes
            ModifyKind::Metadata(_) => false, // Metadata changes (timestamps, permissions)
            ModifyKind::Any => true,          // Any modification
            _ => false,
        },
        EventKind::Remove(remove_kind) => match remove_kind {
            RemoveKind::File => true,   // File removed
            RemoveKind::Folder => true, // Directory removed
            RemoveKind::Any => true,    // Any removal
            _ => false,
        },
        _ => false,
    }
}

#[tokio::main]
pub async fn serve(dev_mode: bool) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    println!("starting server...");
    let mut debouncer =
        new_debouncer(
            Duration::from_secs(1),
            None,
            |res: DebounceEventResult| match res {
                Ok(events) => {
                    let relevant_events: Vec<_> = events
                        .into_iter()
                        .filter(|event| is_relevant_event(&event.kind))
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

    debouncer.watch(
        Path::new(&config.directories.music),
        RecursiveMode::Recursive,
    )?;

    if dev_mode == true {
        debouncer.watch(
            Path::new(&config.directories.templates),
            RecursiveMode::Recursive,
        )?;
    }

    let root_path = format!("{}", &config.server.root_path);
    let music_path = format!("{}/music", &config.server.root_path);

    let app = Router::new()
        .nest_service(&root_path, ServeDir::new(&config.directories.output))
        .nest_service(&music_path, ServeDir::new(&config.directories.music));
    // Start the server
    let addr = format!("{}:{}", &config.server.domain, &config.server.port);
    println!("Server running on http://{}{}", addr, root_path);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
