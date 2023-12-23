#![allow(clippy::format_in_format_args)]
#![allow(dead_code)]
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;

use log::error;
use log::warn;

use super::utils::io::file_write;
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
                    from_yaml_file(&config_file_path).map_err(|e| e.to_string());
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
            warn!(
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
                    from_yaml_file(&config_file_path).map_err(|e| e.to_string());
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

    pub fn get_automations_envs_map(&self) -> HashMap<String, HashMap<Vec<OS>, Automation>> {
        let mut env_automations: HashMap<String, Vec<Automation>> = HashMap::new();

        if let Some(package_config) = &self.cocmd_config {
            if let Some(automations) = &package_config.automations {
                for automation in automations.iter() {
                    let automation_loaded = automation.load_content(&self.location);
                    let automation_name = automation_loaded.name.clone();

                    if let Some(automation_vec) = env_automations.get_mut(&automation_name) {
                        automation_vec.push(automation_loaded.clone());
                    } else {
                        let new_automation_vec = vec![automation_loaded.clone()];
                        env_automations.insert(automation_name.clone(), new_automation_vec);
                    }
                }
            }
        }

        let mut result: HashMap<String, HashMap<Vec<OS>, Automation>> = HashMap::new();
        // convert env_automations to result, make sure
        // that for each automation OS not appear more than once
        for (automation_name, automation_vec) in env_automations {
            let mut os_automation: HashMap<Vec<OS>, Automation> = HashMap::new();

            for automation in automation_vec {
                let mut os_vec: Vec<OS> = vec![];
                if let Some(content) = &automation.content {
                    os_vec = content.get_env();
                }
                if os_vec.is_empty() {
                    os_vec.push(OS::Any);
                }
                os_vec.sort();
                os_vec.dedup();
                os_automation.insert(os_vec, automation.clone());
            }
            result.insert(automation_name, os_automation);
        }

        result
    }

    pub fn get_automations_envs_map_flat(&self) -> HashMap<String, HashMap<OS, Automation>> {
        // use get_automations_envs_map and flatten the OS vector
        let mut result: HashMap<String, HashMap<OS, Automation>> = HashMap::new();
        let automations_envs_map = self.get_automations_envs_map();
        for (automation_name, envs_map) in automations_envs_map {
            let mut os_automation: HashMap<OS, Automation> = HashMap::new();
            for (os_vec, automation) in envs_map {
                for os in os_vec {
                    os_automation.insert(os, automation.clone());
                }
            }
            result.insert(automation_name, os_automation);
        }
        result
    }

    pub fn get_playbook(&self, playbook_name: String) -> Option<Automation> {
        if let Some(package_config) = &self.cocmd_config {
            if let Some(automations) = &package_config.automations {
                for automation in automations.iter() {
                    if automation.name == playbook_name {
                        return Some(automation.clone());
                    }
                }
            }
        }
        None
    }

    pub fn get_playbook_envs_map(
        &self,
        playbook_name: String,
    ) -> Option<HashMap<Vec<OS>, Automation>> {
        // get key value from self.get_automations_envs_map
        // where key is playbook_name

        let automations_envs_map = self.get_automations_envs_map().clone();
        automations_envs_map.get(&playbook_name).cloned()
    }

    pub fn get_playbook_envs_map_flat(
        &self,
        playbook_name: String,
    ) -> Option<HashMap<OS, Automation>> {
        self.get_automations_envs_map_flat()
            .clone()
            .get(&playbook_name)
            .cloned()
    }

    pub fn location(&self) -> &PathBuf {
        &self.location
    }

    pub fn print_doc(
        &self,
        _settings: &Settings,
        print_as_markdown: bool,
        _env_specific: bool,
        output_file: Option<String>,
    ) {
        // i want to print this content as md(with skin) or raw text(just println):
        let package_name = self.name();
        let mut output = String::new();

        let _name = self.name();

        output += &format!("\nlocation: {}\n", self.location.to_string_lossy());

        let automations_envs_map = self.get_automations_envs_map_flat();

        if !automations_envs_map.is_empty() {
            output += &format!(
                "\nThis package contains {} playbooks:\n\n",
                &automations_envs_map.len()
            );

            for (automation_name, envs_map) in &automations_envs_map {
                // write more human readable for this playbook
                // title with the name of the playbook
                // the env, description and how to run it (with cocmd or with cocmd run)

                let supported_envs: Vec<String> =
                    envs_map.keys().cloned().map(|o| o.to_string()).collect();

                output += &format!(
                    "### {}.{} ({})\n",
                    package_name,
                    automation_name,
                    supported_envs.join(", ")
                );

                // print description
                output += &format!(
                    "{}\n\n",
                    envs_map
                        .values()
                        .next()
                        .as_ref()
                        .unwrap()
                        .content
                        .as_ref()
                        .unwrap()
                        .description
                        .clone()
                        .unwrap_or(String::from(""))
                );

                // output += &format!("{}\n", automation.get_detailed_description());
                output += &format!(
                    "\nrun it with: `cocmd run {}.{}`\n\n",
                    package_name, automation_name
                );
            }
            output += "\n\n";
        } else {
            output += "\nThis package contains no playbooks\n\n";
        }

        // if let Some(alias) = &self.aliases() {
        //     output += &format!(
        //         "## aliases ({}):\n```\n{}\n```\n",
        //         self.get_aliases_count(),
        //         alias
        //     );
        // }

        // if !self.paths(false).is_empty() {
        //     output += &format!("## PATH additions ({})\n", self.get_paths_count());
        //     for (_rel_p, abs_p) in self.paths(false).iter().zip(self.paths(true).iter()) {
        //         // list all files in the path p - it's supposed to be executables of shell. make sure it's shell script.
        //         // look for comments in the beginning of each file to understand what it does. write it as a table in markdown format

        //         output += &abs_p.to_string();

        //         if !exists(abs_p) {
        //             output += " (not exists)";
        //             continue;
        //         }

        //         output += ":\n\n";

        //         // write a markdown table for files in fs::read_dir(abs_p).unwrap()
        //         // column 1 filename
        //         // column 2 desc (pick up from comment line that starts with # COCMD-DESC: ...
        //         // column 3 usage (pick up from comment line that starts with # COCMD-USAGE: ...
        //         // column 4 example (pick up from comment line that starts with # COCMD-EXAMPLE: ...
        //         output += "| command | desc | usage \n";
        //         output += "| --- | --- | --- |\n";

        //         for entry in fs::read_dir(abs_p).unwrap() {
        //             let entry = entry.unwrap();
        //             let file_name = entry.file_name();
        //             let file_path = entry.path();

        //             if let Ok(file_content) = fs::read_to_string(&file_path) {
        //                 let mut desc = String::new();
        //                 let mut usage = String::new();

        //                 for line in file_content.lines() {
        //                     if line.starts_with("# COCMD-DESC:") {
        //                         desc = line.replace("# COCMD-DESC:", "").trim().to_string();
        //                     } else if line.starts_with("# COCMD-USAGE:") {
        //                         usage = line.replace("# COCMD-USAGE:", "").trim().to_string();
        //                     }
        //                 }

        //                 output += &format!(
        //                     "| `{}` | {} | run `{}` |\n",
        //                     file_name.to_str().unwrap(),
        //                     desc,
        //                     usage
        //                 );
        //             } else {
        //                 warn!("Unable to read file {}", file_path.to_str().unwrap());
        //             }
        //         }
        //     }
        // }

        if let Some(output_file) = output_file {
            // write output to output_file, if exists overwrite it
            file_write(Path::new(&output_file), &output, true).map_err(|e| {
                error!(
                    "Unable to write to file {}: {}",
                    &output_file,
                    e.to_string()
                );
                e
            });
        } else {
            // write to stdout
            if print_as_markdown {
                print_md(&output);
            } else {
                println!("{}", output);
            }
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
