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

#[cfg(test)]
pub(crate) mod tests {
    use tempdir::TempDir;

    use super::*;

    pub(crate) fn run_with_temp_dir(action: impl FnOnce(&Path)) {
        let tmp_dir = TempDir::new("cocmd-package").unwrap();
        let tmp_path = tmp_dir.path();

        action(tmp_path);
    }
}
