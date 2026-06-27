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
            CommandDef::new("logs", "tail", "Tail Rails development log")
                .with_args(&["-f", "log/development.log"]),
            CommandDef::new("clean", "rm", "Clean temp files and caches").with_args(&[
                "-rf",
                "tmp/cache",
                "tmp/pids",
                "tmp/sockets",
                "log/*.log",
            ]),
            CommandDef::new("install", "bundle", "Install gems").with_args(&["install"]),
            CommandDef::new("lint", "rubocop", "Lint Ruby code").with_args(&[]),
            CommandDef::new("generate", &rails, "Run Rails generator").with_args(&["generate"]),
            CommandDef::new("routes", &rails, "Show all routes").with_args(&["routes"]),
            CommandDef::new("seed", &rails, "Seed the database").with_args(&["db:seed"]),
            CommandDef::new("rollback", &rails, "Rollback last migration")
                .with_args(&["db:rollback"]),
            CommandDef::new("credentials", &rails, "Edit credentials")
                .with_args(&["credentials:edit"]),
            CommandDef::new("about", &rails, "Show Rails version info").with_args(&["about"]),
            CommandDef::new("destroy", &rails, "Destroy a generated resource")
                .with_args(&["destroy"]),
            CommandDef::new("dbcreate", &rails, "Create the database").with_args(&["db:create"]),
            CommandDef::new("dbdrop", &rails, "Drop the database").with_args(&["db:drop"]),
            CommandDef::new("dbreset", &rails, "Drop, create, and migrate the database")
                .with_args(&["db:reset"]),
            CommandDef::new("dbsetup", &rails, "Create and load schema").with_args(&["db:setup"]),
            CommandDef::new("dbschema", &rails, "Dump schema file").with_args(&["db:schema:dump"]),
            CommandDef::new("assets", &rails, "Precompile assets")
                .with_args(&["assets:precompile"]),
            CommandDef::new("doctor", &rails, "Check Rails environment").with_args(&["runner"]),
            CommandDef::new("jobs", &rails, "Process background jobs")
                .with_args(&["solid_queue:start"]),
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
        assert!(cmds.iter().any(|c| c.verb == "generate"));
        assert!(cmds.iter().any(|c| c.verb == "routes"));
        assert!(cmds.iter().any(|c| c.verb == "seed"));
        assert!(cmds.iter().any(|c| c.verb == "rollback"));
        assert!(cmds.iter().any(|c| c.verb == "dbcreate"));
        assert!(cmds.iter().any(|c| c.verb == "dbdrop"));
        assert!(cmds.iter().any(|c| c.verb == "credentials"));
        assert!(cmds.len() > 18);
    }
}
