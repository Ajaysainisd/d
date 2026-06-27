use std::path::Path;
use std::process::Command;

use d_core::platform::Platform;
use d_core::types::{CommandDef, DoctorCheck};

pub struct RailsPlatform;

impl Platform for RailsPlatform {
    fn name(&self) -> &str {
        "rails"
    }

    fn detect(&self, dir: &Path) -> f32 {
        if dir.join("Gemfile").exists() {
            if let Ok(contents) = std::fs::read_to_string(dir.join("Gemfile")) {
                if contents.contains("rails") {
                    return 0.95;
                }
                return 0.5;
            }
            return 0.5;
        }
        if dir.join("config").join("application.rb").exists() {
            return 0.9;
        }
        0.0
    }

    fn commands(&self) -> Vec<CommandDef> {
        let rails = detect_rails_cmd();
        vec![
            CommandDef::new("up", &rails, "Start Rails server").with_args(&["server"]),
            CommandDef::new("run", &rails, "Start Rails server").with_args(&["server"]),
            CommandDef::new("test", &rails, "Run tests").with_args(&["test"]),
            CommandDef::new("migrate", &rails, "Run database migrations")
                .with_args(&["db:migrate"]),
            CommandDef::new("console", &rails, "Open Rails console").with_args(&["console"]),
            CommandDef::new("logs", "tail", "Tail Rails log")
                .with_args(&["-f", "log/development.log"]),
            CommandDef::new("clean", "rm", "Clean temp files").with_args(&[
                "-rf",
                "tmp/cache",
                "tmp/pids",
            ]),
            CommandDef::new("install", "bundle", "Install gems").with_args(&["install"]),
            CommandDef::new("lint", "rubocop", "Lint Ruby code").with_args(&[]),
        ]
    }

    fn doctor_checks(&self, _dir: &Path) -> Vec<DoctorCheck> {
        vec![
            d_core::doctor::check_tool("ruby", "ruby", "sdk"),
            d_core::doctor::check_tool("bundle", "bundler", "tool"),
            d_core::doctor::check_tool("rails", "rails", "tool"),
        ]
    }
}

fn detect_rails_cmd() -> String {
    if Command::new("bin/rails").arg("--version").output().is_ok() {
        "bin/rails".to_string()
    } else {
        "bundle exec rails".to_string()
    }
}

pub fn platform() -> Box<dyn Platform> {
    Box::new(RailsPlatform)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rails_commands_exist() {
        let cmds = RailsPlatform.commands();
        assert!(cmds.iter().any(|c| c.verb == "up"));
        assert!(cmds.iter().any(|c| c.verb == "migrate"));
        assert!(cmds.iter().any(|c| c.verb == "test"));
    }
}
