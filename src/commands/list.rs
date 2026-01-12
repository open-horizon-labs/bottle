use crate::error::Result;
use crate::fetch::{fetch_bottle_manifest, list_available_bottles};
use crate::manifest::bottle::BottleManifest;
use console::style;
use std::fs;
use std::path::PathBuf;

/// List available bottles (curated and bespoke)
pub fn run() -> Result<()> {
    // Fetch curated bottles
    let curated = list_curated_bottles();

    // Find bespoke bottles
    let bespoke = list_bespoke_bottles();

    // Display curated bottles
    println!("{}:", style("Curated bottles (from GitHub)").bold());
    if curated.is_empty() {
        println!("  {}", style("(none available)").dim());
    } else {
        for (name, description) in &curated {
            println!("  {:<12} {}", style(name).cyan(), style(description).dim());
        }
    }
    println!();

    // Display bespoke bottles
    println!("{}:", style("Bespoke bottles (local)").bold());
    if bespoke.is_empty() {
        println!("  {}", style("(none)").dim());
    } else {
        for (name, description) in &bespoke {
            println!("  {:<12} {}", style(name).cyan(), style(description).dim());
        }
    }

    Ok(())
}

/// Fetch curated bottles with descriptions
fn list_curated_bottles() -> Vec<(String, String)> {
    let bottle_names = match list_available_bottles() {
        Ok(names) => names,
        Err(_) => return Vec::new(),
    };

    let mut bottles = Vec::new();
    for name in bottle_names {
        let description = match fetch_bottle_manifest(&name) {
            Ok(manifest) => manifest.description,
            Err(_) => String::from("(unable to fetch description)"),
        };
        bottles.push((name, description));
    }
    bottles
}

/// Find local bespoke bottles in ~/.bottle/bottles/
fn list_bespoke_bottles() -> Vec<(String, String)> {
    let bottles_dir = match get_bespoke_bottles_dir() {
        Some(dir) => dir,
        None => return Vec::new(),
    };

    if !bottles_dir.exists() {
        return Vec::new();
    }

    let mut bottles = Vec::new();

    let entries = match fs::read_dir(&bottles_dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let manifest_path = path.join("manifest.json");
        if !manifest_path.exists() {
            continue;
        }

        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };

        let description = match fs::read_to_string(&manifest_path) {
            Ok(content) => match serde_json::from_str::<BottleManifest>(&content) {
                Ok(manifest) => manifest.description,
                Err(_) => String::from("(invalid manifest)"),
            },
            Err(_) => String::from("(unable to read manifest)"),
        };

        bottles.push((name, description));
    }

    // Sort by name for consistent output
    bottles.sort_by(|a, b| a.0.cmp(&b.0));
    bottles
}

/// Get the path to bespoke bottles directory
fn get_bespoke_bottles_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".bottle").join("bottles"))
}
