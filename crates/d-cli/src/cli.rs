use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "d", about = "One command system for every development project", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Verb>,

    /// Show what would be executed without actually running
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Override auto-detected project type (flutter, docker, node)
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
}

#[derive(Parser, Debug)]
pub enum Verb {
    /// Start the project/services
    Up {
        /// Build variant (detached for Docker)
        variant: Option<String>,
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
    Migrate,

    /// Clean build artifacts
    Clean,

    /// Restart services
    Restart,

    /// Install dependencies
    Install,

    /// Start dev server
    Dev,

    /// Create a release build
    Release {
        /// Build target
        target: Option<String>,
    },
}

impl Verb {
    pub fn name(&self) -> &str {
        match self {
            Verb::Up { .. } => "up",
            Verb::Down => "down",
            Verb::Run { .. } => "run",
            Verb::Build { .. } => "build",
            Verb::Test { .. } => "test",
            Verb::Lint => "lint",
            Verb::Format => "format",
            Verb::Doctor => "doctor",
            Verb::Logs { .. } => "logs",
            Verb::Shell { .. } => "shell",
            Verb::Migrate => "migrate",
            Verb::Clean => "clean",
            Verb::Restart => "restart",
            Verb::Install => "install",
            Verb::Dev => "dev",
            Verb::Release { .. } => "release",
        }
    }

    pub fn target(&self) -> Option<&str> {
        match self {
            Verb::Run { target, .. } => target.as_deref(),
            Verb::Build { target } => target.as_deref(),
            Verb::Test { target } => target.as_deref(),
            Verb::Logs { target } => target.as_deref(),
            Verb::Shell { target } => target.as_deref(),
            Verb::Release { target } => target.as_deref(),
            Verb::Up { variant } => variant.as_deref(),
            _ => None,
        }
    }

    pub fn variant(&self) -> Option<&str> {
        match self {
            Verb::Build { .. } => None,
            _ => None,
        }
    }
}
