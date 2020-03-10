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
        ("init", _) => run_init_command(),
        _ => std::process::exit(1),
    }
}

fn run_init_command() -> () {
    match subcommand::init::call() {
        Ok(configuration) => {
            println!(
                "success: created configuration file ({})",
                configuration.filepath()
            );
        }
        Err(subcommand::init::Error::CreateConfigurationError(error)) => match error {
            configuration::CreateError::ConfigurationAlreadyExists => {
                eprintln!(
                    "fatal: configuration file already exists: {}",
                    configuration::path().to_str().unwrap()
                );
                std::process::exit(1);
            }
            configuration::CreateError::CouldNotSerializeConfiguration(serde_error) => {
                eprintln!(
                    "fatal: could not create the configuration file ({})",
                    serde_error
                );
                std::process::exit(1);
            }
            configuration::CreateError::CouldNotCreateFile(io_error) => {
                eprintln!(
                    "fatal: could not create the configuration file ({})",
                    io_error
                );
                std::process::exit(1);
            }
            configuration::CreateError::CouldNotCreateConfigurationDirectory(io_error) => {
                eprintln!(
                    "fatal: could not create the configuration directory ({})",
                    io_error
                );
                std::process::exit(1);
            }
        },
    }
}
