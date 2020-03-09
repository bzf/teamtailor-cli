extern crate clap;
use clap::App;

fn main() {
    App::new("teamtailor-cli")
        .version("v0.1-beta")
        .get_matches();
}
