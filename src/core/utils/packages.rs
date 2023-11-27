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

pub fn get_package_name_from_uri(uri: &str) -> String {
    // playbook uri is the package name + '.' + the playbook name
    // so grab the package name
    // example: get_package_name_from_uri("cocmd.playbook_name")
    // returns "cocmd"
    uri.split('.').next().unwrap().to_string()
}

pub fn get_playbook_name_from_uri(uri: &str) -> Option<String> {
    // playbook uri is the package name + '.' + the playbook name
    // so grab the playbook name
    // example: get_playbook_name_from_uri("cocmd.playbook_name")
    // returns "playbook_name"

    let splits: Vec<&str> = uri.split('.').collect();
    if splits.len() < 2 {
        return None;
    }

    Some(splits[1..].join("."))
}

pub fn extract_package_name_and_version(package: &str) -> (String, Option<String>) {
    // extract_package_name_and_version takes a package name and returns a tuple of the package name and the package version
    // example: extract_package_name_and_version("cocmd@0.1.0")
    // returns ("cocmd", Some("0.1.0"))

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

    #[test]
    fn test_get_package_name_from_uri() {
        let uri = "cocmd.playbook_name";
        let package_name = get_package_name_from_uri(uri);
        assert_eq!(package_name, "cocmd");

        let uri = "cocmd";
        let package_name = get_package_name_from_uri(uri);
        assert_eq!(package_name, "cocmd");
    }

    #[test]
    fn test_get_playbook_name_from_uri() {
        let uri = "cocmd.playbook_name";
        let playbook_name = get_playbook_name_from_uri(uri);
        assert_eq!(playbook_name, Some("playbook_name".to_string()));

        let uri = "cocmd.playbook.name";
        let playbook_name = get_playbook_name_from_uri(uri);
        assert_eq!(playbook_name, Some("playbook.name".to_string()));

        let uri = "cocmd";
        let playbook_name = get_playbook_name_from_uri(uri);
        assert_eq!(playbook_name, None);
    }
}
