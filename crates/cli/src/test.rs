use std::error::Error;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

use colored::Colorize;
use std::collections::HashMap;
use std::time::Instant;
use std::{fs, thread};

use console::{Emoji, Style};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};

use pipeline_types::schema::ProgramSchema;
use reqwest::blocking::{Client, Response};
use serde_json;

use crate::run::run_in_background;
use crate::{RunArgs, TestArgs};

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "OK");

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
    OutputError(String, String, String, String),
}

use serde::{Deserialize, Serialize};
use similar::{ChangeTag, TextDiff};

#[derive(Serialize, Deserialize, Debug)]
struct LogData {
    sequence_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_data: Option<String>,
}

fn fetch_view_data(relation: &str, port: u16) -> String {
    let url = format!("http://localhost:{port}/egress/{relation}?format=csv&query=neighborhood");
    let mut map = HashMap::new();
    map.insert("after", 100);
    map.insert("before", 0);
    let response = Client::new()
        .post(&url)
        .json(&map)
        .send()
        .expect("Can't get CSV data");
    if !response.status().is_success() {
        panic!("Can't query view?");
    }

    let mut reader = BufReader::new(response);
    let mut response_body = String::new();
    'outer: loop {
        for line in reader.by_ref().lines() {
            if let Ok(line) = line {
                if !line.starts_with("{\"sequence_number\":}") {
                    let parsed: LogData = serde_json::from_str(&line).expect("Can't parse JSON");
                    if let Some(data) = parsed.text_data {
                        response_body += &*data.replace("\\n", "\n");
                        break 'outer;
                    }
                } else {
                    // skip
                }
            }
        }
    }

    response_body
}

fn execute_csv_test(
    test_dir: PathBuf,
    _args: TestArgs,
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

    for output in schema.outputs.iter() {
        let relation = output.name.as_str();
        let csv_path = test_dir.join(format!("{}.csv", relation));
        pb.set_message(format!("{test} Receiving: `{relation}`"));
        pb.inc(1);

        if csv_path.exists() {
            let response_body = fetch_view_data(relation, port);
            let expected_csv_data = fs::read_to_string(csv_path).expect("Can't read CSV file");
            if response_body != expected_csv_data {
                return TestResult::OutputError(
                    test.to_string(),
                    relation.to_string(),
                    response_body,
                    expected_csv_data,
                );
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

    let mut count_tests = 0;
    if let Ok(entries) = fs::read_dir(Path::new(args.tests.as_str())) {
        for entry in entries.filter_map(Result::ok) {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    count_tests += 1;
                }
            }
        }
    }

    println!("running {count_tests} tests");
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
                    pb.set_prefix(format!("[{}/{}]", idx + 1, count_tests));

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
                        match &result {
                            TestResult::Success(test_name) => {
                                pb.finish_with_message(format!(
                                    "test {test_name} ... {}",
                                    "ok".green(),
                                ));
                            }
                            TestResult::InputError(test_name, relation) => {
                                pb.finish_with_message(format!(
                                    "test {test_name} ... {} (Unable to import CSV into table {})",
                                    "FAILED".red(),
                                    relation,
                                ));
                            }
                            TestResult::OutputError(
                                test_name,
                                relation,
                                _retrieved_csv,
                                _expected_csv,
                            ) => {
                                pb.finish_with_message(format!(
                                    "test {test_name} {} ... {} (Output from pipeline doesn't match CSV)",
                                    "FAILED".red(),
                                    relation
                                ));
                            }
                        }
                        (result, stdout, stderr)
                    });
                    handles.push(handle);
                }
            }
        }
    }

    // Wait for all threads to complete
    let test_results = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect::<Vec<_>>();
    let failed = test_results
        .iter()
        .filter(|(result, _, _)| match result {
            TestResult::Success(_) => false,
            _ => true,
        })
        .count();
    let passed = test_results
        .iter()
        .filter(|(result, _, _)| match result {
            TestResult::Success(_) => true,
            _ => false,
        })
        .count();

    if failed > 0 {
        println!();
        println!("failures:");
        println!();
        for (result, _stdout, stderr) in test_results {
            match result {
                TestResult::Success(_test_name) => {}
                TestResult::InputError(test_name, relation) => {
                    println!("---- Import {test_name} {relation}.csv stderr ----");
                    for line in stderr.lines() {
                        println!("{}", line.expect("Failed to read line from stderr"));
                    }
                    println!();
                }
                TestResult::OutputError(test_name, relation, ref actual, ref expected) => {
                    println!("---- Output {test_name} {relation}.csv stderr ----");
                    for line in stderr.lines() {
                        println!("{}", line.expect("Failed to read line from stderr"));
                    }
                    println!();

                    println!("---- Output {test_name} CSV file vs. {relation} ----");
                    let diff = TextDiff::from_lines(actual, expected);

                    for op in diff.ops() {
                        for change in diff.iter_changes(op) {
                            let (sign, style) = match change.tag() {
                                ChangeTag::Delete => ("-", Style::new().red()),
                                ChangeTag::Insert => ("+", Style::new().green()),
                                ChangeTag::Equal => (" ", Style::new()),
                            };
                            print!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
                        }
                    }
                }
            }
        }
    }

    println!(
        "{} test result: {}. {} passed; {} failed; finished in {}",
        SPARKLE,
        if failed == 0 {
            "ok".green()
        } else {
            "FAILED".red()
        },
        passed,
        failed,
        HumanDuration(started.elapsed())
    );
}
