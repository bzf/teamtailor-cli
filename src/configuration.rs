use serde::{Serialize, Deserialize};

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

#[derive(Deserialize, Serialize, Debug)]
pub struct Configuration {
    root_directory: String,
}
