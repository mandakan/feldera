use std::error::Error;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Child;
use std::time::Instant;
use std::{fs, thread};

use console::{style, Emoji};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use log::error;
use pipeline_types::schema::ProgramSchema;
use reqwest::blocking::{Client, Response};
use serde_json;

use crate::run::run_in_background;
use crate::{RunArgs, TestArgs};

static FAIL: Emoji<'_, '_> = Emoji("❌ ", "OK");
static CHECKMARK: Emoji<'_, '_> = Emoji("✔️ ", "OK");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "OK");

/// Reads a JSON file and deserializes it into a ProgramSchema.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the JSON file.
///
/// # Returns
///
/// This function returns `ProgramSchema` on success or an error if it fails.
fn parse_schema<P: AsRef<Path>>(path: P) -> Result<ProgramSchema, Box<dyn Error>> {
    let file_contents = fs::read_to_string(path)?;
    let schema: ProgramSchema = serde_json::from_str(&file_contents)?;
    Ok(schema)
}

fn post_csv_data(data: &str, relation: &str, port: u16) -> reqwest::Result<Response> {
    let url = format!("http://localhost:{port}/ingress/{relation}?format=csv");
    Client::new()
        .post(&url)
        .body(data.to_string())
        .header("Content-Type", "text/csv")
        .send()
}

enum TestResult {
    Success(String),
    InputError(String, String),
    OutputError(String, String),
}

fn execute_csv_test(
    test_dir: PathBuf,
    args: TestArgs,
    schema: ProgramSchema,
    port: u16,
    pb: &ProgressBar,
) -> TestResult {
    let test = test_dir
        .iter()
        .last()
        .unwrap_or(OsStr::new("<invalid>"))
        .to_string_lossy();

    for input in schema.inputs.iter() {
        let relation = input.name.as_str();
        let csv_path = test_dir.join(format!("{}.csv", relation));
        pb.set_message(format!("{test} Sending: `{relation}`"));
        pb.inc(1);

        if csv_path.exists() {
            let csv_data = fs::read_to_string(csv_path).expect("Can't read CSV file");
            let response = post_csv_data(&csv_data, relation, port).expect("Can't send CSV data");
            if !response.status().is_success() {
                return TestResult::InputError(test.to_string(), relation.to_string());
            }
        } else {
            log::debug!("No CSV file found for table: {}", relation);
        }
    }

    TestResult::Success(test.to_string())
}

// Usage example (You can include this in your main function or wherever needed)
pub(crate) fn test_command(args: TestArgs, m: MultiProgress) {
    const SCHEMA_PATH: &'static str = "build/schema.json";
    let program_schema = parse_schema(SCHEMA_PATH).expect("Unable to parse build/schema.json");

    let mut handles = vec![];

    let started = Instant::now();
    let spinner_style =
        ProgressStyle::with_template("{prefix:.bold.dim} {spinner:.green} {wide_msg}").unwrap();

    if let Ok(entries) = fs::read_dir(Path::new(args.tests.as_str())) {
        for entry in entries.filter_map(Result::ok) {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    let test_dir = entry.path();
                    let args = args.clone();
                    let idx = handles.len() as u16;
                    let schema = program_schema.clone();

                    let pb = m.add(ProgressBar::new(3));
                    pb.set_style(spinner_style.clone());
                    pb.set_prefix(format!("[{}/?]", idx + 1));

                    let handle = thread::spawn(move || {
                        let test_args = args.clone();
                        let mut run_args: RunArgs = args.into();
                        let port = 19990 + idx;
                        run_args.default_port = Some(port);
                        let mut child = run_in_background(&run_args);
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        let result =
                            execute_csv_test(test_dir.clone(), test_args, schema, port, &pb);

                        let stdout =
                            BufReader::new(child.stdout.take().expect("Failed to take stdout"));
                        let stderr =
                            BufReader::new(child.stderr.take().expect("Failed to take stderr"));
                        child.kill().expect("Can't terminate pipeline");

                        match result {
                            TestResult::Success(test_name) => {
                                pb.finish_with_message(format!(
                                    "{} {test_name}: Successful",
                                    CHECKMARK
                                ));
                            }
                            TestResult::InputError(test_name, relation) => {
                                pb.finish_with_message(format!(
                                    "{} {test_name}: Unable to import CSV into table {}",
                                    FAIL, relation
                                ));

                                println!("================ stderr {test_name} ================");
                                for line in stdout.lines() {
                                    println!("{}", line.expect("Failed to read line from stdout"));
                                }
                                println!("================ stdout {test_name} ================");
                                for line in stderr.lines() {
                                    println!("{}", line.expect("Failed to read line from stderr"));
                                }
                                println!("================ end {test_name} ================");
                            }
                            TestResult::OutputError(test_name, relation) => {
                                pb.finish_with_message(format!(
                                    "{} {test_name}: Unexpected output from pipeline (doesn't match CSV) {}",
                                    FAIL, relation
                                ));
                            }
                        }
                    });
                    handles.push(handle);
                }
            }
        }
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    println!(
        "{} Tests completed in {}",
        SPARKLE,
        HumanDuration(started.elapsed())
    );
}
