use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

use log::error;

use crate::core::utils::io::from_yaml_file;
use crate::core::utils::sys::get_os;
use crate::core::{consts, utils::sys::OS};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Settings {
    pub home: String,
    pub terminal: String,
    pub config_file: PathBuf,
    pub packages_file: PathBuf,
    pub runtime_dir: PathBuf,
    pub scan_depth: usize,
    pub os: OS,
    pub params: HashMap<String, String>,
    // packages_manager: PackagesManager, // You'll need to define this
    // credentials: CredsConfigModel, // You'll need to define this
}

impl Settings {
    pub fn new(home: Option<&str>, terminal: Option<&str>) -> Self {
        let home = home.unwrap_or(&consts::HOME);
        let terminal = terminal.unwrap_or(consts::DEFAULT_TERMINAL);
        let runtime_dir = Path::new(home).join(consts::RUNTIME_DIR);
        let config_file = Path::new(&home).join(consts::CONFIG_FILE);
        let packages_file = Path::new(&home).join(consts::SOURCES_FILE);
        let params_file_path = Path::new(&home).join(consts::PARAMS_FILE);

        // Create directories and files
        fs::create_dir_all(home).unwrap();
        fs::create_dir_all(&runtime_dir).unwrap();
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&packages_file)
            .unwrap();

        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&packages_file)
            .unwrap();

        // Initialize other fields
        Settings {
            home: home.to_string(),
            terminal: terminal.to_string(),
            runtime_dir,
            config_file,
            packages_file,
            scan_depth: 2,
            os: get_os(), // packages_manager: PackagesManager::new(), // Initialize this
            // credentials: CredsConfigModel::new(), // Initialize this
            params: Settings::read_params(params_file_path.as_path()),
        }
    }

    // create function 'read_params' that read from home/consts::PARAMS_FILE path yaml file and return a map of params
    // from params, should return HashMap<String, String>
    pub fn read_params(params_file_path: &Path) -> HashMap<String, String> {
        let params: Result<HashMap<String, String>, String> =
            from_yaml_file(params_file_path.to_str().unwrap()).map_err(|e| e.to_string());
        match params {
            Ok(params_res) => {
                // Successfully loaded the configuration
                // You can use 'config' here
                params_res
            }
            Err(err) => {
                // Handle the error, for example, log it
                error!("{}: {}", params_file_path.to_str().unwrap(), err);
                HashMap::new()
            }
        }
    }

    // get a specific param from self.params
    pub fn get_param(&self, param_name: &str) -> Option<String> {
        self.params.get(param_name).cloned()
    }

    // create a function save_param that save a param to self.params and to home/consts::PARAMS_FILE
    pub fn save_param(&mut self, param_name: &str, param_value: &str) {
        self.params
            .insert(param_name.to_string(), param_value.to_string());
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        let params_file_path = Path::new(&self.home).join(consts::PARAMS_FILE);
        let _ = fs::remove_file(&params_file_path);
        let _ = fs::File::create(&params_file_path);
        let _ = fs::write(
            &params_file_path,
            serde_yaml::to_string(&self.params).unwrap(),
        );
    }
}
