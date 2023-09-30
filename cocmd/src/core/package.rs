use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use tracing::error;

use crate::consts;
use crate::core::models::package_config_model::Automation;
use crate::core::models::package_config_model::PackageConfigModel;
use crate::utils::io::{from_yaml_file, normalize_path};
use crate::utils::sys::OS;
use crate::Settings;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Package {
    pub uri: String,
    pub location: PathBuf,
    pub cocmd_config: Option<PackageConfigModel>,
}

impl Package {
    pub fn new(uri: String, location: &PathBuf, _settings: &Settings) -> Self {
        let mut package = Package {
            uri: uri.clone(),
            location: location.to_path_buf(),
            cocmd_config: None,
        };

        if package.location.exists() {
            let config_file_path = Path::new(&package.location).join(consts::SOURCE_CONFIG_FILE);

            if config_file_path.exists() {
                let config: Result<PackageConfigModel, String> =
                    from_yaml_file(config_file_path.to_str().unwrap()).map_err(|e| e.to_string());
                match config {
                    Ok(config_res) => {
                        // Successfully loaded the configuration
                        // You can use 'config' here
                        package.cocmd_config = Some(config_res);
                    }
                    Err(err) => {
                        // Handle the error, for example, log it
                        error!("{}: {}", config_file_path.to_str().unwrap(), err);
                    }
                };
            } else {
                error!(
                    "Config Path {:?} does not exist.",
                    config_file_path.to_str().unwrap(),
                );
            }
        } else {
            error!(
                "Package Path {} does not exist.",
                package.location.to_str().unwrap()
            )
        }
        package
    }

    pub fn is_exists_locally(&self) -> bool {
        self.location.exists()
    }

    pub fn is_legit_cocmd_package(&self) -> bool {
        if self.location.exists() {
            let config_file_path = Path::new(&self.location).join(consts::SOURCE_CONFIG_FILE);

            if config_file_path.exists() {
                let config: Result<PackageConfigModel, String> =
                    from_yaml_file(config_file_path.to_str().unwrap()).map_err(|e| e.to_string());
                config.is_ok()
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn aliases(&self) -> Option<String> {
        match &self.cocmd_config {
            Some(config) => config.aliases.clone(),
            None => None, // or any other default behavior
        }
    }

    pub fn name(&self) -> &str {
        match &self.cocmd_config {
            Some(config) => &config.name,
            None => {
                panic!(
                    "Unable to get name for {}",
                    &self.location.to_str().unwrap()
                );
            }
        }
    }

    pub fn paths(&self) -> Vec<String> {
        match &self.cocmd_config {
            Some(config) => {
                match &config.paths {
                    Some(paths) => paths
                        .iter()
                        .map(|p| normalize_path(p, &self.location))
                        .filter(|p| Path::new(p).exists())
                        .collect(),
                    None => vec![], // or any other default behavior
                }
            }
            None => vec![], // or any other default behavior
        }
    }

    pub fn automations(&self, settings: &Settings, env_specific: Option<bool>) -> Vec<Automation> {
        let mut result = vec![];
        let env_specific = env_specific.unwrap_or(true);

        if let Some(package_config) = &self.cocmd_config {
            if let Some(automations) = &package_config.automations {
                for automation in automations.iter() {
                    let automation_loaded = automation.load_content(&self.location);
                    if !env_specific || automation_loaded.supports_os(&settings.os) {
                        result.push(automation_loaded);
                    }
                }
            }
        }
        result
    }

    pub fn location(&self) -> &PathBuf {
        &self.location
    }

    pub fn print_doc(&self, settings: &Settings, print_as_md: bool, env_specific: bool) {
        // i want to print this content as md(with skin) or raw text(just println):

        let mut output = String::new();

        // output += &format!("## {}\n", self.name());
        output += &format!("- location: {}\n", self.location().to_str().unwrap());

        if let Some(alias) = &self.aliases() {
            output += &format!("## aliases\n```\n{}\n```\n", alias);
        }

        let automations = self.automations(settings, Some(env_specific));
        if !automations.is_empty() {
            output += "## automations\n";

            // write a markdown table for automation. columns are name, env, description, number of steps
            output += "| name | env | description | steps |\n";
            output += "| --- | --- | --- | --- |\n";

            for automation in &automations {
                let env = &automation.content.as_ref().unwrap().env.unwrap_or(OS::ANY);
                output += &format!(
                    "| {} | {} | {} | {} |\n",
                    automation.name,
                    env,
                    automation
                        .content
                        .as_ref()
                        .unwrap()
                        .description
                        .as_ref()
                        .unwrap_or(&"Not provided".to_string()),
                    automation.content.as_ref().unwrap().steps.len()
                );
            }
            output += "\n";
        }

        if !self.paths().is_empty() {
            output += "## paths\n";
            for p in &self.paths() {
                // list all files in the path p - it's supposed to be executables of shell. make sure it's shell script.
                // look for comments in the beginning of each file to understand what it does. write it as a table in markdown format
                output += &format!("### {}\n", p);

                for entry in fs::read_dir(p).unwrap() {
                    let entry = entry.unwrap();
                    output += &format!("  - {}\n", entry.file_name().to_str().unwrap());
                }
            }
        }

        if print_as_md {
            let skin = termimad::MadSkin::default();
            skin.print_text(&output);
        } else {
            println!("{}", output);
        }
    }

    pub fn get_aliases_count(&self) -> usize {
        if self.is_legit_cocmd_package() {
            match self.aliases() {
                Some(aliases_str) => aliases_str.split('\n').count(),
                _ => 0,
            }
        } else {
            0
        }
    }

    pub fn get_automations_count(&self, settings: &Settings) -> usize {
        if self.is_legit_cocmd_package() {
            self.automations(settings, Some(true)).len()
        } else {
            0
        }
    }

    pub fn get_paths_count(&self) -> usize {
        if self.is_legit_cocmd_package() {
            self.paths().len()
        } else {
            0
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.location.to_str().unwrap())
    }
}
