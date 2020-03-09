pub fn path() -> String {
    return format!(
        "{}/teamtailor/config.yml",
        std::string::String::from(dirs::config_dir().unwrap().to_str().unwrap())
    );
}

pub fn default() -> Configuration {
    let home_dir = dirs::home_dir().unwrap();

    return Configuration {
        root_directory: format!("{}/src/teamtailor", home_dir.to_str().unwrap()),
    };
}

pub struct Configuration {
    root_directory: String,
}
