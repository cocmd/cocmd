pub mod add;
pub mod profile_loader;
pub mod run;
pub mod setup;
pub mod show;
// use std::process::exit;

// use anyhow::Result;
// use cocmd::CmdExit;
// use console::Style;
// use tracing::debug;
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn tracing(verbose: bool) {
    let level = if verbose {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };
    Registry::default()
        .with(tracing_tree::HierarchicalLayer::new(2))
        .with(
            EnvFilter::builder()
                .with_default_directive(level.into())
                .with_env_var("LOG")
                .from_env_lossy(),
        )
        .init();
}

// const DEFAULT_ERR_EXIT_CODE: i32 = 1;
// pub fn result_exit(res: Result<CmdExit>) {
//     let exit_with = match res {
//         Ok(cmd) => {
//             if let Some(message) = cmd.message {
//                 let style = if exitcode::is_success(cmd.code) {
//                     Style::new().green()
//                 } else {
//                     Style::new().red()
//                 };
//                 eprintln!("{}", style.apply_to(message));
//             }
//             cmd.code
//         }
//         Err(e) => {
//             debug!("{:?}", e);
//             DEFAULT_ERR_EXIT_CODE
//         }
//     };
//     exit(exit_with)
// }
