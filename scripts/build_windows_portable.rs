//! ```cargo
//! [dependencies]
//! glob = "0.3.0"
//! envmnt = "*"
//! fs_extra = "1.2.0"
//! ```

use std::path::PathBuf;

const TARGET_DIR: &str = "target/windows/portable";
const RESOURCE_DIR: &str = "target/windows/repackages";

fn main() {
    // Clean the target directory
    let _ = std::fs::remove_dir_all(TARGET_DIR);
    // Create the target directory
    std::fs::create_dir_all(TARGET_DIR).expect("unable to create target directory");
    let target_dir = PathBuf::from(TARGET_DIR);
    if !target_dir.is_dir() {
        panic!("expected target directory, found none");
    }

    let repackages_dir = PathBuf::from(RESOURCE_DIR);
    if !repackages_dir.is_dir() {
        panic!("expected repackages dir, found none");
    }

    // Copy all the repackages
    fs_extra::dir::copy(
        &repackages_dir,
        &target_dir,
        &fs_extra::dir::CopyOptions {
            content_only: true,
            ..Default::default()
        },
    )
    .expect("unable to copy repackages");

    // Create the launcher
    std::fs::write(
        target_dir.join("START_COCMD.bat"),
        r#"start cocmdd.exe launcher"#,
    )
    .unwrap();

    // Create the necessary folders
    std::fs::create_dir_all(target_dir.join(".cocmd")).expect("unable to create data directory");
    std::fs::create_dir_all(target_dir.join(".cocmd-runtime"))
        .expect("unable to create runtime directory");

    std::fs::write(
        target_dir.join("README.txt"),
        r##"Welcome to Cocmd (Portable edition)!

To start cocmd, you can double click on "START_COCMD.bat"  

After the first run, you will see some files in the ".cocmd" directory.
This is where your snippets and configurations should be defined.

For more information, please visit the official documentation: 
https://cocmd.org/docs/

IMPORTANT: Don't delete any file or directory, otherwise cocmd won't work.


FOR ADVANCED USERS:  

Cocmd also offers a rich CLI interface. To start it from the terminal, cd into the 
current directory and run "cocmd start". You can also run "cocmd --help" for more information.

You might have noticed that the directory contains both an "cocmdd.exe" and an "cocmd.cmd" file.
You should generally avoid running "cocmdd.exe" directly, and instead use the "cocmd.cmd"
wrapper (which can simply be run as "cocmd" in the terminal). This is needed to correctly manage
STD console handles on Windows.
  "##,
    )
    .unwrap();
}
