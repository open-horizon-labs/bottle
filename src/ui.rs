use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use indicatif::{ProgressBar, ProgressStyle};

use crate::error::BottleError;

/// Print an error message
pub fn print_error(error: &BottleError) {
    eprintln!("{} {}", style("error:").red().bold(), error);
}

/// Print a success message
pub fn print_success(message: &str) {
    println!("{} {}", style("✓").green().bold(), message);
}

/// Print an info message
pub fn print_info(message: &str) {
    println!("{} {}", style("•").blue(), message);
}

/// Print a warning message
pub fn print_warning(message: &str) {
    println!("{} {}", style("!").yellow().bold(), message);
}

/// Print a header
pub fn print_header(message: &str) {
    println!("\n{}\n", style(message).bold());
}

/// Ask for confirmation
pub fn confirm(message: &str, default: bool) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .default(default)
        .interact()
        .unwrap_or(default)
}

/// Select from a list of options
pub fn select(prompt: &str, options: &[&str]) -> Option<usize> {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(options)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .ok()
        .flatten()
}

/// Create a progress bar for installations
pub fn progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:.bold.dim} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .expect("Invalid progress bar template")
            .progress_chars("━━─"),
    );
    pb
}

/// Create a spinner for indeterminate operations
pub fn spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .expect("Invalid spinner template"),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb
}

/// Print tool status line
pub fn print_tool_status(name: &str, version: &str, installed: bool) {
    let status = if installed {
        style("✓").green()
    } else {
        style("✗").red()
    };
    println!("  {:<12} {:<8} {}", name, version, status);
}

/// Print bottle header
pub fn print_bottle_header(name: &str, version: &str) {
    println!(
        "{}: {} ({})",
        style("Bottle").bold(),
        style(name).cyan(),
        version
    );
    println!();
}
