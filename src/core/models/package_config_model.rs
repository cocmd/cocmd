use std::path::{Path, PathBuf};

use log::error;
use serde_derive::{Deserialize as De, Serialize as Se};

use super::script_model::{ScriptModel, StepModel};
use crate::core::utils::io::{from_file, from_yaml_file, normalize_path};
use crate::core::utils::sys::OS;

#[derive(Debug, Se, De, PartialEq, Eq, Hash, Clone)]
pub struct Automation {
    pub name: String,
    pub file: Option<String>,
    pub content: Option<ScriptModel>,
}

impl Automation {
    pub fn load_content(&self, location: &Path) -> Automation {
        let mut automation_clone = self.clone();
        if let Some(file) = &self.file {
            let normalized_path = normalize_path(file, location);

            match from_yaml_file::<ScriptModel>(&PathBuf::from(normalized_path)) {
                Ok(script_model) => {
                    let updated_steps: Vec<StepModel> = script_model
                        .steps
                        .iter()
                        .map(|step| {
                            if let Some(file) = &step.file {
                                let normalized_path = normalize_path(file, location);

                                match from_file(&normalized_path) {
                                    Ok(file_content) => {
                                        let mut step_clone = step.clone(); // Clone the existing StepModel
                                        step_clone.content = Some(file_content);
                                        step_clone
                                    }
                                    Err(err) => {
                                        // Handle the error if needed
                                        error!("{}", err);
                                        step.clone() // Return the original step on error
                                    }
                                }
                            } else {
                                step.clone() // No file specified, return the original step
                            }
                        })
                        .collect();

                    // Update the content's steps with the updated_steps
                    let mut content_clone = script_model.clone();
                    content_clone.steps = updated_steps;
                    automation_clone.content = Some(content_clone);
                }
                Err(err) => {
                    // Handle the error if needed
                    error!("{}", err);
                }
            }
        }
        automation_clone
    }

    pub fn supports_os(&self, os: &OS) -> bool {
        if let Some(content) = &self.content {
            if let Some(content_env) = &content.env {
                return *content_env == *os || *content_env == OS::Any;
            }
        }
        true
    }

    pub fn get_detailed_description(&self) -> String {
        let mut description = self
            .content
            .as_ref()
            .unwrap()
            .description
            .as_ref()
            .unwrap_or(&"".to_string())
            .to_string();
        let steps = &self.content.as_ref().unwrap().steps;

        if steps.len() > 1 {
            description += ": ";

            for step in steps {
                description += &format!("{}. ", step.title);
            }
        }

        description
    }
}

#[derive(Debug, Se, De, PartialEq, Eq, Hash, Clone)]
pub struct PackageConfigModel {
    pub name: String,
    pub version: Option<String>,
    pub aliases: Option<String>,
    pub paths: Option<Vec<String>>,
    pub automations: Option<Vec<Automation>>,
}
