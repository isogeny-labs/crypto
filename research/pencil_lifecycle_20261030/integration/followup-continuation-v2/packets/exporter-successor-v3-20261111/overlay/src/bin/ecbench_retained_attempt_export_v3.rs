//! Source-only postrun entry point for the fail-closed retained-attempt join.
//! It performs no launch, solve, replay, controller mutation, or cloud action.
use crypto_lib::cryptanalysis::ecbench::{
    diagnostic_binding_core::parse,
    retained_attempt_join_v3::{validate_and_export, ExportRequest, MAX_REQUEST_BYTES},
};
use std::{
    env,
    fs::OpenOptions,
    io::Read,
    path::PathBuf,
    process::ExitCode,
};

fn run() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let request_path = PathBuf::from(args.next().ok_or("request path required")?);
    let output_path = PathBuf::from(args.next().ok_or("output path required")?);
    if args.next().is_some() {
        return Err("usage: ecbench_retained_attempt_export_v3 REQUEST OUTPUT".into());
    }
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let file = options.open(&request_path).map_err(|error| error.to_string())?;
    if !file.metadata().map_err(|error| error.to_string())?.is_file() {
        return Err("retained-attempt request must be a regular file".into());
    }
    let mut bytes = Vec::new();
    file.take((MAX_REQUEST_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() > MAX_REQUEST_BYTES {
        return Err("retained-attempt request byte bound".into());
    }
    let request: ExportRequest = serde_json::from_value(parse(&bytes, MAX_REQUEST_BYTES)?)
        .map_err(|error| error.to_string())?;
    let pin = validate_and_export(&request, &output_path)?;
    println!("{}", serde_json::to_string(&pin).map_err(|error| error.to_string())?);
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("ecbench retained attempt export v3: {error}");
            ExitCode::from(2)
        }
    }
}
