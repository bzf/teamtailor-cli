extern crate clap;

use clap::App;

mod configuration;
mod error;
mod subcommand;

fn main() {
    let init = App::new("init").about("Initialize a new configuration file");

    let matches = App::new("teamtailor-cli")
        .version("v0.1-beta")
        .about("Helps out with your development environment")
        .subcommand(init)
        .get_matches();

    let _exit_code: Option<error::Error> = match matches.subcommand() {
        ("init", Some(_init_matches)) => match subcommand::init::call() {
            Some(subcommand::init::Error::ConfigurationAlreadyExists) => {
                eprintln!(
                    "fatal: configuration file already exists: {}",
                    configuration::path().to_str().unwrap()
                );
                std::process::exit(1);
            }
            Some(subcommand::init::Error::CouldNotCreateFile) => {
                eprintln!("fatal: could not create the configuration file");
                std::process::exit(1);
            }
            Some(subcommand::init::Error::CouldNotCreateConfigurationDirectory) => {
                eprintln!("fatal: could not create the configuration directory");
                std::process::exit(1);
            }
            None => {
                println!("success: created configuration file");
                None
            }
        },
        _ => None,
    };
}
