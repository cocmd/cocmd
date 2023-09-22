/*
 * This file is part of cocmd.
 *
 * Copyright (C) 2019-2021 Moshe Roth
 *
 * cocmd is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * cocmd is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with cocmd.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::path::Path;

use anyhow::{bail, Result};

#[macro_use]
mod logging;
mod provider;
mod util;
use provider::PackageProvider;

pub fn get_provider(source: &String, runtime_dir: &Path) -> Result<Box<dyn PackageProvider>> {
    // parse "source" if it's a local path create a LocalPackageProvider
    // if it's a git url create a GitPackageProvider
    // otherwise look for it in the hub and create a HubPackageProvider

    if let Some(local_path) = util::path::extract_local_path(source) {
        return Ok(Box::new(provider::local::LocalPackageProvider::new(
            source, local_path,
        )));
    } else if let Some(github_parts) = util::git::extract_git_url_parts(source) {
        return Ok(Box::new(provider::git::GitPackageProvider::new(
            source,
            &github_parts,
            runtime_dir,
        )));
    } else {
        return Ok(Box::new(provider::hub::CocmdHubPackageProvider::new(
            source,
            runtime_dir,
        )));
    }
}

// write a unit-test for this function. make sure all cases are covered
// and that the correct provider is returned for each case
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_provider() {
        let runtime_dir = Path::new("/tmp");
        let git_url = "git@github.com:mzsrtgzt2/cocmd.git";
        let git_url2 = "https://github.com/mzsrtgzr2/cocmd";
        let hub_url = "cocmd-hub";
        let local_url = "/tmp/test/no-existing";

        let provider = get_provider(&git_url.to_string(), &runtime_dir).unwrap();
        assert_eq!(provider.name(), "git");
        assert_eq!(
            provider.local_path(),
            Path::new("/tmp/mzsrtgzt2.cocmd").to_path_buf()
        );

        let provider = get_provider(&git_url2.to_string(), &runtime_dir).unwrap();
        assert_eq!(provider.name(), "git");
        assert_eq!(
            provider.local_path(),
            Path::new("/tmp/mzsrtgzr2.cocmd").to_path_buf()
        );

        let provider = get_provider(&hub_url.to_string(), &runtime_dir).unwrap();
        assert_eq!(provider.name(), "cocmd-hub");
        assert_eq!(
            provider.local_path(),
            Path::new("/tmp/cocmd-hub").to_path_buf()
        );

        let provider = get_provider(&local_url.to_string(), &runtime_dir).unwrap();
        assert_eq!(provider.name(), "local");
        assert_eq!(provider.local_path(), Path::new(local_url).to_path_buf());
    }
}
