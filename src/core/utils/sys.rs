use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Hash, Clone, Copy)]
pub enum OS {
    #[serde(alias = "windows", alias = "Windows")]
    Windows,
    #[serde(
        alias = "linux",
        alias = "Linux",
        alias = "deb",
        alias = "debian",
        alias = "Debian"
    )]
    Linux,
    #[serde(alias = "osx", alias = "Mac", alias = "Macos")]
    MacOS,
    Other,
    #[serde(alias = "any")]
    Any,
}

impl Display for OS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OS::Windows => write!(f, "Windows"),
            OS::Linux => write!(f, "Linux"),
            OS::MacOS => write!(f, "MacOS"),
            OS::Other => write!(f, "Other"),
            OS::Any => write!(f, "Any"),
        }
    }
}

pub fn get_os() -> OS {
    match std::env::consts::OS {
        "windows" => OS::Windows,
        "linux" => OS::Linux,
        "macos" => OS::MacOS,
        "any" => OS::Any,
        _ => OS::Other,
    }
}
