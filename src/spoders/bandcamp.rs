use regex::Regex;
use serde::{Deserialize, Serialize};

use select::document::Document;
use select::predicate::Name;

use std::error::Error;

pub fn get_album(username: &str, albumname: &str) -> Result<(), Box<Error>> {
    let body = reqwest::get(
        format!("https://{}.bandcamp.com/album/{}", username, albumname)
            .as_str(),
    )?
    .text()?;

    if let Some(trackinfo) =
        &Regex::new(r"(?m)trackinfo: (.*?),$")?.captures(body.as_str())
    {
        let tracks: Vec<TrackInfo> = serde_json::from_str(&trackinfo[1])?;

        let artist = &Regex::new("(?m)artist: \"(.*?)\",\n")?
            .captures(body.as_str())
            .expect("artist")[1];

        let album = &Regex::new("(?m)\"title\":\"(.*?)\",")?
            .captures(body.as_str())
            .expect("album")[1];

        println!("{} - {}", artist, album);

        tracks.iter().enumerate().for_each(|(i, track)| {
            println!("{}", track.title);

            download(
                track.file.url.as_str(),
                format!("{}/{}/{:02}.{}.mp3", artist, album, i, track.title).as_str(),
            );
        })
    }

    Ok(())
}

fn download(url: &str, dst: &str) -> Result<(), Box<Error>> {
    use std::fs;
    use std::fs::File;
    use std::path::Path;
    use std::io::copy;

    fs::create_dir_all(Path::new(dst).parent().unwrap())?;
    copy(&mut reqwest::get(url)?, &mut File::create(dst)?)?;

    Ok(())
}

pub fn get_user(username: &str) -> Result<(), Box<Error>> {
    let body =
        reqwest::get(format!("https://{}.bandcamp.com/", username).as_str())?;

    Document::from_read(body)?
        .find(Name("a"))
        .filter_map(|n| {
            if let Some(yes) = n.attr("href") {
                if yes.contains("/album/") {
                    Some(yes)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .for_each(|path| {
            if let Some(albumname) = path.split("/").last() {
                get_album(username, albumname).unwrap();
            }
        });

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackInfo {
    title: String,
    file: File,

    // has_free_download: Option<serde_json::Value>,
    // free_album_download: bool,
    // is_downloadable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    #[serde(rename = "mp3-128")]
    url: String,
}
