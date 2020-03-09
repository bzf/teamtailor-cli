extern crate dirs;

use super::configuration;

pub enum Error {
    ConfigurationAlreadyExists,
}

pub fn call() -> Option<Error> {
    let configuration_path = configuration::path();
    let configuration_file = std::path::Path::new(&configuration_path);

    if configuration_file.exists() {
        return Some(Error::ConfigurationAlreadyExists);
    }

    let _configuration = configuration::default();
    println!("configuration file created at {}", configuration_path);

    return None;
}
