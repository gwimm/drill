extern crate reqwest;
extern crate serde_json;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::error::Error;

const CLIENT_ID: &str = "a3e059563d7fd3372b49b37f00a00bcf";

pub fn get_track(url: &str) -> Result<(), Box<Error>> {
    let mut uri = Url::parse("https://api.soundcloud.com/resolve")?;
    uri.query_pairs_mut()
        .append_pair("client_id", CLIENT_ID)
        .append_pair("url", url);

    let resp: Track = reqwest::get(uri)?.json()?;

    let dl_url = match resp.download_url {
        Some(download_url) => download_url,
        None => resp.stream_url,
    };

    let mut dl_uri = Url::parse(dl_url.as_str())?;
    dl_uri.query_pairs_mut()
        .append_pair("client_id", CLIENT_ID);

    println!("{}", resp.title);
    println!("{}", dl_uri.as_str());

    Ok(())
}

pub fn get_user(url: &str) -> Result<(), Box<Error>> {
    let mut uri = Url::parse(url)?;
    uri.query_pairs_mut()
        .append_pair("client_id", CLIENT_ID)
        .append_pair("linked_partitioning", "1");

    let resp = reqwest::get(uri)?.text()?;

    println!("{}", resp);

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    title: String,
    user: User,
    stream_url: String,
    download_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
}
