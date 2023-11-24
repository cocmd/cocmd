#![allow(clippy::upper_case_acronyms)]
#![allow(unused_variables)]
use std::path::Path;

use log::error;
use serde_derive::{Deserialize, Serialize};
use termimad::print_text;

use super::settings::Settings;
use crate::{
    core::{
        consts::GEN_MESSAGE,
        utils::{io::file_write, sys::OS},
    },
    output::print_md,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum StepRunnerType {
    #[serde(alias = "shell", alias = "SHELL")]
    SHELL,
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
    pub title: String,
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
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct ScriptModel {
    pub steps: Vec<StepModel>,
    pub env: Option<OS>,
    pub description: Option<String>,
    pub params: Option<Vec<StepParamModel>>,
}

impl ScriptModel {
    pub fn print_doc(
        &self,
        settings: &Settings,
        print_as_markdown: bool,
        env_specific: bool,
        output_file: Option<String>,
        title: String,
    ) {
        let mut doc = String::new();
        doc.push_str(&format!("# {}\n", title));

        if let Some(description) = &self.description {
            doc.push_str(&format!("{}\n\n", description));
        }
        if let Some(params) = &self.params {
            doc.push_str("## Some parameters\n\n");
            // add an explanation
            doc.push_str("Make sure to add the following parameters to the command below\n");

            for param in params {
                doc.push_str(&format!("* `{}`\n", param.name));
            }
            doc.push_str("\n");
        }
        for step in &self.steps {
            let is_optional = step.approval_message.is_some();
            if is_optional {
                doc.push_str(&format!(
                    "<details><summary>(Optional) {}</summary>\n\n",
                    step.title
                ));
            } else {
                doc.push_str(&format!("<details><summary>{}</summary>\n\n", step.title));
            }

            if let Some(step_params) = &step.params {
                doc.push_str("This step requires some additional parameters, make sure to add them to the command below\n\n");
                for param in step_params {
                    doc.push_str(&format!("* `{}`\n", param.name));
                }
                doc.push_str("\n");
            }

            match step.runner {
                StepRunnerType::SHELL => {
                    doc.push_str(&format!(
                        "```shell\n{}\n```\n\n",
                        step.content.as_ref().unwrap()
                    ));
                }
                StepRunnerType::MARKDOWN => {
                    doc.push_str(&format!("\n{}\n\n", step.content.as_ref().unwrap()));
                }
                StepRunnerType::PYTHON => {
                    doc.push_str(&format!(
                        "```python\n{}\n```\n\n",
                        step.content.as_ref().unwrap()
                    ));
                }
                StepRunnerType::LINK => {
                    doc.push_str(&format!("```\n{}\n```\n\n", step.content.as_ref().unwrap()));
                }
                StepRunnerType::COCMD => {
                    doc.push_str(&format!("```\n{}\n```\n\n", step.content.as_ref().unwrap()));
                }
            }

            doc.push_str(&format!("</details>\n\n"));
        }

        if let Some(output_file) = output_file {
            doc.push_str(GEN_MESSAGE);
            file_write(Path::new(&output_file), &doc, true).or_else(|e| {
                error!("Unable to write to file {}: {}", output_file, e.to_string());
                Err(e)
            });
        } else {
            if print_as_markdown {
                doc.push_str(GEN_MESSAGE);
                print_md(&doc);
            } else {
                print_text(&doc);
            }
        }
    }
}
