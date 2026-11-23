//! Postrun attempt-join exporter. It performs no launch, solve, replay, or
//! controller authentication; supplied FilePins must already come from the
//! retained native-session exporter and external custody checker.
use crypto_lib::cryptanalysis::ecbench::{
    diagnostic_binding_core::parse,
    retained_attempt_join::{export_full_cohort_new, AttemptJoin},
};
use serde::Deserialize;
use std::{
    collections::BTreeSet,
    env,
    fs::OpenOptions,
    io::Read,
    path::PathBuf,
    process::ExitCode,
};

const MAX_REQUEST_BYTES: usize = 256 * 1024 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    schema: String,
    expected_plan_hashes: BTreeSet<String>,
    attempts: Vec<AttemptJoin>,
}

fn run() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let request_path = PathBuf::from(args.next().ok_or("request path required")?);
    let output_path = PathBuf::from(args.next().ok_or("output path required")?);
    if args.next().is_some() {
        return Err("usage: ecbench_retained_attempt_export REQUEST OUTPUT".into());
    }
    let mut request_bytes = Vec::new();
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let file = options.open(&request_path).map_err(|error| error.to_string())?;
    if !file.metadata().map_err(|error| error.to_string())?.is_file() {
        return Err("attempt export request must be regular file".into());
    }
    file.take((MAX_REQUEST_BYTES + 1) as u64)
        .read_to_end(&mut request_bytes)
        .map_err(|error| error.to_string())?;
    if request_bytes.len() > MAX_REQUEST_BYTES {
        return Err("attempt export request byte bound".into());
    }
    let request: Request = serde_json::from_value(parse(&request_bytes, MAX_REQUEST_BYTES)?)
        .map_err(|error| error.to_string())?;
    if request.schema != "ecbench.pencil-attempt-export-request/v1" {
        return Err("attempt export request schema".into());
    }
    let pin = export_full_cohort_new(
        &request.attempts,
        &request.expected_plan_hashes,
        &output_path,
    )?;
    println!("{}", serde_json::to_string(&pin).map_err(|error| error.to_string())?);
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("ecbench retained attempt export: {error}");
            ExitCode::from(2)
        }
    }
}
