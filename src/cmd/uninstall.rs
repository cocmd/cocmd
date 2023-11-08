use std::path::{Path, PathBuf};
use anyhow::{anyhow, Result};
use log::{info, error};

use crate::core::packages_manager::PackagesManager;

pub fn uninstall_package(packages_manager: &mut PackagesManager, package_name: &str) -> Result<()> {
    // Construct the path to the package directory
    let package_dir = PathBuf::from(&packages_manager.settings.runtime_dir).join(package_name);

    // Check if the package directory exists
    if !package_dir.exists() {
        error!("ERROR: no '{}' folder to uninstall", package_name);
        return Ok(());
    }

    // Check if the package exists in the package manager
    if packages_manager.get_package(package_name.to_string()).is_none() {
        error!("Package '{}' is not installed.", package_name);
        return Ok(());
    }

    info!("Uninstalling package {:?}", package_name);

    // Remove the package
    match packages_manager.remove_package(package_name) {
        Ok(_) => {
            info!("Package '{}' was successfully uninstalled.", package_name);
            Ok(())
        }
        Err(e) => Err(anyhow!(e)),
    }
}
