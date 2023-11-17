use reqwest::blocking::Client;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use std::{thread};

use crate::RunArgs;

fn wait_for_initialization(args: &RunArgs, child: &mut Child) {
    if let Some(ref mut stdout) = child.stderr {
        let mut reader = BufReader::new(stdout);
        'outer: loop {
            for line in reader.by_ref().lines() {
                match line {
                    Ok(line) => {
                        if line.contains("Pipeline initialization complete.") {
                            if args.verbose > 0 {
                                println!("Pipeline initialization complete");
                            }
                            break 'outer;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading line: {}", e);
                        break 'outer;
                    }
                }
            }
            thread::sleep(Duration::from_millis(25));
        }
    }
}

fn send_start(port: u16) {
    let url = format!("http://localhost:{port}/start");
    Client::new()
        .get(&url)
        .send()
        .expect("Can't send start request");
}

pub(crate) fn run_command(args: &RunArgs) {
    let build_dir = Path::new("build");
    let port = args.default_port.unwrap_or(9999);
    let _cargo_process = Command::new("cargo")
        .arg("run")
        .arg("--manifest-path")
        .arg(build_dir.join("pipeline").join("Cargo.toml"))
        .arg("--")
        .arg("--config-file")
        .arg(&args.config)
        .arg("--default-port")
        .arg(port.to_string())
        .spawn()
        .expect("Can't spawn cargo check")
        .wait_with_output()
        .expect("Can't spawn pipeline");

    //wait_for_initialization(args, &mut child);
    //send_start(port);
}

pub(crate) fn run_in_background(args: &RunArgs) -> Child {
    let build_dir = Path::new("build");
    let port = args.default_port.unwrap_or(9999);
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--manifest-path")
        .arg(build_dir.join("pipeline").join("Cargo.toml"))
        .arg("--")
        .arg("--config-file")
        .arg(&args.config)
        .arg("--default-port")
        .arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Can't spawn pipeline");

    wait_for_initialization(args, &mut child);
    send_start(port);

    child
}
