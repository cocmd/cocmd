pub mod add;
pub mod docs;
pub mod profile_loader;
pub mod run;
pub mod setup;
pub mod show;

pub struct CmdExit {
    pub code: exitcode::ExitCode,
    pub message: Option<String>,
}
// use std::process::exit;

// use anyhow::Result;
// use console::Style;
// use tracing::debug;

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
