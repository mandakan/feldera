use clap::{ArgAction, Args, Parser, Subcommand};
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;

mod assets;
mod bench;
mod build;
mod init;
mod run;
mod test;
mod types;

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
    #[clap(long, default_value = "project.sql")]
    path: String,
    /// Also run `cargo check` after SQL generation.
    #[clap(long)]
    cargo: bool,
    /// Check the release profile of the binary.
    #[clap(long)]
    release: bool,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = ArgAction::Count)]
    verbose: u8,
}

#[derive(Args)]
pub(crate) struct BuildArgs {
    /// Path to SQL file.
    #[clap(long, default_value = "project.sql")]
    path: String,
    /// Build the release version of the binary.
    #[clap(long)]
    release: bool,
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
            release: self.release,
        }
    }
}

#[derive(Args, Clone)]
pub(crate) struct RunArgs {
    /// Path to SQL file.
    #[clap(long, default_value = "project.sql")]
    path: String,
    /// Path to config file.
    #[clap(long, default_value = "config.json")]
    config: String,
    /// Build the release version of the binary.
    #[clap(long)]
    release: bool,
    /// Override which port the binary should listen on.
    #[clap(long)]
    default_port: Option<u16>,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = ArgAction::Count)]
    verbose: u8,
}

impl Into<CheckArgs> for RunArgs {
    fn into(self) -> CheckArgs {
        CheckArgs {
            path: self.path,
            cargo: false,
            release: self.release,
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

#[derive(Args, Clone)]
pub(crate) struct TestArgs {
    /// Path to SQL file.
    #[clap(long, default_value = "project.sql")]
    path: String,
    /// Path to config file.
    #[clap(long, default_value = "config.json")]
    config: String,
    /// Path of tests directory.
    #[clap(long, default_value = "tests")]
    tests: String,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = ArgAction::Count)]
    verbose: u8,
}

impl Into<RunArgs> for TestArgs {
    fn into(self) -> RunArgs {
        RunArgs {
            path: self.path,
            config: self.config,
            verbose: self.verbose,
            release: false,
            default_port: None,
        }
    }
}

#[derive(Args, Clone)]
pub(crate) struct BenchArgs {
    /// Path to SQL file.
    #[clap(long, default_value = "project.sql")]
    path: String,
    /// Path to config file.
    #[clap(long, default_value = "config.json")]
    config: String,
    /// How long to send data (in seconds).
    #[clap(long, default_value = "10")]
    duration: u64,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, action = ArgAction::Count)]
    verbose: u8,
}

impl Into<RunArgs> for BenchArgs {
    fn into(self) -> RunArgs {
        RunArgs {
            path: self.path,
            config: self.config,
            verbose: self.verbose,
            release: true,
            default_port: None,
        }
    }
}

impl Into<CheckArgs> for BenchArgs {
    fn into(self) -> CheckArgs {
        CheckArgs {
            path: self.path,
            verbose: self.verbose,
            release: true,
            cargo: true,
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
    Test(TestArgs),
    /// Benchmarking the pipeline.
    Bench(BenchArgs),
    /// Syncs program to a Feldera Cloud instance.
    Sync,
    /// Interact with pipeline manager
    Cloud,
}

fn main() {
    let opts = Opts::parse();
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).build();
    let multi = MultiProgress::new();
    LogWrapper::new(multi.clone(), logger).try_init().unwrap();

    match opts.subcmd {
        SubCommand::Init { name } => init::init_command(&name),
        SubCommand::Check(args) => build::build_command(&args, CargoCmd::Check),
        SubCommand::Build(args) => build::build_command(&args.into(), CargoCmd::Build),
        SubCommand::Run(args) => {
            build::build_command(&args.clone().into(), CargoCmd::Check);
            run::run_command(&args)
        }
        SubCommand::Test(args) => test::test_command(args, multi),
        SubCommand::Bench(args) => {
            build::build_command(&args.clone().into(), CargoCmd::Build);
            bench::bench_command(args, multi)
        }
        SubCommand::Sync => unimplemented!("Sync command not implemented"),
        SubCommand::Cloud => unimplemented!("Cloud command not implemented"),
    }
}
