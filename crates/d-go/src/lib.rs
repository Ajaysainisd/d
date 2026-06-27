use std::path::Path;
use std::process::Command;

use d_core::platform::Platform;
use d_core::types::{CommandDef, DoctorCheck, DoctorStatus};

pub struct GoPlatform;

impl Platform for GoPlatform {
    fn name(&self) -> &str {
        "go"
    }

    fn detect(&self, dir: &Path) -> f32 {
        if dir.join("go.mod").exists() {
            return 0.95;
        }
        if dir.join("go.sum").exists() {
            return 0.6;
        }
        let go_files = std::fs::read_dir(dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .any(|e| e.path().extension().is_some_and(|ext| ext == "go"))
            })
            .unwrap_or(false);
        if go_files {
            return 0.4;
        }
        0.0
    }

    fn commands(&self) -> Vec<CommandDef> {
        vec![
            CommandDef::new("build", "go", "Build the project").with_args(&["build", "./..."]),
            CommandDef::new("run", "go", "Run the application").with_args(&["run", "."]),
            CommandDef::new("test", "go", "Run tests").with_args(&["test", "./..."]),
            CommandDef::new("lint", "go", "Lint with vet").with_args(&["vet", "./..."]),
            CommandDef::new("format", "go", "Format code (gofmt)").with_args(&["fmt", "./..."]),
            CommandDef::new("clean", "go", "Clean build cache").with_args(&["clean", "-cache"]),
            CommandDef::new("install", "go", "Download dependencies")
                .with_args(&["mod", "download"]),
            CommandDef::new("release", "go", "Build release binary").with_args(&[
                "build",
                "-ldflags=-s -w",
                "./...",
            ]),
            CommandDef::new("bench", "go", "Run benchmarks")
                .with_args(&["test", "-bench=.", "./..."]),
            CommandDef::new("doc", "go", "Serve documentation").with_args(&["doc"]),
            CommandDef::new("generate", "go", "Run go generate").with_args(&["generate", "./..."]),
            CommandDef::new("race", "go", "Test with race detector")
                .with_args(&["test", "-race", "./..."]),
            CommandDef::new("tidy", "go", "Tidy module dependencies").with_args(&["mod", "tidy"]),
            CommandDef::new("vendor", "go", "Vendor dependencies").with_args(&["mod", "vendor"]),
            CommandDef::new("verify", "go", "Verify module hashes").with_args(&["mod", "verify"]),
            CommandDef::new("coverage", "go", "Run tests with coverage").with_args(&[
                "test",
                "-coverprofile=coverage.out",
                "./...",
            ]),
            CommandDef::new("mod-init", "go", "Initialize go module").with_args(&["mod", "init"]),
            CommandDef::new("get", "go", "Add a dependency").with_args(&["get"]),
            CommandDef::new("fmt-check", "go", "Check if files need formatting")
                .with_args(&["fmt", "-n", "./..."]),
            CommandDef::new("doctor", "go", "Show Go version and env").with_args(&["version"]),
            CommandDef::new("env", "go", "Show Go environment").with_args(&["env"]),
            CommandDef::new("list", "go", "List packages").with_args(&["list", "./..."]),
        ]
    }

    fn doctor_checks(&self, _dir: &Path) -> Vec<DoctorCheck> {
        vec![
            d_core::doctor::check_tool("go", "go-sdk", "sdk"),
            DoctorCheck {
                name: "go-version".to_string(),
                category: "sdk".to_string(),
                status: check_go_version(),
                message: Some(get_go_version()),
                suggestion: None,
            },
        ]
    }
}

fn check_go_version() -> DoctorStatus {
    if Command::new("go").arg("version").output().is_ok() {
        DoctorStatus::Pass
    } else {
        DoctorStatus::Fail
    }
}

fn get_go_version() -> String {
    if let Ok(output) = Command::new("go").arg("version").output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "unknown".to_string()
    }
}

pub fn platform() -> Box<dyn Platform> {
    Box::new(GoPlatform)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_commands_exist() {
        let cmds = GoPlatform.commands();
        assert!(cmds.iter().any(|c| c.verb == "build"));
        assert!(cmds.iter().any(|c| c.verb == "run"));
        assert!(cmds.iter().any(|c| c.verb == "test"));
        assert!(cmds.iter().any(|c| c.verb == "bench"));
        assert!(cmds.iter().any(|c| c.verb == "doc"));
        assert!(cmds.iter().any(|c| c.verb == "generate"));
        assert!(cmds.iter().any(|c| c.verb == "race"));
        assert!(cmds.iter().any(|c| c.verb == "tidy"));
        assert!(cmds.iter().any(|c| c.verb == "vendor"));
        assert!(cmds.iter().any(|c| c.verb == "coverage"));
        assert!(cmds.len() > 15);
    }
}
