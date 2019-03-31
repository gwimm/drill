extern crate reqwest;

extern crate clap;
use clap::{Arg, App};

mod spoders;

fn main() {
    let matches = App::new("drill")
                          .version("1.0")
                          .author("me")
                          .about("dab")
                          .arg(Arg::with_name("urls")
                               .short("u")
                               .long("url")
                               .multiple(true)
                               .takes_value(true)
                               .help("set url"))
                          .get_matches();

    if let Some(url) = matches.value_of("urls") {
        spoders::get_url(String::from(url));
    }
}
