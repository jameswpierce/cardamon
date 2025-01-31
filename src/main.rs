use serde::Deserialize;
use std::fs::{read_dir, read_to_string};
use std::path::Path;
use toml;

#[derive(Debug)]
struct Track {
    file_name: String,
    name: String,
    artist: String,
    album: String,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_raw =
        read_to_string("config.toml").expect("No config.toml found in working directory.");
    let config: Config = toml::from_str(&config_raw)?;
    let music_path = Path::new(&config.directories.music);

    let mut tracks: Vec<Track> = vec![];

    match read_dir(music_path) {
        Err(why) => panic!("{:?}", why),
        Ok(artists) => {
            for artist in artists {
                match artist {
                    Err(why) => panic!("{:?}", why),
                    Ok(artist) => {
                        let path = artist.path();
                        if path.is_dir() {
                            // println!("artist: {:?}", artist.path());
                            match read_dir(path) {
                                Err(why) => panic!("{:?}", why),
                                Ok(albums) => {
                                    for album in albums {
                                        match album {
                                            Err(why) => panic!("{:?}", why),
                                            Ok(album) => {
                                                let path = album.path();
                                                if path.is_dir() {
                                                    // println!("album: {:?}", album.path());
                                                    match read_dir(path) {
                                                        Err(why) => panic!("{:?}", why),
                                                        Ok(files) => {
                                                            for file in files {
                                                                match file {
                                                                    Err(why) => panic!("{:?}", why),
                                                                    Ok(file) => {
                                                                        let file_path = file.path();
                                                                        let extension =
                                                                            Path::new(&file_path)
                                                                                .extension();
                                                                        // println!(
                                                                        //     "track: {:?}",
                                                                        //     &file_path
                                                                        // );
                                                                        match extension {
                                                                            None => {}
                                                                            Some(extension) => {
                                                                                match extension
                                                                                    .to_str()
                                                                                {
                                                                                    None => {}
                                                                                    Some("mp3") => {
                                                                                        // println!(
                                                                                        //     "MP3 U BASTERD"
                                                                                        // );
                                                                                        let track: Track = Track {
                                                                                            file_name: file.file_name().into_string().unwrap(),
                                                                                            name: "track name".to_string(),
                                                                                            artist: artist.file_name().into_string().unwrap(),
                                                                                            album: album.file_name().into_string().unwrap(),
                                                                                        };
                                                                                        tracks
                                                                                            .push(
                                                                                            track,
                                                                                        );
                                                                                    }
                                                                                    Some("jpg") => {
                                                                                        // println!(
                                                                                        //     "JPG U BASTERD"
                                                                                        // );
                                                                                    }
                                                                                    Some("png") => {
                                                                                        // println!(
                                                                                        //     "PNG U BASTERD"
                                                                                        // );
                                                                                    }
                                                                                    Some(&_) => {}
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for track in tracks {
        println!("{:?}", track);
    }
    Ok(())
}
