use log::{Level, LevelFilter};
use termimad::MadSkin;

// Global variable to store the log level.
static mut LOG_LEVEL: Level = Level::Info; // Initialize with a default level.

pub fn set_logging_level(verbose: bool) {
    let level = if verbose { Level::Debug } else { Level::Info };

    std::env::set_var("RUST_LOG", "reqwest=off");

    // Set the desired log level using the env_logger crate or any other method.
    env_logger::builder()
        .filter_level(level.to_level_filter())
        .try_init();

    // Update the global log level variable.
    unsafe {
        LOG_LEVEL = level; // Set it to your desired level.
    }
}

pub fn print_md(markdown: &str) {
    // print with termimad to stdout
    let skin = MadSkin::default();
    skin.print_text(markdown);
}

pub fn print_md_debug(markdown: &str) {
    // get tracking log level and if it's DEBUG than print the markdown

    if unsafe { LOG_LEVEL } == LevelFilter::Debug {
        print_md(markdown);
    }
}

// write a test for set_logging_level function. call it with different values and check if the global variable is set correctly and that env_logger level is correct

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_logging_level() {
        set_logging_level(false);
        assert_eq!(unsafe { LOG_LEVEL }, Level::Info);

        set_logging_level(true);
        assert_eq!(unsafe { LOG_LEVEL }, Level::Debug);
    }
}
