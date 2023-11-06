#![allow(clippy::upper_case_acronyms)]
use serde_derive::{Deserialize, Serialize};

use crate::core::utils::sys::OS;

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
