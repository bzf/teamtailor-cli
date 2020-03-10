use serde::{Deserialize, Serialize};
use std::io::Write;

pub fn directory() -> std::path::PathBuf {
    let name = std::path::Path::new("teamtailor/");
    dirs::config_dir().unwrap().join(name)
}

pub fn path() -> std::path::PathBuf {
    directory().join("config.yml")
}

pub fn default() -> Configuration {
    let home_dir = dirs::home_dir().unwrap();

    return Configuration {
        root_directory: format!("{}/src/teamtailor", home_dir.to_str().unwrap()),
    };
}

pub enum CreateError {
    CouldNotCreateConfigurationDirectory(std::io::Error),
    ConfigurationAlreadyExists,
    CouldNotCreateFile(std::io::Error),
    CouldNotSerializeConfiguration(serde_yaml::Error),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Configuration {
    root_directory: String,
}

impl Configuration {
    pub fn filepath(&self) -> String {
        String::from(path().to_str().unwrap())
    }

    pub fn create_default_configration() -> Result<Configuration, CreateError> {
        let configuration_directory = directory();
        let configuration_path = path();

        if !configuration_directory.exists() {
            let builder = std::fs::DirBuilder::new();
            if let Err(e) = builder.create(&configuration_directory) {
                return Err(CreateError::CouldNotCreateConfigurationDirectory(e));
            }
        }

        if configuration_path.exists() {
            return Err(CreateError::ConfigurationAlreadyExists);
        }

        let default_configuration = default();

        match std::fs::File::create(&configuration_path) {
            Ok(mut file) => match serde_yaml::to_string(&default_configuration) {
                Ok(yml) => match file.write_all(yml.as_bytes()) {
                    Ok(_) => Ok(default_configuration),
                    Err(e) => Err(CreateError::CouldNotCreateFile(e)),
                },
                Err(e) => Err(CreateError::CouldNotSerializeConfiguration(e)),
            },
            Err(e) => Err(CreateError::CouldNotCreateFile(e)),
        }
    }
}
