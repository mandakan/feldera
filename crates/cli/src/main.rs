use clap::{Parser, ArgAction, Subcommand, Args};

mod init;
mod build;
mod assets;
mod run;

/// CLI tool for Feldera developers.
#[derive(Parser)]
#[clap(version = "1.0", author = "Feldera Inc.")]
pub(crate) struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Args)]
pub(crate) struct CheckArgs {
    /// Path to SQL file.
    #[clap(long, default_value = "src/project.sql")]
    path: String,
    /// Also run `cargo check` after SQL generation.
    #[clap(long)]
    cargo: bool,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = ArgAction::Count)]
    verbose: u8,
}

#[derive(Args)]
pub(crate) struct BuildArgs {
    /// Path to SQL file.
    #[clap(long, default_value = "src/project.sql")]
    path: String,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = ArgAction::Count)]
    verbose: u8,
}

impl Into<CheckArgs> for BuildArgs {
    fn into(self) -> CheckArgs {
        CheckArgs {
            path: self.path,
            cargo: true,
            verbose: self.verbose,
        }
    }
}

#[derive(Args, Clone)]
pub(crate) struct RunArgs {
    /// Path to SQL file.
    #[clap(long, default_value = "src/project.sql")]
    path: String,
    /// Path to config file.
    #[clap(long, default_value = "config.json")]
    config: String,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = ArgAction::Count)]
    verbose: u8,
}

impl Into<CheckArgs> for RunArgs {
    fn into(self) -> CheckArgs {
        CheckArgs {
            path: self.path,
            cargo: false,
            verbose: self.verbose,
        }
    }
}

pub(crate) enum CargoCmd {
    Check,
    Build,
}

impl CargoCmd {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            CargoCmd::Check => "check",
            CargoCmd::Build => "build",
        }
    }
}

#[derive(Subcommand)]
enum SubCommand {
    /// Sets up a scaffolded git repository with initial SQL, config, and tests.
    Init { name: String },
    /// Type checks SQL.
    Check(CheckArgs),
    /// Compiles a pipeline to a native binary.
    Build(BuildArgs),
    /// Starts a pipeline from the repo as a local process.
    Run(RunArgs),
    /// Runs unit tests against the pipeline.
    Test,
    /// Benchmarking the pipeline.
    Bench,
    /// Syncs program to a Feldera Cloud instance.
    Sync,
}

fn main() {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Init { name } => init::init_command(&name),
        SubCommand::Check(args) => build::build_command(&args, CargoCmd::Check),
        SubCommand::Build(args) => build::build_command(&args.into(), CargoCmd::Build),
        SubCommand::Run(args) =>  {
            build::build_command(&args.clone().into(), CargoCmd::Check);
            run::run_command(&args)
        },
        SubCommand::Test => test_command(),
        SubCommand::Bench => bench_command(),
        SubCommand::Sync => sync_command(),
    }
}

fn test_command() {
    println!("Running tests...");
}

fn bench_command() {
    println!("Benchmarking...");
}

fn sync_command() {
    println!("Syncing to cloud...");
    unimplemented!()
}
