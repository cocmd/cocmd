/*
 * This file is part of cocmd.
 *
 * Copyright (C) 2019-2021 Moshe Roth
 *
 * cocmd is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either VERSION 3 of the License, or
 * (at your option) any later VERSION.
 *
 * cocmd is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with cocmd.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use log::info;

use super::PackageProvider;
use super::{
    util::download::download_and_extract_zip_verify_sha256, util::download::read_string_from_url,
    COCMDHUB_PROVIDER,
};

pub const COCMD_HUB_PACKAGE_INDEX_URL: &str =
    "https://github.com/cocmd/hub/releases/latest/download/package_index.json";
// "http://www.google.com";

const PACKAGE_INDEX_CACHE_FILE: &str = "package_index_cache.json";
const PACKAGE_INDEX_CACHE_INVALIDATION_SECONDS: u64 = 60 * 60;

const VERSION: &str = "0.0.0";

pub struct CocmdHubPackageProvider {
    package: String,
    local_path: PathBuf,
    runtime_dir: PathBuf,
}

impl PackageProvider for CocmdHubPackageProvider {
    fn name(&self) -> String {
        COCMDHUB_PROVIDER.to_string()
    }

    fn local_path(&self) -> PathBuf {
        self.local_path.to_path_buf()
    }

    fn download(&self) -> Result<PathBuf> {
        let index = self.get_index(true)?;
        // .context("unable to get package index from cocmd hub")?;

        let package_info = index
            .get_package(&self.package, Some(VERSION))
            .ok_or_else(|| {
                anyhow!(
                    "unable to find package '{}@{}' in the cocmd hub",
                    &self.package,
                    &VERSION
                )
            })?;

        let archive_sha256 = read_string_from_url(&package_info.archive_sha256_url)
            .context("unable to read archive sha256 signature")?;

        let local_path = self.local_path.clone(); // Clone the PathBuf

        download_and_extract_zip_verify_sha256(
            &package_info.archive_url,
            &local_path,
            Some(&archive_sha256),
        )?;

        Ok(local_path)
    }

    fn package(&self) -> String {
        self.package.clone()
    }
}

impl CocmdHubPackageProvider {
    pub fn new(package: &String, runtime_dir: &Path) -> Self {
        let binding = runtime_dir.join(package);
        let local_path = binding.as_path();
        Self {
            package: (*package.clone()).to_string(),
            local_path: local_path.to_path_buf(),
            runtime_dir: runtime_dir.to_path_buf(),
        }
    }

    pub fn get_index(&self, force_update: bool) -> Result<PackageIndex> {
        let old_index = self.get_index_from_cache()?;

        if let Some(old_index) = old_index {
            if !force_update {
                let current_time = std::time::SystemTime::now().duration_since(UNIX_EPOCH)?;
                let current_unix = current_time.as_secs();
                if old_index.cached_at >= (current_unix - PACKAGE_INDEX_CACHE_INVALIDATION_SECONDS)
                {
                    info!("using cached package index");
                    return Ok(old_index.index);
                }
            }
        }

        let new_index = CocmdHubPackageProvider::download_index()?;
        self.save_index_to_cache(new_index.clone())?;
        Ok(new_index)
    }

    fn download_index() -> Result<PackageIndex> {
        info!("fetching from hub...");
        let json_body = read_string_from_url(COCMD_HUB_PACKAGE_INDEX_URL)?;
        let index: PackageIndex = serde_json::from_str(&json_body)?;
        Ok(index)
    }

    fn get_index_from_cache(&self) -> Result<Option<CachedPackageIndex>> {
        let target_file = self.runtime_dir.join(PACKAGE_INDEX_CACHE_FILE);
        if !target_file.is_file() {
            return Ok(None);
        }

        let content =
            std::fs::read_to_string(&target_file).context("unable to read package index cache")?;
        let index: CachedPackageIndex = serde_json::from_str(&content)?;
        Ok(Some(index))
    }

    fn save_index_to_cache(&self, index: PackageIndex) -> Result<()> {
        let target_file = self.runtime_dir.join(PACKAGE_INDEX_CACHE_FILE);
        let current_time = std::time::SystemTime::now().duration_since(UNIX_EPOCH)?;
        let current_unix = current_time.as_secs();
        let cached_index = CachedPackageIndex {
            cached_at: current_unix,
            index,
        };
        let serialized = serde_json::to_string(&cached_index)?;
        std::fs::write(target_file, serialized)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedPackageIndex {
    cached_at: u64,
    index: PackageIndex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageIndex {
    pub last_update: u64,
    pub packages: Vec<PackageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub version: String,

    pub archive_url: String,
    pub archive_sha256_url: String,
}

impl PackageIndex {
    fn get_package(&self, name: &str, version: Option<&str>) -> Option<PackageInfo> {
        let mut matching_packages: Vec<PackageInfo> = self
            .packages
            .iter()
            .filter(|package| package.name == name)
            .cloned()
            .collect();

        matching_packages.sort_by(|a, b| natord::compare(&a.version, &b.version));

        if let Some(explicit_version) = version {
            matching_packages
                .into_iter()
                .find(|package| package.version == explicit_version)
        } else {
            matching_packages.into_iter().last()
        }
    }
}
