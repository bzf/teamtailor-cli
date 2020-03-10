extern crate clap;

use clap::App;

mod configuration;
mod subcommand;

fn main() {
    let init = App::new("init").about("Initialize a new configuration file");

    let matches = App::new("teamtailor-cli")
        .version("v0.1-beta")
        .about("Helps out with your development environment")
        .subcommand(init)
        .get_matches();

    match matches.subcommand() {
        ("init", _) => match subcommand::init::call() {
            Ok(configuration) => {
                println!(
                    "success: created configuration file ({})",
                    configuration.filepath()
                );
            }
            Err(subcommand::init::Error::ConfigurationError(error)) => match error {
                configuration::Error::ConfigurationAlreadyExists => {
                    eprintln!(
                        "fatal: configuration file already exists: {}",
                        configuration::path().to_str().unwrap()
                    );
                    std::process::exit(1);
                }
                configuration::Error::CouldNotSerializeConfiguration(serde_error) => {
                    eprintln!(
                        "fatal: could not create the configuration file ({})",
                        serde_error
                    );
                    std::process::exit(1);
                }
                configuration::Error::CouldNotCreateFile(io_error) => {
                    eprintln!(
                        "fatal: could not create the configuration file ({})",
                        io_error
                    );
                    std::process::exit(1);
                }
                configuration::Error::CouldNotCreateConfigurationDirectory(io_error) => {
                    eprintln!(
                        "fatal: could not create the configuration directory ({})",
                        io_error
                    );
                    std::process::exit(1);
                }
            },
        },
        _ => std::process::exit(1),
    }
}
