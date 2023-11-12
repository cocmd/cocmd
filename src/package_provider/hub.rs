/*
 * This file is part of cocmd.
 *
 * Copyright (C) 2023 Moshe Roth
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
#[allow(unused_imports)]
use std::{
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use anyhow::{anyhow, Context, Result};
use log::info;
use serde::{Deserialize, Serialize};

use super::util::path::resolve_hub_package_locally;
use super::PackageProvider;
use super::{
    util::download::download_and_extract_zip_verify_sha256, util::download::read_string_from_url,
    COCMDHUB_PROVIDER,
};
use crate::core::{
    consts, models::package_config_model::PackageConfigModel, utils::io::from_yaml_file,
};

pub const COCMD_HUB_PACKAGE_INDEX_URL: &str =
    "https://github.com/cocmd/hub/releases/latest/download/package_index.json";

const PACKAGE_INDEX_CACHE_FILE: &str = "package_index_cache.json";
const PACKAGE_INDEX_CACHE_INVALIDATION_SECONDS: u64 = 60 * 60;

pub struct CocmdHubPackageProvider {
    package: String,
    version: String,
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

    fn is_exists_locally(&self) -> bool {
        // check for existsance of the local path
        // read cocmd.yaml (consts::SOURCE_CONFIG_FILE) file look for version field
        let config_file_path = self.local_path().join(consts::SOURCE_CONFIG_FILE);

        // if version is specified, check if the version in cocmd.yaml matches
        if config_file_path.exists() {
            let config: PackageConfigModel = from_yaml_file(&config_file_path).unwrap();
            if config.version == Some(self.version.clone()) {
                return true;
            }
        }
        false
    }

    fn download(&self) -> Result<PathBuf> {
        let index = Self::get_index(&self.runtime_dir, false)?;
        // .context("unable to get package index from cocmd hub")?;

        let package_info = index
            .get_package(&self.package, &Some(self.version.clone()))
            .ok_or_else(|| {
                anyhow!(
                    "unable to find package '{}@{}' in the cocmd hub",
                    &self.package,
                    &self.version
                )
            })?;

        let archive_sha256 = read_string_from_url(&package_info.archive_sha256_url)
            .context("unable to read archive sha256 signature")?;

        // remove self.local_path if exists
        if self.local_path.exists() {
            std::fs::remove_dir_all(&self.local_path)?;
        }

        download_and_extract_zip_verify_sha256(
            &package_info.archive_url,
            &self.local_path,
            Some(&archive_sha256),
        )?;

        Ok(self.local_path.clone())
    }

    fn package(&self) -> String {
        self.package.clone()
    }
}

impl CocmdHubPackageProvider {
    pub fn new(package: &String, runtime_dir: &Path, version: Option<String>) -> Self {
        let binding = runtime_dir.join(package);

        let default_path = binding.as_path();

        // if version not provided use the latest version you find in index
        // if version is provided, use it
        let mut version = version.clone();
        if version.is_none() {
            let index = Self::get_index(runtime_dir, false).unwrap();
            version = index
                .get_package(package, &None)
                .map(|package| package.version)
                .unwrap_or_else(|| "0.0.0".to_string())
                .into();
        }

        let res = resolve_hub_package_locally(runtime_dir, package.as_str(), version.as_deref());

        Self {
            package: (*package.clone()).to_string(),
            local_path: res.unwrap_or_else(|_| default_path.to_path_buf()),
            runtime_dir: runtime_dir.to_path_buf(),
            version: version.unwrap(),
        }
    }

    pub fn get_index(runtime_dir: &Path, force_update: bool) -> Result<PackageIndex> {
        let old_index = Self::get_index_from_cache(runtime_dir)?;

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
        Self::save_index_to_cache(runtime_dir, new_index.clone())?;
        Ok(new_index)
    }

    fn download_index() -> Result<PackageIndex> {
        info!("fetching from hub...");
        let json_body = read_string_from_url(COCMD_HUB_PACKAGE_INDEX_URL)?;
        let index: PackageIndex = serde_json::from_str(&json_body)?;
        Ok(index)
    }

    fn get_index_from_cache(runtime_dir: &Path) -> Result<Option<CachedPackageIndex>> {
        let target_file = runtime_dir.join(PACKAGE_INDEX_CACHE_FILE);
        if !target_file.is_file() {
            return Ok(None);
        }

        let content =
            std::fs::read_to_string(&target_file).context("unable to read package index cache")?;
        let index: CachedPackageIndex = serde_json::from_str(&content)?;
        Ok(Some(index))
    }

    fn save_index_to_cache(runtime_dir: &Path, index: PackageIndex) -> Result<()> {
        let target_file = runtime_dir.join(PACKAGE_INDEX_CACHE_FILE);
        let current_time = std::time::SystemTime::now().duration_since(UNIX_EPOCH)?;
        let current_unix = current_time.as_secs();
        let cached_index = CachedPackageIndex {
            cached_at: current_unix,
            index,
        };
        let serialized_res = serde_json::to_string(&cached_index);
        if let Err(err) = serialized_res {
            return Err(anyhow!("unable to serialize package index: {}", err));
        }
        let serialized = serialized_res.unwrap();
        let write_res = std::fs::write(target_file, serialized);
        if let Err(err) = write_res {
            return Err(anyhow!("unable to write package index cache: {}", err));
        }
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
    pub fn get_package(&self, name: &str, version: &Option<String>) -> Option<PackageInfo> {
        let mut matching_packages: Vec<PackageInfo> = self
            .packages
            .iter()
            .filter(|package| package.name == name)
            .cloned()
            .collect();

        matching_packages.sort_by(|a, b| natord::compare(&a.version, &b.version));

        if let Some(explicit_version) = version {
            let exact_match = matching_packages
                .clone()
                .into_iter()
                .find(|package| package.version == *explicit_version);
            if exact_match.is_some() {
                return exact_match.clone();
            }
        }

        matching_packages.into_iter().last()
    }
}

// write a test for hub provider
// download package called "docker" from the hub

#[cfg(test)]
mod tests {

    use temp_testdir::TempDir;

    use super::*;

    #[test]
    fn test_get_index() {
        let runtime_dir = TempDir::default();
        let index =
            CocmdHubPackageProvider::get_index(runtime_dir.to_path_buf().as_path(), true).unwrap();
        assert!(!index.packages.is_empty());
    }

    #[test]
    fn test_get_package() {
        let runtime_dir = TempDir::default();
        let provider = CocmdHubPackageProvider::new(&"docker".to_string(), &runtime_dir, None);
        provider.download();
        let index =
            CocmdHubPackageProvider::get_index(runtime_dir.to_path_buf().as_path(), true).unwrap();
        let package = index.get_package("docker", &None).unwrap();
        assert_eq!(package.name, "docker");
    }

    #[test]
    fn test_get_package_with_version() {
        // generate a temporary directory, with random name for every test execution
        // it should be deleted in the end of the test automatically
        let runtime_dir = TempDir::default();
        let provider = CocmdHubPackageProvider::new(
            &"aws-s3".to_string(),
            &runtime_dir,
            Some(String::from("0.0.0")),
        );
        provider.download();
        let index =
            CocmdHubPackageProvider::get_index(runtime_dir.to_path_buf().as_path(), true).unwrap();
        let package = index
            .get_package("aws-s3", &Some("0.0.0".to_string()))
            .unwrap();
        assert_eq!(package.name, "aws-s3");
        assert_eq!(package.version, "0.0.0");
    }

    #[test]
    fn test_get_package_with_version_not_found() {
        let runtime_dir = TempDir::default();
        let provider = CocmdHubPackageProvider::new(
            &"docker".to_string(),
            &runtime_dir,
            Some(String::from("0.0.0")),
        );
        provider.download();
        let index =
            CocmdHubPackageProvider::get_index(runtime_dir.to_path_buf().as_path(), true).unwrap();
        let package = index
            .get_package("docker", &Some("20.10.9".to_string()))
            .unwrap();
        assert_eq!(package.name, "docker");
        assert_eq!(package.version, "0.0.0");
    }

    #[test]
    fn test_get_package_not_found() {
        let runtime_dir = TempDir::default();
        let provider = CocmdHubPackageProvider::new(&"docker".to_string(), &runtime_dir, None);
        provider.download();
        let index =
            CocmdHubPackageProvider::get_index(runtime_dir.to_path_buf().as_path(), true).unwrap();
        let package = index.get_package("docker2", &None);
        assert!(package.is_none());
    }
}
