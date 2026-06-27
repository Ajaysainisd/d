use std::path::Path;
use std::process::Command;

use d_core::platform::Platform;
use d_core::types::{CommandDef, DoctorCheck, DoctorStatus};

pub struct SwiftPlatform;

impl Platform for SwiftPlatform {
    fn name(&self) -> &str {
        "swift"
    }

    fn detect(&self, dir: &Path) -> f32 {
        if dir.join("Package.swift").exists() {
            return 0.95;
        }
        let has_xcodeproj = std::fs::read_dir(dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .any(|e| e.path().extension().is_some_and(|ext| ext == "xcodeproj"))
            })
            .unwrap_or(false);
        if has_xcodeproj {
            return 0.9;
        }
        let has_xcworkspace = std::fs::read_dir(dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .any(|e| e.path().extension().is_some_and(|ext| ext == "xcworkspace"))
            })
            .unwrap_or(false);
        if has_xcworkspace {
            return 0.9;
        }
        if dir.join("Sources").exists() || dir.join("Tests").exists() {
            return 0.5;
        }
        0.0
    }

    fn commands(&self) -> Vec<CommandDef> {
        let spm = dir_has_spm();
        if spm {
            vec![
                CommandDef::new("build", "swift", "Build with SwiftPM").with_args(&["build"]),
                CommandDef::new("run", "swift", "Run with SwiftPM").with_args(&["run"]),
                CommandDef::new("test", "swift", "Run tests").with_args(&["test"]),
                CommandDef::new("clean", "swift", "Clean build artifacts")
                    .with_args(&["package", "clean"]),
                CommandDef::new("lint", "swift", "Lint with SwiftLint if available")
                    .with_args(&["lint"]),
                CommandDef::new("format", "swift", "Format with swift-format if available")
                    .with_args(&["format"]),
                CommandDef::new("release", "swift", "Build in release mode")
                    .with_args(&["build", "-c", "release"]),
                CommandDef::new("install", "swift", "Resolve dependencies")
                    .with_args(&["package", "resolve"]),
            ]
        } else {
            vec![
                CommandDef::new("build", "xcodebuild", "Build with Xcode").with_args(&["build"]),
                CommandDef::new("test", "xcodebuild", "Run tests").with_args(&["test"]),
                CommandDef::new("clean", "xcodebuild", "Clean build artifacts")
                    .with_args(&["clean"]),
                CommandDef::new("run", "open", "Open Xcode project").with_args(&["*.xcodeproj"]),
            ]
        }
    }

    fn doctor_checks(&self, _dir: &Path) -> Vec<DoctorCheck> {
        vec![
            d_core::doctor::check_tool("swift", "swift-sdk", "sdk"),
            d_core::doctor::check_tool("xcodebuild", "xcode-cli", "tool"),
            DoctorCheck {
                name: "swift-version".to_string(),
                category: "sdk".to_string(),
                status: check_swift_version(),
                message: Some(get_swift_version()),
                suggestion: None,
            },
        ]
    }
}

fn dir_has_spm() -> bool {
    Path::new("Package.swift").exists()
}

fn check_swift_version() -> DoctorStatus {
    if Command::new("swift").arg("--version").output().is_ok() {
        DoctorStatus::Pass
    } else {
        DoctorStatus::Fail
    }
}

fn get_swift_version() -> String {
    if let Ok(output) = Command::new("swift").arg("--version").output() {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()
            .unwrap_or("unknown")
            .to_string()
    } else {
        "unknown".to_string()
    }
}

pub fn platform() -> Box<dyn Platform> {
    Box::new(SwiftPlatform)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swift_commands_exist() {
        let cmds = SwiftPlatform.commands();
        assert!(cmds.iter().any(|c| c.verb == "build"));
        assert!(cmds.iter().any(|c| c.verb == "test"));
    }
}
