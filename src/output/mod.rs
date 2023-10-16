use termimad::MadSkin;
use tracing::debug;
use tracing::metadata::LevelFilter;
use tracing::Level;
use tracing_subscriber::{fmt, prelude::*};

// Global variable to store the log level.
static mut LOG_LEVEL: Level = Level::INFO; // Initialize with a default level.

pub fn set_tracing(verbose: bool) {
    let level = if verbose { Level::DEBUG } else { Level::INFO };

    // Set the desired log level using the env_logger crate or any other method.
    std::env::set_var("RUST_LOG", level.to_string()); // Change "info" to your desired level.

    // Initialize the tracing subscriber with the configured level.
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE) // This sets the maximum level.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    // Update the global log level variable.
    unsafe {
        LOG_LEVEL = level; // Set it to your desired level.
    }
}

pub fn print_md(markdown: &String) {
    // print with termimad to stdout
    let skin = MadSkin::default();
    skin.print_text(markdown);
}

pub fn print_md_debug(markdown: &String) {
    // get tracking log level and if it's DEBUG than print the markdown

    if unsafe { LOG_LEVEL } == LevelFilter::DEBUG {
        print_md(markdown);
    }
}
