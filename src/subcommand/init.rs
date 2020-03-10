extern crate dirs;

use super::configuration;

pub enum Error {
    CreateConfigurationError(configuration::CreateError),
}

pub fn call() -> Result<configuration::Configuration, Error> {
    let result = configuration::Configuration::create_default_configration();

    match result {
        Ok(configuration) => Ok(configuration),
        Err(error) => Err(Error::CreateConfigurationError(error)),
    }
}
