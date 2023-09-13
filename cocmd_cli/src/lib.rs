#![allow(clippy::missing_const_for_fn)]
// #![warn(missing_docs)] // uncomment for docs

pub mod core;
mod data;
mod runner;
pub mod utils;
pub use self::core::consts;
pub use self::core::models::settings::Settings;
pub use self::runner::run;
pub use data::{CmdExit, CMD};
