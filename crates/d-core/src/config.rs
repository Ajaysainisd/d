use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::types::CommandDef;

#[derive(Debug, Deserialize, Default)]
pub struct ProjectConfig {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub project_type: Option<String>,
    pub commands: Option<CommandsConfig>,
    pub workspace: Option<WorkspaceConfig>,
    pub hooks: Option<HooksConfig>,
}

#[derive(Debug, Deserialize, Default)]
pub struct CommandsConfig {
    #[serde(flatten)]
    pub verbs: HashMap<String, VerbConfig>,
}

#[derive(Debug, Deserialize, Default)]
pub struct VerbConfig {
    #[serde(flatten)]
    pub targets: HashMap<String, TargetConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TargetConfig {
    Simple(String),
    Variants(HashMap<String, VariantValue>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VariantValue {
    Command(String),
    Nested(HashMap<String, String>),
}

#[derive(Debug, Deserialize, Default)]
pub struct WorkspaceConfig {
    pub projects: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct HooksConfig {
    #[serde(rename = "pre-run")]
    pub pre_run: Option<String>,
    #[serde(rename = "post-run")]
    pub post_run: Option<String>,
    #[serde(rename = "pre-build")]
    pub pre_build: Option<String>,
    #[serde(rename = "post-build")]
    pub post_build: Option<String>,
    #[serde(rename = "pre-test")]
    pub pre_test: Option<String>,
    #[serde(rename = "post-test")]
    pub post_test: Option<String>,
    #[serde(rename = "pre-up")]
    pub pre_up: Option<String>,
    #[serde(rename = "post-up")]
    pub post_up: Option<String>,
    #[serde(rename = "pre-down")]
    pub pre_down: Option<String>,
    #[serde(rename = "post-down")]
    pub post_down: Option<String>,
}

impl HooksConfig {
    pub fn get_pre_hook(&self, verb: &str) -> Option<&str> {
        match verb {
            "run" => self.pre_run.as_deref(),
            "build" => self.pre_build.as_deref(),
            "test" => self.pre_test.as_deref(),
            "up" => self.pre_up.as_deref(),
            "down" => self.pre_down.as_deref(),
            _ => None,
        }
    }

    pub fn get_post_hook(&self, verb: &str) -> Option<&str> {
        match verb {
            "run" => self.post_run.as_deref(),
            "build" => self.post_build.as_deref(),
            "test" => self.post_test.as_deref(),
            "up" => self.post_up.as_deref(),
            "down" => self.post_down.as_deref(),
            _ => None,
        }
    }
}

pub fn load_d_yaml(dir: &Path) -> Option<ProjectConfig> {
    let config_path = dir.join("d.yaml");
    if !config_path.exists() {
        return None;
    }
    let content = fs::read_to_string(&config_path).ok()?;
    serde_yaml::from_str(&content).ok()
}

pub fn load_workspace_yaml(dir: &Path) -> Option<WorkspaceConfig> {
    let config_path = dir.join("workspace.yaml");
    if !config_path.exists() {
        return None;
    }
    let content = fs::read_to_string(&config_path).ok()?;
    let raw: RawWorkspace = serde_yaml::from_str(&content).ok()?;
    Some(WorkspaceConfig {
        projects: raw.workspace,
    })
}

#[derive(Debug, Deserialize)]
struct RawWorkspace {
    workspace: Vec<String>,
}

pub fn parse_d_yaml_override(
    config: &ProjectConfig,
    verb: &str,
    target: &Option<String>,
    variant: &Option<String>,
) -> Option<CommandDef> {
    let verbs = config.commands.as_ref()?;
    let verb_cfg = verbs.verbs.get(verb)?;

    let target = target.as_deref().unwrap_or("default");

    let target_cfg = verb_cfg
        .targets
        .get(target)
        .or_else(|| verb_cfg.targets.get("default"))?;

    match target_cfg {
        TargetConfig::Simple(cmd_str) => {
            let parts: Vec<&str> = cmd_str.split_whitespace().collect();
            if parts.is_empty() {
                return None;
            }
            Some(CommandDef {
                verb: verb.to_string(),
                targets: vec![],
                variants: vec![],
                executable: parts[0].to_string(),
                args_template: parts[1..].iter().map(|s| s.to_string()).collect(),
                description: String::new(),
                env_vars: HashMap::new(),
            })
        }
        TargetConfig::Variants(variants_map) => {
            let vkey = variant.as_deref().unwrap_or("default");
            let cmd_str = match variants_map.get(vkey) {
                Some(VariantValue::Command(s)) => s.as_str(),
                Some(VariantValue::Nested(map)) => {
                    map.get("command").map(|s| s.as_str()).unwrap_or("")
                }
                None => return None,
            };
            let parts: Vec<&str> = cmd_str.split_whitespace().collect();
            if parts.is_empty() {
                return None;
            }
            Some(CommandDef {
                verb: verb.to_string(),
                targets: vec![],
                variants: vec![],
                executable: parts[0].to_string(),
                args_template: parts[1..].iter().map(|s| s.to_string()).collect(),
                description: String::new(),
                env_vars: HashMap::new(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_project_type_from_config() {
        let yaml = r#"
name: myapp
type: flutter
commands:
  run:
    web:
      command: flutter run -d chrome --web-port=8080
"#;
        let config: ProjectConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.project_type.as_deref(), Some("flutter"));
        assert_eq!(config.name.as_deref(), Some("myapp"));
    }

    #[test]
    fn test_parse_simple_command_override() {
        let yaml = r#"
commands:
  run:
    web: flutter run -d chrome --web-port=8080
"#;
        let config: ProjectConfig = serde_yaml::from_str(yaml).unwrap();
        let cmd = parse_d_yaml_override(&config, "run", &Some("web".into()), &None);
        assert!(cmd.is_some());
        let cmd = cmd.unwrap();
        assert_eq!(cmd.executable, "flutter");
        assert!(cmd.args_template.contains(&"-d".to_string()));
    }

    #[test]
    fn test_d_yaml_variants() {
        let yaml = r#"
commands:
  build:
    ios:
      release: flutter build ios --release
      debug: flutter build ios --debug
"#;
        let config: ProjectConfig = serde_yaml::from_str(yaml).unwrap();
        let cmd = parse_d_yaml_override(
            &config,
            "build",
            &Some("ios".into()),
            &Some("release".into()),
        );
        assert!(cmd.is_some());
        assert!(cmd
            .unwrap()
            .args_template
            .contains(&"--release".to_string()));
    }

    #[test]
    fn test_no_commands_section() {
        let config = ProjectConfig::default();
        let cmd = parse_d_yaml_override(&config, "run", &Some("web".into()), &None);
        assert!(cmd.is_none());
    }
}
