use anyhow::{anyhow, Result};
use log::info;

use crate::core::packages_manager::PackagesManager;

pub fn uninstall_package(packages_manager: &mut PackagesManager, package_name: &str) -> Result<()> {
    info!("Uninstalling package {:?}", package_name);

    // Check if the package exists
    if packages_manager
        .get_package(package_name.to_string())
        .is_none()
    {
        info!("Package '{}' is not installed.", package_name);
        return Ok(());
    }

    // Remove the package
    match packages_manager.remove_package(package_name) {
        Ok(_) => {
            info!("Package '{}' was successfully uninstalled.", package_name);
            Ok(())
        }
        Err(e) => Err(anyhow!(e)),
    }
}
