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
                .with_variants(&["debug", "release", "profile"])
                .with_args(&["build", "{target}"]),
            CommandDef::new("test", "flutter", "Run Flutter tests").with_args(&["test"]),
            CommandDef::new("clean", "flutter", "Clean build artifacts").with_args(&["clean"]),
            CommandDef::new("format", "dart", "Format Dart code").with_args(&["format", "."]),
            CommandDef::new("lint", "dart", "Analyze Dart code").with_args(&["analyze"]),
            CommandDef::new("doctor", "flutter", "Run Flutter doctor").with_args(&["doctor"]),
            CommandDef::new("install", "flutter", "Install dependencies (pub get)")
                .with_args(&["pub", "get"]),
            CommandDef::new("analyze", "flutter", "Analyze the project").with_args(&["analyze"]),
            CommandDef::new("upgrade", "flutter", "Upgrade Flutter SDK").with_args(&["upgrade"]),
            CommandDef::new("gen", "flutter", "Run code generation (build_runner)").with_args(&[
                "pub",
                "run",
                "build_runner",
                "build",
            ]),
            CommandDef::new("create", "flutter", "Create a new Flutter project")
                .with_targets(&[""])
                .with_args(&["create", "{target}"]),
            CommandDef::new("drive", "flutter", "Run integration tests (drive)")
                .with_args(&["drive"]),
            CommandDef::new("pub", "flutter", "Run pub commands").with_args(&["pub"]),
            CommandDef::new("attach", "flutter", "Attach to a running Flutter app")
                .with_args(&["attach"]),
            CommandDef::new("config", "flutter", "Configure Flutter settings")
                .with_args(&["config"]),
            CommandDef::new("logs", "flutter", "Show Flutter device logs").with_args(&["logs"]),
            CommandDef::new("devices", "flutter", "List connected devices").with_args(&["devices"]),
            CommandDef::new("emulators", "flutter", "List/create emulators")
                .with_args(&["emulators"]),
            CommandDef::new("screenshot", "flutter", "Take a screenshot")
                .with_args(&["screenshot"]),
            CommandDef::new("precache", "flutter", "Precache platform artifacts")
                .with_args(&["precache"]),
            CommandDef::new("update-packages", "flutter", "Update packages")
                .with_args(&["pub", "upgrade"]),
            CommandDef::new("downgrade", "flutter", "Downgrade Flutter SDK")
                .with_args(&["downgrade"]),
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
        assert!(cmds.iter().any(|c| c.verb == "run"));
        assert!(cmds.iter().any(|c| c.verb == "build"));
        assert!(cmds.iter().any(|c| c.verb == "test"));
        assert!(cmds.iter().any(|c| c.verb == "format"));
        assert!(cmds.iter().any(|c| c.verb == "lint"));
        assert!(cmds.iter().any(|c| c.verb == "install"));
        assert!(cmds.iter().any(|c| c.verb == "analyze"));
        assert!(cmds.iter().any(|c| c.verb == "create"));
        assert!(cmds.iter().any(|c| c.verb == "logs"));
        assert!(cmds.len() > 15);
    }
}
