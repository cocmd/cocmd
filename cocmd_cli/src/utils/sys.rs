use serde_derive::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Hash, Clone, Copy)]
pub enum OS {
    #[serde(alias="windows", alias="Windows")]
    Windows,
    #[serde(alias="linux", alias="Linux", alias="deb", alias="debian", alias="Debian")]
    Linux,
    #[serde(alias="osx", alias="Mac", alias="Macos")]
    MacOS,
    Other,
    #[serde(alias="any")]
    ANY
}


pub fn get_os() -> OS {
    match std::env::consts::OS {
        "windows" => OS::Windows,
        "linux" => OS::Linux,
        "macos" => OS::MacOS,
        "any" => OS::ANY,
        _ => OS::Other,
    }
}