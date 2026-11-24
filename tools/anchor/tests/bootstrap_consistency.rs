//! Repository relationships only; engine installation and identity execution are
//! covered by the launcher's existing tests.
use std::{fs, path::Path};

const GUIDE: &str = "working-on-the-project.md";
const LIFECYCLE: &str = "lifecycle.md";
const DELIVERY: &str = "change-delivery.md";

fn validate(read: impl Fn(&str) -> Option<String>) -> Result<(), String> {
    let get = |path: &str| read(path).ok_or_else(|| format!("Missing required file: {path}"));
    let agents = get("AGENTS.md")?;
    if !agents
        .lines()
        .any(|line| line.starts_with("1. Run `cargo anchor contract --config anchorbook.toml`"))
    {
        return Err("AGENTS.md must retrieve the contract with --config anchorbook.toml".into());
    }
    let configuration = get("anchorbook.toml")?;
    let project = configuration
        .split_once("[project]")
        .map(|(_, rest)| rest.split('[').next().unwrap_or_default())
        .ok_or("anchorbook.toml must declare the project phase")?;
    if !project
        .lines()
        .any(|line| line.trim() == "phase = \"port\"")
    {
        return Err("The port bootstrap requires project.phase = port".into());
    }
    let summary = get("book/SUMMARY.md")?;
    for chapter in [GUIDE, LIFECYCLE, DELIVERY] {
        get(&format!("book/{chapter}"))?;
        if !summary.lines().any(|line| {
            line.trim_start().starts_with("- [") && line.ends_with(&format!("]({chapter})"))
        }) {
            return Err(format!("book/SUMMARY.md must navigate to {chapter}"));
        }
    }
    let launcher = get("tools/anchor/src/main.rs")?;
    let revision = launcher
        .lines()
        .find_map(|line| line.strip_prefix("const REVISION: &str = \""))
        .and_then(|rest| rest.strip_suffix("\";"))
        .ok_or(
            "Cannot read launcher revision; update the consistency check for the new pin format",
        )?;
    let version = launcher
        .split("format!(\"anchorbook ")
        .nth(1)
        .and_then(|rest| {
            rest.split_once(" ({REVISION})\"")
                .map(|(version, _)| version)
        })
        .ok_or("Cannot read launcher identity format")?;
    let identity = format!("anchorbook {version} ({revision})");
    if !get("CONTRIBUTING.md")?.contains(&format!("`{identity}`")) {
        return Err(format!("CONTRIBUTING.md must document `{identity}`"));
    }
    let guide = get(&format!("book/{GUIDE}"))?;
    let prefix = "https://github.com/anchor-book/anchor-book/";
    let mut links = 0;
    for tail in guide.split(prefix).skip(1) {
        let mut parts = tail.split('/');
        let kind = parts.next().unwrap_or_default();
        let pin = parts
            .next()
            .unwrap_or_default()
            .split(')')
            .next()
            .unwrap_or_default();
        if !matches!(kind, "tree" | "blob") || pin != revision {
            return Err("Working guide Anchor Book links must use the launcher revision".into());
        }
        links += 1;
    }
    if links == 0 {
        return Err("Working guide must link pinned Anchor Book authoring references".into());
    }
    let ci = get(".github/workflows/ci.yml")?;
    for command in [
        "cargo anchor --version",
        "cargo anchor check --config anchorbook.toml",
        "cargo test -p anchor-runner --test bootstrap_consistency --locked",
    ] {
        if !ci.lines().any(|line| {
            let line = line.trim();
            line == command || line == format!("run: {command}")
        }) {
            return Err(format!("CI must run `{command}`"));
        }
    }
    if ci.contains("anchorbook ") || ci.contains("cargo install anchor-book") {
        return Err(
            "CI must use repository-local cargo anchor, not direct engine invocation/install"
                .into(),
        );
    }
    if !get("book.toml")?.contains("command = \"cargo anchor md\"") {
        return Err("mdBook must use cargo anchor md".into());
    }
    if !get(".cargo/config.toml")?
        .contains("anchor = \"run --quiet --locked --package anchor-runner --\"")
    {
        return Err("Cargo anchor alias must select the repository launcher".into());
    }
    Ok(())
}

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
}

#[test]
fn repository_bootstrap_relationships_hold() {
    validate(|path| fs::read_to_string(root().join(path)).ok()).unwrap();
}

#[test]
fn detects_broken_bootstrap_relationships() {
    validate(|path| fs::read_to_string(root().join(path)).ok()).unwrap();
    // Exercise each relationship against an otherwise valid checkout, without
    // changing files or invoking/installing the engine.
    for (path, from, to) in [
        ("AGENTS.md", "cargo anchor contract", "anchorbook contract"),
        ("AGENTS.md", " --config anchorbook.toml", ""),
        (
            "anchorbook.toml",
            "phase = \"port\"",
            "phase = \"prototyping\"",
        ),
        ("anchorbook.toml", "[project]", "[other]"),
        (
            "book/SUMMARY.md",
            "](working-on-the-project.md)",
            "](missing.md)",
        ),
        ("book/SUMMARY.md", "](lifecycle.md)", "](missing.md)"),
        ("book/SUMMARY.md", "](change-delivery.md)", "](missing.md)"),
        ("CONTRIBUTING.md", "`anchorbook ", "`wrong-engine "),
        (
            "tools/anchor/src/main.rs",
            "const REVISION: &str = \"",
            "const REVISION: &str = \"wrong-",
        ),
        ("book/working-on-the-project.md", "/blob/", "/blob/wrong-"),
        (
            ".github/workflows/ci.yml",
            "cargo anchor check",
            "anchorbook check",
        ),
        ("book.toml", "cargo anchor md", "anchorbook md"),
        (
            ".cargo/config.toml",
            "--package anchor-runner",
            "--package other",
        ),
    ] {
        let result = validate(|name| {
            let text = fs::read_to_string(root().join(name)).ok()?;
            Some(if name == path {
                text.replace(from, to)
            } else {
                text
            })
        });
        assert!(result.is_err(), "missed drift in {path}");
    }
    for missing in [GUIDE, LIFECYCLE, DELIVERY] {
        assert!(
            validate(|name| {
                if name == format!("book/{missing}") {
                    None
                } else {
                    fs::read_to_string(root().join(name)).ok()
                }
            })
            .is_err()
        );
    }
}
