use super::common::{check_prerequisites, fetch_or_load_manifest};
use crate::error::{BottleError, Result};
use crate::fetch::fetch_tool_definition;
use crate::install;
use crate::manifest::bottle::BottleManifest;
use crate::manifest::state::{BottleState, CustomInstallMethod, CustomToolState, Mode, ToolState};
use crate::ui;
use chrono::Utc;
use console::style;
use std::collections::HashMap;
use std::process::Command;

/// Install a bottle (stable, edge, or bespoke)
pub fn run(
    bottle: &str,
    manifest_path: Option<&std::path::Path>,
    yes: bool,
    dry_run: bool,
    force: bool,
) -> Result<()> {
    // 1. Check if already installed (skip if --force or using explicit manifest)
    if !force && manifest_path.is_none() {
        if let Some(state) = BottleState::load() {
            if state.bottle == bottle && state.is_managed() {
                ui::print_warning(&format!(
                    "Bottle '{}' is already installed. Use 'bottle update' to refresh, or --force to reinstall.",
                    bottle
                ));
                return Ok(());
            }
            // Different bottle - this is a switch, not install
            return Err(BottleError::Other(format!(
                "Bottle '{}' is currently installed. Use 'bottle switch {}' to change bottles.",
                state.bottle, bottle
            )));
        }
    }

    // 2. Fetch manifest (explicit path, local bespoke, or remote curated)
    let spinner = ui::spinner("Fetching bottle manifest...");
    let manifest = fetch_or_load_manifest(bottle, manifest_path)?;
    spinner.finish_and_clear();

    // 3. Check prerequisites
    check_prerequisites(&manifest)?;

    // 4. Show what will be installed (or would be installed for dry-run)
    if dry_run {
        show_dry_run_plan(&manifest);
        return Ok(());
    }

    show_install_plan(&manifest);

    // 5. Confirm (unless -y)
    if !yes && !ui::confirm("Proceed with installation?", true) {
        return Err(BottleError::Cancelled);
    }

    println!();

    // 6. Install tools (binaries + MCP)
    let tool_states = install_tools(&manifest)?;

    // 7. Install bespoke MCP servers (if any)
    install_mcp_servers(&manifest)?;

    // 8. Build AGENTS.md snippet (agent applies it later)
    let snippet = save_agents_md_snippet(&manifest)?;

    // 9. Install custom tools (if any)
    let custom_tool_states = install_custom_tools(&manifest)?;

    // 10. Write state (includes snippet if present)
    let state = BottleState {
        bottle: manifest.name.clone(),
        bottle_version: manifest.version.clone(),
        installed_at: Utc::now(),
        tools: tool_states,
        mode: Mode::Managed,
        integrations: HashMap::new(),
        custom_tools: custom_tool_states,
    };
    state
        .save()
        .map_err(|e| BottleError::Other(format!("Failed to save state: {}", e)))?;

    // Save snippet alongside state if present
    if let Some(snippet_content) = &snippet {
        state
            .save_snippet(snippet_content)
            .map_err(|e| BottleError::Other(format!("Failed to save AGENTS.md snippet: {}", e)))?;
    }

    // 11. Show success
    show_success(&manifest);

    Ok(())
}

/// Display the installation plan
fn show_install_plan(manifest: &BottleManifest) {
    println!();
    println!(
        "{} {} ({})",
        style("Installing bottle:").bold(),
        style(&manifest.name).cyan(),
        &manifest.version
    );
    println!("{}", style(&manifest.description).dim());
    println!();

    // Show tools
    println!("{}:", style("Tools").bold());
    let mut tools: Vec<_> = manifest.tools.iter().collect();
    tools.sort_by_key(|(name, _)| *name);
    for (name, version) in &tools {
        println!("  {:<12} {}", name, style(version).dim());
    }
    println!();
}

/// Display the dry-run plan showing what would be installed
fn show_dry_run_plan(manifest: &BottleManifest) {
    println!();
    println!("{}", style("[DRY RUN]").yellow().bold());
    println!(
        "Would install bottle {} ({}):",
        style(&manifest.name).cyan(),
        &manifest.version
    );
    println!("{}", style(&manifest.description).dim());
    println!();

    // Show tools with installation status
    println!("{}:", style("Tools").bold());
    let mut tools: Vec<_> = manifest.tools.iter().collect();
    tools.sort_by_key(|(name, _)| *name);
    for (name, target_version) in &tools {
        let current = get_tool_version(name);
        match current {
            Some(installed_ver) if installed_ver == **target_version => {
                println!(
                    "  {:<12} {} {}",
                    name,
                    style("current").green(),
                    style(target_version).dim()
                );
            }
            Some(installed_ver) => {
                let arrow = match compare_versions(&installed_ver, target_version) {
                    "upgrade" => style("↑").green(),
                    "downgrade" => style("↓").red(),
                    _ => style("→").yellow(),
                };
                println!(
                    "  {:<12} {} {} {}",
                    name,
                    arrow,
                    style(&installed_ver).dim(),
                    target_version
                );
            }
            None => {
                println!(
                    "  {:<12} {} {}",
                    name,
                    style("install").yellow(),
                    target_version
                );
            }
        }
    }
    println!();

    // Show plugins available via integrate
    if !manifest.plugins.is_empty() {
        println!(
            "{} {}:",
            style("Plugins").bold(),
            style("(via bottle integrate)").dim()
        );
        for plugin in &manifest.plugins {
            println!("  {}", plugin);
        }
        println!();
    }

    // Show bespoke MCP servers
    if !manifest.mcp_servers.is_empty() {
        println!("{}:", style("MCP Servers").bold());
        let mut servers: Vec<_> = manifest.mcp_servers.iter().collect();
        servers.sort_by_key(|(name, _)| *name);
        for (name, server) in servers {
            let args_str = if server.args.is_empty() {
                String::new()
            } else {
                format!(" {}", server.args.join(" "))
            };
            println!(
                "  {:<20} {} {}{}",
                name,
                style(&server.command).dim(),
                style(format!("[{}]", server.scope)).dim(),
                style(&args_str).dim()
            );
            // Show env vars that need to be set
            for (key, value) in &server.env {
                if value.contains("${") {
                    println!(
                        "    {} {}",
                        style("env:").dim(),
                        style(format!("{}={}", key, value)).yellow()
                    );
                }
            }
        }
        println!();
    }

    // Show AGENTS.md snippet info
    if let Some(agents_config) = &manifest.agents_md {
        if !agents_config.sections.is_empty() || agents_config.snippets_url.is_some() {
            println!("{}:", style("AGENTS.md Snippet").bold());
            println!("  {}", style("(saved for agent to apply)").dim());
            for section in &agents_config.sections {
                println!("  {} {}", style("Section:").dim(), &section.heading);
            }
            if let Some(url) = &agents_config.snippets_url {
                println!("  {} {}", style("Snippets URL:").dim(), style(url).cyan());
            }
            println!(
                "  {} ~/.bottle/bottles/{}/agents-md-snippet",
                style("Saved to:").dim(),
                manifest.name
            );
            println!();
        }
    }

    // Show custom tools
    if !manifest.custom_tools.is_empty() {
        println!("{}:", style("Custom Tools").bold());
        let mut tools: Vec<_> = manifest.custom_tools.iter().collect();
        tools.sort_by_key(|(name, _)| *name);
        for (name, tool) in tools {
            let methods: Vec<&str> = [
                tool.install.brew.as_ref().map(|_| "brew"),
                tool.install.cargo.as_ref().map(|_| "cargo"),
                tool.install.npm.as_ref().map(|_| "npm"),
                tool.install.binary_url.as_ref().map(|_| "binary"),
            ]
            .into_iter()
            .flatten()
            .collect();

            println!(
                "  {:<20} {} {}",
                name,
                style(&tool.version).dim(),
                style(format!("[{}]", methods.join(", "))).dim()
            );
            if let Some(verify) = &tool.verify {
                println!("    {} {}", style("verify:").dim(), style(verify).dim());
            }
        }
        println!();
    }

    // Show detected platforms for integration
    println!(
        "{} {}:",
        style("Platform Integrations").bold(),
        style("(optional, run after install)").dim()
    );
    println!();
    show_claude_code_integration();
    show_opencode_integration();
    show_codex_integration();
    println!();

    // Show state changes
    println!("{}:", style("State changes").bold());
    println!(
        "  Create ~/.bottle/bottles/{}/state.json with:",
        manifest.name
    );
    println!("    bottle: {}", manifest.name);
    println!("    version: {}", manifest.version);
    println!("    mode: managed");
    println!("  Set active bottle: ~/.bottle/active → {}", manifest.name);
    if manifest
        .agents_md
        .as_ref()
        .map(|a| !a.sections.is_empty() || a.snippets_url.is_some())
        .unwrap_or(false)
    {
        println!(
            "  Save AGENTS.md snippet: ~/.bottle/bottles/{}/agents-md-snippet",
            manifest.name
        );
    }
    println!();

    println!("{}", style("No changes made.").dim());
    println!();
}

fn show_claude_code_integration() {
    let detected = crate::integrate::claude_code::is_detected();
    if detected {
        println!(
            "  {} {}",
            style("Claude Code").cyan().bold(),
            style("(~/.claude/ detected)").dim()
        );
        println!("    {} bottle integrate claude_code", style("→").dim());
        println!("    Adds /bottle commands: status, update, switch, integrate, list");
        println!("    Plugin: bottle@open-horizon-labs");
    } else {
        println!(
            "  {} {}",
            style("Claude Code").dim(),
            style("not detected").dim()
        );
    }
    println!();
}

fn show_opencode_integration() {
    let detected = crate::integrate::opencode::is_detected();
    if detected {
        println!(
            "  {} {}",
            style("OpenCode").cyan().bold(),
            style("(~/.opencode/ detected)").dim()
        );
        println!("    {} bottle integrate opencode", style("→").dim());
        println!("    Adds bottle-* tools to OpenCode");
        println!("    Config: adds @cloud-atlas-ai/bottle to plugins");
    } else {
        println!(
            "  {} {}",
            style("OpenCode").dim(),
            style("not detected").dim()
        );
    }
    println!();
}

fn show_codex_integration() {
    let detected = crate::integrate::codex::is_detected();
    if detected {
        println!(
            "  {} {}",
            style("Codex").cyan().bold(),
            style("(~/.codex/ detected)").dim()
        );
        println!("    {} bottle integrate codex", style("→").dim());
        println!("    Adds $bottle commands as Codex skill");
        println!("    Skill: ~/.codex/skills/bottle/SKILL.md");
    } else {
        println!("  {} {}", style("Codex").dim(), style("not detected").dim());
    }
}

/// Compare two version strings, returns "upgrade", "downgrade", or "update"
fn compare_versions(current: &str, target: &str) -> &'static str {
    // Simple semver comparison - split by dots and compare numerically
    let parse = |v: &str| -> Vec<u32> { v.split('.').filter_map(|s| s.parse().ok()).collect() };

    let current_parts = parse(current);
    let target_parts = parse(target);

    for (c, t) in current_parts.iter().zip(target_parts.iter()) {
        if t > c {
            return "upgrade";
        } else if t < c {
            return "downgrade";
        }
    }

    // If we get here, compare lengths (e.g., 1.0 vs 1.0.1)
    match target_parts.len().cmp(&current_parts.len()) {
        std::cmp::Ordering::Greater => "upgrade",
        std::cmp::Ordering::Less => "downgrade",
        std::cmp::Ordering::Equal => "update", // shouldn't happen if versions are equal
    }
}

/// Get installed version of a tool, or None if not installed
fn get_tool_version(tool: &str) -> Option<String> {
    let binary = match tool {
        "superego" => "sg",
        "datasphere" => "ds",
        "oh-mcp" => return get_mcp_version("oh-mcp"),
        _ => tool,
    };

    Command::new(binary)
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| {
            let stdout = String::from_utf8_lossy(&o.stdout);
            // Parse "tool x.y.z" or "x.y.z" format
            stdout
                .split_whitespace()
                .find(|s| {
                    s.chars()
                        .next()
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or(false)
                })
                .map(|s| s.trim().to_string())
        })
}

/// Get version of an MCP server (if registered)
fn get_mcp_version(name: &str) -> Option<String> {
    // MCP servers don't have a standard version query
    // Just check if registered
    if is_mcp_registered(name) {
        Some("registered".to_string())
    } else {
        None
    }
}

/// Check if an MCP server is registered
fn is_mcp_registered(name: &str) -> bool {
    Command::new("claude")
        .args(["mcp", "list"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains(name))
        .unwrap_or(false)
}

/// Install all tools from the manifest
/// AIDEV-NOTE: Intentionally continues on failure and returns Ok with partial results.
/// State tracks what succeeded. User sees warnings for failures and can retry.
/// This is a design decision to avoid leaving users in a broken state when one
/// tool fails but others succeed. Consider adding --strict flag if needed later.
fn install_tools(manifest: &BottleManifest) -> Result<HashMap<String, ToolState>> {
    let mut states = HashMap::new();
    let mut failures: Vec<(String, BottleError)> = Vec::new();

    println!("{}:", style("Installing tools").bold());

    let mut tools: Vec<_> = manifest.tools.iter().collect();
    tools.sort_by_key(|(name, _)| *name);

    for (tool_name, version) in tools {
        print!("  {:<12} {} ", tool_name, style(version).dim());

        // Fetch tool definition
        let tool_def = match fetch_tool_definition(tool_name) {
            Ok(def) => def,
            Err(e) => {
                println!("{}", style("failed").red());
                failures.push((tool_name.clone(), e));
                continue;
            }
        };

        // Install the tool
        match install::install_tool(&tool_def, version) {
            Ok(method) => {
                println!("{}", style("installed").green());
                states.insert(
                    tool_name.clone(),
                    ToolState {
                        version: version.clone(),
                        installed_at: Utc::now(),
                        method,
                    },
                );
            }
            Err(e) => {
                println!("{}", style("failed").red());
                failures.push((tool_name.clone(), e));
            }
        }
    }

    println!();

    if !failures.is_empty() {
        ui::print_warning(&format!("{} tool(s) failed to install:", failures.len()));
        for (name, err) in &failures {
            println!("  {} - {}", style(name).red(), err);
        }
        println!();
    }

    Ok(states)
}

/// Build and save AGENTS.md snippet for the manifest (agent applies it later)
fn save_agents_md_snippet(manifest: &BottleManifest) -> Result<Option<String>> {
    use super::common::build_agents_md_snippet;

    let Some(agents_config) = &manifest.agents_md else {
        return Ok(None);
    };

    // Skip if no sections and no snippets_url
    if agents_config.sections.is_empty() && agents_config.snippets_url.is_none() {
        return Ok(None);
    }

    println!("{}:", style("AGENTS.md snippet").bold());

    // Show progress for URL fetch
    if let Some(url) = &agents_config.snippets_url {
        print!("  Fetching from {}... ", style(url).dim());
    }

    // Build snippet using shared function
    match build_agents_md_snippet(manifest) {
        Ok(Some(snippet)) => {
            if agents_config.snippets_url.is_some() {
                println!("{}", style("ok").green());
            }
            let section_count = agents_config.sections.len();
            let has_url = agents_config.snippets_url.is_some();
            let msg = match (section_count, has_url) {
                (0, true) => "saved (from URL)".to_string(),
                (n, false) => format!("saved ({} section(s))", n),
                (n, true) => format!("saved ({} section(s) + URL)", n),
            };
            println!("  {} {}", style("Snippet").cyan(), msg);
            println!();
            Ok(Some(snippet))
        }
        Ok(None) => Ok(None),
        Err(e) => {
            if agents_config.snippets_url.is_some() {
                println!("{}", style("failed").red());
            }
            ui::print_warning(&format!("Could not build snippet: {}", e));
            // Continue without snippet rather than fail the whole install
            Ok(None)
        }
    }
}

/// Install custom tools from the manifest
fn install_custom_tools(manifest: &BottleManifest) -> Result<HashMap<String, CustomToolState>> {
    let mut installed: HashMap<String, CustomToolState> = HashMap::new();

    if manifest.custom_tools.is_empty() {
        return Ok(installed);
    }

    println!("{}:", style("Installing custom tools").bold());

    let mut failures: Vec<(String, BottleError)> = Vec::new();
    let mut tools: Vec<_> = manifest.custom_tools.iter().collect();
    tools.sort_by_key(|(name, _)| *name);

    for (name, tool) in tools {
        print!("  {:<20} {} ", name, style(&tool.version).dim());

        match install_custom_tool(name, tool) {
            Ok(method) => {
                let method_name = match method {
                    CustomInstallMethod::Brew => "brew",
                    CustomInstallMethod::Cargo => "cargo",
                    CustomInstallMethod::Npm => "npm",
                    CustomInstallMethod::Binary => "binary",
                };
                println!(
                    "{} {}",
                    style("installed").green(),
                    style(format!("({})", method_name)).dim()
                );

                // Track in state
                installed.insert(
                    name.clone(),
                    CustomToolState {
                        version: tool.version.clone(),
                        installed_at: Utc::now(),
                        method,
                    },
                );

                // Run verify command only after successful install
                if let Some(verify) = &tool.verify {
                    print!("    {} ", style("verify:").dim());
                    match run_verify_command(verify) {
                        Ok(()) => println!("{}", style("ok").green()),
                        Err(e) => {
                            println!("{}", style("failed").red());
                            ui::print_warning(&format!("Verification failed: {}", e));
                        }
                    }
                }
            }
            Err(e) => {
                println!("{}", style("failed").red());
                failures.push((name.clone(), e));
            }
        }
    }

    println!();

    if !failures.is_empty() {
        ui::print_warning(&format!(
            "{} custom tool(s) failed to install:",
            failures.len()
        ));
        for (name, err) in &failures {
            println!("  {} - {}", style(name).red(), err);
        }
        println!();
    }

    Ok(installed)
}

/// Install a single custom tool, trying methods in order
fn install_custom_tool(
    name: &str,
    tool: &crate::manifest::bottle::CustomToolDef,
) -> Result<CustomInstallMethod> {
    let install = &tool.install;

    // Try brew first
    // Note: Versioned formulas (formula@version) only work for some packages.
    // Tap formulas (org/tap/formula) don't support the @version syntax.
    if let Some(formula) = &install.brew {
        if which::which("brew").is_ok() {
            let has_version = !tool.version.is_empty() && tool.version != "latest";
            let is_tap_formula = formula.contains('/');

            // Warn if trying to version a tap formula (unlikely to work)
            if has_version && is_tap_formula {
                eprintln!(
                    "Warning: Tap formula '{}' may not support @{} syntax. Consider using 'latest'.",
                    formula, tool.version
                );
            }

            let version_formula = if has_version {
                format!("{}@{}", formula, tool.version)
            } else {
                formula.clone()
            };

            let status = Command::new("brew")
                .args(["install", &version_formula])
                .status()
                .map_err(|e| BottleError::InstallError {
                    tool: name.to_string(),
                    reason: format!("brew install failed: {}", e),
                })?;

            if status.success() {
                return Ok(CustomInstallMethod::Brew);
            }
        }
    }

    // Try cargo
    if let Some(crate_name) = &install.cargo {
        if which::which("cargo").is_ok() {
            let mut args = vec!["install", crate_name];
            let version_arg;
            if !tool.version.is_empty() && tool.version != "latest" {
                version_arg = format!("--version={}", tool.version);
                args.push(&version_arg);
            }

            let status = Command::new("cargo").args(&args).status().map_err(|e| {
                BottleError::InstallError {
                    tool: name.to_string(),
                    reason: format!("cargo install failed: {}", e),
                }
            })?;

            if status.success() {
                return Ok(CustomInstallMethod::Cargo);
            }
        }
    }

    // Try npm
    if let Some(package) = &install.npm {
        if which::which("npm").is_ok() {
            let package_spec = if tool.version.is_empty() || tool.version == "latest" {
                package.clone()
            } else {
                format!("{}@{}", package, tool.version)
            };

            let status = Command::new("npm")
                .args(["install", "-g", &package_spec])
                .status()
                .map_err(|e| BottleError::InstallError {
                    tool: name.to_string(),
                    reason: format!("npm install failed: {}", e),
                })?;

            if status.success() {
                return Ok(CustomInstallMethod::Npm);
            }
        }
    }

    // Try binary_url
    if let Some(url_template) = &install.binary_url {
        let url = expand_binary_url(url_template);
        install_from_binary_url(name, &url)?;
        return Ok(CustomInstallMethod::Binary);
    }

    Err(BottleError::InstallError {
        tool: name.to_string(),
        reason: "No installation method available or all methods failed".to_string(),
    })
}

/// Expand placeholders in binary URL
/// Supported: {arch}, {os}, {platform}, {arm64}
fn expand_binary_url(url: &str) -> String {
    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;

    // Alternative arch name for Apple Silicon (many releases use "arm64" instead of "aarch64")
    let arm64_name = match arch {
        "aarch64" => "arm64",
        _ => arch,
    };

    let os_name = match os {
        "macos" => "darwin",
        "linux" => "linux",
        "windows" => "windows",
        _ => os,
    };

    url.replace("{arch}", arch)
        .replace("{arm64}", arm64_name)
        .replace("{os}", os_name)
        .replace("{platform}", &format!("{}-{}", os_name, arch))
}

/// Install a tool from a binary URL (raw binary only, no archive support)
fn install_from_binary_url(name: &str, url: &str) -> Result<()> {
    use std::time::Duration;

    // Enforce HTTPS for security
    if !url.starts_with("https://") {
        return Err(BottleError::InstallError {
            tool: name.to_string(),
            reason: format!("Binary URL must use HTTPS: {}", url),
        });
    }

    // Build client with timeout
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| BottleError::InstallError {
            tool: name.to_string(),
            reason: format!("Failed to create HTTP client: {}", e),
        })?;

    // Download the binary
    let response = client
        .get(url)
        .send()
        .map_err(|e| BottleError::InstallError {
            tool: name.to_string(),
            reason: format!("Failed to download from {}: {}", url, e),
        })?;

    if !response.status().is_success() {
        return Err(BottleError::InstallError {
            tool: name.to_string(),
            reason: format!("HTTP {} from {}", response.status(), url),
        });
    }

    let bytes = response.bytes().map_err(|e| BottleError::InstallError {
        tool: name.to_string(),
        reason: format!("Failed to read response: {}", e),
    })?;

    // Determine install path (~/.local/bin)
    let bin_dir = dirs::home_dir()
        .map(|h| h.join(".local").join("bin"))
        .ok_or_else(|| BottleError::InstallError {
            tool: name.to_string(),
            reason: "Could not determine home directory".to_string(),
        })?;

    std::fs::create_dir_all(&bin_dir).map_err(|e| BottleError::InstallError {
        tool: name.to_string(),
        reason: format!("Failed to create bin directory: {}", e),
    })?;

    let bin_path = bin_dir.join(name);

    // Write the binary
    std::fs::write(&bin_path, &bytes).map_err(|e| BottleError::InstallError {
        tool: name.to_string(),
        reason: format!("Failed to write binary: {}", e),
    })?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Err(e) = std::fs::set_permissions(&bin_path, std::fs::Permissions::from_mode(0o755))
        {
            eprintln!(
                "Warning: could not set executable permissions on {}: {}",
                bin_path.display(),
                e
            );
        }
    }

    Ok(())
}

/// Run a verification command after tool installation.
/// Note: Commands are split on whitespace, so quoted arguments are not supported.
/// Keep verify commands simple (e.g., "mytool --version").
fn run_verify_command(verify: &str) -> Result<()> {
    let parts: Vec<&str> = verify.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    let status = Command::new(parts[0])
        .args(&parts[1..])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| BottleError::Other(format!("Verify command failed: {}", e)))?;

    if status.success() {
        Ok(())
    } else {
        Err(BottleError::Other(format!(
            "Verify command '{}' exited with code {}",
            verify, status
        )))
    }
}

/// Install bespoke MCP servers from the manifest
fn install_mcp_servers(manifest: &BottleManifest) -> Result<()> {
    if manifest.mcp_servers.is_empty() {
        return Ok(());
    }

    // Validate all env vars upfront before any registration
    for (name, server) in &manifest.mcp_servers {
        install::mcp::validate_env_vars(name, server)?;
    }

    println!("{}:", style("Registering MCP servers").bold());

    let mut failures: Vec<(String, crate::error::BottleError)> = Vec::new();
    let mut servers: Vec<_> = manifest.mcp_servers.iter().collect();
    servers.sort_by_key(|(name, _)| *name);

    for (name, server) in servers {
        print!("  {:<20} ", name);

        // Register with Claude Code
        match install::mcp::register_bespoke(name, server) {
            Ok(()) => {
                println!("{}", style("registered").green());
            }
            Err(e) => {
                println!("{}", style("failed").red());
                failures.push((name.clone(), e));
            }
        }
    }

    println!();

    // Also register with OpenCode if detected
    if crate::integrate::opencode::is_detected() && !manifest.mcp_servers.is_empty() {
        print!("  {} ", style("OpenCode integration").dim());
        match install::mcp::register_bespoke_opencode(&manifest.mcp_servers) {
            Ok(()) => println!("{}", style("done").green()),
            Err(e) => {
                println!("{}", style("failed").red());
                ui::print_warning(&format!("OpenCode MCP registration: {}", e));
            }
        }
        println!();
    }

    if !failures.is_empty() {
        ui::print_warning(&format!(
            "{} MCP server(s) failed to register:",
            failures.len()
        ));
        for (name, err) in &failures {
            println!("  {} - {}", style(name).red(), err);
        }
        println!();
    }

    Ok(())
}

/// Display success message with next steps
fn show_success(manifest: &BottleManifest) {
    println!();
    ui::print_success(&format!(
        "Bottle '{}' installed successfully!",
        manifest.name
    ));
    println!();
    println!("{}:", style("Next steps").bold());
    println!(
        "  {} - Check installed tools",
        style("bottle status").cyan()
    );
    println!(
        "  {} - Initialize ba for task tracking",
        style("ba init").cyan()
    );
    println!("  {} - Initialize working memory", style("wm init").cyan());
    println!();
}
