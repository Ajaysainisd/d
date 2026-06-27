use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "d",
    about = "One command system for every development project",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Verb>,

    /// Show what would be executed without actually running
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Override auto-detected project type
    #[arg(long = "project-type", global = true)]
    pub project_type: Option<String>,

    /// Run workspace commands in parallel
    #[arg(long, global = true)]
    pub parallel: bool,

    /// Log level: info, warn, error, debug
    #[arg(long = "log-level", default_value = "info", global = true)]
    pub log_level: String,

    /// Skip hooks
    #[arg(long = "no-hooks", global = true)]
    pub no_hooks: bool,

    /// Environment name (dev, staging, production)
    #[arg(long = "env", global = true)]
    pub env: Option<String>,

    /// Build or run variant/variation
    #[arg(long, global = true)]
    pub variant: Option<String>,
}

#[derive(Parser, Debug)]
pub enum Verb {
    /// Start the project/services
    Up {
        /// Service or variant name
        target: Option<String>,
    },
    /// Stop the project/services
    Down,
    /// Run a target (web, ios, android, etc.)
    Run {
        /// Target to run
        target: Option<String>,
    },
    /// Build a target
    Build {
        /// Target to build
        target: Option<String>,
    },
    /// Run tests
    Test {
        /// Test target or filter
        target: Option<String>,
    },
    /// Lint the codebase
    Lint,
    /// Format code
    Format,
    /// Run doctor checks
    Doctor,
    /// View logs
    Logs {
        /// Specific service/target
        target: Option<String>,
    },
    /// Open a shell
    Shell {
        /// Service to shell into
        target: Option<String>,
    },
    /// Run database migrations
    Migrate {
        /// Migration target
        target: Option<String>,
    },
    /// Clean build artifacts
    Clean,
    /// Restart services
    Restart {
        /// Service to restart
        target: Option<String>,
    },
    /// Install dependencies
    Install,
    /// Start dev server
    Dev,
    /// Create a release build
    Release {
        /// Build target
        target: Option<String>,
    },
    /// Open a REPL/console
    Console,
    /// Run benchmarks
    Bench,
    /// Run with coverage
    Coverage,
    /// Build documentation
    Docs,
    /// Publish to registry
    Publish,
    /// Update dependencies
    Update,
    /// Audit for security issues
    Audit,
    /// Auto-fix lints/compiler warnings
    Fix,
    /// Watch for changes and rebuild
    Watch,
    /// Generate code/resources
    Generate {
        /// What to generate
        target: Option<String>,
    },
    /// Show dependency tree
    Deps,
    /// Create a new project/component
    Create {
        /// What to create
        target: Option<String>,
    },
    /// Any other verb not listed above (pass-through to platform)
    #[command(external_subcommand)]
    Custom(Vec<String>),
}

pub struct ParsedVerb {
    pub name: String,
    pub target: Option<String>,
    pub variant: Option<String>,
}

impl Verb {
    pub fn parse_verb(&self, cli: &Cli) -> ParsedVerb {
        match self {
            Verb::Up { target } => ParsedVerb {
                name: "up".into(),
                target: target.clone(),
                variant: cli.variant.clone(),
            },
            Verb::Down => ParsedVerb {
                name: "down".into(),
                target: None,
                variant: None,
            },
            Verb::Run { target } => ParsedVerb {
                name: "run".into(),
                target: target.clone(),
                variant: cli.variant.clone(),
            },
            Verb::Build { target } => ParsedVerb {
                name: "build".into(),
                target: target.clone(),
                variant: cli.variant.clone(),
            },
            Verb::Test { target } => ParsedVerb {
                name: "test".into(),
                target: target.clone(),
                variant: cli.variant.clone(),
            },
            Verb::Lint => ParsedVerb {
                name: "lint".into(),
                target: None,
                variant: None,
            },
            Verb::Format => ParsedVerb {
                name: "format".into(),
                target: None,
                variant: None,
            },
            Verb::Doctor => ParsedVerb {
                name: "doctor".into(),
                target: None,
                variant: None,
            },
            Verb::Logs { target } => ParsedVerb {
                name: "logs".into(),
                target: target.clone(),
                variant: None,
            },
            Verb::Shell { target } => ParsedVerb {
                name: "shell".into(),
                target: target.clone(),
                variant: None,
            },
            Verb::Migrate { target } => ParsedVerb {
                name: "migrate".into(),
                target: target.clone(),
                variant: None,
            },
            Verb::Clean => ParsedVerb {
                name: "clean".into(),
                target: None,
                variant: None,
            },
            Verb::Restart { target } => ParsedVerb {
                name: "restart".into(),
                target: target.clone(),
                variant: None,
            },
            Verb::Install => ParsedVerb {
                name: "install".into(),
                target: None,
                variant: None,
            },
            Verb::Dev => ParsedVerb {
                name: "dev".into(),
                target: None,
                variant: None,
            },
            Verb::Release { target } => ParsedVerb {
                name: "release".into(),
                target: target.clone(),
                variant: None,
            },
            Verb::Console => ParsedVerb {
                name: "console".into(),
                target: None,
                variant: None,
            },
            Verb::Bench => ParsedVerb {
                name: "bench".into(),
                target: None,
                variant: None,
            },
            Verb::Coverage => ParsedVerb {
                name: "coverage".into(),
                target: None,
                variant: None,
            },
            Verb::Docs => ParsedVerb {
                name: "docs".into(),
                target: None,
                variant: None,
            },
            Verb::Publish => ParsedVerb {
                name: "publish".into(),
                target: None,
                variant: None,
            },
            Verb::Update => ParsedVerb {
                name: "update".into(),
                target: None,
                variant: None,
            },
            Verb::Audit => ParsedVerb {
                name: "audit".into(),
                target: None,
                variant: None,
            },
            Verb::Fix => ParsedVerb {
                name: "fix".into(),
                target: None,
                variant: None,
            },
            Verb::Watch => ParsedVerb {
                name: "watch".into(),
                target: None,
                variant: None,
            },
            Verb::Generate { target } => ParsedVerb {
                name: "generate".into(),
                target: target.clone(),
                variant: None,
            },
            Verb::Deps => ParsedVerb {
                name: "deps".into(),
                target: None,
                variant: None,
            },
            Verb::Create { target } => ParsedVerb {
                name: "create".into(),
                target: target.clone(),
                variant: None,
            },
            Verb::Custom(args) => {
                let name = args.first().cloned().unwrap_or_default();
                let target = args.get(1).cloned();
                let variant = args.get(2).cloned();
                ParsedVerb {
                    name,
                    target,
                    variant,
                }
            }
        }
    }
}
