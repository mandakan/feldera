use crate::assets::ASSETS;
use crate::{CargoCmd, CheckArgs};
use colored::Colorize;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

fn ensure_compiler_jar_exists(build_dir: &Path) -> PathBuf {
    let jar_path = build_dir.join("compiler.jar");

    if !jar_path.exists() {
        fs::write(&jar_path, crate::assets::COMPILER_JAR).expect("Can't write compiler JAR");
        // Make the file executable (Unix-specific)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = jar_path
                .metadata()
                .expect("Can't get metadata for SQL compiler executable");
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o755); // Read, write, and execute for owner; read and execute for others
            fs::set_permissions(&jar_path, permissions)
                .expect("Can't mark SQL compiler executable");
        }
    }

    jar_path
}

fn create_output_path(input_path: &str) -> String {
    let path = Path::new(input_path);
    let mut output_path = Path::new("build/pipeline").join(path);
    output_path.set_file_name("main");
    output_path.set_extension("rs");
    output_path.to_string_lossy().into_owned()
}

pub(crate) fn build_command(args: &CheckArgs, build: CargoCmd) {
    let start = Instant::now();
    let project_sql_path = args.path.as_str();
    let output_file_path = create_output_path(project_sql_path);

    println!(
        "{:>width$} {} ({} -> {})",
        "Generating".green().bold(),
        "Rust",
        project_sql_path,
        output_file_path,
        width = 12
    );

    let build_dir = Path::new("build");
    std::fs::create_dir_all(build_dir).expect("can't create build/ dir");

    let jar_path = ensure_compiler_jar_exists(build_dir);

    let compiler_process = Command::new("java")
        .arg("-jar")
        .arg(jar_path)
        .arg("-js")
        .arg(build_dir.join("schema.json"))
        .arg("-i")
        .arg("-je")
        .arg("--alltables")
        .arg("--outputsAreSets")
        .arg("--ignoreOrder")
        .arg(project_sql_path)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Can't spawn sql compiler")
        .wait_with_output()
        .expect("Can't wait for SQL compiler output"); // Captures the output

    let duration = start.elapsed();

    if compiler_process.status.success() {
        println!(
            "{:>width$} SQL target(s) in {}.{}s.",
            "Finished".green().bold(),
            duration.as_secs(),
            duration.subsec_millis(),
            width = 12
        );
        let compiler_output = String::from_utf8_lossy(&compiler_process.stdout);
        if args.verbose > 0 {
            println!("{}", compiler_output);
        }

        // Ensure the directory structure exists
        if let Some(parent) = Path::new(&output_file_path).parent() {
            fs::create_dir_all(parent)
                .expect("Failed to create directory structure for output file");
        }

        let mut main_rs_content = compiler_output.to_string();
        main_rs_content.push_str(ASSETS.get("main.rs").expect("main.rs asset not found"));

        // Write the output to a source file, overwriting if it exists, but only write of the contents have changed
        let current_content = fs::read_to_string(&output_file_path).unwrap_or_default();
        if current_content != main_rs_content {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&output_file_path)
                .expect("Failed to open compiler output file for writing");
            file.write_all(main_rs_content.as_bytes())
                .expect("Failed to write compiler output to source file");
        }

        fs::write(
            build_dir.join("pipeline").join("Cargo.toml"),
            &ASSETS
                .get("Cargo.toml")
                .expect("Cargo.toml asset not found")
                .as_bytes(),
        )
        .expect("Failed to write compiler output to source file");

        if args.cargo {
            let _cargo_process = Command::new("cargo")
                .arg(build.as_str())
                .arg("--manifest-path")
                .arg(build_dir.join("pipeline").join("Cargo.toml"))
                .arg(if args.release { "--release" } else { "--" })
                .spawn()
                .expect("Can't spawn cargo check")
                .wait_with_output()
                .expect("Can't wait for cargo check output"); // Captures the output
        }
    } else {
        eprintln!("SQL check failed.");
        eprintln!(
            "Error:\n{}",
            String::from_utf8_lossy(&compiler_process.stderr)
        );
    }
}
