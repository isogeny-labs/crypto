//! Emit one producer-side symbolic strict-history certificate.

use hashed_ecdl_tensor::admission::dimensions::{
    symbolic_history_certificate, UNIFORM_HISTORY_LAMBDA_THRESHOLD,
};
use std::io::Write;

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let lambda = match args.as_slice() {
        [] => UNIFORM_HISTORY_LAMBDA_THRESHOLD,
        [lambda] => lambda
            .parse::<u64>()
            .map_err(|error| format!("lambda: {error}"))?,
        _ => return Err("usage: symbolic-history-certificate-emit [LAMBDA]".into()),
    };
    let certificate = symbolic_history_certificate(lambda).map_err(|error| error.to_string())?;
    let stdout = std::io::stdout();
    let mut output = stdout.lock();
    serde_json::to_writer(&mut output, &certificate).map_err(|error| error.to_string())?;
    output.write_all(b"\n").map_err(|error| error.to_string())?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("symbolic-history-certificate-emit: {error}");
        std::process::exit(2);
    }
}
