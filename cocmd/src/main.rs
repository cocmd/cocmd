#![allow(unused_must_use)]
mod cmd;
use clap::{Parser, Subcommand};
use cmd::add;
use cmd::tracing;
use cocmd::core::sources_manager::SourcesManager;
use cocmd::Settings;
use tracing::error;

use crate::cmd::profile_loader::run_profile_loader;
use crate::cmd::run::run_automation;
use crate::cmd::setup::run_setup;
use crate::cmd::show::{show_source, show_sources};

#[derive(Parser)]
#[command(author = "Your Name", version = "1.0", about = "CoCmd CLI")]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ProfileLoader,
    Refresh,
    Run { name: Option<String> },
    Show(ShowArgs),
    Add(AddArgs),
    Remove,
    Setup(SetupArgs),
}

#[derive(Parser)]
struct AddArgs {
    #[command(subcommand)]
    add_commands: AddCommands,
}

#[derive(Subcommand)]
enum AddCommands {
    Source { name: String },
}

#[derive(Parser)]
struct ShowArgs {
    #[command(subcommand)]
    show_commands: ShowCommands,
}

#[derive(Subcommand)]
enum ShowCommands {
    Source { name: String },
    Sources,
}

#[derive(Subcommand)]
enum Remove {
    Source(RemoveSourceArgs),
}

#[derive(Parser)]
struct AddSourceArgs {
    source: String,
}

#[derive(Parser)]
struct RemoveSourceArgs {
    source: String,
}

#[derive(Parser)]
struct ShowSourceArgs {
    source: String,
}

#[derive(Parser)]
struct SetupArgs {
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
        Commands::Run { name } => {
            res = run_automation(&mut sources_manager, name);
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
