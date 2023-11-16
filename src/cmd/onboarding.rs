use std::path::Path;

use anyhow::{bail, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use log::{error, info};
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

use crate::core::packages_manager::PackagesManager;

pub async fn run_onboarding(_packages_manager: &mut PackagesManager) -> Result<()> {
    let ollama = Ollama::default();

    loop {
        let model = "choose-a-model-name".to_string();
        let prompt = "Show me a new fact about python programming".to_string();
        let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

        let answer = res.unwrap().response;
        // if its not containing the word "function" try again
        if !answer.contains("function") {
            continue;
        } else {
            println!("{}", answer);
            break;
        }
    }

    Ok(())
}
