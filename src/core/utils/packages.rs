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

pub fn get_package_name_from_uri(playbook_uri: &str) -> String {
    // playbook uri is the package name + '.' + the playbook name
    // so grab the package name
    playbook_uri.split('.').next().unwrap().to_string()
}
