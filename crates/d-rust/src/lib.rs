use std::path::Path;
use std::process::Command;

use d_core::platform::Platform;
use d_core::types::{CommandDef, DoctorCheck, DoctorStatus};

pub struct RustPlatform;

impl Platform for RustPlatform {
    fn name(&self) -> &str {
        "rust"
    }

    fn detect(&self, dir: &Path) -> f32 {
        if dir.join("Cargo.toml").exists() {
            if let Ok(contents) = std::fs::read_to_string(dir.join("Cargo.toml")) {
                if contents.contains("[package]") || contents.contains("[workspace]") {
                    if dir.join("pubspec.yaml").exists() {
                        return 0.3;
                    }
                    return 0.95;
                }
            }
            return 0.7;
        }
        0.0
    }

    fn commands(&self) -> Vec<CommandDef> {
        vec![
            CommandDef::new("build", "cargo", "Build the project").with_args(&["build"]),
            CommandDef::new("run", "cargo", "Run the project").with_args(&["run"]),
            CommandDef::new("test", "cargo", "Run tests").with_args(&["test"]),
            CommandDef::new("lint", "cargo", "Lint with clippy").with_args(&["clippy"]),
            CommandDef::new("format", "cargo", "Format code").with_args(&["fmt"]),
            CommandDef::new("clean", "cargo", "Clean build artifacts").with_args(&["clean"]),
            CommandDef::new("install", "cargo", "Check/verify dependencies").with_args(&["check"]),
            CommandDef::new("release", "cargo", "Build in release mode")
                .with_args(&["build", "--release"]),
            CommandDef::new("bench", "cargo", "Run benchmarks").with_args(&["bench"]),
            CommandDef::new("doc", "cargo", "Build documentation").with_args(&["doc"]),
            CommandDef::new("publish", "cargo", "Publish to crates.io").with_args(&["publish"]),
            CommandDef::new("update", "cargo", "Update dependencies").with_args(&["update"]),
            CommandDef::new("check", "cargo", "Check compilation without building")
                .with_args(&["check"]),
            CommandDef::new("audit", "cargo", "Audit for security vulnerabilities")
                .with_args(&["audit"]),
            CommandDef::new("watch", "cargo", "Watch for changes and rebuild")
                .with_args(&["watch"]),
            CommandDef::new("fix", "cargo", "Auto-fix compiler suggestions")
                .with_args(&["fix", "--allow-dirty"]),
            CommandDef::new("expand", "cargo", "Expand macros").with_args(&["expand"]),
            CommandDef::new("doctor", "cargo", "Show cargo version").with_args(&["version"]),
            CommandDef::new("tree", "cargo", "Show dependency tree").with_args(&["tree"]),
            CommandDef::new("outdated", "cargo", "Check for outdated dependencies")
                .with_args(&["outdated"]),
            CommandDef::new("init", "cargo", "Initialize a new Rust project").with_args(&["init"]),
            CommandDef::new("new", "cargo", "Create a new Rust project").with_args(&["new"]),
            CommandDef::new("creates", "cargo", "Create a new crate").with_args(&["new"]),
        ]
    }

    fn doctor_checks(&self, _dir: &Path) -> Vec<DoctorCheck> {
        vec![
            d_core::doctor::check_tool("cargo", "cargo", "tool"),
            d_core::doctor::check_tool("rustc", "rustc", "sdk"),
            DoctorCheck {
                name: "rustup".to_string(),
                category: "tool".to_string(),
                status: check_rustup(),
                message: Some(get_rust_version()),
                suggestion: None,
            },
        ]
    }
}

fn check_rustup() -> DoctorStatus {
    if let Ok(output) = Command::new("rustc").arg("--version").output() {
        if output.status.success() {
            return DoctorStatus::Pass;
        }
    }
    DoctorStatus::Fail
}

fn get_rust_version() -> String {
    if let Ok(output) = Command::new("rustc").arg("--version").output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "unknown".to_string()
    }
}

pub fn platform() -> Box<dyn Platform> {
    Box::new(RustPlatform)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_commands_exist() {
        let cmds = RustPlatform.commands();
        assert!(cmds.iter().any(|c| c.verb == "build"));
        assert!(cmds.iter().any(|c| c.verb == "test"));
        assert!(cmds.iter().any(|c| c.verb == "run"));
        assert!(cmds.iter().any(|c| c.verb == "bench"));
        assert!(cmds.iter().any(|c| c.verb == "doc"));
        assert!(cmds.iter().any(|c| c.verb == "publish"));
        assert!(cmds.iter().any(|c| c.verb == "update"));
        assert!(cmds.iter().any(|c| c.verb == "audit"));
        assert!(cmds.iter().any(|c| c.verb == "fix"));
        assert!(cmds.len() > 15);
    }
}
