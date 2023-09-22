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

use std::path::PathBuf;

use anyhow::Result;
pub(crate) mod git;

pub(crate) mod hub;
pub(crate) mod local;

pub trait PackageProvider {
    fn name(&self) -> String;
    fn source(&self) -> String;
    fn local_path(&self) -> PathBuf;
    fn is_exists_locally(&self) -> bool {
        // check for existsance of the local path
        self.local_path().exists()
    }
    fn download(&self) -> Result<PathBuf>;
    // TODO: fn check update available? (probably should be only available in the hub)
}
