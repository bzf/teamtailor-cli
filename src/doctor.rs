pub struct Executable {
    name: String,
}

impl Executable {
    fn new(name: &str) -> Executable {
        Executable {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> Option<String> {
        if let Ok(path) = std::env::var("PATH") {
            for p in path.split(":") {
                let p_str = format!("{}/{}", p, self.name);

                if let Ok(_) = std::fs::File::open(&p_str) {
                    return Some(p_str);
                }
            }
        }

        return None;
    }
}

pub fn check_executables() -> Vec<Executable> {
    vec!["volta", "rbenv", "heroku"]
        .iter()
        .map(|name| Executable::new(name))
        .collect()
}
