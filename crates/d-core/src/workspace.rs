use std::path::Path;

use tracing::info;

use crate::config::WorkspaceConfig;

pub fn list_projects(config: &WorkspaceConfig, root_dir: &Path) -> Vec<std::path::PathBuf> {
    config
        .projects
        .iter()
        .map(|p| root_dir.join(p))
        .collect()
}

pub fn is_workspace_root(dir: &Path) -> bool {
    dir.join("workspace.yaml").exists() || {
        if let Some(config) = crate::config::load_d_yaml(dir) {
            config.workspace.is_some()
        } else {
            false
        }
    }
}

pub fn get_workspace_config(dir: &Path) -> Option<WorkspaceConfig> {
    if let Some(config) = crate::config::load_d_yaml(dir) {
        if let Some(ws) = config.workspace {
            info!("Found workspace config in d.yaml");
            return Some(ws);
        }
    }

    crate::config::load_workspace_yaml(dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_projects() {
        let config = WorkspaceConfig {
            projects: vec!["backend".into(), "mobile".into()],
        };
        let root = Path::new("/project");
        let dirs = list_projects(&config, root);
        assert_eq!(dirs, vec![
            std::path::PathBuf::from("/project/backend"),
            std::path::PathBuf::from("/project/mobile"),
        ]);
    }
}
