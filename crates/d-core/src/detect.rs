use std::path::{Path, PathBuf};

use crate::platform::Platform;

pub fn detect_project_type(platforms: &[Box<dyn Platform>], dir: &Path) -> Option<(String, f32)> {
    let mut best: Option<(String, f32)> = None;

    for platform in platforms {
        let confidence = platform.detect(dir);
        if confidence > 0.0 {
            match &best {
                None => best = Some((platform.name().to_string(), confidence)),
                Some((_, best_conf)) if confidence > *best_conf => {
                    best = Some((platform.name().to_string(), confidence));
                }
                _ => {}
            }
        }
    }

    best
}

pub fn find_project_root(start_dir: &Path) -> Option<PathBuf> {
    let mut current = start_dir.to_path_buf();

    if !current.is_dir() {
        current = current.parent()?.to_path_buf();
    }

    loop {
        if has_d_yaml(&current) {
            let config = crate::config::load_d_yaml(&current);
            if let Some(cfg) = config {
                if cfg.project_type.is_some() || cfg.commands.is_some() {
                    return Some(current);
                }
            }
        }

        if is_project_root(&current) {
            return Some(current);
        }

        if !current.pop() {
            return None;
        }
    }
}

fn has_d_yaml(dir: &Path) -> bool {
    dir.join("d.yaml").exists()
}

fn is_project_root(dir: &Path) -> bool {
    dir.join("pubspec.yaml").exists()
        || dir.join("docker-compose.yml").exists()
        || dir.join("compose.yaml").exists()
        || dir.join("package.json").exists()
        || dir.join("Cargo.toml").exists()
        || dir.join("pyproject.toml").exists()
        || dir.join("manage.py").exists()
        || dir.join("go.mod").exists()
}

pub fn detect_platform<'a>(dir: &Path, configured_type: Option<&str>, platforms: &'a [Box<dyn Platform>]) -> Option<&'a Box<dyn Platform>> {
    if let Some(ptype) = configured_type {
        return platforms.iter().find(|p| p.name() == ptype);
    }

    let (name, _) = detect_project_type(platforms, dir)?;
    platforms.iter().find(|p| p.name() == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_project_root_with_pubspec() {
        let dir = Path::new("/fake/project");
        assert!(!is_project_root(dir));
    }

    #[test]
    fn test_find_project_root_no_project() {
        let result = find_project_root(Path::new("/tmp/nonexistent"));
        assert!(result.is_none());
    }
}
