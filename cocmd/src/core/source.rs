use std::fmt;

use std::path::Path;
use std::path::PathBuf;

use tracing::error;

use crate::consts;
use crate::core::models::source_config_model::Automation;
use crate::core::models::source_config_model::SourceConfigModel;
use crate::utils::io::{from_yaml_file, normalize_path};
use crate::utils::sys::OS;
use crate::Settings;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Source {
    pub uri: String,
    pub location: PathBuf,
    pub cocmd_config: Option<SourceConfigModel>,
}

impl Source {
    pub fn new(uri: String, location: &PathBuf, _settings: &Settings) -> Self {
        let mut source = Source {
            uri: uri.clone(),
            location: location.to_path_buf(),
            cocmd_config: None,
        };

        if source.location.exists() {
            let config_file_path = Path::new(&source.location).join(consts::SOURCE_CONFIG_FILE);

            if config_file_path.exists() {
                let config: Result<SourceConfigModel, String> =
                    from_yaml_file(config_file_path.to_str().unwrap()).map_err(|e| e.to_string());
                match config {
                    Ok(config_res) => {
                        // Successfully loaded the configuration
                        // You can use 'config' here
                        source.cocmd_config = Some(config_res);
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
                "Source Path {} does not exist.",
                source.location.to_str().unwrap()
            )
        }
        source
    }

    pub fn is_exists_locally(&self) -> bool {
        self.location.exists()
    }

    pub fn is_legit_cocmd_source(&self) -> bool {
        if self.location.exists() {
            let config_file_path = Path::new(&self.location).join(consts::SOURCE_CONFIG_FILE);

            if config_file_path.exists() {
                let config: Result<SourceConfigModel, String> =
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

        if let Some(source_config) = &self.cocmd_config {
            if let Some(automations) = &source_config.automations {
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

        output += "## automations\n";
        for automation in &self.automations(settings, Some(env_specific)) {
            output += &format!("- `{}.{}`\n", self.name(), automation.name);
            output += &format!(
                "  - description: {}\n",
                automation
                    .content
                    .as_ref()
                    .unwrap()
                    .description
                    .as_ref()
                    .unwrap_or(&"Not provided".to_string())
            );
            if !env_specific {
                let env = &automation.content.as_ref().unwrap().env.unwrap_or(OS::ANY);
                output += &format!("  - env: {}\n", env);
            }
            output += "\n";
        }

        output += "## paths\n";
        for p in &self.paths() {
            output += &format!("- `{}`\n", p);
        }

        if print_as_md {
            let skin = termimad::MadSkin::default();
            skin.print_text(&output);
        } else {
            println!("{}", output);
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.location.to_str().unwrap())
    }
}
