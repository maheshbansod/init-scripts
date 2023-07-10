use serde::Deserialize;
use std::{
    fs::{self, read_dir, read_to_string, Metadata},
    path::Path,
};

use crate::{
    repo::get_repo,
    result_writer::{add_statement_to_bashrc, statement_exists_in_bashrc},
};

mod repo;
mod result_writer;

#[derive(Deserialize)]
struct Config {
    repo: String,
}

#[derive(Debug)]
struct FileEntry {
    metadata: Metadata,
    path: String,
}

fn main() {
    let config = read_to_string("user-conf-mgr.conf.toml")
        .expect("The config file `user-conf-mgr.conf.toml` doesn't exist.");
    let config: Config =
        toml::from_str(&config).expect("Invalid configuration file. See example config.");

    println!("{}", config.repo);

    let dir = format!("{}/.config/lightuconfig/sh-scripts", env!("HOME"));

    get_repo(&config.repo, Path::new(&dir)).expect("Couldn't clone the repo");

    let files = read_dir(&dir).unwrap_or_else(|_| panic!("Couldn't open directory '{}'", dir));
    let files = files
        .map(|entry| match entry {
            Ok(file) => match file.metadata() {
                Ok(metadata) => Ok(FileEntry {
                    path: fs::canonicalize(file.path()).unwrap().display().to_string(),
                    metadata,
                }),
                Err(err) => Err(format!("{err}")),
            },
            Err(err) => Err(format!("{err}")),
        })
        .filter(|f| match f {
            Ok(f) => {
                if f.metadata.is_file() {
                    return true;
                }
                false
            }
            Err(_) => true,
        });

    let source_statements = files
        .filter_map(|f| f.ok())
        .map(|f| format!("source {}", f.path));

    let mut added_count = 0;
    let mut total_statements = 0;
    for statement in source_statements {
        if !statement_exists_in_bashrc(&statement) {
            if let Ok(_) = add_statement_to_bashrc(&statement) {
                added_count += 1;
            }
        }
        total_statements += 1;
    }
    println!("Total {} source statements.", total_statements);
    println!("{} new added.", added_count);
}
