use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

use crate::utils::sys::get_os;
use crate::{consts, utils::sys::OS};

#[derive(PartialEq, Eq)]
pub struct Settings {
    pub home: String,
    pub terminal: String,
    pub config_file: String,
    pub sources_file: String,
    pub runtime_dir: PathBuf,
    pub scan_depth: usize,
    pub os: OS,
    // sources_manager: SourcesManager, // You'll need to define this
    // credentials: CredsConfigModel, // You'll need to define this
}

impl Settings {
    pub fn new(home: Option<&str>, terminal: Option<&str>) -> Self {
        let home = home.unwrap_or(&consts::HOME);
        let terminal = terminal.unwrap_or(consts::DEFAULT_TERMINAL);
        let runtime_dir = Path::new(home).join(consts::RUNTIME_DIR);
        let config_file = format!("{}/{}", home, consts::CONFIG_FILE);
        let sources_file = format!("{}/{}", home, consts::SOURCES_FILE);

        // Create directories and files
        fs::create_dir_all(home).unwrap();
        fs::create_dir_all(&runtime_dir).unwrap();
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&sources_file)
            .unwrap();

        // Initialize other fields
        Settings {
            home: home.to_string(),
            terminal: terminal.to_string(),
            runtime_dir,
            config_file,
            sources_file,
            scan_depth: 2,
            os: get_os(), // sources_manager: SourcesManager::new(), // Initialize this
                          // credentials: CredsConfigModel::new(), // Initialize this
        }
    }

    // fn read_creds(&self) -> CredsConfigModel {
    //     // Implement this function
    // }
}
