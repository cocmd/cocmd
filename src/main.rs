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
use log::info;
use tui_app::tui_runner;

pub(crate) use crate::core::models::settings::Settings;
use crate::core::packages_manager::PackagesManager;
use crate::output::print_md;
use crate::output::set_logging_level;

/// Main CLI struct with meta-information
#[derive(Parser)]
#[command(
    author = "Moshe Roth",
    version = "1.0.78",
    about = "
    Cocmd is a CLI utility to collaborate on anything in the CMD in the community and internal teams. 
    Use it to sync Aliases, Scripts, and Workflows."
)]
struct Cli {
    /// No-Verbose flag for less output
    #[arg(short, long, default_value_t = false)]
    no_verbose: bool,

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
        #[arg(long = "raw-markdown", short = 'r', default_value_t = false)]
        raw_markdown: bool,
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
        set_logging_level(!cli.no_verbose);
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
        Commands::Docs { name, raw_markdown } => match name {
            Some(name) => {
                res = run_docs(&mut packages_manager, &name, raw_markdown);
            }
            None => {
                let markdown: String = clap_markdown::help_markdown::<Cli>();
                let md = markdown.to_string();
                if raw_markdown {
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
                let hub_provider = package_provider::hub::CocmdHubPackageProvider::new(
                    &"placeholder".to_string(),
                    &packages_manager.settings.runtime_dir,
                );
                let index = hub_provider
                    .get_index(false)
                    .expect("unable to get index from hub");

                // create with dialoguer MultiSelect, what packages the user asks to install. use index.packages.iter() and use package.name as the text
                let packages: Vec<String> = index.packages.iter().map(|p| p.name.clone()).collect();

                let selections = MultiSelect::new()
                    .items(&packages)
                    .with_prompt("What Packages to install?")
                    .interact()
                    .unwrap();

                // set packages into selected_names (Vec[String])
                selected_names = selections.iter().map(|s| packages[*s].clone()).collect();
            }
            info!("Ok, I will install: {}", selected_names.join(", "));
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
