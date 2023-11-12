/*
 * This file is part of cocmd.
 *
 * Copyright (C) 2023 Moshe Roth
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
use std::path::PathBuf;

use anyhow::Result;

mod util;

pub mod git;
pub mod hub;
pub mod local;

pub const LOCAL_PROVIDER: &str = "local";
pub const GIT_PROVIDER: &str = "git";
pub const COCMDHUB_PROVIDER: &str = "cocmd-hub";

pub trait PackageProvider {
    fn name(&self) -> String;
    fn package(&self) -> String;
    fn local_path(&self) -> PathBuf;
    fn get_installation_path(&self) -> PathBuf {
        self.local_path().clone()
    }
    fn is_exists_locally(&self) -> bool {
        // check for existsance of the local path
        self.local_path().exists()
    }
    fn download(&self) -> Result<PathBuf>;
    // TODO: fn check update available? (probably should be only available in the hub)

    fn is_provider_local(&self) -> bool {
        self.name() == LOCAL_PROVIDER
    }

    fn is_provider_git(&self) -> bool {
        self.name() == GIT_PROVIDER
    }

    fn is_provider_hub(&self) -> bool {
        self.name() == COCMDHUB_PROVIDER
    }
}

pub fn get_provider(
    uri: &String,
    runtime_dir: &Path,
    version: Option<String>,
) -> Result<Box<dyn PackageProvider>> {
    // parse "package" if it's a local path create a LocalPackageProvider
    // if it's a git url create a GitPackageProvider
    // otherwise look for it in the hub and create a HubPackageProvider

    if let Some(local_path) = util::path::extract_local_path(uri) {
        Ok(Box::new(local::LocalPackageProvider::new(uri, &local_path)))
    } else if let Some(github_parts) = util::git::extract_git_url_parts(uri) {
        return Ok(Box::new(git::GitPackageProvider::new(
            uri,
            &github_parts,
            runtime_dir,
        )));
    } else {
        return Ok(Box::new(hub::CocmdHubPackageProvider::new(
            uri,
            runtime_dir,
            version,
        )));
    }
}

// write a unit-test for this function. make sure all cases are covered
// and that the correct provider is returned for each case
#[cfg(test)]
mod tests {
    use temp_testdir::TempDir;

    use super::*;

    #[test]
    fn test_get_provider() {
        let tmp_home_dir = TempDir::default();
        let runtime_dir = tmp_home_dir.join("runtime");

        // create runtime_dir if not exists
        if !runtime_dir.exists() {
            std::fs::create_dir_all(&runtime_dir).unwrap();
        }

        let git_url = "git@github.com:mzsrtgzt2/cocmd.git";
        let git_url2 = "https://github.com/mzsrtgzr2/cocmd";
        let hub_url = "cocmd-hub";
        let local_url = runtime_dir
            .join("no-existing")
            .to_string_lossy()
            .to_string();

        let provider =
            get_provider(&git_url.to_string(), &runtime_dir.to_path_buf(), None).unwrap();
        assert_eq!(provider.name(), GIT_PROVIDER);
        assert!(provider.is_provider_git());
        assert_eq!(
            provider.local_path(),
            runtime_dir.join("mzsrtgzt2.cocmd").to_path_buf()
        );

        let provider =
            get_provider(&git_url2.to_string(), &runtime_dir.to_path_buf(), None).unwrap();
        assert_eq!(provider.name(), GIT_PROVIDER);
        assert!(provider.is_provider_git());
        assert_eq!(
            provider.local_path(),
            runtime_dir.join("mzsrtgzr2.cocmd").to_path_buf()
        );

        let provider =
            get_provider(&hub_url.to_string(), &runtime_dir.to_path_buf(), None).unwrap();
        assert_eq!(provider.name(), COCMDHUB_PROVIDER);
        assert!(provider.is_provider_hub());
        assert_eq!(
            provider.local_path(),
            runtime_dir.join("cocmd-hub").to_path_buf()
        );

        let provider = get_provider(&local_url, &runtime_dir.to_path_buf(), None).unwrap();
        assert_eq!(provider.name(), LOCAL_PROVIDER);
        assert!(provider.is_provider_local());
        assert_eq!(provider.local_path(), Path::new(&local_url).to_path_buf());
    }
}
