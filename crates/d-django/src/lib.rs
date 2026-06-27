use std::path::Path;

use d_core::platform::Platform;
use d_core::types::{CommandDef, DoctorCheck};

pub struct DjangoPlatform;

impl Platform for DjangoPlatform {
    fn name(&self) -> &str {
        "django"
    }

    fn detect(&self, dir: &Path) -> f32 {
        if !dir.join("manage.py").exists() {
            return 0.0;
        }
        if let Ok(contents) = std::fs::read_to_string(dir.join("manage.py")) {
            if contents.contains("django") {
                return 0.95;
            }
        }
        0.85
    }

    fn commands(&self) -> Vec<CommandDef> {
        let python = detect_python();
        vec![
            CommandDef::new("run", &python, "Run Django dev server")
                .with_args(&["manage.py", "runserver"]),
            CommandDef::new("migrate", &python, "Run database migrations")
                .with_args(&["manage.py", "migrate"]),
            CommandDef::new("test", &python, "Run Django tests").with_args(&["manage.py", "test"]),
            CommandDef::new("shell", &python, "Open Django shell")
                .with_args(&["manage.py", "shell"]),
            CommandDef::new("clean", "rm", "Clean Python cache").with_args(&[
                "-rf",
                "__pycache__",
                "*.pyc",
                ".pytest_cache",
            ]),
            CommandDef::new("doctor", &python, "Run Django checks")
                .with_args(&["manage.py", "check"]),
            CommandDef::new("install", &python, "Install dependencies").with_args(&[
                "-m",
                "pip",
                "install",
                "-r",
                "requirements.txt",
            ]),
        ]
    }

    fn doctor_checks(&self, _dir: &Path) -> Vec<DoctorCheck> {
        let python = detect_python();
        vec![
            d_core::doctor::check_tool(&python, "python", "sdk"),
            d_core::doctor::check_tool("pip", "pip", "tool"),
            d_core::doctor::check_command_run(
                &python,
                &["-c", "import django; print(django.VERSION)"],
                "django-installed",
                "sdk",
            ),
        ]
    }
}

fn detect_python() -> String {
    "python3".to_string()
}

pub fn platform() -> Box<dyn Platform> {
    Box::new(DjangoPlatform)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_django_commands_exist() {
        let cmds = DjangoPlatform.commands();
        assert!(cmds.iter().any(|c| c.verb == "run"));
        assert!(cmds.iter().any(|c| c.verb == "migrate"));
        assert!(cmds.iter().any(|c| c.verb == "shell"));
    }
}
