use checkpoint_discovery::{admission_allowed, discover, write_report, Options};
use std::path::PathBuf;

fn run() -> Result<i32, String> {
    let mut options = Options::new("/srv/experiments".into());
    let mut output: Option<PathBuf> = None;
    let mut require_valid = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("checkpoint-discovery [--root PATH] [--search-root PATH] [--proc-root PATH]\n  [--output PATH] [--job ID --require-valid] [--max-entries N] [--max-hash-bytes N]\nRead-only inventory. --job validates only that job; --require-valid exits 2 unless fresh, complete or integrity-validated.\nNo commands, environment values, or checkpoint contents are included in reports.");
                return Ok(0);
            }
            "--require-valid" => require_valid = true,
            "--root" | "--search-root" | "--proc-root" | "--output" | "--job" | "--max-entries"
            | "--max-hash-bytes" => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{arg} requires a value"))?;
                match arg.as_str() {
                    "--root" => options.root = value.into(),
                    "--search-root" => options.search_roots.push(value.into()),
                    "--proc-root" => options.proc_root = value.into(),
                    "--output" => output = Some(value.into()),
                    "--job" => options.job = Some(value),
                    "--max-entries" => {
                        options.max_entries = value.parse().map_err(|_| "invalid entry limit")?
                    }
                    _ => {
                        options.max_hash_bytes =
                            value.parse().map_err(|_| "invalid hash byte limit")?
                    }
                }
            }
            _ => return Err(format!("unknown argument: {arg}")),
        }
    }
    if require_valid && options.job.is_none() {
        return Err("--require-valid requires --job".into());
    }
    let report = discover(&options)?;
    if let Some(path) = output {
        write_report(&path, &report)?;
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
        );
    }
    Ok(if require_valid && !admission_allowed(&report) {
        2
    } else {
        0
    })
}

fn main() {
    match run() {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            eprintln!("Checkpoint discovery failed: {error}");
            std::process::exit(1);
        }
    }
}
