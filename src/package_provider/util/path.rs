#![allow(clippy::unnecessary_unwrap)]
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

pub fn extract_local_path(package: &String) -> Option<PathBuf> {
    // find out if package is a local path, even if it doesn't exist
    // if it is, return Some(path)
    // otherwise return None
    let path = Path::new(package);

    // check for local file system pattern
    if path.is_absolute() {
        return Some(path.to_path_buf());
    }

    if path.is_relative() {
        // return the absolute path
        // fetch the current working directory and join with path
        let cwd = std::env::current_dir().unwrap();
        let abs_path = cwd.join(path);

        // Check if the path exists
        if abs_path.exists() {
            // Canonicalize the path to get the absolute path
            if let Ok(canonical_path) = fs::canonicalize(abs_path) {
                // Convert the result to a PathBuf
                let result_path: PathBuf = canonical_path;
                // println!("{:?}", result_path);
                return Some(result_path);
            }
        }
    }

    None
}

pub fn resolve_hub_package_locally(
    base_dir: &Path,
    name: &str,
    version: Option<&str>,
) -> Result<PathBuf> {
    // write this function that will look for the package in the runtime_dir
    // just list the directories that match name-<version>. and match
    // the version if it's provided. if not, pick up the latest one
    // and return the path to the directory

    let mut dirs = std::fs::read_dir(base_dir)
        .context("unable to read runtime directory")?
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            if file_name == name || file_name.starts_with(&format!("{}-", name)) {
                                return Some(entry.path());
                            }
                        }
                    }
                }
            }

            None
        })
        .collect::<Vec<PathBuf>>();

    if dirs.is_empty() {
        bail!("unable to find package '{}' in the runtime directory", name);
    }

    if version.is_none() {
        dirs.sort();

        let latest = dirs.pop().unwrap();

        Ok(latest)
    } else {
        let version = version.unwrap();

        let mut dirs = dirs
            .iter()
            .filter(|path| {
                if let Some(path) = path.to_str() {
                    if path.ends_with(version) {
                        return true;
                    }
                }

                false
            })
            .collect::<Vec<&PathBuf>>();

        if dirs.is_empty() {
            bail!(
                "unable to find package '{}' version '{}' in the runtime directory",
                name,
                version
            );
        }

        let latest = dirs.pop().unwrap();

        Ok(latest.to_path_buf())
    }
}
