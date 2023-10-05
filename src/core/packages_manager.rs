use std::collections::HashMap;

use tracing::error;

use crate::core::models::package_config_model::Automation;
use crate::core::package::Package;
use crate::core::utils::io::{file_read_lines, file_write_lines};
use crate::package_provider::get_provider;
use crate::Settings;

#[derive(Debug)]
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

    pub fn remove_package(&mut self, package: Package) {
        self.packages.remove(package.uri.as_str());
        self.save();
    }

    pub fn add_package(&mut self, package: Package) {
        self.packages.insert(package.uri.clone(), package);
        self.save();
    }

    pub fn save(&self) {
        Self::save_packages(&self.packages_file, &self.packages);
    }

    fn save_packages(packages_file: &str, packages: &HashMap<String, Package>) {
        let package_strings: Vec<String> = packages.values().map(|s| s.uri.to_string()).collect();
        let _ = file_write_lines(packages_file, &package_strings);
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
