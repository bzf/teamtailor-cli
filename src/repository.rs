extern crate git2;

use super::configuration;

use git2::{AutotagOption, Cred, FetchOptions, RemoteCallbacks};

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
    println!("FETCHING CRENDETINALS");
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

    pub fn fetch_origin(&self) -> Result<(), git2::Error> {
        let remote = "origin";

        // Figure out whether it's a named remote or a URL
        println!("Fetching {} for repo", remote);
        let mut cb = RemoteCallbacks::new();
        let mut remote = self
            ._repository
            .find_remote(remote)
            .or_else(|_| self._repository.remote_anonymous(remote))?;

        // This callback gets called for each remote-tracking branch that gets
        // updated. The message we output depends on whether it's a new one or an
        // update.
        cb.update_tips(|refname, a, b| {
            if a.is_zero() {
                println!("[new]     {:20} {}", b, refname);
            } else {
                println!("[updated] {:10}..{:10} {}", a, b, refname);
            }
            true
        });

        // Set the callback for fetching the SSH-crendentials
        cb.credentials(&git_remote_callback);

        // let mut callbacks = RemoteCallbacks::new();
        // callbacks.credentials(&git_remote_callback);

        // Prepare fetch options.
        // let mut fetch_options = git2::FetchOptions::new();
        // fetch_options.remote_callbacks(cb);

        // Download the packfile and index it. This function updates the amount of
        // received data and the indexer stats which lets you inform the user about
        // progress.
        let mut fo = FetchOptions::new();
        fo.remote_callbacks(cb);
        remote.download(&[] as &[&str], Some(&mut fo))?;

        // {
        //     // If there are local objects (we got a thin pack), then tell the user
        //     // how many objects we saved from having to cross the network.
        //     let stats = remote.stats();
        //     if stats.local_objects() > 0 {
        //         println!(
        //             "\rReceived {}/{} objects in {} bytes (used {} local \
        //          objects)",
        //             stats.indexed_objects(),
        //             stats.total_objects(),
        //             stats.received_bytes(),
        //             stats.local_objects()
        //         );
        //     } else {
        //         println!(
        //             "\rReceived {}/{} objects in {} bytes",
        //             stats.indexed_objects(),
        //             stats.total_objects(),
        //             stats.received_bytes()
        //         );
        //     }
        // }

        // Disconnect the underlying connection to prevent from idling.
        remote.disconnect()?;

        // Update the references in the remote's namespace to point to the right
        // commits. This may be needed even if there was no packfile to download,
        // which can happen e.g. when the branches have been changed but all the
        // needed objects are available locally.
        remote.update_tips(None, true, AutotagOption::Unspecified, None)?;

        // let refs_remote = remote.list()?;
        // let latest_remote_commit = &refs_remote[0];
        // println!("{:?}", latest_remote_commit.oid());

        // let

        // let mut checkout_builder = git2::build::CheckoutBuilder::new();
        // checkout_builder.force();
        // let treeish = self._repository.revparse_single("master")?;
        // println!("{:?}", treeish);
        // self._repository
        //     .checkout_tree(&treeish, Some(&mut checkout_builder))?;

        // remote.fetch(&["refs/heads/*:refs/heads/*"], Some(&mut fo), None)?;
        // remote.fetch

        // remote.fetch(&["refs/heads/*:refs/heads/*"], Some(&mut fo), None)?;
        // let head = self._repository.head()?.target().unwrap();
        // let obj = self._repository.find_object(head, None)?;
        // self._repository.reset(&obj, git2::ResetType::Hard, None)?;
        // // repo.reset(&obj, git2::ResetType::Hard,

        // Ok, we need to update, so fetch and reset --hard
        // remote.fetch(&["refs/heads/*:refs/heads/*"], Some(&mut fo), None)?;
        let head = self._repository.head()?.peel_to_commit()?;
        // // let obj = self
        // //     ._repository
        // //     .find_object(latest_remote_commit.oid(), None)?;

        let remote_master_branch = self
            ._repository
            .find_branch("origin/master", git2::BranchType::Remote)?;

        let latest_remote_commit = remote_master_branch.get().peel_to_commit()?;

        let annotated_commit = self
            ._repository
            .reference_to_annotated_commit(remote_master_branch.get())?;

        self._repository
            .merge_commits(&head, &latest_remote_commit, None)?;
        // self._repository.cleanup_state()?;

        // if let Some(oid) = latest_remote_commit {
        //     // println!("{:?}", oid);
        //     // let obj = self._repository.find_object(oid, None)?;
        //     // self._repository
        //     //     .merge(remote_master_branch, git2::ResetType::Soft, None);
        //     // self._repository.reset(&obj, git2::ResetType::Hard, None)?;
        // }

        // println!("{:?}", remote.

        // self._repository.set_head("refs/origin/master")?;

        // self._repository.checkout_head(Some(head))?;
        // &head);
        // self._repository.checkout_tree(&head, None);
        // self._repository
        //     .set_head_detached(latest_remote_commit.oid())?;
        // self._repository.checkout_index(Some(head), None)?;
        // self._repository
        //     .checkout_tree(latest_remote_commit.oid(), None);

        Ok(())
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
