#![allow(unused_must_use)]
mod cmd;
use clap::{Parser, Subcommand};
use clap_markdown;
use cmd::add;
use cmd::tracing;
use cocmd::core::sources_manager::SourcesManager;
use cocmd::Settings;
use tracing::error;

use crate::cmd::docs::run_docs;
use crate::cmd::profile_loader::run_profile_loader;
use crate::cmd::run::run_automation;
use crate::cmd::setup::run_setup;
#[cfg(feature = "howto")]
use crate::cmd::show::howto;
use crate::cmd::show::{show_source, show_sources};

/// Main CLI struct with meta-information
#[derive(Parser)]
#[command(
    author = "Moshe Roth",                             // Author of the CLI tool
    version = "1.1.0",                               // Version of the CLI tool
    about = "
    Cocmd is a CLI utility to collaborate on anything in the CMD in the community and internal teams. 
    Use it to sync Aliases, Scripts, and Workflows."
)]
struct Cli {
    /// Verbose flag for extra output
    #[arg(short, long)]
    verbose: bool,

    /// Subcommands
    #[command(subcommand)]
    command: Commands,
}

/// Subcommands enum with meta-information
#[derive(Subcommand)]
enum Commands {
    /// Profile Loader command - Loads profiles
    ProfileLoader,

    /// Refresh command - Refreshes something (add a description here)
    Refresh,

    /// Docs command with a name argument - Generates and displays documentation
    Docs {
        /// Optional name argument for specific documentation generation
        name: Option<String>,
    },

    /// Run command with a name argument - Runs a specific automation
    Run {
        /// Optional name argument for specifying which automation to run
        name: Option<String>,
    },
    #[cfg(feature = "howto")]
    Howto { query: String },

    /// Show command with subcommands
    Show(ShowArgs),

    /// Add command with subcommands
    Add(AddArgs),

    /// Remove command (no subcommands) - Removes something (add a description here)
    Remove,

    /// Setup command with a shell argument - Set up the CLI tool, specify shell
    Setup(SetupArgs),
}

/// Arguments for the 'add' subcommand with meta-information
#[derive(Parser)]
struct AddArgs {
    /// Subcommands for 'add' command
    #[command(subcommand)]
    add_commands: AddCommands,
}

/// Subcommands enum for 'add' with meta-information
#[derive(Subcommand)]
enum AddCommands {
    /// Source subcommand with a 'name' argument - Adds a source with a specified name
    Source {
        /// Name argument for 'add source' subcommand - Specifies the name of the source to add
        name: String,
    },
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
    /// Source subcommand with a 'name' argument - Shows information about a specific source
    Source {
        /// Name argument for 'show source' subcommand - Specifies the name of the source to show
        name: String,
    },

    /// Sources subcommand - Shows information about all sources
    Sources,
}

/// Arguments for the 'setup' subcommand with meta-information
#[derive(Parser)]
struct SetupArgs {
    /// Optional shell argument for 'setup' command - Specifies the shell to set up
    #[arg(short, long)]
    shell: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let settings = Settings::new(None, None);
    let mut sources_manager = SourcesManager::new(settings);
    tracing(cli.verbose);
    let mut res = Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    });

    match cli.command {
        Commands::Setup(args) => {
            res = run_setup(&mut sources_manager, args.shell);
        }
        Commands::ProfileLoader => {
            res = run_profile_loader(&mut sources_manager);
        }
        Commands::Refresh => {
            println!("'cocmd refresh' was used");
        }
        Commands::Docs { name } => {
            let markdown: String = clap_markdown::help_markdown::<Cli>();
            println!("{}", markdown);
            res = run_docs(&mut sources_manager, name);
        }
        Commands::Run { name } => {
            res = run_automation(&mut sources_manager, name);
        }
        #[cfg(feature = "howto")]
        Commands::Howto { query } => {
            res = howto(&mut sources_manager, query);
        }
        Commands::Show(args) => match args.show_commands {
            ShowCommands::Source { name } => {
                res = show_source(&mut sources_manager, name);
            }
            ShowCommands::Sources => {
                res = show_sources(&mut sources_manager);
            }
        },
        Commands::Add(args) => match args.add_commands {
            AddCommands::Source { name } => {
                res = add::add_source(&mut sources_manager, &name);
            }
        },
        Commands::Remove => {
            println!("'cocmd remove' was used");
        }
    }

    // if res returned an error, print it to stderr
    if let Err(e) = res {
        error!("{}", e);
    }
}
