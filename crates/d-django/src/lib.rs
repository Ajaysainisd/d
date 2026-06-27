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
            CommandDef::new("migrate", &python, "Apply database migrations")
                .with_args(&["manage.py", "migrate"]),
            CommandDef::new("test", &python, "Run Django tests").with_args(&["manage.py", "test"]),
            CommandDef::new("shell", &python, "Open Django shell")
                .with_args(&["manage.py", "shell"]),
            CommandDef::new("clean", "rm", "Clean Python/Django cache").with_args(&[
                "-rf",
                "__pycache__",
                "*.pyc",
                ".pytest_cache",
                "staticfiles",
            ]),
            CommandDef::new("doctor", &python, "Run Django system checks")
                .with_args(&["manage.py", "check"]),
            CommandDef::new("install", &python, "Install dependencies").with_args(&[
                "-m",
                "pip",
                "install",
                "-r",
                "requirements.txt",
            ]),
            CommandDef::new("makemigrations", &python, "Create new migrations")
                .with_args(&["manage.py", "makemigrations"]),
            CommandDef::new("createsuperuser", &python, "Create admin superuser")
                .with_args(&["manage.py", "createsuperuser"]),
            CommandDef::new("collectstatic", &python, "Collect static files").with_args(&[
                "manage.py",
                "collectstatic",
                "--noinput",
            ]),
            CommandDef::new("loaddata", &python, "Load data from fixture")
                .with_args(&["manage.py", "loaddata"]),
            CommandDef::new("dumpdata", &python, "Dump data to fixture")
                .with_args(&["manage.py", "dumpdata"]),
            CommandDef::new("testserver", &python, "Run test server with fixtures")
                .with_args(&["manage.py", "testserver"]),
            CommandDef::new("flush", &python, "Flush database").with_args(&[
                "manage.py",
                "flush",
                "--noinput",
            ]),
            CommandDef::new("squashmigrations", &python, "Squash migrations for an app")
                .with_args(&["manage.py", "squashmigrations"]),
            CommandDef::new("showmigrations", &python, "Show migration status")
                .with_args(&["manage.py", "showmigrations"]),
            CommandDef::new("dbshell", &python, "Open database shell")
                .with_args(&["manage.py", "dbshell"]),
            CommandDef::new("startapp", &python, "Create a new Django app")
                .with_args(&["manage.py", "startapp"]),
            CommandDef::new("startproject", &python, "Create a new Django project").with_args(&[
                "-m",
                "django",
                "startproject",
            ]),
            CommandDef::new("sqlmigrate", &python, "Show SQL for a migration")
                .with_args(&["manage.py", "sqlmigrate"]),
            CommandDef::new("changepassword", &python, "Change user password")
                .with_args(&["manage.py", "changepassword"]),
            CommandDef::new("inspectdb", &python, "Introspect database to models")
                .with_args(&["manage.py", "inspectdb"]),
            CommandDef::new("diffsettings", &python, "Show settings differences")
                .with_args(&["manage.py", "diffsettings"]),
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
        assert!(cmds.iter().any(|c| c.verb == "makemigrations"));
        assert!(cmds.iter().any(|c| c.verb == "createsuperuser"));
        assert!(cmds.iter().any(|c| c.verb == "collectstatic"));
        assert!(cmds.iter().any(|c| c.verb == "flush"));
        assert!(cmds.iter().any(|c| c.verb == "dbshell"));
        assert!(cmds.iter().any(|c| c.verb == "dumpdata"));
        assert!(cmds.iter().any(|c| c.verb == "loaddata"));
        assert!(cmds.iter().any(|c| c.verb == "showmigrations"));
        assert!(cmds.iter().any(|c| c.verb == "squashmigrations"));
        assert!(cmds.len() > 18);
    }
}
