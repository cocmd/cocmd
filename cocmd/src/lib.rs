#![allow(clippy::missing_const_for_fn)]
// #![warn(missing_docs)] // uncomment for docs

pub mod core;
pub mod utils;

pub use self::core::consts;
pub use self::core::models::cmd_exit::CmdExit;
pub use self::core::models::settings::Settings;
