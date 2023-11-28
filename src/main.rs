#![allow(unused_must_use)]
#![allow(clippy::missing_const_for_fn)]
pub(crate) mod cmd;
pub(crate) mod core;
pub(crate) mod output;
pub(crate) mod package_provider;
pub(crate) mod runner;
pub(crate) mod tui_app;
use std::process::ExitCode;

use anyhow::Error;
use clap::{Parser, Subcommand};
use cmd::add;
use cmd::docs::run_docs;
use cmd::profile_loader::run_profile_loader;
use cmd::run::run_automation;
use cmd::setup::run_setup;
#[cfg(feature = "howto")]
use cmd::show::howto;
use cmd::show::{show_package, show_packages};
use cmd::uninstall::uninstall_package;
use dialoguer::{Confirm, MultiSelect};
use itertools::Itertools;
use log::trace;
use tui_app::tui_runner;

pub(crate) use crate::core::models::settings::Settings;
use crate::core::packages_manager::PackagesManager;
use crate::output::print_md;
use crate::output::set_logging_level;

/// Main CLI struct with meta-information
#[derive(Parser)]
#[command(
    author = "Moshe Roth",
    version = "1.0.82",
    about = "
    Cocmd is a CLI utility to collaborate on anything in the CMD in the community and internal teams. 
    Use it to sync Aliases, Scripts, and Workflows."
)]
struct Cli {
    /// No-Verbose flag for less output
    #[arg(short, long, default_value_t = false)]
    silent: bool,

    /// Subcommands
    #[command(subcommand)]
    command: Commands,
}

/// Subcommands enum with meta-information
#[derive(Subcommand)]
enum Commands {
    /// Terminal UI for browsing and running automations
    Browse,

    /// Profile Loader command - Loads profiles
    ProfileLoader,

    /// Refresh command - Refreshes something (add a description here)
    Refresh,

    /// Docs command with a name argument - Generates and displays documentation
    Docs {
        /// Optional name argument for specific documentation generation
        name: Option<String>,
        /// Optional flag to show raw markdown
        #[arg(long = "raw", short = 'r', default_value_t = false)]
        raw: bool,

        #[arg(long = "output", short = 'o')]
        output: Option<String>,
    },

    /// Run command with a name argument - Runs a specific automation
    Run {
        /// Optional name argument for specifying which automation to run
        name: Option<String>,

        /// Optional argument for input parameters
        #[arg(long = "param", short = 'p')]
        params: Vec<String>,

        /// Optional argument to specify the source of the playbook
        #[arg(short, long)]
        from: Option<String>,
    },
    #[cfg(feature = "howto")]
    Howto { query: String },

    /// Show command with subcommands
    Show(ShowArgs),

    /// Install command with subcommands
    Install {
        /// Name argument for 'install' - Specifies the name of the package to add
        names: Option<Vec<String>>,

        #[arg(long = "yes", short = 'y', default_value_t = false)]
        dont_ask: bool,
    },

    /// Uninstall command with a package name argument - Uninstalls a specific package
    Uninstall {
        /// Name argument for 'uninstall' - Specifies the name of the package to uninstall
        name: String,
    },
    /// Remove command (no subcommands) - Removes something (add a description here)
    Remove,

    /// Setup command with a shell argument - Set up the CLI tool, specify shell
    Setup(SetupArgs),
}

/// Arguments for the 'show' subcommand with meta-information
#[derive(Parser)]
struct ShowArgs {
    /// Subcommands for 'show' command
    #[command(subcommand)]
    show_commands: ShowCommands,
}

/// Subcommands enum for 'show' with meta-information
#[derive(Subcommand)]
enum ShowCommands {
    /// Package subcommand with a 'name' argument - Shows information about a specific package
    Package {
        /// Name argument for 'show package' subcommand - Specifies the name of the package to show
        name: String,
    },

    /// Packages subcommand - Shows information about all packages
    Packages,
}

/// Arguments for the 'setup' subcommand with meta-information
#[derive(Parser)]
struct SetupArgs {
    /// Optional shell argument for 'setup' command - Specifies the shell to set up
    #[arg(short, long)]
    shell: Option<String>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    if let Commands::Install { .. } = cli.command {
        set_logging_level(false);
    } else {
        set_logging_level(!cli.silent);
    }

    let settings = Settings::new(None, None);
    let mut packages_manager = PackagesManager::new(settings);

    let mut res: Result<(), Error> = Ok(());

    match cli.command {
        Commands::Browse => {
            while let Ok(Some(automation_name)) = tui_runner(packages_manager.clone()) {
                res = run_automation(&mut packages_manager, Some(automation_name), None, None);
                if Confirm::new()
                    .with_prompt("Do you want to continue browsing?")
                    .interact()
                    .unwrap()
                {
                    continue;
                } else {
                    break;
                }
            }
        }
        Commands::Setup(args) => {
            res = run_setup(&mut packages_manager, args.shell);
        }
        Commands::Uninstall { name } => {
            res = uninstall_package(&mut packages_manager, &name);
        }
        Commands::ProfileLoader => {
            res = run_profile_loader(&mut packages_manager);
        }
        Commands::Refresh => {
            todo!("Refresh command not implemented yet")
        }
        Commands::Docs { name, raw, output } => match name {
            Some(name) => {
                res = run_docs(&mut packages_manager, &name, raw, output);
            }
            None => {
                let markdown: String = clap_markdown::help_markdown::<Cli>();
                let md = markdown.to_string();
                if raw {
                    println!("{}", md);
                } else {
                    print_md(&md);
                }
            }
        },
        Commands::Run { name, params, from } => {
            res = run_automation(&mut packages_manager, name, Some(params), from);
        }
        #[cfg(feature = "howto")]
        Commands::Howto { query } => {
            res = howto(&mut packages_manager, query);
        }
        Commands::Show(args) => match args.show_commands {
            ShowCommands::Package { name } => {
                res = show_package(&mut packages_manager, name);
            }
            ShowCommands::Packages => {
                res = show_packages(&mut packages_manager);
            }
        },
        Commands::Install { names, dont_ask } => {
            let selected_names;
            if let Some(names) = names {
                selected_names = names.clone();
            } else {
                let index = package_provider::hub::CocmdHubPackageProvider::get_index(
                    &packages_manager.settings.runtime_dir,
                    false,
                )
                .expect("unable to get index from hub");

                // create with dialoguer MultiSelect, what packages the user asks to install. use index.packages.iter() and use package.name as the text
                let packages: Vec<String> = index
                    .packages
                    .iter()
                    .map(|p| p.name.clone())
                    .unique()
                    .sorted()
                    .collect();

                let selections = MultiSelect::new()
                    .items(&packages)
                    .with_prompt("What Packages to install?")
                    .interact()
                    .unwrap();

                // set packages into selected_names (Vec[String])
                selected_names = selections.iter().map(|s| packages[*s].clone()).collect();
            }
            trace!("Ok, I will install: {}", selected_names.join(", "));
            for name in selected_names {
                res = add::install_package(&mut packages_manager, &name, dont_ask);
            }
        }
        Commands::Remove => {
            println!("'cocmd remove' was used");
        }
    }

    // if res returned an error, print it to stderr
    if let Err(_e) = res {
        // error!("{}", e);
        ExitCode::from(1)
    } else {
        ExitCode::from(0)
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use maplit::hashmap;
    use temp_testdir::TempDir;

    use super::*;
    use crate::core::{consts, utils::io::to_yaml_file};

    #[test]
    fn test_install_latest_package() {
        let tmp_home_dir = TempDir::default();
        let mut packages_manager = PackagesManager::new(Settings::new(tmp_home_dir.to_str(), None));
        let res = add::install_package(&mut packages_manager, "aws-s3", true);
        assert!(res.is_ok());

        // get the latest version from index and make sure this is the one we download
        let package = packages_manager.get_package("aws-s3".to_string());
        assert!(package.is_some());

        let index = package_provider::hub::CocmdHubPackageProvider::get_index(
            &packages_manager.settings.runtime_dir,
            false,
        )
        .expect("unable to get index from hub");

        let latest_version = index
            .get_package("aws-s3", &None::<String>)
            .unwrap()
            .version
            .clone();

        assert_eq!(latest_version, package.unwrap().version());
    }

    #[test]
    fn test_install_specific_package() {
        let tmp_home_dir = TempDir::default();
        let mut packages_manager = PackagesManager::new(Settings::new(tmp_home_dir.to_str(), None));
        let res = add::install_package(&mut packages_manager, "aws-s3@0.0.0", true);
        assert!(res.is_ok());

        let package = packages_manager.get_package("aws-s3".to_string());
        assert!(package.is_some());
        assert_eq!("0.0.0", package.unwrap().version());
    }

    #[test]
    fn test_show_package() {
        let tmp_home_dir = TempDir::default();
        let mut packages_manager = PackagesManager::new(Settings::new(tmp_home_dir.to_str(), None));
        let res = add::install_package(&mut packages_manager, "docker", true);
        assert!(res.is_ok());

        let res = show_package(&mut packages_manager, "docker".to_string());
        assert!(res.is_ok());
    }

    #[test]
    fn test_show_package_w_version() {
        let tmp_home_dir = TempDir::default();
        let mut packages_manager = PackagesManager::new(Settings::new(tmp_home_dir.to_str(), None));
        let res = add::install_package(&mut packages_manager, "docker@0.0.0", true);
        assert!(res.is_ok());

        let res = show_package(&mut packages_manager, "docker".to_string());
        assert!(res.is_ok());
    }

    #[test]
    fn test_show_packages() {
        let tmp_home_dir = TempDir::default();
        let mut packages_manager = PackagesManager::new(Settings::new(tmp_home_dir.to_str(), None));
        let res = add::install_package(&mut packages_manager, "docker", true);
        assert!(res.is_ok());

        let res = show_packages(&mut packages_manager);
        assert!(res.is_ok());
    }

    // write a test where we install aws-s3@0.0.0 and aws-s3@0.0.1
    // and we show packages and make sure both are shown
    // then we uninstall aws-s3@0.0.0
    // and we show packages and make sure only aws-s3@0.0.1
    // is shown
    #[test]
    fn test_show_packages_after_uninstall() {
        let tmp_home_dir = TempDir::default();
        let mut packages_manager = PackagesManager::new(Settings::new(tmp_home_dir.to_str(), None));
        let res = add::install_package(&mut packages_manager, "aws-s3@0.0.0", true);
        assert!(res.is_ok());
        let package = packages_manager.get_package("aws-s3".to_string());
        assert!(package.is_some());
        assert_eq!("0.0.0", package.unwrap().version());

        let res = add::install_package(&mut packages_manager, "aws-s3@0.0.1", true);
        assert!(res.is_ok());

        // use packages_manager.get_package("aws-s3".to_string()) to make sure.
        let binding = packages_manager.clone();
        let package = binding.get_package("aws-s3".to_string());
        assert!(package.is_some());
        assert_eq!("0.0.1", package.unwrap().version());

        let package_path = package.unwrap().location();

        let res = uninstall_package(&mut packages_manager, "aws-s3");
        assert!(res.is_ok());

        let res = show_packages(&mut packages_manager);
        assert!(res.is_ok());

        let package = packages_manager.get_package("aws-s3".to_string());
        assert!(package.is_none());

        // make sure package_path doesn't exist
        assert!(!package_path.exists());
    }

    // write a test for checking that when a latest version is installed
    #[test]
    fn test_install_latest_version_after_old() {
        let tmp_home_dir = TempDir::default();
        let mut packages_manager = PackagesManager::new(Settings::new(tmp_home_dir.to_str(), None));
        let res = add::install_package(&mut packages_manager, "aws-s3@0.0.0", true);
        assert!(res.is_ok());

        let package = packages_manager.get_package("aws-s3".to_string());
        assert!(package.is_some());
        assert_eq!("0.0.0", package.unwrap().version());

        let res = add::install_package(&mut packages_manager, "aws-s3", true);
        assert!(res.is_ok());

        // use packages_manager.get_package("aws-s3".to_string()) to make sure.
        let package = packages_manager.get_package("aws-s3".to_string());
        assert!(package.is_some());
        assert_eq!("0.0.1", package.unwrap().version());
    }

    // write a test that simulate two packages intsalled with the same name,
    // one of them will be local and the other will be from hub
    // this test will generate the cocmd.yaml file with the same name as the
    // package from the hub
    // then, call uninstall package and make sure both doesn't apear as a package
    // in cocmd
    #[test]
    fn test_uninstall_package_after_double_installation() {
        let tmp_home_dir = TempDir::default();
        let mut packages_manager = PackagesManager::new(Settings::new(tmp_home_dir.to_str(), None));

        // generate a local path in {tmp_home_dir}/local_path
        // generate a cocmd.yaml with "name: aws-s3" inside
        // and call install_package with this path
        let local_path = tmp_home_dir.join("local_path");
        fs::create_dir_all(&local_path).unwrap();

        // write name: aws-s3 to the file &local_path.join(&consts::SOURCE_CONFIG_FILE
        // use io to_yaml_file

        to_yaml_file(
            &hashmap! {
                "name" => "aws-s3",
                "version" => "0.0.1",
            },
            &local_path.join(consts::SOURCE_CONFIG_FILE),
        )
        .unwrap();

        let res = add::install_package(
            &mut packages_manager,
            local_path.to_string_lossy().as_ref(),
            true,
        );
        assert!(res.is_ok());

        let package = packages_manager.get_package("aws-s3".to_string());
        assert!(package.is_some());
        assert_eq!(&local_path, package.unwrap().location());

        let res = add::install_package(&mut packages_manager, "aws-s3", true);
        assert!(res.is_ok());

        let res = uninstall_package(&mut packages_manager, "aws-s3");
        assert!(res.is_ok());

        let res = show_packages(&mut packages_manager);
        assert!(res.is_ok());

        let package = packages_manager.get_package("aws-s3".to_string());
        assert!(package.is_none());
    }
}
