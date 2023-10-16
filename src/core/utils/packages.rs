use crate::core::packages_manager::PackagesManager;

pub fn get_all_paths(packages_manager: &PackagesManager) -> Vec<String> {
    packages_manager
        .packages
        .values()
        .map(|p| {
            if !(p.is_legit_cocmd_package()) {
                vec![]
            } else {
                p.paths(true)
            }
        })
        .flatten()
        .collect()
}
