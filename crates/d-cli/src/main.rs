mod cli;

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::Parser;
use cli::Cli;
use d_core::detect::{detect_platform, find_project_root};
use d_core::hooks;
use d_core::platform::Platform;
use d_core::resolve;
use d_core::types::{ExitCode, ResolvedCommand};
use tracing::{debug, error, info, warn};

fn main() -> ExitCode {
    let cli = Cli::parse();
    d_core::logging::init(&cli.log_level);

    let platforms: Vec<Box<dyn Platform>> = vec![
        d_flutter::platform(),
        d_docker::platform(),
        d_node::platform(),
        d_python::platform(),
        d_django::platform(),
        d_rust::platform(),
        d_go::platform(),
        d_gradle::platform(),
        d_rails::platform(),
        d_swift::platform(),
    ];

    let cwd = match env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            error!("Failed to get current directory: {}", e);
            return ExitCode::Failure;
        }
    };

    let command = match &cli.command {
        Some(cmd) => cmd,
        None => {
            let _ = Cli::try_parse_from(std::iter::empty::<String>());
            return ExitCode::Success;
        }
    };

    if matches!(command, cli::Verb::Doctor) {
        return handle_doctor(&platforms, &cwd);
    }

    let project_root = find_project_root(&cwd);
    debug!("Project root: {:?}", project_root);

    let working_dir = project_root.unwrap_or_else(|| cwd.clone());

    let configured_type = cli.project_type.as_deref();
    let platform = detect_platform(&working_dir, configured_type, &platforms);
    debug!(
        "Detected platform: {:?}",
        platform.as_ref().map(|p| p.name())
    );

    let verb = command.name();
    let target = command.target();
    let variant = command.variant();

    let project_config = d_core::config::load_d_yaml(&working_dir);

    let mut cli_env = HashMap::new();
    if let Some(ref env_name) = cli.env {
        cli_env.insert("D_ENV".to_string(), env_name.clone());
    }

    let resolved = resolve::resolve_command(
        verb,
        target,
        variant,
        platform,
        project_config.as_ref(),
        &cli_env,
    );

    match resolved {
        Some(cmd) => {
            if cli.dry_run {
                print_dry_run(&cmd);
                return ExitCode::Success;
            }

            if verb == "up" || verb == "down" || verb == "restart" {
                let workspace_config = d_core::workspace::get_workspace_config(&working_dir);
                if let Some(ws_config) = workspace_config {
                    if !ws_config.projects.is_empty() {
                        return execute_workspace(
                            &ws_config.projects,
                            &working_dir,
                            verb,
                            &cli,
                            &platforms,
                        );
                    }
                }
            }

            execute_command(cmd, project_config.as_ref(), &cli)
        }
        None => {
            error!(
                "No command found for '{}' {}. Unknown command for this project type.",
                verb,
                target.unwrap_or("")
            );
            error!("Run 'd --help' for usage.");
            ExitCode::InvalidCommand
        }
    }
}

fn execute_workspace(
    projects: &[String],
    root_dir: &std::path::Path,
    verb: &str,
    cli: &Cli,
    platforms: &[Box<dyn Platform>],
) -> ExitCode {
    info!("Workspace: {} projects", projects.len());

    let mut exit = ExitCode::Success;

    for project in projects {
        let project_dir = root_dir.join(project);
        if !project_dir.exists() {
            warn!("Project directory '{}' does not exist, skipping", project);
            continue;
        }

        info!("→ {}", project);
        let cwd = &project_dir;

        let platform = detect_platform(cwd, cli.project_type.as_deref(), platforms);
        let project_config = d_core::config::load_d_yaml(cwd);
        let mut cli_env = HashMap::new();
        if let Some(ref env_name) = cli.env {
            cli_env.insert("D_ENV".to_string(), env_name.clone());
        }

        let resolved = resolve::resolve_command(
            verb,
            None,
            None,
            platform,
            project_config.as_ref(),
            &cli_env,
        );

        match resolved {
            Some(mut cmd) => {
                cmd.working_dir = Some(cwd.to_path_buf());
                let result = execute_command(cmd, project_config.as_ref(), cli);
                if result != ExitCode::Success {
                    exit = result;
                }
            }
            None => {
                warn!("  No '{}' command for project {}, skipping", verb, project);
            }
        }
    }

    exit
}

fn execute_command(
    cmd: ResolvedCommand,
    config: Option<&d_core::config::ProjectConfig>,
    cli: &Cli,
) -> ExitCode {
    let working_dir = cmd
        .working_dir
        .as_deref()
        .map(|d| d.to_path_buf())
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    info!("Executing: {} {}", cmd.executable, cmd.args.join(" "));

    if !cli.no_hooks {
        if let Some(cfg) = config {
            if let Some(ref hooks_cfg) = cfg.hooks {
                if !hooks::run_pre_hook(hooks_cfg, &cmd.verb) {
                    error!("Pre-hook failed, aborting command");
                    return ExitCode::Failure;
                }
            }
        }
    }

    let mut child = match Command::new(&cmd.executable)
        .args(&cmd.args)
        .current_dir(&working_dir)
        .envs(&cmd.env_vars)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                error!(
                    "'{}' not found. Is it installed and on your PATH?",
                    cmd.executable
                );
                return ExitCode::DependencyMissing;
            }
            error!("Failed to execute '{}': {}", cmd.executable, e);
            return ExitCode::Failure;
        }
    };

    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to wait on process: {}", e);
            return ExitCode::Failure;
        }
    };

    if !cli.no_hooks {
        if let Some(cfg) = config {
            if let Some(ref hooks_cfg) = cfg.hooks {
                hooks::run_post_hook(hooks_cfg, &cmd.verb, false);
            }
        }
    }

    if status.success() {
        info!("Command completed successfully");
        ExitCode::Success
    } else {
        let code = status.code().unwrap_or(1);
        error!("Command exited with code: {}", code);
        ExitCode::Failure
    }
}

fn print_dry_run(cmd: &ResolvedCommand) {
    println!("Would execute:");
    println!("  Executable: {}", cmd.executable);
    println!("  Args: {}", cmd.args.join(" "));
    if let Some(ref dir) = cmd.working_dir {
        println!("  Working dir: {}", dir.display());
    }
    if !cmd.env_vars.is_empty() {
        println!("  Environment:");
        for (k, v) in &cmd.env_vars {
            println!("    {}={}", k, v);
        }
    }
    println!("  Source: {:?}", cmd.source);
}

fn handle_doctor(platforms: &[Box<dyn Platform>], cwd: &std::path::Path) -> ExitCode {
    let project_root = find_project_root(cwd);
    let working_dir = project_root.as_deref().unwrap_or(cwd);

    println!("\n🔍 d doctor\n");

    let universal = d_core::doctor::builtin_checks();
    let platform = detect_platform(working_dir, None, platforms);
    let platform_checks = platform
        .map(|p| p.doctor_checks(working_dir))
        .unwrap_or_default();

    let all_checks = d_core::doctor::run_doctor(working_dir, &universal, &platform_checks);

    let mut pass = 0;
    let mut warn = 0;
    let mut fail = 0;

    for check in &all_checks {
        let icon = match check.status {
            d_core::types::DoctorStatus::Pass => {
                pass += 1;
                "✓"
            }
            d_core::types::DoctorStatus::Warn => {
                warn += 1;
                "⚠"
            }
            d_core::types::DoctorStatus::Fail => {
                fail += 1;
                "✗"
            }
        };
        println!("  {} {}", icon, check.name);
        if let Some(ref msg) = check.message {
            println!("    {}", msg);
        }
        if let Some(ref suggestion) = check.suggestion {
            println!("    → {}", suggestion);
        }
    }

    println!(
        "\n  {} passed, {} warnings, {} failures\n",
        pass, warn, fail
    );

    if fail > 0 {
        ExitCode::DependencyMissing
    } else {
        ExitCode::Success
    }
}
