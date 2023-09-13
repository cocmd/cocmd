#![allow(clippy::missing_const_for_fn)]
// #![warn(missing_docs)] // uncomment for docs

mod data;
mod runner;
pub mod core;
pub mod utils;
pub use data::{CmdExit, CMD};
pub use self::core::consts;
pub use self::runner::run;
pub use self::core::models::settings::Settings;
