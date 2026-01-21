use clap::{Parser, Subcommand, ValueEnum};

mod commands;
mod error;
mod fetch;
mod install;
mod integrate;
mod manifest;
mod ui;

use error::Result;

#[derive(Parser)]
#[command(name = "bottle")]
#[command(author = "Open Horizon Labs")]
#[command(version)]
#[command(about = "Curated snapshot manager for the Open Horizon Labs tool stack")]
#[command(long_about = "Bottle provides one-command installation, coherent versioning, and seamless updates \
for users who want a batteries-included experience with the Open Horizon Labs tools.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a bottle (stable, edge)
    Install {
        /// Bottle name to install
        #[arg(default_value = "stable")]
        bottle: String,

        /// Path to a local manifest file (overrides bottle name lookup)
        #[arg(long, value_name = "PATH")]
        manifest: Option<std::path::PathBuf>,

        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,

        /// Show what would be done without making changes
        #[arg(long)]
        dry_run: bool,

        /// Reinstall even if already installed
        #[arg(short, long)]
        force: bool,
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

    /// Add or remove platform integrations (Claude Code, OpenCode, Codex)
    Integrate {
        /// Platform to integrate: claude_code, opencode, codex
        #[arg(value_enum)]
        platform: Option<PlatformArg>,

        /// Path to a local manifest file (for project-local bottles)
        #[arg(long, value_name = "PATH")]
        manifest: Option<std::path::PathBuf>,

        /// List available and installed integrations
        #[arg(short, long)]
        list: bool,

        /// Remove the integration instead of adding it
        #[arg(short, long)]
        remove: bool,

        /// Show what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },

    /// List available bottles (curated and bespoke)
    List,

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

    /// Create a new bespoke bottle
    Create {
        /// Name for the new bottle
        name: String,

        /// Copy manifest from an existing bottle (curated or bespoke)
        #[arg(long)]
        from: Option<String>,
    },

    /// Output AGENTS.md snippet for the active bottle
    #[command(name = "agents-md")]
    AgentsMd,
}

/// Platform integration targets
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PlatformArg {
    /// Claude Code plugin (detected via ~/.claude/)
    #[value(name = "claude_code")]
    ClaudeCode,
    /// OpenCode plugin (detected via opencode.json)
    #[value(name = "opencode")]
    OpenCode,
    /// Codex skill (detected via ~/.codex/)
    #[value(name = "codex")]
    Codex,
}

impl PlatformArg {
    /// Convert to the internal Platform type
    pub fn to_platform(self) -> integrate::Platform {
        match self {
            PlatformArg::ClaudeCode => integrate::Platform::ClaudeCode,
            PlatformArg::OpenCode => integrate::Platform::OpenCode,
            PlatformArg::Codex => integrate::Platform::Codex,
        }
    }
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
        Commands::Install { bottle, manifest, yes, dry_run, force } => commands::install::run(&bottle, manifest.as_deref(), yes, dry_run, force),
        Commands::Status { check_updates } => commands::status::run(check_updates),
        Commands::Update { yes } => commands::update::run(yes),
        Commands::Switch { bottle, yes } => commands::switch::run(&bottle, yes),
        Commands::Eject { yes } => commands::eject::run(yes),
        Commands::Integrate {
            platform,
            manifest,
            list,
            remove,
            dry_run,
        } => commands::integrate::run(platform.map(|p| p.to_platform()), manifest.as_deref(), list, remove, dry_run),
        Commands::List => commands::list::run(),
        Commands::Diff { from, to } => commands::diff::run(&from, &to),
        Commands::Upgrade {
            bottle,
            tool,
            version,
        } => commands::upgrade::run(&bottle, &tool, &version),
        Commands::Validate { bottle } => commands::validate::run(&bottle),
        Commands::Release { bottle, message } => commands::release::run(&bottle, message.as_deref()),
        Commands::Create { name, from } => commands::create::run(&name, from.as_deref()),
        Commands::AgentsMd => commands::agents_md::run(),
    }
}
