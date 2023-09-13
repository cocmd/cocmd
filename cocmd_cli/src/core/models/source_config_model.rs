use crate::utils::sys::OS;
use crate::utils::io::{normalize_path, from_yaml_file, from_file};
use serde_derive::{Serialize as Se, Deserialize as De};
use super::script_model::{ScriptModel, StepModel};
use tracing::error;

#[derive(Debug, Se, De, PartialEq, Eq, Hash, Clone)]
pub struct Automation {
    pub name: String,
    pub file: Option<String>,
    pub content: Option<ScriptModel>,
}

impl Automation {
    pub fn load_content(&self, location: &str)->Automation {
        let mut automation_clone = self.clone();
        if let Some(file) = &self.file {
            let normalized_path = normalize_path(file, Some(location));

            match from_yaml_file::<ScriptModel>(&normalized_path) {

                Ok(script_model) => {
                    let updated_steps: Vec<StepModel> = script_model
                        .steps
                        .iter()
                        .map(|step| {
                            if let Some(file) = &step.file {
                                let normalized_path = normalize_path(file, Some(location));

                                match from_file(&normalized_path) {
                                    Ok(file_content) => {
                                        let mut step_clone = step.clone(); // Clone the existing StepModel
                                        step_clone.content = Some(file_content);
                                        step_clone
                                    }
                                    Err(err) => {
                                        // Handle the error if needed
                                        error!(err);
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
                    error!(err);
                }
            }
        }
        automation_clone
    }

    pub fn supports_os(&self, os: &OS) -> bool {
        if let Some(content) = &self.content {
            if let Some(content_env) = &content.env {
                return *content_env == *os || *content_env == OS::ANY;
            }
        }
        false
    }
}


#[derive(Debug, Se, De, PartialEq, Eq, Hash)]
pub struct SourceConfigModel {
    pub name: String,
    pub aliases: Option<String>,
    pub paths: Option<Vec<String>>,
    pub automations: Option<Vec<Automation>>,
}

