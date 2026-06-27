use std::path::Path;

use d_core::platform::Platform;
use d_core::types::{CommandDef, DoctorCheck};

pub struct FlutterPlatform;

impl Platform for FlutterPlatform {
    fn name(&self) -> &str {
        "flutter"
    }

    fn detect(&self, dir: &Path) -> f32 {
        let pubspec = dir.join("pubspec.yaml");
        if !pubspec.exists() {
            return 0.0;
        }
        if let Ok(contents) = std::fs::read_to_string(&pubspec) {
            if contents.contains("flutter") || contents.contains("sdk: flutter") {
                return 0.95;
            }
            if contents.contains("dart") {
                return 0.7;
            }
            return 0.5;
        }
        0.3
    }

    fn commands(&self) -> Vec<CommandDef> {
        vec![
            CommandDef::new("run", "flutter", "Run the Flutter application")
                .with_targets(&["web", "ios", "android", "macos", "linux", "windows"])
                .with_args(&["run"]),
            CommandDef::new("build", "flutter", "Build the Flutter application")
                .with_targets(&["ios", "android", "web", "macos", "linux", "windows"])
                .with_args(&["build", "{target}"]),
            CommandDef::new("test", "flutter", "Run Flutter tests")
                .with_args(&["test"]),
            CommandDef::new("clean", "flutter", "Clean build artifacts")
                .with_args(&["clean"]),
            CommandDef::new("format", "dart", "Format Dart code")
                .with_args(&["format", "."]),
            CommandDef::new("doctor", "flutter", "Run Flutter doctor")
                .with_args(&["doctor"]),
            CommandDef::new("lint", "dart", "Analyze Dart code")
                .with_args(&["analyze"]),
        ]
    }

    fn doctor_checks(&self, _dir: &Path) -> Vec<DoctorCheck> {
        vec![
            d_core::doctor::check_tool("flutter", "flutter-sdk", "sdk"),
            d_core::doctor::check_tool("dart", "dart-sdk", "sdk"),
        ]
    }
}

pub fn platform() -> Box<dyn Platform> {
    Box::new(FlutterPlatform)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flutter_commands_exist() {
        let cmds = FlutterPlatform.commands();
        assert!(cmds.len() >= 5);
        assert!(cmds.iter().any(|c| c.verb == "run"));
        assert!(cmds.iter().any(|c| c.verb == "build"));
    }
}
