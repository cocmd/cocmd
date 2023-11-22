use anyhow::{Context, Result};
use chatgpt::{prelude::*, types::CompletionResponse};
use chatgpt_functions::{chat_gpt::ChatGPTBuilder, function_specification::FunctionSpecification};
use dotenv::dotenv;
use log::error;

use crate::core::packages_manager::PackagesManager;

pub async fn run_onboarding(_packages_manager: &mut PackagesManager) -> Result<()> {
    dotenv().ok();
    let key = std::env::var("OPENAI_API_KEY")?;

    let client = ChatGPT::new(key)?;

    /// Sending a message and getting the completion
    let response = client
        .send_message("Describe in five words the Rust programming language.")
        .await;

    if let Err(err) = response {
        error!("Error: {}", err);
        return Err(err.into());
    }

    let response: CompletionResponse = response.unwrap();

    println!("Response: {}", response.message().content);

    Ok(())
}
