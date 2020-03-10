extern crate dirs;

use super::configuration;
use std::io::Write;

pub enum Error {
    CouldNotCreateFile,
    CouldNotCreateConfigurationDirectory,
    ConfigurationAlreadyExists,
}

pub fn call() -> Option<Error> {
    let configuration_path = configuration::path();
    let configuration_file = std::path::Path::new(&configuration_path);

    if !configuration::directory().exists() {
        let builder = std::fs::DirBuilder::new();
        if let Err(_) = builder.create(&configuration::directory()) {
            return Some(Error::CouldNotCreateConfigurationDirectory);
        }
    }

    if configuration_file.exists() {
        return Some(Error::ConfigurationAlreadyExists);
    }

    match std::fs::File::create(&configuration::path()) {
        Ok(mut file) => {
            let default_configuration = configuration::default();
            match serde_yaml::to_string(&default_configuration) {
                Ok(yml) => match file.write_all(yml.as_bytes()) {
                    Ok(_) => None,
                    Err(_) => Some(Error::CouldNotCreateFile),
                },
                _ => Some(Error::CouldNotCreateFile),
            }
        }
        Err(_) => Some(Error::CouldNotCreateFile),
    }
}
