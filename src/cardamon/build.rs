use serde::Deserialize;
use std::fs;
use std::fs::{read_dir, read_to_string};
use std::path::Path;
use toml;
use uuid::Uuid;

use askama::Template;
use walkdir::{DirEntry, Error, WalkDir};

use crate::cardamon::namespaces::{ALBUM_NAMESPACE, ARTIST_NAMESPACE, TRACK_NAMESPACE};

#[derive(Debug, Deserialize)]
struct Artist {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Album {
    id: String,
    artist_id: String,
    title: String,
}

#[derive(Debug, Deserialize)]
struct AlbumCover {
    album_id: String,
    file_name: String,
}

#[derive(Debug, Deserialize)]
struct Track {
    id: String,
    file_path: String,
    name: String,
    artist_id: String,
    album_id: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    directories: Directories,
    theme: Theme,
}

#[derive(Debug, Deserialize)]
struct Directories {
    music: String,
    output: String,
}

#[derive(Debug, Deserialize)]
struct Theme {
    title: String,
}

#[derive(Debug, Deserialize)]
struct Data {
    artists: Vec<Artist>,
    albums: Vec<Album>,
    album_covers: Vec<AlbumCover>,
    tracks: Vec<Track>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    data: Data,
}

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    let config_raw =
        read_to_string("config.toml").expect("No config.toml found in working directory.");
    let config: Config = toml::from_str(&config_raw)?;
    let music_path = Path::new(&config.directories.music);

    let mut artists: Vec<Artist> = vec![];
    let mut albums: Vec<Album> = vec![];
    let mut album_covers: Vec<AlbumCover> = vec![];
    let mut tracks: Vec<Track> = vec![];

    println!("reading directories...");
    for entry in WalkDir::new(music_path).into_iter().filter_map(|e| e.ok()) {
        let extension = entry.path().extension();
        match extension {
            None => {}
            Some(ext) => {
                println!("{:?}", ext);
            }
        };
    }
    // match read_dir(music_path) {
    //     Err(why) => panic!("{:?}", why),
    //     Ok(artist_dirs) => {
    //         for artist in artist_dirs {
    //             match artist {
    //                 Err(why) => panic!("{:?}", why),
    //                 Ok(artist) => {
    //                     let path = artist.path();
    //                     if path.is_dir() {
    //                         let artist_id = Uuid::new_v5(
    //                             &ARTIST_NAMESPACE,
    //                             &artist.file_name().as_encoded_bytes(),
    //                         );
    //                         let artist = Artist {
    //                             id: artist_id.to_string(),
    //                             name: artist.file_name().into_string().unwrap(),
    //                         };
    //                         artists.push(artist);
    //                         match read_dir(path) {
    //                             Err(why) => panic!("{:?}", why),
    //                             Ok(album_dirs) => {
    //                                 for album in album_dirs {
    //                                     match album {
    //                                         Err(why) => panic!("{:?}", why),
    //                                         Ok(album) => {
    //                                             let path = album.path();
    //                                             if path.is_dir() {
    //                                                 let album_id = Uuid::new_v5(
    //                                                     &ALBUM_NAMESPACE,
    //                                                     &album.file_name().as_encoded_bytes(),
    //                                                 );
    //                                                 let album = Album {
    //                                                     id: album_id.to_string(),
    //                                                     artist_id: artist_id.to_string(),
    //                                                     title: album
    //                                                         .file_name()
    //                                                         .into_string()
    //                                                         .unwrap(),
    //                                                 };
    //                                                 albums.push(album);
    //                                                 match read_dir(path) {
    //                                                     Err(why) => panic!("{:?}", why),
    //                                                     Ok(files) => {
    //                                                         for file in files {
    //                                                             match file {
    //                                                                 Err(why) => panic!("{:?}", why),
    //                                                                 Ok(file) => {
    //                                                                     let file_path = file.path();
    //                                                                     println!("{:?}", file_path);
    //                                                                     let extension =
    //                                                                         Path::new(&file_path)
    //                                                                             .extension();
    //                                                                     match extension {
    //                                                                         None => {}
    //                                                                         Some(extension) => {
    //                                                                             match extension
    //                                                                                 .to_str()
    //                                                                             {
    //                                                                                 None => {}
    //                                                                                 Some("mp3") => {
    //                                                                                     let track: Track = Track {
    //                                                                                         id: Uuid::new_v5(
    //                                                                                             &TRACK_NAMESPACE,
    //                                                                                             &file.file_name().as_encoded_bytes(),
    //                                                                                         ).to_string(),
    //                                                                                         file_path: file.path().to_string_lossy().to_string(),
    //                                                                                         name: file.file_name().into_string().unwrap(),
    //                                                                                         artist_id: artist_id.to_string(),
    //                                                                                         album_id: album_id.to_string(),
    //                                                                                     };
    //                                                                                     tracks
    //                                                                                         .push(
    //                                                                                         track,
    //                                                                                     );
    //                                                                                 }
    //                                                                                 Some("jpg") => {
    //                                                                                     let album_cover = AlbumCover {
    //                                                                                         album_id: album_id.to_string(),
    //                                                                                         file_name: file.file_name().into_string().unwrap(),
    //                                                                                     };
    //                                                                                     album_covers.push(album_cover);
    //                                                                                 }
    //                                                                                 Some("png") => {
    //                                                                                     let album_cover = AlbumCover {
    //                                                                                         album_id: album_id.to_string(),
    //                                                                                         file_name: file.file_name().into_string().unwrap(),
    //                                                                                     };
    //                                                                                     album_covers.push(album_cover);
    //                                                                                 }
    //                                                                                 Some(&_) => {}
    //                                                                             }
    //                                                                         }
    //                                                                     }
    //                                                                 }
    //                                                             }
    //                                                         }
    //                                                     }
    //                                                 }
    //                                             }
    //                                         }
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    let index = IndexTemplate {
        title: config.theme.title,
        data: Data {
            tracks,
            albums,
            album_covers,
            artists,
        },
    };

    let index_html = index.render().unwrap();
    let output_path = Path::new(&config.directories.output);

    match fs::write(output_path.join("index.html"), index_html) {
        Err(why) => panic!("{:?}", why),
        Ok(_) => {}
    };

    Ok(())
}
