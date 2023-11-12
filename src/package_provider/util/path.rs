#![allow(clippy::unnecessary_unwrap)]
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

use crate::core::consts;
use crate::core::models::package_config_model::PackageConfigModel;
use crate::core::utils::io::from_yaml_file;

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
    // write this function that will look for the package in the base_dir
    // looks for {base_dir}/{name}
    // look in cocmd.yaml if exists, in optional field 'version'
    // if version is not specified, just return the path if cocmd.yaml exists

    let path = base_dir.join(name);

    if path.exists() {
        if let Some(version) = version {
            // read cocmd.yaml (consts::SOURCE_CONFIG_FILE) file look for version field
            let config_file_path = path.join(consts::SOURCE_CONFIG_FILE);

            if config_file_path.exists() {
                let config: Result<PackageConfigModel, String> =
                    from_yaml_file(config_file_path.to_str().unwrap()).map_err(|e| e.to_string());

                if let Ok(config) = config {
                    if let Some(config_version) = config.version {
                        if config_version == version {
                            Ok(path)
                        } else {
                            bail!(
                                "unable to find package '{}' version '{}' in '{}'",
                                name,
                                version,
                                base_dir.display()
                            );
                        }
                    } else {
                        bail!(
                            "unable to find package '{}' version '{}' in '{}'",
                            name,
                            version,
                            base_dir.display()
                        );
                    }
                } else {
                    bail!(
                        "unable to find package '{}' version '{}' in '{}'",
                        name,
                        version,
                        base_dir.display()
                    );
                }
            } else {
                bail!(
                    "unable to find package '{}' version '{}' in '{}'",
                    name,
                    version,
                    base_dir.display()
                );
            }
        } else {
            // return ok for any version, we don't care
            Ok(path)
        }
    } else {
        bail!(
            "unable to find package '{}' in '{}'",
            name,
            base_dir.display()
        );
    }
}
