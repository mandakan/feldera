use crate::run::run_in_background;
use crate::{RunArgs, TestArgs};
use pipeline_types::schema::ProgramSchema;
use reqwest::blocking::{Client, Response};
use serde_json;
use std::error::Error;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Child;
use std::{fs, thread};

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
    println!("Sending to: {}", url);
    Client::new()
        .post(&url)
        .body(data.to_string())
        .header("Content-Type", "text/csv")
        .send()
}

fn execute_csv_test(
    test_dir: PathBuf,
    args: TestArgs,
    schema: ProgramSchema,
    port: u16,
    _child: &mut Child,
) {
    println!(
        "Executing test: {:?}",
        test_dir.iter().last().unwrap_or(OsStr::new("<invalid>"))
    );

    for input in schema.inputs.iter() {
        let relation = input.name.as_str();
        let csv_path = test_dir.join(format!("{}.csv", relation));
        println!("Sending: `{}`", relation);

        if csv_path.exists() {
            let csv_data = fs::read_to_string(csv_path).expect("Can't read CSV file");
            let response = post_csv_data(&csv_data, relation, port);
            if let Err(e) = response {
                println!("Error sending CSV data: {}", e);
            } else {
                println!("Response: {:?}", response);
            }
        } else {
            println!("No CSV file found for relation: {}", relation);
        }
    }
}

fn execute_tests_in_parallel(args: TestArgs) {
    const SCHEMA_PATH: &'static str = "build/schema.json";
    let program_schema = parse_schema(SCHEMA_PATH).expect("Unable to parse build/schema.json");

    let mut handles = vec![];

    if let Ok(entries) = fs::read_dir(Path::new(args.tests.as_str())) {
        for entry in entries.filter_map(Result::ok) {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    let test_dir = entry.path();
                    let args = args.clone();
                    let idx = handles.len() as u16;
                    let schema = program_schema.clone();

                    let handle = thread::spawn(move || {
                        let test_args = args.clone();
                        let mut run_args: RunArgs = args.into();
                        let port = 19990 + idx;
                        run_args.default_port = Some(port);
                        let mut child = run_in_background(&run_args);
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        execute_csv_test(test_dir, test_args, schema, port, &mut child);
                        child.kill().expect("Can't terminate pipeline");
                        // Add your folder processing logic here
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
}

// Usage example (You can include this in your main function or wherever needed)
pub(crate) fn test_command(args: TestArgs) {
    execute_tests_in_parallel(args);
}
