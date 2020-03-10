extern crate git2;

use super::configuration;

#[derive(Clone)]
pub struct RemoteRepository {
    url: String,
}

pub enum CloneError {
    AlreadyCloned(RemoteRepository),
    FailedToClone(RemoteRepository),
}

impl RemoteRepository {
    pub fn all() -> Vec<RemoteRepository> {
        vec![
            RemoteRepository::new("https://github.com/Teamtailor/insights"),
            RemoteRepository::new("https://github.com/Teamtailor/teamtailor"),
            RemoteRepository::new("https://github.com/Teamtailor/always-on"),
            RemoteRepository::new("https://github.com/Teamtailor/ttmobile"),
            RemoteRepository::new("https://github.com/Teamtailor/adapters"),
            RemoteRepository::new("https://github.com/Teamtailor/marketingsite"),
            RemoteRepository::new("https://github.com/Teamtailor/tt-ml"),
            RemoteRepository::new("https://github.com/Teamtailor/tt-partner-docs"),
            RemoteRepository::new("https://github.com/Teamtailor/docs"),
            RemoteRepository::new("https://github.com/Teamtailor/sourcing"),
            RemoteRepository::new("https://github.com/Teamtailor/tt-yearly-review"),
            RemoteRepository::new("https://github.com/Teamtailor/screenshots"),
        ]
    }

    pub fn name(&self) -> &str {
        self.url.split('/').last().unwrap_or("")
    }

    pub fn clone_repostiory(
        &self,
        configuration: &configuration::Configuration,
    ) -> Result<LocalRepository, CloneError> {
        let directory: std::path::PathBuf = self.get_local_directory(&configuration);

        if directory.exists() {
            return Err(CloneError::AlreadyCloned(self.clone()));
        }

        match git2::Repository::clone(&self.url, directory) {
            Ok(repo) => Ok(LocalRepository::new(&self.url, repo)),
            Err(_) => Err(CloneError::FailedToClone(self.clone())),
        }
    }

    fn new(url: &str) -> RemoteRepository {
        RemoteRepository {
            url: String::from(url),
        }
    }

    fn get_local_directory(
        &self,
        configuration: &configuration::Configuration,
    ) -> std::path::PathBuf {
        std::path::Path::new(&configuration.projects_directory()).join(self.name())
    }
}

pub struct LocalRepository {
    _url: String,
    _repository: git2::Repository,
}

impl LocalRepository {
    fn new(url: &str, repository: git2::Repository) -> LocalRepository {
        LocalRepository {
            _url: String::from(url),
            _repository: repository,
        }
    }
}

mod tests {
    #[test]
    fn test_repository_name() {
        let repository = RemoteRepository::new("https//github.com/Teamtailor/favicons");
        assert_eq!(repository.name(), "favicons");

        let repository = RemoteRepository::new("https//github.com/rails/rails");
        assert_eq!(repository.name(), "rails");
    }
}
