use std::path::Path;
use std::process::Command;
use crate::{ RunArgs};

pub(crate) fn run_command(args: &RunArgs) {
    let build_dir = Path::new("build");
    let _cargo_process = Command::new("cargo")
        .arg("run")
        .arg("--manifest-path")
        .arg(build_dir.join("pipeline").join("Cargo.toml"))
        .arg("--")
        .arg("--config-file")
        .arg(&args.config)
        .spawn().expect("Can't spawn cargo check")
        .wait_with_output().expect("Can't wait for cargo check output"); // Captures the output
}
