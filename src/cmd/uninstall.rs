use std::path::PathBuf;
use anyhow::{anyhow, Result};
use log::{info, error};

use crate::core::packages_manager::PackagesManager;

pub fn uninstall_package(packages_manager: &mut PackagesManager, package_name: &str) -> Result<()> {
    // Retrieve the package
    let package = match packages_manager.get_package(package_name.to_string()) {
        Some(pkg) => pkg,
        None => {
            error!("Package '{}' is not installed.", package_name);
            return Ok(());
        }
    };

    // The provider should be determined by the actual installation path of the package, not just the name
    let installation_path = package.location(); // Assuming 'location' gives the installation path

    // Check if the installation path is within the runtime directory
    let runtime_dir = PathBuf::from(&packages_manager.settings.runtime_dir);
    if !installation_path.starts_with(&runtime_dir) {
        error!("Package '{}' is not in the runtime directory and will not be uninstalled.", package_name);
        return Ok(());
    }

    // If the package is in the runtime directory, proceed with uninstallation
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
