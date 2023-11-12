#![allow(clippy::format_in_format_args)]
#![allow(dead_code)]
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use log::{error, warn};

use super::utils::io::exists;
use crate::core::consts;
use crate::core::models::package_config_model::Automation;
use crate::core::models::package_config_model::PackageConfigModel;
use crate::core::utils::io::{from_yaml_file, normalize_path};
use crate::core::utils::sys::OS;
use crate::output::print_md;
use crate::Settings;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Package {
    pub uri: String,
    pub location: PathBuf,
    pub cocmd_config: Option<PackageConfigModel>,
}

impl Package {
    pub fn new(uri: String, location: &Path, _settings: &Settings) -> Self {
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
        self.cocmd_config
            .as_ref()
            .map_or("???", |config| config.name.as_str())
    }

    pub fn version(&self) -> String {
        self.cocmd_config
            .as_ref()
            .map_or(String::from("0.0.0"), |config| {
                config
                    .version
                    .as_ref()
                    .unwrap_or(&String::from("0.0.0"))
                    .to_string()
            })
    }

    pub fn paths(&self, absolute: bool) -> Vec<String> {
        match &self.cocmd_config {
            Some(config) => {
                match &config.paths {
                    Some(paths) => {
                        if absolute {
                            paths
                                .iter()
                                .map(|p| normalize_path(p, &self.location))
                                .collect()
                        } else {
                            paths.iter().map(|p| p.to_string()).collect()
                        }
                    }
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

    pub fn print_doc(&self, settings: &Settings, print_as_markdown: bool, env_specific: bool) {
        // i want to print this content as md(with skin) or raw text(just println):

        let mut output = String::new();

        let _name = self.name();

        let automations = self.automations(settings, Some(env_specific));
        if !automations.is_empty() {
            output += &format!(
                "## automations ({})\n",
                self.get_automations_count(settings)
            );

            // write a markdown table for automation. columns are name, env, description, number of steps
            output += "| command | env | description | how to run? |\n";
            output += "| --- | --- | --- | --- |\n";

            for automation in &automations {
                let env = &automation.content.as_ref().unwrap().env.unwrap_or(OS::Any);
                let package_name = self.name();

                output += &format!(
                    "| {}.{} | {} | {} | {} |\n",
                    package_name,
                    automation.name,
                    env,
                    automation.get_detailed_description(),
                    format!(
                        "run `{}.{}` or `cocmd run {}.{}`",
                        package_name, automation.name, package_name, automation.name
                    )
                );
            }
            output += "\n";
        }

        if let Some(alias) = &self.aliases() {
            output += &format!(
                "## aliases ({}):\n```\n{}\n```\n",
                self.get_aliases_count(),
                alias
            );
        }

        if !self.paths(false).is_empty() {
            output += &format!("## PATH additions ({})\n", self.get_paths_count());
            for (_rel_p, abs_p) in self.paths(false).iter().zip(self.paths(true).iter()) {
                // list all files in the path p - it's supposed to be executables of shell. make sure it's shell script.
                // look for comments in the beginning of each file to understand what it does. write it as a table in markdown format

                output += &abs_p.to_string();

                if !exists(abs_p) {
                    output += " (not exists)";
                    continue;
                }

                output += ":\n\n";

                // write a markdown table for files in fs::read_dir(abs_p).unwrap()
                // column 1 filename
                // column 2 desc (pick up from comment line that starts with # COCMD-DESC: ...
                // column 3 usage (pick up from comment line that starts with # COCMD-USAGE: ...
                // column 4 example (pick up from comment line that starts with # COCMD-EXAMPLE: ...
                output += "| command | desc | usage \n";
                output += "| --- | --- | --- |\n";

                for entry in fs::read_dir(abs_p).unwrap() {
                    let entry = entry.unwrap();
                    let file_name = entry.file_name();
                    let file_path = entry.path();

                    if let Ok(file_content) = fs::read_to_string(&file_path) {
                        let mut desc = String::new();
                        let mut usage = String::new();

                        for line in file_content.lines() {
                            if line.starts_with("# COCMD-DESC:") {
                                desc = line.replace("# COCMD-DESC:", "").trim().to_string();
                            } else if line.starts_with("# COCMD-USAGE:") {
                                usage = line.replace("# COCMD-USAGE:", "").trim().to_string();
                            }
                        }

                        output += &format!(
                            "| `{}` | {} | run `{}` |\n",
                            file_name.to_str().unwrap(),
                            desc,
                            usage
                        );
                    } else {
                        warn!("Unable to read file {}", file_path.to_str().unwrap());
                    }
                }
            }
        }

        if print_as_markdown {
            print_md(&output);
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
            self.paths(false).len()
        } else {
            0
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.location.to_string_lossy())
    }
}
