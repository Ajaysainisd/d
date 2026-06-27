use std::path::Path;
use std::process::Command;

use d_core::platform::Platform;
use d_core::types::{CommandDef, DoctorCheck, DoctorStatus};

pub struct PythonPlatform;

impl Platform for PythonPlatform {
    fn name(&self) -> &str {
        "python"
    }

    fn detect(&self, dir: &Path) -> f32 {
        if dir.join("pyproject.toml").exists() {
            return 0.9;
        }
        if dir.join("requirements.txt").exists() {
            return 0.7;
        }
        if dir.join("Pipfile").exists() {
            return 0.75;
        }
        if dir.join("setup.py").exists() || dir.join("setup.cfg").exists() {
            return 0.65;
        }
        if dir.join("manage.py").exists() {
            return 0.2;
        }
        0.0
    }

    fn commands(&self) -> Vec<CommandDef> {
        let pm = detect_package_manager();
        vec![
            CommandDef::new("install", &pm, "Install dependencies").with_args(&["install"]),
            CommandDef::new("dev", &pm, "Run in development").with_args(&["run", "dev"]),
            CommandDef::new("build", &pm, "Build Python package").with_args(&["build"]),
            CommandDef::new("test", &pm, "Run tests").with_args(&["test"]),
            CommandDef::new("lint", &pm, "Lint code").with_args(&["run", "lint"]),
            CommandDef::new("format", &pm, "Format code").with_args(&["run", "format"]),
            CommandDef::new("run", "python", "Run a Python script or module")
                .with_args(&["{target}"]),
            CommandDef::new("clean", "rm", "Clean build artifacts").with_args(&[
                "-rf",
                "__pycache__",
                ".pytest_cache",
                "dist",
                "*.egg-info",
            ]),
        ]
    }

    fn doctor_checks(&self, _dir: &Path) -> Vec<DoctorCheck> {
        let pm = detect_package_manager();
        vec![
            d_core::doctor::check_tool("python3", "python", "sdk"),
            d_core::doctor::check_tool("pip", "pip", "tool"),
            DoctorCheck {
                name: "python-version".to_string(),
                category: "sdk".to_string(),
                status: check_python_version(),
                message: Some(get_python_version()),
                suggestion: if check_python_version() != DoctorStatus::Pass {
                    Some("Python 3.8+ recommended".to_string())
                } else {
                    None
                },
            },
            d_core::doctor::check_tool(&pm, "package-manager", "tool"),
        ]
    }
}

fn detect_package_manager() -> String {
    for pm in &["uv", "poetry", "pipenv", "pip"] {
        if Command::new(pm).arg("--version").output().is_ok() {
            return pm.to_string();
        }
    }
    "pip".to_string()
}

fn check_python_version() -> DoctorStatus {
    if let Ok(output) = Command::new("python3").arg("--version").output() {
        let version = String::from_utf8_lossy(&output.stdout);
        if version.contains("Python 3.") {
            let minor = version
                .split("Python 3.")
                .nth(1)
                .and_then(|v| v.split('.').next())
                .and_then(|n| n.parse::<u32>().ok())
                .unwrap_or(0);
            if minor >= 8 {
                return DoctorStatus::Pass;
            }
            return DoctorStatus::Warn;
        }
        DoctorStatus::Fail
    } else {
        DoctorStatus::Fail
    }
}

fn get_python_version() -> String {
    if let Ok(output) = Command::new("python3").arg("--version").output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "unknown".to_string()
    }
}

pub fn platform() -> Box<dyn Platform> {
    Box::new(PythonPlatform)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_commands_exist() {
        let cmds = PythonPlatform.commands();
        assert!(cmds.iter().any(|c| c.verb == "install"));
        assert!(cmds.iter().any(|c| c.verb == "test"));
    }
}
