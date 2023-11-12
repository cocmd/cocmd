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

use std::path::{Path, PathBuf};

use anyhow::Result;

use super::PackageProvider;
use super::LOCAL_PROVIDER;

pub struct LocalPackageProvider {
    package: String,
    local_path: PathBuf,
}

impl LocalPackageProvider {
    pub fn new(package: &str, path: &Path) -> Self {
        Self {
            package: package.to_string(),
            local_path: path.to_path_buf(),
        }
    }
}

impl PackageProvider for LocalPackageProvider {
    fn name(&self) -> String {
        LOCAL_PROVIDER.to_string()
    }

    fn local_path(&self) -> PathBuf {
        self.local_path.to_path_buf()
    }

    fn download(&self) -> Result<PathBuf> {
        Ok(self.local_path.to_path_buf())
    }

    fn package(&self) -> String {
        self.package.clone()
    }

    fn get_installation_path(&self) -> PathBuf {
        self.local_path.to_path_buf()
    }
}
