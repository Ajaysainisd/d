use std::process::Command;

use tracing::{info, warn};

use crate::config::HooksConfig;

pub fn run_pre_hook(config: &HooksConfig, verb: &str) -> bool {
    if let Some(hook) = config.get_pre_hook(verb) {
        info!("Running pre-{} hook: {}", verb, hook);
        match Command::new("sh").arg("-c").arg(hook).status() {
            Ok(status) if status.success() => {
                info!("Pre-{} hook succeeded", verb);
                true
            }
            Ok(status) => {
                warn!("Pre-{} hook failed with exit code: {}", verb, status);
                false
            }
            Err(e) => {
                warn!("Pre-{} hook execution failed: {}", verb, e);
                false
            }
        }
    } else {
        true
    }
}

pub fn run_post_hook(config: &HooksConfig, verb: &str, skip: bool) {
    if skip {
        return;
    }
    if let Some(hook) = config.get_post_hook(verb) {
        info!("Running post-{} hook: {}", verb, hook);
        match Command::new("sh").arg("-c").arg(hook).status() {
            Ok(status) if status.success() => {
                info!("Post-{} hook succeeded", verb);
            }
            Ok(status) => {
                warn!("Post-{} hook exited with status: {}", verb, status);
            }
            Err(e) => {
                warn!("Post-{} hook execution failed: {}", verb, e);
            }
        }
    }
}
