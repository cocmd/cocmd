mod cmd;
use clap::{Parser, Subcommand};
use cmd::add;
use cmd::tracing;
use cocmd::core::sources_manager::SourcesManager;
use cocmd::Settings;

use crate::cmd::profile_loader::run_profile_loader;
use crate::cmd::run::run_automation;
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

fn main() {
    let cli = Cli::parse();

    let settings = Settings::new(None, None);
    let mut sources_manager = SourcesManager::new(settings);
    tracing(cli.verbose);

    match cli.command {
        Commands::ProfileLoader => {
            let _ = run_profile_loader(&mut sources_manager);
        }
        Commands::Refresh => {
            println!("'cocmd refresh' was used");
        }
        Commands::Run { name } => {
            let _ = run_automation(&mut sources_manager, name);
        }
        Commands::Show(args) => match args.show_commands {
            ShowCommands::Source { name } => {
                show_source(&mut sources_manager, name);
            }
            ShowCommands::Sources => {
                show_sources(&mut sources_manager);
            }
        },
        Commands::Add(args) => match args.add_commands {
            AddCommands::Source { name } => {
                add::add_source(&mut sources_manager, &name);
            }
        },
        Commands::Remove => {
            println!("'cocmd remove' was used");
        }
    }
}
