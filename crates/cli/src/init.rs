use crate::assets::ASSETS;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub(crate) fn init_command(name: &str) {
    // Create the project directory
    if let Err(e) = fs::create_dir(name) {
        eprintln!("Failed to create project directory: {}", e);
        return;
    }

    // Change to the new directory
    if let Err(e) = std::env::set_current_dir(Path::new(name)) {
        eprintln!("Failed to change to project directory: {}", e);
        return;
    }

    // Initialize git repository
    match Command::new("git").arg("init").output() {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Failed to initialize git repository");
                return;
            }
            println!("Initialized empty Git repository");
        }
        Err(e) => {
            eprintln!("Failed to run git init: {}", e);
            return;
        }
    }

    // create config.json
    let mut file = match File::create("config.json") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create config.json: {}", e);
            return;
        }
    };
    if let Err(e) = file.write_all(
        &ASSETS
            .get("config.json")
            .expect("config.json asset not found")
            .as_bytes(),
    ) {
        eprintln!("Failed to write to config.json: {}", e);
    }

    // Create src/ directory and project.sql file
    if let Err(e) = fs::create_dir("src") {
        eprintln!("Failed to create src directory: {}", e);
        return;
    }
    let mut file = match File::create("src/project.sql") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create src/project.sql: {}", e);
            return;
        }
    };
    if let Err(e) = file.write_all(
        &ASSETS
            .get("project.sql")
            .expect("project.sql asset not found")
            .as_bytes(),
    ) {
        eprintln!("Failed to write to src/project.sql: {}", e);
    }

    // Create tests/ directory and .csv files
    if let Err(e) = fs::create_dir("tests") {
        eprintln!("Failed to create tests directory: {}", e);
        return;
    }
    for filename in ["PART.csv", "PRICE.csv", "VENDOR.csv"].iter() {
        let filepath = format!("tests/{}", filename);
        let mut file = match File::create(&filepath) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to create {}: {}", filepath, e);
                return;
            }
        };
        if let Err(e) = file.write_all(
            &ASSETS
                .get(filename)
                .expect("CSV asset not found")
                .as_bytes(),
        ) {
            eprintln!("Failed to write to {}: {}", filepath, e);
        }
    }

    println!("Project initialized successfully.");
}
