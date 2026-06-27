use std::path::Path;
use std::process::Command;

use d_core::platform::Platform;
use d_core::types::{CommandDef, DoctorCheck, DoctorStatus};

pub struct GradlePlatform;

impl Platform for GradlePlatform {
    fn name(&self) -> &str {
        "gradle"
    }

    fn detect(&self, dir: &Path) -> f32 {
        if dir.join("build.gradle").exists() || dir.join("build.gradle.kts").exists() {
            return 0.95;
        }
        if dir.join("pom.xml").exists() {
            return 0.8;
        }
        if dir.join("settings.gradle").exists() || dir.join("settings.gradle.kts").exists() {
            return 0.7;
        }
        if dir.join("build.xml").exists() {
            return 0.5;
        }
        0.0
    }

    fn commands(&self) -> Vec<CommandDef> {
        let tool = detect_build_tool();
        match tool.as_str() {
            "gradle" => vec![
                CommandDef::new("build", "gradle", "Build the project").with_args(&["build"]),
                CommandDef::new("test", "gradle", "Run tests").with_args(&["test"]),
                CommandDef::new("run", "gradle", "Run the application").with_args(&["run"]),
                CommandDef::new("clean", "gradle", "Clean build artifacts").with_args(&["clean"]),
                CommandDef::new("lint", "gradle", "Run lint checks").with_args(&["lint"]),
                CommandDef::new("format", "gradle", "Format with spotless")
                    .with_args(&["spotlessApply"]),
                CommandDef::new("install", "gradle", "Set up dependencies")
                    .with_args(&["dependencies"]),
                CommandDef::new("release", "gradle", "Create release build")
                    .with_args(&["assembleRelease"]),
            ],
            _ => vec![
                CommandDef::new("build", "mvn", "Build the project")
                    .with_args(&["package", "-DskipTests"]),
                CommandDef::new("test", "mvn", "Run tests").with_args(&["test"]),
                CommandDef::new("clean", "mvn", "Clean build artifacts").with_args(&["clean"]),
                CommandDef::new("install", "mvn", "Install dependencies").with_args(&["install"]),
            ],
        }
    }

    fn doctor_checks(&self, _dir: &Path) -> Vec<DoctorCheck> {
        let tool = detect_build_tool();
        vec![
            d_core::doctor::check_tool(&tool, "build-tool", "tool"),
            d_core::doctor::check_tool("java", "java", "sdk"),
            DoctorCheck {
                name: "java-version".to_string(),
                category: "sdk".to_string(),
                status: check_java_version(),
                message: Some(get_java_version()),
                suggestion: None,
            },
        ]
    }
}

fn detect_build_tool() -> String {
    if Command::new("gradle").arg("--version").output().is_ok() {
        "gradle".to_string()
    } else if Command::new("mvn").arg("--version").output().is_ok() {
        "mvn".to_string()
    } else {
        "gradle".to_string()
    }
}

fn check_java_version() -> DoctorStatus {
    if Command::new("java").arg("--version").output().is_ok() {
        DoctorStatus::Pass
    } else {
        DoctorStatus::Fail
    }
}

fn get_java_version() -> String {
    if let Ok(output) = Command::new("java").arg("--version").output() {
        let s = String::from_utf8_lossy(&output.stderr);
        s.lines().next().unwrap_or("unknown").to_string()
    } else {
        "unknown".to_string()
    }
}

pub fn platform() -> Box<dyn Platform> {
    Box::new(GradlePlatform)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradle_commands_exist() {
        let cmds = GradlePlatform.commands();
        assert!(cmds.iter().any(|c| c.verb == "build"));
        assert!(cmds.iter().any(|c| c.verb == "test"));
    }
}
