use std::{env, path::Path};

use git2::{Cred, RemoteCallbacks, Repository};

pub fn get_repo(repo: &str, dir: &Path) -> Result<git2::Repository, git2::Error> {
    if let Ok(repo) = Repository::open(dir) {
        // todo git pull ?
        return Ok(repo);
    }
    // Prepare callbacks.
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap())),
            None,
        )
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    builder.clone(repo, dir)
}
