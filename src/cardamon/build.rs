use crate::cardamon::config::load_config;
use id3::{Tag, TagLike};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use askama::Template;
use walkdir::WalkDir;

use crate::cardamon::namespaces::{ALBUM_NAMESPACE, ARTIST_NAMESPACE, TRACK_NAMESPACE};

#[derive(Debug, Deserialize)]
struct Artist {
    id: String,
    name: String,
    albums: HashMap<String, Album>,
}

#[derive(Debug, Deserialize)]
struct Album {
    id: String,
    artist_id: String,
    title: String,
    tracks: HashMap<String, Track>,
}

// #[derive(Debug, Deserialize)]
// struct AlbumCover {
//     album_id: String,
//     file_name: String,
// }

#[derive(Debug, Deserialize)]
struct Track {
    id: String,
    number: u32,
    file_path: String,
    name: String,
    artist_id: String,
    album_id: String,
}

#[derive(Debug, Deserialize)]
struct Data {
    artists: HashMap<String, Artist>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    data: Data,
}

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let music_path = Path::new(&config.directories.music);

    let mut artists: HashMap<String, Artist> = HashMap::new();
    // let mut album_covers: HashMap<String, AlbumCover> = HashMap::new();
    // let mut tracks: HashMap<String, Track> = HashMap::new();

    for entry in WalkDir::new(music_path).into_iter().filter_map(|e| e.ok()) {
        let is_dir = entry.path().is_dir();

        if !is_dir {
            let extension = entry.path().extension().and_then(|ext| ext.to_str());
            match extension {
                None => {}
                Some("mp3") => {
                    let tag = Tag::read_from_path(entry.path())?;
                    let artist_id = Uuid::new_v5(
                        &ARTIST_NAMESPACE,
                        &tag.artist().unwrap_or("Unknown Artist").as_bytes(),
                    )
                    .to_string();
                    let album_id = Uuid::new_v5(
                        &ALBUM_NAMESPACE,
                        &tag.album().unwrap_or("Unknown Album").as_bytes(),
                    )
                    .to_string();
                    match artists.get_mut(&artist_id) {
                        Some(artist) => match artist.albums.get_mut(&album_id) {
                            Some(album) => {
                                let id = Uuid::new_v5(
                                    &TRACK_NAMESPACE,
                                    &entry.file_name().as_encoded_bytes(),
                                )
                                .to_string();
                                let track = Track {
                                    id: id.clone(),
                                    file_path: entry.path().to_string_lossy().to_string(),
                                    number: tag.track().unwrap_or(0),
                                    name: tag.title().unwrap_or("Unknown Title").to_string(),
                                    album_id: album_id.to_string(),
                                    artist_id: artist_id.to_string(),
                                };
                                album.tracks.insert(id.clone(), track);
                            }
                            None => {
                                let mut album = Album {
                                    id: album_id.clone(),
                                    artist_id: artist_id.clone(),
                                    title: tag.album().unwrap_or("Unknown Album").to_string(),
                                    tracks: HashMap::new(),
                                };
                                let id = Uuid::new_v5(
                                    &TRACK_NAMESPACE,
                                    &entry.file_name().as_encoded_bytes(),
                                )
                                .to_string();
                                let track = Track {
                                    id: id.clone(),
                                    file_path: entry.path().to_string_lossy().to_string(),
                                    number: tag.track().unwrap_or(0),
                                    name: tag.title().unwrap_or("Unknown Title").to_string(),
                                    album_id: album_id.to_string(),
                                    artist_id: artist_id.to_string(),
                                };
                                album.tracks.insert(id.clone(), track);
                                artist.albums.insert(album_id.clone(), album);
                            }
                        },
                        None => {
                            let mut artist = Artist {
                                id: artist_id.to_string(),
                                name: tag.artist().unwrap_or("Unknown Artist").to_string(),
                                albums: HashMap::new(),
                            };
                            match artist.albums.get_mut(&album_id) {
                                Some(album) => {
                                    let id = Uuid::new_v5(
                                        &TRACK_NAMESPACE,
                                        &entry.file_name().as_encoded_bytes(),
                                    )
                                    .to_string();
                                    let track = Track {
                                        id: id.clone(),
                                        file_path: entry.path().to_string_lossy().to_string(),
                                        number: tag.track().unwrap_or(0),
                                        name: tag.title().unwrap_or("Unknown Title").to_string(),
                                        album_id: album_id.to_string(),
                                        artist_id: artist_id.to_string(),
                                    };
                                    album.tracks.insert(id.clone(), track);
                                }
                                None => {
                                    let mut album = Album {
                                        id: album_id.clone(),
                                        artist_id: artist_id.clone(),
                                        title: tag.album().unwrap_or("Unknown Album").to_string(),
                                        tracks: HashMap::new(),
                                    };
                                    let id = Uuid::new_v5(
                                        &TRACK_NAMESPACE,
                                        &entry.file_name().as_encoded_bytes(),
                                    )
                                    .to_string();
                                    let track = Track {
                                        id: id.clone(),
                                        file_path: entry.path().to_string_lossy().to_string(),
                                        number: tag.track().unwrap_or(0),
                                        name: tag.title().unwrap_or("Unknown Title").to_string(),
                                        album_id: album_id.to_string(),
                                        artist_id: artist_id.to_string(),
                                    };
                                    album.tracks.insert(id.clone(), track);
                                    artist.albums.insert(album_id.clone(), album);
                                }
                            }
                            artists.insert(artist_id.clone(), artist);
                        }
                    };
                }
                Some(&_) => {}
            };
        }
    }

    let index = IndexTemplate {
        title: config.theme.title,
        data: Data { artists },
    };

    let index_html = index.render().unwrap();
    let output_path = Path::new(&config.directories.output);

    match fs::write(output_path.join("index.html"), index_html) {
        Err(why) => panic!("{:?}", why),
        Ok(_) => {}
    };

    Ok(())
}
