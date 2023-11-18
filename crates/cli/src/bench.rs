use crate::run::run_in_background;
use crate::test::{parse_schema, post_csv_data};
use crate::{BenchArgs, RunArgs};
use indicatif::{HumanBytes, HumanCount, MultiProgress, ProgressBar, ProgressStyle};
use pipeline_types::schema::Relation;
use rand::distributions::{Alphanumeric, DistString};
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use std::time::{Duration, Instant};

fn generate_rows(rows: &mut String, relation: &Relation, bytes: usize) -> usize {
    let mut rng = SmallRng::from_entropy();
    let mut count = 0;
    while rows.len() <= bytes {
        for field in relation.fields.iter() {
            match field.columntype.typ.as_str() {
                "BIGINT" => {
                    let mut x = rng.gen_range(0..1024).to_string();
                    x += ",";
                    rows.push_str(&x);
                }
                "VARCHAR" => {
                    let mut string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
                    string += ",";
                    rows.push_str(&string);
                }
                "DECIMAL" => {
                    let mut random_decimal = rng.gen::<f64>().to_string();
                    random_decimal += ",";
                    rows.push_str(&random_decimal);
                }
                _ => unimplemented!("need to support all types"),
            }
        }
        rows.pop();
        count += 1;
    }

    count
}

pub(crate) fn bench_command(args: BenchArgs, m: MultiProgress) {
    const SCHEMA_PATH: &'static str = "build/schema.json";
    let program_schema = parse_schema(SCHEMA_PATH).expect("Unable to parse build/schema.json");

    let bench_args = args.clone();
    let mut run_args: RunArgs = args.into();
    let port = 29990;
    run_args.default_port = Some(port);
    let mut child = run_in_background(&run_args);

    let mut handles = vec![];
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    //let spinner_style = ProgressStyle::with_template(
    //    "{prefix:>9.bold.dim} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {wide_msg}",
    //)
    //.unwrap()
    //.progress_chars("##-");

    println!("running benchmark");
    println!();

    let started = Instant::now();

    let input_len = program_schema.inputs.len();

    for (idx, input) in program_schema.inputs.into_iter().enumerate() {
        let pb = m.add(ProgressBar::new(bench_args.duration as u64));
        pb.set_style(spinner_style.clone());
        pb.set_prefix(format!("[{}/{}] {}", idx + 1, input_len, input.name));
        pb.enable_steady_tick(Duration::from_millis(100));

        let handle = std::thread::spawn(move || {
            let mut total_rows_sent = 0;
            let mut total_bytes_sent = 0;
            while started.elapsed().as_secs() < bench_args.duration {
                let mut rows = String::with_capacity(256 * 1024);
                let row_cnt = generate_rows(&mut rows, &input, 1 * 1024 * 1024);
                let resp = post_csv_data(rows.as_str(), input.name.as_str(), port)
                    .expect("Can't send data");

                if !resp.status().is_success() {
                    pb.finish_with_message(format!("Error: {}", resp.status()));
                    return;
                }
                pb.inc(row_cnt as u64);

                total_bytes_sent += rows.len();
                total_rows_sent += row_cnt;
                rows.clear();
            }

            pb.finish_with_message(format!(
                "sent {} rows/s ({}/s)",
                HumanCount(total_rows_sent as u64 / bench_args.duration),
                HumanBytes(total_bytes_sent as u64 / bench_args.duration)
            ));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    child.kill().expect("Can't terminate pipeline");
}
