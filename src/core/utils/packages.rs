use crate::core::packages_manager::PackagesManager;

pub fn get_all_paths(packages_manager: &PackagesManager) -> Vec<String> {
    packages_manager
        .packages
        .values()
        .flat_map(|p| {
            if !(p.is_legit_cocmd_package()) {
                vec![]
            } else {
                p.paths(true)
            }
        })
        .collect()
}

pub fn extract_package_name_and_version(package: &str) -> (String, Option<String>) {
    let mut package_name = package.to_string();
    let mut package_version = None;

    if let Some(version_index) = package_name.find('@') {
        package_version = Some(package_name.split_off(version_index + 1));
        package_name.pop();
    }

    (package_name, package_version)
}

// write a test for extract_package_name_and_version
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_extract_package_name_and_version() {
        let package = "cocmd@0.1.0";
        let (package_name, package_version) = extract_package_name_and_version(package);
        assert_eq!(package_name, "cocmd");
        assert_eq!(package_version, Some("0.1.0".to_string()));

        let package = "cocmd";
        let (package_name, package_version) = extract_package_name_and_version(package);
        assert_eq!(package_name, "cocmd");
        assert_eq!(package_version, None);
    }
}
