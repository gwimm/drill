extern crate reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub fn get_projects(username: &str, i: i32) -> Result<(), Box<Error>> {
    let resp: Projects = reqwest::get(&format!(
        "https://www.artstation.com/users/{}/projects.json?page={}",
        username,
        i.to_string(),
    ))?
    .json()?;

    resp.data.iter().for_each(|project: &Project| {
        get_assets(project.hash_id.as_str()).expect("> - >");
    });

    if resp.data.len() >= 30 {
        get_projects(username, i + 1)?;
    }

    Ok(())
}

pub fn get_assets(hash_id: &str) -> Result<(), Box<Error>> {
    let project: Projectus = reqwest::get(&format!(
        "https://www.artstation.com/projects/{}.json",
        hash_id
    ))?
    .json()?;

    println!("{}", project.title);

    for asset in project.assets {
        println!("{:?}", asset.image_url);
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Projects {
    data: Vec<Project>,
    total_count: i64,
}

#[derive(Serialize, Deserialize)]
struct Project {
    title: String,
    hash_id: String,
}

#[derive(Serialize, Deserialize)]
struct Projectus {
    title: String,
    assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize)]
struct Asset {
    image_url: String,
}
