use std::path::Path;
use std::process::Command;

use tracing::{debug, info};

use crate::types::{DoctorCheck, DoctorStatus};

pub fn run_doctor(
    working_dir: &Path,
    universal_checks: &[DoctorCheck],
    platform_checks: &[DoctorCheck],
) -> Vec<DoctorCheck> {
    let mut all = vec![];

    info!("Running doctor checks...");
    info!("  Working directory: {}", working_dir.display());

    let mut universal = universal_checks.to_vec();
    for check in &mut universal {
        debug!("  Running universal check: {}", check.name);
    }
    all.append(&mut universal);

    let mut platform = platform_checks.to_vec();
    for check in &mut platform {
        debug!("  Running platform check: {}", check.name);
    }
    all.append(&mut platform);

    all
}

pub fn check_tool(name: &str, check_name: &str, category: &str) -> DoctorCheck {
    match which::which(name) {
        Ok(path) => DoctorCheck {
            name: check_name.to_string(),
            category: category.to_string(),
            status: DoctorStatus::Pass,
            message: Some(format!("{} found at {}", name, path.display())),
            suggestion: None,
        },
        Err(_) => DoctorCheck {
            name: check_name.to_string(),
            category: category.to_string(),
            status: DoctorStatus::Fail,
            message: Some(format!("{} not found on PATH", name)),
            suggestion: Some(format!("Install {} and ensure it is on your PATH", name)),
        },
    }
}

pub fn check_command_run(
    name: &str,
    args: &[&str],
    check_name: &str,
    category: &str,
) -> DoctorCheck {
    match Command::new(name).args(args).output() {
        Ok(output) if output.status.success() => DoctorCheck {
            name: check_name.to_string(),
            category: category.to_string(),
            status: DoctorStatus::Pass,
            message: Some(format!("{} {} ran successfully", name, args.join(" "))),
            suggestion: None,
        },
        Ok(output) => DoctorCheck {
            name: check_name.to_string(),
            category: category.to_string(),
            status: DoctorStatus::Warn,
            message: Some(format!("{} exited with status: {}", name, output.status)),
            suggestion: Some(format!("Check {} installation", name)),
        },
        Err(e) => DoctorCheck {
            name: check_name.to_string(),
            category: category.to_string(),
            status: DoctorStatus::Fail,
            message: Some(format!("{} not available: {}", name, e)),
            suggestion: Some(format!("Install {}", name)),
        },
    }
}

pub fn builtin_checks() -> Vec<DoctorCheck> {
    vec![
        check_tool("git", "git", "tool"),
        check_tool("which", "command-lookup", "tool"),
    ]
}
