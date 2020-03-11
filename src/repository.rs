extern crate git2;

use super::configuration;

use git2::{Cred, RemoteCallbacks};

#[derive(Clone)]
pub struct RemoteRepository {
    name: String,
}

pub enum CloneError {
    AlreadyCloned(RemoteRepository),
    FailedToClone(RemoteRepository, git2::Error),
}

fn git_remote_callback(
    _user: &str,
    user_from_url: Option<&str>,
    _cred: git2::CredentialType,
) -> Result<git2::Cred, git2::Error> {
    let user = user_from_url.unwrap_or("git");
    Cred::ssh_key_from_agent(user)
}

impl RemoteRepository {
    pub fn all() -> Vec<RemoteRepository> {
        vec![
            RemoteRepository::new("Teamtailor/insights"),
            RemoteRepository::new("Teamtailor/teamtailor"),
            RemoteRepository::new("Teamtailor/always-on"),
            RemoteRepository::new("Teamtailor/ttmobile"),
            RemoteRepository::new("Teamtailor/adapters"),
            RemoteRepository::new("Teamtailor/marketingsite"),
            RemoteRepository::new("Teamtailor/tt-ml"),
            RemoteRepository::new("Teamtailor/tt-partner-docs"),
            RemoteRepository::new("Teamtailor/docs"),
            RemoteRepository::new("Teamtailor/sourcing"),
            RemoteRepository::new("Teamtailor/tt-yearly-review"),
            RemoteRepository::new("Teamtailor/screenshots"),
        ]
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn clone_repostiory(
        &self,
        configuration: &configuration::Configuration,
    ) -> Result<LocalRepository, CloneError> {
        let directory: std::path::PathBuf = self.get_local_directory(&configuration);

        if directory.exists() {
            return Err(CloneError::AlreadyCloned(self.clone()));
        }

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(&git_remote_callback);

        // Prepare fetch options.
        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        // Prepare builder.
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        // Clone the project.
        let result = builder.clone(&self.url(), &directory);

        match result {
            Ok(repo) => Ok(LocalRepository::new(&self.name, repo)),
            Err(e) => Err(CloneError::FailedToClone(self.clone(), e)),
        }
    }

    pub fn url(&self) -> String {
        format!("git@github.com:{}.git", &self.name)
    }

    fn new(name: &str) -> RemoteRepository {
        RemoteRepository {
            name: String::from(name),
        }
    }

    fn get_local_directory(
        &self,
        configuration: &configuration::Configuration,
    ) -> std::path::PathBuf {
        let directory_name = self.name().split('/').last().unwrap();

        std::path::Path::new(&configuration.projects_directory()).join(directory_name)
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
    use super::*;

    #[test]
    fn test_repository_name() {
        let repository = RemoteRepository::new("Teamtailor/favicons");
        assert_eq!(repository.name(), "Teamtailor/favicons");

        let repository = RemoteRepository::new("rails/rails");
        assert_eq!(repository.name(), "rails/rails");
    }
}
