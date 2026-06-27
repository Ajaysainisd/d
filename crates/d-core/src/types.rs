use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct CommandDef {
    pub verb: String,
    pub targets: Vec<String>,
    pub variants: Vec<String>,
    pub executable: String,
    pub args_template: Vec<String>,
    pub description: String,
    pub env_vars: HashMap<String, String>,
}

impl CommandDef {
    pub fn new(verb: &str, executable: &str, description: &str) -> Self {
        Self {
            verb: verb.to_string(),
            targets: vec![],
            variants: vec![],
            executable: executable.to_string(),
            args_template: vec![],
            description: description.to_string(),
            env_vars: HashMap::new(),
        }
    }

    pub fn with_targets(mut self, targets: &[&str]) -> Self {
        self.targets = targets.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_variants(mut self, variants: &[&str]) -> Self {
        self.variants = variants.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_args(mut self, args: &[&str]) -> Self {
        self.args_template = args.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env_vars.insert(key.to_string(), value.to_string());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedCommand {
    pub verb: String,
    pub target: Option<String>,
    pub variant: Option<String>,
    pub executable: String,
    pub args: Vec<String>,
    pub env_vars: HashMap<String, String>,
    pub working_dir: Option<PathBuf>,
    pub source: ResolutionSource,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResolutionSource {
    CliArg,
    ProjectConfig,
    WorkspaceConfig,
    PlatformDefault,
    BuiltInDefault,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExitCode {
    Success = 0,
    Failure = 1,
    InvalidCommand = 2,
    DependencyMissing = 3,
}

impl std::process::Termination for ExitCode {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::from(self as u8)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoctorCheck {
    pub name: String,
    pub category: String,
    pub status: DoctorStatus,
    pub message: Option<String>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DoctorStatus {
    Pass,
    Warn,
    Fail,
}
