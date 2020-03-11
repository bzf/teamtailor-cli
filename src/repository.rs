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
            Ok(repo) => Ok(LocalRepository::new(repo, directory)),
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
    _repository: git2::Repository,
    path: std::path::PathBuf,
}

pub enum LocalRepositoryError {
    NoRepositoryInDirectory(git2::Error),
}

impl LocalRepository {
    pub fn all(configuration: &configuration::Configuration) -> Vec<LocalRepository> {
        match configuration.projects_directory().read_dir() {
            Ok(children) => children
                .filter_map(Result::ok)
                .map(|dir| LocalRepository::new_from_path(&dir.path()))
                .filter_map(Result::ok)
                .collect(),
            Err(_) => vec![],
        }
    }

    pub fn name(&self) -> &str {
        match self.path.file_name() {
            Some(file_name) => file_name.to_str().unwrap(),
            None => "",
        }
    }

    fn new_from_path(path: &std::path::PathBuf) -> Result<LocalRepository, LocalRepositoryError> {
        match git2::Repository::open(&path) {
            Ok(repo) => Ok(LocalRepository::new(repo, path.to_path_buf())),
            Err(e) => Err(LocalRepositoryError::NoRepositoryInDirectory(e)),
        }
    }

    fn new(repository: git2::Repository, path: std::path::PathBuf) -> LocalRepository {
        LocalRepository {
            _repository: repository,
            path,
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
