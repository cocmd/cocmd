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
use std::process::Command;

use lazy_static::lazy_static;
use regex::Regex;

pub fn is_private_repo(url: &str) -> bool {
    if let Ok(output) = Command::new("git").arg("ls-remote").arg(url).output() {
        if output.status.success() {
            return true;
        }
    }

    false
}
lazy_static! {
    static ref GIT_REGEX: Regex =
        Regex::new(r"^(https://|git@)(?P<host>[^:/]+)([/|:])(?P<author>.*?)/(?P<name>.*?)(/|\.|$)")
            .unwrap();
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GitParts {
    pub host: String,
    pub author: String,
    pub name: String,
}

pub fn extract_git_url_parts(url: &str) -> Option<GitParts> {
    let captures = GIT_REGEX.captures(url)?;
    let host = captures.name("host")?;
    let author = captures.name("author")?;
    let name = captures.name("name")?;

    Some(GitParts {
        host: host.as_str().to_string(),
        author: author.as_str().to_string(),
        name: name.as_str().to_string(),
    })
}
