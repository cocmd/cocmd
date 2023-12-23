#![allow(clippy::upper_case_acronyms)]
#![allow(unused_variables)]
use std::path::Path;

use log::error;
use serde_derive::{Deserialize, Serialize};
use serde_with::{serde_as, OneOrMany};
use termimad::print_text;

use super::settings::Settings;
use crate::{
    core::utils::{io::file_write, sys::OS},
    output::print_md,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum StepRunnerType {
    #[serde(alias = "shell", alias = "SHELL")]
    SHELL,
    #[serde(alias = "welcome", alias = "WELCOME")]
    WELCOME,
    #[serde(alias = "markdown", alias = "MARKDOWN")]
    MARKDOWN,
    #[serde(alias = "python", alias = "PYTHON", alias = "py", alias = "PY")]
    PYTHON,
    #[serde(alias = "link", alias = "LINK", alias = "href", alias = "HREF")]
    LINK,
    #[serde(alias = "cocmd", alias = "COCMD")]
    COCMD,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Default)]
pub struct StepParamModel {
    pub name: String,
    pub save: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct StepModel {
    pub runner: StepRunnerType,
    pub content: Option<String>,
    pub file: Option<String>,
    pub title: Option<String>,
    pub params: Option<Vec<StepParamModel>>,
    pub approval_message: Option<String>,
}

impl StepModel {
    // create function that gets script level params and returnes a united version with step model params
    // the step model params takes priority (by the name)
    // the script level params are the default
    // the step level params are the override
    pub fn get_params(&self, script_params: Option<Vec<StepParamModel>>) -> Vec<StepParamModel> {
        let mut params = script_params.unwrap_or_default();
        if let Some(step_params) = &self.params {
            for step_param in step_params {
                if let Some(param_index) = params.iter().position(|p| p.name == step_param.name) {
                    params[param_index] = step_param.clone();
                } else {
                    params.push(step_param.clone());
                }
            }
        }
        if params.is_empty() {
            vec![]
        } else {
            params
        }
    }

    pub fn get_title(&self) -> String {
        self.title.clone().unwrap_or_default()
    }
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct ScriptModel {
    pub steps: Vec<StepModel>,
    #[serde_as(deserialize_as = "OneOrMany<_>")]
    #[serde(default = "default_env")]
    pub env: Vec<OS>,
    pub description: Option<String>,
    pub params: Option<Vec<StepParamModel>>,
}

fn default_env() -> Vec<OS> {
    vec![OS::Any]
}

impl ScriptModel {
    pub fn get_env(&self) -> Vec<OS> {
        self.env.clone()
    }

    pub fn print_doc(
        &self,
        settings: &Settings,
        print_as_markdown: bool,
        env_specific: bool,
        output_file: Option<String>,
        title: String,
    ) {
        let mut doc = String::new();
        doc.push_str(&format!("## {}\n", title));

        // if !env_specific {
        //     doc.push_str(&format!(
        //         "\nThis script is only available for the following OSes: {:?}\n",
        //         self.env
        //     ));
        // }

        if let Some(description) = &self.description {
            doc.push_str(&format!("{}\n\n", description));
        }
        if let Some(params) = &self.params {
            doc.push_str("### parameters\n\n");
            // add an explanation
            doc.push_str("Make sure to add the following parameters to the command below\n");

            for param in params {
                doc.push_str(&format!("* `{}`\n", param.name));
            }
            doc.push('\n');
        }
        for step in &self.steps {
            let title = step.get_title();
            if !title.is_empty() {
                let is_optional = step.approval_message.is_some();
                if is_optional {
                    doc.push_str(&format!("### (Optional) {}\n", step.get_title()));
                } else {
                    doc.push_str(&format!("### {}\n", step.get_title()));
                }
            }

            if let Some(step_params) = &step.params {
                doc.push_str("This step requires some additional parameters, make sure to add them to the command below\n\n");
                for param in step_params {
                    doc.push_str(&format!("* `{}`\n", param.name));
                }
                doc.push('\n');
            }

            let wrap_script_text = |script_text: &str, script_type: &str| -> String {
                // if the script text is more than 3 lines, put in in <details> collapsable section
                // otherwise, just print it as is
                let script_text = script_text.to_string();
                let lines = script_text.lines();
                let mut wrapped_script_text = String::new();
                if lines.clone().count() > 3 {
                    wrapped_script_text.push_str("<details><summary>script to run</summary>\n\n");
                    wrapped_script_text
                        .push_str(&format!("```{}\n{}\n```\n\n", script_type, script_text));
                    wrapped_script_text.push_str("</details>\n\n");
                } else {
                    wrapped_script_text
                        .push_str(&format!("```{}\n{}\n```\n\n", script_type, script_text));
                }
                wrapped_script_text
            };

            match step.runner {
                StepRunnerType::SHELL => {
                    doc.push_str(
                        &wrap_script_text(step.content.as_ref().unwrap(), "shell").to_string(),
                    );
                }
                StepRunnerType::MARKDOWN | StepRunnerType::WELCOME => {
                    doc.push_str(&format!("\n{}\n\n", step.content.as_ref().unwrap()));
                }
                StepRunnerType::PYTHON => {
                    doc.push_str(
                        &wrap_script_text(step.content.as_ref().unwrap(), "python").to_string(),
                    );
                }
                StepRunnerType::LINK => {
                    doc.push_str(&format!("\n{}\n\n\n", step.content.as_ref().unwrap()));
                }
                StepRunnerType::COCMD => {
                    doc.push_str(&format!(
                        "```shell\ncocmd run {}\n```\n\n",
                        step.content.as_ref().unwrap()
                    ));
                }
            }

            doc.push_str("\n\n");
        }

        doc.push_str("\n\n");

        if let Some(output_file) = output_file {
            file_write(Path::new(&output_file), &doc, false).map_err(|e| {
                error!("Unable to write to file {}: {}", output_file, e.to_string());
                e
            });
        } else if print_as_markdown {
            print_md(&doc);
        } else {
            print_text(&doc);
        }
    }
}
