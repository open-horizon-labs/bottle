use clap::{Parser, Subcommand};

mod commands;
mod error;
mod fetch;
mod install;
mod manifest;
mod ui;

use error::Result;

#[derive(Parser)]
#[command(name = "bottle")]
#[command(author = "Cloud Atlas AI")]
#[command(version)]
#[command(about = "Curated snapshot manager for the Cloud Atlas AI tool stack")]
#[command(long_about = "Bottle provides one-command installation, coherent versioning, and seamless updates \
for users who want a batteries-included experience with the Cloud Atlas AI tools.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a bottle (stable, edge, minimal)
    Install {
        /// Bottle name to install
        #[arg(default_value = "stable")]
        bottle: String,

        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Show current bottle status and installed tools
    Status {
        /// Check for available updates
        #[arg(short, long)]
        check_updates: bool,
    },

    /// Update to the latest bottle snapshot
    Update {
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Switch to a different bottle
    Switch {
        /// Bottle name to switch to
        bottle: String,

        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Eject from bottle management (keep tools, manage manually)
    Eject {
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Compare bottles or check for updates (curator command)
    Diff {
        /// First bottle or 'latest' for latest tool versions
        from: String,

        /// Second bottle to compare against
        #[arg(default_value = "latest")]
        to: String,
    },

    /// Bump a tool version in a bottle manifest (curator command)
    Upgrade {
        /// Bottle to upgrade
        bottle: String,

        /// Tool to upgrade
        tool: String,

        /// New version
        version: String,
    },

    /// Validate a bottle manifest (curator command)
    Validate {
        /// Bottle to validate
        bottle: String,
    },

    /// Tag and publish a bottle update (curator command)
    Release {
        /// Bottle to release
        bottle: String,

        /// Release message
        #[arg(short, long)]
        message: Option<String>,
    },
}

fn main() {
    if let Err(e) = run() {
        ui::print_error(&e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { bottle, yes } => commands::install::run(&bottle, yes),
        Commands::Status { check_updates } => commands::status::run(check_updates),
        Commands::Update { yes } => commands::update::run(yes),
        Commands::Switch { bottle, yes } => commands::switch::run(&bottle, yes),
        Commands::Eject { yes } => commands::eject::run(yes),
        Commands::Diff { from, to } => commands::diff::run(&from, &to),
        Commands::Upgrade {
            bottle,
            tool,
            version,
        } => commands::upgrade::run(&bottle, &tool, &version),
        Commands::Validate { bottle } => commands::validate::run(&bottle),
        Commands::Release { bottle, message } => commands::release::run(&bottle, message.as_deref()),
    }
}
