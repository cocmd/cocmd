use std::collections::HashMap;
use log::error;

use crate::core::models::package_config_model::Automation;
use crate::core::package::Package;
use crate::core::utils::io::{file_read_lines, file_write_lines};
use crate::package_provider::get_provider;
use crate::Settings;

#[derive(Debug, Clone)]
pub struct PackagesManager {
    pub settings: Settings,
    pub packages_file: String,
    pub packages: HashMap<String, Package>,
}

impl PackagesManager {
    pub fn new(settings: Settings) -> Self {
        let packages_file = settings.packages_file.clone();
        let packages = Self::load_packages(&packages_file, &settings);
        Self {
            settings,
            packages_file,
            packages,
        }
    }

    pub fn remove_package(&mut self, package_name: &str) -> Result<(), String> {

        let package_uri = self
            .packages
            .iter()
            .find(|(_uri, pkg)| pkg.name() == package_name)
            .map(|(uri, _pkg)| uri.clone());

        if let Some(uri) = package_uri {
            self.packages.remove(&uri);

            let provider = get_provider(&uri, &self.settings.runtime_dir).map_err(|e| e.to_string())?;
            let package_dir = provider.get_installation_path();

            if let Err(e) = std::fs::remove_dir_all(&package_dir) {
                return Err(format!("Failed to delete package directory: {}", e));
            }

            // Update the packages.txt file
            self.save()
                .map_err(|e| format!("Failed to update packages file: {}", e))
        } else {
            Err(format!("Package '{}' not found.", package_name))
        }


    }



    pub fn add_package(&mut self, package: Package) {
        self.packages.insert(package.uri.clone(), package);
        self.save();
    }

    pub fn save(&self) -> Result<(), String> {
        // Convert the HashMap into a Vec of package URIs
        let package_strings: Vec<String> =
            self.packages.values().map(|s| s.uri.to_string()).collect();

        // Write the updated list of packages back to the packages.txt file
        match file_write_lines(&self.packages_file, &package_strings) {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!("Failed to write to packages file: {}", e);
                Err(format!("Failed to write to packages file: {}", e))
            }
        }
    }

    fn load_packages(packages_file: &str, settings: &Settings) -> HashMap<String, Package> {
        match file_read_lines(packages_file) {
            Ok(lines) => {
                let mut packages = HashMap::new();
                for line in lines {
                    let uri = line.trim().to_string();

                    let provider = get_provider(&uri, &settings.runtime_dir);
                    if let Err(err) = provider {
                        error!("failed to get location for {} - {}", uri, err);
                        continue;
                    }
                    let package =
                        Package::new(uri.clone(), &provider.unwrap().local_path(), settings);

                    packages.insert(package.uri.clone(), package);
                }
                packages
            }
            Err(err) => {
                error!("failed reading {} - {}", packages_file, err);
                HashMap::new()
            }
        }
    }

    pub fn automations(&self) -> HashMap<String, Automation> {
        let mut automations = HashMap::new();
        for (_name, package) in self.packages.iter() {
            for automation in package.automations(&self.settings, Some(true)) {
                let key = format!("{}.{}", package.name(), automation.name);
                automations.insert(key, automation);
            }
        }
        automations
    }

    pub fn get_package(&self, uri: String) -> Option<&Package> {
        let mut id = uri.clone();
        if !self.packages.contains_key(&uri) {
            // look for packages .name() value and compare with uri. if yes, uri should be the package.uri
            let mut found = false;
            for package in self.packages.values() {
                if package.name() == uri {
                    found = true;
                    id = package.uri.clone();
                    break;
                }
            }
            if !found {
                return None;
            }
        }

        let package = &self.packages[&id];
        Some(package)
    }
}
