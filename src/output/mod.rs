use termimad::MadSkin;
use log::{Level, LevelFilter};

// Global variable to store the log level.
static mut LOG_LEVEL: Level = Level::Info; // Initialize with a default level.

pub fn set_tracing(verbose: bool) {
    let level = if verbose { Level::Debug } else { Level::Info };

    // Set the desired log level using the env_logger crate or any other method.
    env_logger::builder()
        .filter_level(level.to_level_filter())
        .init();

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

    if unsafe { LOG_LEVEL } == LevelFilter::Debug {
        print_md(markdown);
    }
}
