mod artstation;
mod bandcamp;
mod soundcloud;

pub fn get_url(url: String) {
    if let Ok(uri) = reqwest::Url::parse(url.as_str()) {
        if uri.host_str().unwrap() == "www.artstation.com" {
            artstation::get_projects(
                uri.path_segments().unwrap().last().unwrap(),
                1,
            )
            .expect("oof");
        }

        if uri.host_str().unwrap().contains("artstation.com") {
            artstation::get_projects(
                uri.host_str().unwrap().split('.').nth(0).unwrap(),
                1,
            )
            .expect("oof");
        }

        if uri.host_str().unwrap().contains("bandcamp.com") {
            // if uri.path().len() != 0 {
            //     bandcamp::get_album(url.as_str()).expect("oof");
            // } else {
                bandcamp::get_user(
                    uri.host_str().unwrap().split('.').nth(0).unwrap(),
                )
                .expect("oof");
            // }
        }

        if uri.host_str().unwrap() == "soundcloud.com" {
            if uri.path_segments().unwrap().last().unwrap() == "tracks"
                || uri.path_segments().unwrap().count() == 1
            {
                soundcloud::get_user(url.as_str()).expect("oof");
            } else {
                soundcloud::get_track(url.as_str()).expect("oof");
            }
        }
    }
}
