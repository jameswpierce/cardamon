use crate::cardamon::config::load_config;
use id3::{Tag, TagLike};
use minijinja::{Environment, context};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;
use walkdir::WalkDir;

use crate::cardamon::namespaces::{ALBUM_NAMESPACE, ARTIST_NAMESPACE, TRACK_NAMESPACE};

#[derive(Debug, Deserialize, Serialize)]
struct Artist {
    id: String,
    name: String,
    albums: BTreeMap<String, Album>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Album {
    id: String,
    artist_id: String,
    title: String,
    tracks: Vec<Track>,
}

// #[derive(Debug, Deserialize)]
// struct AlbumCover {
//     album_id: String,
//     file_name: String,
// }

#[derive(Debug, Deserialize, Serialize)]
struct Track {
    id: String,
    number: u32,
    file_path: String,
    name: String,
    artist_id: String,
    album_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    artists: BTreeMap<String, Artist>,
}

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let mut env = Environment::new();
    let music_path = Path::new(&config.directories.music);

    let mut artists: BTreeMap<String, Artist> = BTreeMap::new();
    // let mut album_covers: HashMap<String, AlbumCover> = HashMap::new();

    for entry in WalkDir::new(music_path).into_iter().filter_map(|e| e.ok()) {
        let is_dir = entry.path().is_dir();

        if !is_dir {
            let extension = entry.path().extension().and_then(|ext| ext.to_str());
            match extension {
                None => {}
                Some("mp3") => match Tag::read_from_path(entry.path()) {
                    Err(_) => {}
                    Ok(tag) => {
                        let artist_name = tag.album_artist().unwrap_or(tag.artist().unwrap_or("Unknown Artist")).to_string();
                        let album_title = tag.album().unwrap_or("Unknown Album").to_string();
                        let artist_id =
                            Uuid::new_v5(&ARTIST_NAMESPACE, &artist_name.as_bytes()).to_string();
                        let album_id =
                            Uuid::new_v5(&ALBUM_NAMESPACE, &album_title.as_bytes()).to_string();
                        match artists.get_mut(&artist_name) {
                            Some(artist) => match artist.albums.get_mut(&album_title) {
                                Some(album) => {
                                    let id = Uuid::new_v5(
                                        &TRACK_NAMESPACE,
                                        &entry.file_name().as_encoded_bytes(),
                                    )
                                    .to_string();
                                    let track = Track {
                                        id: id.clone(),
                                        file_path: entry
                                            .path()
                                            .to_string_lossy()
                                            .to_string()
                                            .replace(&config.directories.music, ""),
                                        number: tag.track().unwrap_or(0),
                                        name: tag.title().unwrap_or("Unknown Title").to_string(),
                                        album_id: album_id.to_string(),
                                        artist_id: artist_id.to_string(),
                                    };
                                    album.tracks.push(track);
                                    album.tracks.sort_by(|a, b| a.number.cmp(&b.number));
                                }
                                None => {
                                    let mut album = Album {
                                        id: album_id.clone(),
                                        artist_id: artist_id.clone(),
                                        title: tag.album().unwrap_or("Unknown Album").to_string(),
                                        tracks: vec![],
                                    };
                                    let id = Uuid::new_v5(
                                        &TRACK_NAMESPACE,
                                        &entry.file_name().as_encoded_bytes(),
                                    )
                                    .to_string();
                                    let track = Track {
                                        id: id.clone(),
                                        file_path: entry
                                            .path()
                                            .to_string_lossy()
                                            .to_string()
                                            .replace(&config.directories.music, ""),
                                        number: tag.track().unwrap_or(0),
                                        name: tag.title().unwrap_or("Unknown Title").to_string(),
                                        album_id: album_id.to_string(),
                                        artist_id: artist_id.to_string(),
                                    };
                                    album.tracks.push(track);
                                    artist.albums.insert(album.title.clone(), album);
                                }
                            },
                            None => {
                                let mut artist = Artist {
                                    id: artist_id.to_string(),
                                    name: tag.artist().unwrap_or("Unknown Artist").to_string(),
                                    albums: BTreeMap::new(),
                                };
                                let mut album = Album {
                                    id: album_id.clone(),
                                    artist_id: artist_id.clone(),
                                    title: tag.album().unwrap_or("Unknown Album").to_string(),
                                    tracks: vec![],
                                };
                                let id = Uuid::new_v5(
                                    &TRACK_NAMESPACE,
                                    &entry.file_name().as_encoded_bytes(),
                                )
                                .to_string();
                                let track = Track {
                                    id: id.clone(),
                                    file_path: entry
                                        .path()
                                        .to_string_lossy()
                                        .to_string()
                                        .replace(&config.directories.music, ""),
                                    number: tag.track().unwrap_or(0),
                                    name: tag.title().unwrap_or("Unknown Title").to_string(),
                                    album_id: album_id.to_string(),
                                    artist_id: artist_id.to_string(),
                                };
                                album.tracks.push(track);
                                artist.albums.insert(album_title.clone(), album);
                                artists.insert(artist_name.clone(), artist);
                            }
                        };
                    }
                },

                Some(&_) => {}
            };
        }
    }

    let index_html = fs::read_to_string("templates/index.html")
        .expect("No template/index.html found in working directory.");
    env.add_template_owned("index_html", index_html).unwrap();
    let index_js = fs::read_to_string("templates/index.js")
        .expect("No template/index.js found in working directory.");
    env.add_template_owned("index_js", index_js).unwrap();

    let template = env.get_template("index_html").unwrap();
    let js_template = env.get_template("index_js").unwrap();
    let index_html_rendered = template
        .render(context! {
            title => config.theme.title,
            data => Data { artists },
        })
        .unwrap();
    let index_js_rendered = js_template.render(context! {}).unwrap();
    let output_path = Path::new(&config.directories.output);

    fs::write(output_path.join("index.html"), index_html_rendered)?;
    fs::write(output_path.join("index.js"), index_js_rendered)?;

    Ok(())
}
