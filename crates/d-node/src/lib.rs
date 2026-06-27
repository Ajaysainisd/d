use std::path::Path;
use std::process::Command;

use d_core::platform::Platform;
use d_core::types::{CommandDef, DoctorCheck, DoctorStatus};

pub struct NodePlatform;

impl Platform for NodePlatform {
    fn name(&self) -> &str {
        "node"
    }

    fn detect(&self, dir: &Path) -> f32 {
        let package_json = dir.join("package.json");
        if !package_json.exists() {
            return 0.0;
        }
        let pubspec = dir.join("pubspec.yaml");
        if pubspec.exists() {
            return 0.3;
        }
        0.85
    }

    fn commands(&self) -> Vec<CommandDef> {
        let pm = detect_package_manager();
        vec![
            CommandDef::new("install", &pm, "Install dependencies")
                .with_args(&["install"]),
            CommandDef::new("dev", &pm, "Start dev server")
                .with_args(&["run", "dev"]),
            CommandDef::new("build", &pm, "Build the project")
                .with_args(&["run", "build"]),
            CommandDef::new("test", &pm, "Run tests")
                .with_args(&["test"]),
            CommandDef::new("lint", &pm, "Lint code")
                .with_args(&["run", "lint"]),
            CommandDef::new("format", &pm, "Format code")
                .with_args(&["run", "format"]),
            CommandDef::new("start", &pm, "Start the application")
                .with_args(&["start"]),
        ]
    }

    fn doctor_checks(&self, _dir: &Path) -> Vec<DoctorCheck> {
        let pm = detect_package_manager();
        vec![
            d_core::doctor::check_tool("node", "node-runtime", "sdk"),
            DoctorCheck {
                name: "node-version".to_string(),
                category: "sdk".to_string(),
                status: check_node_version(),
                message: Some(get_node_version()),
                suggestion: if check_node_version() != DoctorStatus::Pass {
                    Some("Node 18+ recommended".to_string())
                } else {
                    None
                },
            },
            d_core::doctor::check_tool(&pm, "package-manager", "tool"),
        ]
    }
}

fn detect_package_manager() -> String {
    for pm in &["pnpm", "yarn", "npm"] {
        if Command::new(pm).arg("--version").output().is_ok() {
            return pm.to_string();
        }
    }
    "npm".to_string()
}

fn check_node_version() -> DoctorStatus {
    if let Ok(output) = Command::new("node").arg("--version").output() {
        let version = String::from_utf8_lossy(&output.stdout);
        let version = version.trim().trim_start_matches('v');
        if let Some(major) = version.split('.').next() {
            if let Ok(num) = major.parse::<u32>() {
                if num >= 18 {
                    return DoctorStatus::Pass;
                }
                if num >= 16 {
                    return DoctorStatus::Warn;
                }
            }
        }
        DoctorStatus::Fail
    } else {
        DoctorStatus::Fail
    }
}

fn get_node_version() -> String {
    if let Ok(output) = Command::new("node").arg("--version").output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "unknown".to_string()
    }
}

pub fn platform() -> Box<dyn Platform> {
    Box::new(NodePlatform)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_commands_exist() {
        let cmds = NodePlatform.commands();
        assert!(cmds.iter().any(|c| c.verb == "install"));
        assert!(cmds.iter().any(|c| c.verb == "test"));
    }
}
