//! Recommendation Validator
//!
//! Validates recommendation files and optionally concatenates them for publishing.

use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use symposium_recommendations::{ComponentSource, Recommendation, Recommendations};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: validate <recommendations-dir> [--output <file>] [--generate-docs <file>]");
        std::process::exit(1);
    }

    let recommendations_dir = PathBuf::from(&args[1]);
    let output_file = args
        .iter()
        .position(|a| a == "--output")
        .map(|i| PathBuf::from(&args[i + 1]));
    let docs_file = args
        .iter()
        .position(|a| a == "--generate-docs")
        .map(|i| PathBuf::from(&args[i + 1]));

    // Discover all recommendation files
    let files = discover_files(&recommendations_dir)?;

    if files.is_empty() {
        bail!("No recommendation files found in {}", recommendations_dir.display());
    }

    println!("Found {} recommendation files", files.len());

    // Validate and collect all recommendations
    let mut all_contents = Vec::new();
    let mut errors = Vec::new();

    for file in &files {
        print!("  Validating {}... ", file.display());

        let content = fs::read_to_string(file)
            .with_context(|| format!("Failed to read {}", file.display()))?;

        match Recommendations::parse_single(&content) {
            Ok(rec) => {
                // Check for local sources (not allowed in public recommendations)
                if rec.source.is_local() {
                    errors.push(format!(
                        "{}: local sources are not allowed in public recommendations",
                        file.display()
                    ));
                    println!("FAILED (local source)");
                } else {
                    all_contents.push(content);
                    println!("OK ({})", rec.display_name());
                }
            }
            Err(e) => {
                errors.push(format!("{}: {}", file.display(), e));
                println!("FAILED");
            }
        }
    }

    // Report errors
    if !errors.is_empty() {
        eprintln!("\nValidation errors:");
        for error in &errors {
            eprintln!("  - {}", error);
        }
        std::process::exit(1);
    }

    println!("\nAll {} files validated successfully!", files.len());

    // Concatenate and output if requested
    if let Some(output) = output_file {
        let refs: Vec<&str> = all_contents.iter().map(|s| s.as_str()).collect();
        let combined = Recommendations::concatenate_files(&refs)?;

        // Verify the combined output parses correctly
        let parsed = Recommendations::from_toml(&combined)?;
        println!("Combined {} recommendations into {}", parsed.mods.len(), output.display());

        fs::write(&output, &combined)
            .with_context(|| format!("Failed to write {}", output.display()))?;

        // Generate documentation if requested
        if let Some(docs_path) = docs_file {
            let docs = generate_docs(&parsed.mods);
            fs::write(&docs_path, &docs)
                .with_context(|| format!("Failed to write {}", docs_path.display()))?;
            println!("Generated documentation at {}", docs_path.display());
        }
    }

    Ok(())
}

fn generate_docs(recommendations: &[Recommendation]) -> String {
    let mut doc = String::new();
    doc.push_str("# Current Recommendations\n\n");
    doc.push_str("This page is auto-generated from the recommendation files in this repository.\n\n");

    for rec in recommendations {
        let name = rec.display_name();
        doc.push_str(&format!("## {}\n\n", name));

        // Source info
        match &rec.source {
            ComponentSource::Cargo(cargo) => {
                doc.push_str(&format!("**Install:** `cargo install {}`\n\n", cargo.crate_name));
                if !cargo.args.is_empty() {
                    doc.push_str(&format!("**Args:** `{}`\n\n", cargo.args.join(" ")));
                }
            }
            ComponentSource::Npx(npx) => {
                doc.push_str(&format!("**Install:** `npx {}`\n\n", npx.package));
                if !npx.args.is_empty() {
                    doc.push_str(&format!("**Args:** `{}`\n\n", npx.args.join(" ")));
                }
            }
            ComponentSource::Pipx(pipx) => {
                doc.push_str(&format!("**Install:** `pipx install {}`\n\n", pipx.package));
                if !pipx.args.is_empty() {
                    doc.push_str(&format!("**Args:** `{}`\n\n", pipx.args.join(" ")));
                }
            }
            ComponentSource::Builtin(name) => {
                doc.push_str(&format!("**Built-in:** `{}`\n\n", name));
            }
            ComponentSource::Registry(id) => {
                doc.push_str(&format!("**Registry:** `{}`\n\n", id));
            }
            ComponentSource::Url(url) => {
                doc.push_str(&format!("**URL:** `{}`\n\n", url));
            }
            _ => {}
        }

        // Conditions
        if let Some(when) = &rec.when {
            let explanations = when.explain_why_added();
            if !explanations.is_empty() {
                doc.push_str("**When:**\n");
                for exp in explanations {
                    doc.push_str(&format!("- {}\n", exp));
                }
                doc.push_str("\n");
            }
        } else {
            doc.push_str("**When:** Always recommended\n\n");
        }

        doc.push_str("---\n\n");
    }

    doc
}

/// Discover all recommendation files in a directory.
///
/// Supports both:
/// - `recommendations/foo.toml` (file directly in dir)
/// - `recommendations/foo/config.toml` (directory with config.toml)
fn discover_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    if !dir.is_dir() {
        bail!("{} is not a directory", dir.display());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_stem().map(|s| s.to_string_lossy().to_string());

        if path.is_file() && path.extension().map(|e| e == "toml").unwrap_or(false) {
            // Direct .toml file
            if let Some(ref n) = name {
                if seen_names.contains(n) {
                    bail!(
                        "Duplicate recommendation name '{}': found both {}.toml and {}/config.toml",
                        n, n, n
                    );
                }
                seen_names.insert(n.clone());
            }
            files.push(path);
        } else if path.is_dir() {
            // Directory - look for config.toml
            let config_path = path.join("config.toml");
            if config_path.exists() {
                if let Some(ref n) = name {
                    if seen_names.contains(n) {
                        bail!(
                            "Duplicate recommendation name '{}': found both {}.toml and {}/config.toml",
                            n, n, n
                        );
                    }
                    seen_names.insert(n.clone());
                }
                files.push(config_path);
            }
        }
    }

    // Sort for consistent output
    files.sort();

    Ok(files)
}
