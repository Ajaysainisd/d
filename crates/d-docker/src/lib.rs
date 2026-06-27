use std::path::Path;
use std::process::Command;

use d_core::platform::Platform;
use d_core::types::{CommandDef, DoctorCheck, DoctorStatus};

pub struct DockerPlatform;

impl Platform for DockerPlatform {
    fn name(&self) -> &str {
        "docker"
    }

    fn detect(&self, dir: &Path) -> f32 {
        let compose_file = dir.join("docker-compose.yml");
        let compose_yaml = dir.join("compose.yaml");
        let compose_yml = dir.join("compose.yml");
        let dockerfile = dir.join("Dockerfile");

        if compose_file.exists() || compose_yaml.exists() || compose_yml.exists() {
            return 0.95;
        }
        if dockerfile.exists() {
            return 0.6;
        }
        0.0
    }

    fn commands(&self) -> Vec<CommandDef> {
        vec![
            CommandDef::new("up", "docker", "Start services")
                .with_variants(&["detached", "build", "force-recreate"])
                .with_args(&["compose", "up"]),
            CommandDef::new("down", "docker", "Stop and remove services")
                .with_variants(&["volumes", "images"])
                .with_args(&["compose", "down"]),
            CommandDef::new("restart", "docker", "Restart services")
                .with_targets(&[""])
                .with_args(&["compose", "restart"]),
            CommandDef::new("logs", "docker", "View service logs")
                .with_targets(&[""])
                .with_args(&["compose", "logs"]),
            CommandDef::new("build", "docker", "Build service images")
                .with_targets(&[""])
                .with_args(&["compose", "build"]),
            CommandDef::new("shell", "docker", "Open a shell in a service container")
                .with_targets(&[""])
                .with_args(&["compose", "exec", "{target}", "sh"]),
            CommandDef::new("test", "docker", "Run tests via compose")
                .with_args(&["compose", "run", "--rm", "test"]),
            CommandDef::new("ps", "docker", "List running containers")
                .with_args(&["compose", "ps"]),
            CommandDef::new("pull", "docker", "Pull service images")
                .with_args(&["compose", "pull"]),
            CommandDef::new("push", "docker", "Push service images")
                .with_args(&["compose", "push"]),
            CommandDef::new("stop", "docker", "Stop running services")
                .with_targets(&[""])
                .with_args(&["compose", "stop"]),
            CommandDef::new("start", "docker", "Start stopped services")
                .with_targets(&[""])
                .with_args(&["compose", "start"]),
            CommandDef::new("rm", "docker", "Remove stopped service containers")
                .with_args(&["compose", "rm", "-f"]),
            CommandDef::new("run", "docker", "Run a one-off command in a service")
                .with_targets(&[""])
                .with_args(&["compose", "run", "--rm", "{target}"]),
            CommandDef::new("exec", "docker", "Execute a command in a running service")
                .with_targets(&[""])
                .with_args(&["compose", "exec", "{target}"]),
            CommandDef::new("config", "docker", "Validate and view compose config")
                .with_args(&["compose", "config"]),
            CommandDef::new("images", "docker", "List images").with_args(&["compose", "images"]),
            CommandDef::new("events", "docker", "Stream container events")
                .with_args(&["compose", "events"]),
            CommandDef::new("top", "docker", "Display running processes in a service")
                .with_targets(&[""])
                .with_args(&["compose", "top"]),
            CommandDef::new("install", "docker", "Pull all service images")
                .with_args(&["compose", "pull"]),
            CommandDef::new("doctor", "docker", "Check Docker setup")
                .with_args(&["compose", "version"]),
            CommandDef::new(
                "clean",
                "docker",
                "Remove stopped containers, unused networks, images",
            )
            .with_args(&["compose", "down", "--rmi", "all", "--volumes"]),
        ]
    }

    fn doctor_checks(&self, _dir: &Path) -> Vec<DoctorCheck> {
        vec![
            d_core::doctor::check_tool("docker", "docker-cli", "tool"),
            check_docker_daemon(),
        ]
    }
}

fn check_docker_daemon() -> DoctorCheck {
    match Command::new("docker").arg("info").output() {
        Ok(output) if output.status.success() => DoctorCheck {
            name: "docker-daemon".to_string(),
            category: "tool".to_string(),
            status: DoctorStatus::Pass,
            message: Some("Docker daemon is running".to_string()),
            suggestion: None,
        },
        Ok(_) => DoctorCheck {
            name: "docker-daemon".to_string(),
            category: "tool".to_string(),
            status: DoctorStatus::Fail,
            message: Some("Docker daemon is not running".to_string()),
            suggestion: Some(
                "Start Docker Desktop or run `sudo systemctl start docker`".to_string(),
            ),
        },
        Err(e) => DoctorCheck {
            name: "docker-daemon".to_string(),
            category: "tool".to_string(),
            status: DoctorStatus::Fail,
            message: Some(format!("Cannot connect to Docker: {}", e)),
            suggestion: Some("Ensure Docker is installed and running".to_string()),
        },
    }
}

pub fn platform() -> Box<dyn Platform> {
    Box::new(DockerPlatform)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docker_commands_exist() {
        let cmds = DockerPlatform.commands();
        assert!(cmds.iter().any(|c| c.verb == "up"));
        assert!(cmds.iter().any(|c| c.verb == "down"));
        assert!(cmds.iter().any(|c| c.verb == "logs"));
        assert!(cmds.iter().any(|c| c.verb == "ps"));
        assert!(cmds.iter().any(|c| c.verb == "pull"));
        assert!(cmds.iter().any(|c| c.verb == "push"));
        assert!(cmds.iter().any(|c| c.verb == "stop"));
        assert!(cmds.iter().any(|c| c.verb == "start"));
        assert!(cmds.iter().any(|c| c.verb == "exec"));
        assert!(cmds.iter().any(|c| c.verb == "config"));
        assert!(cmds.iter().any(|c| c.verb == "images"));
        assert!(cmds.len() > 15);
    }
}
