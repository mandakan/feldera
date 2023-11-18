use reqwest::blocking::Client;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

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
    let mut cmd_args = vec![
        String::from("run"),
        String::from("--manifest-path"),
        build_dir
            .join("pipeline")
            .join("Cargo.toml")
            .to_string_lossy()
            .to_string(),
    ];
    if args.release {
        cmd_args.push(String::from("--release"));
    }
    cmd_args.append(&mut vec![
        String::from("--"),
        String::from("--config-file"),
        args.config.clone(),
        String::from("--default-port"),
        port.to_string(),
    ]);

    let _cargo_process = Command::new("cargo")
        .args(cmd_args)
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
    let mut cmd_args = vec![
        String::from("run"),
        String::from("--manifest-path"),
        build_dir
            .join("pipeline")
            .join("Cargo.toml")
            .to_string_lossy()
            .to_string(),
    ];
    if args.release {
        cmd_args.push(String::from("--release"));
    }
    cmd_args.append(&mut vec![
        String::from("--"),
        String::from("--config-file"),
        args.config.clone(),
        String::from("--default-port"),
        port.to_string(),
    ]);

    let mut child = Command::new("cargo")
        .args(cmd_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Can't spawn pipeline");

    wait_for_initialization(args, &mut child);
    send_start(port);

    child
}
