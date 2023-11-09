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
#![allow(dead_code)]
use std::{path::Path, path::PathBuf, process::Command};

use anyhow::{bail, Context, Result};

use super::PackageProvider;
use super::{util::git::GitParts, GIT_PROVIDER};

pub struct GitPackageProvider {
    package: String,
    local_path: PathBuf,
    git_parts: GitParts,
}

impl GitPackageProvider {
    pub fn new(package: &str, git_parts: &GitParts, runtime_dir: &Path) -> Self {
        // localpath is in runtime_dir with the name of the repo
        let binding = runtime_dir.join(format!("{}.{}", git_parts.author, git_parts.name));
        let local_path = binding.as_path();
        Self {
            package: package.to_string(),
            git_parts: (*git_parts).clone(),
            local_path: local_path.to_path_buf(),
        }
    }

    fn is_git_installed() -> bool {
        if let Ok(output) = Command::new("git").arg("--version").output() {
            if output.status.success() {
                return true;
            }
        }

        false
    }

    fn clone_repo(&self) -> Result<()> {
        let mut args = vec!["clone"];

        args.push(self.package.as_str());

        let dest_dir_str = self.local_path.to_str().unwrap();
        args.push(dest_dir_str);

        let output = Command::new("git")
            .args(&args)
            .output()
            .context("git command reported error")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("git command exited with non-zero status: {}", stderr);
        } else {
            Ok(())
        }
    }
}

impl PackageProvider for GitPackageProvider {
    fn name(&self) -> String {
        GIT_PROVIDER.to_string()
    }

    fn local_path(&self) -> PathBuf {
        self.local_path.to_path_buf()
    }

    fn package(&self) -> String {
        self.package.clone()
    }

    fn download(&mut self) -> Result<PathBuf> {
        if !Self::is_git_installed() {
            bail!("unable to invoke 'git' command, please make sure it is installed and visible in PATH");
        }

        self.clone_repo()?;

        Ok(self.local_path.clone())
    }
}
