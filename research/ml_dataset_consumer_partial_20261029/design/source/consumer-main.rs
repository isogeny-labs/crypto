use anyhow::{bail, ensure, Context, Result};
use ml_dataset_independent_consumer::{audit, common, transport};
use std::collections::BTreeMap;
use std::path::Path;

const HELP: &str = "Independent native saved-dataset consumer\n\
  dataset-consumer audit --run DIR [--arithmetic] [--expected-run-sha256 SHA] [--native-verifier BIN]\n\
  dataset-consumer export --run DIR --out FILE [--arithmetic] [--native-verifier BIN | --allow-unreplayed-preview]\n\
  dataset-consumer fetch --receipt FILE --expected-sha256 SHA --out NEW_DIR\n\
  dataset-consumer certificate --public FILE --base FILE --config FILE --certificate FILE --role ROLE\n\
Independent arithmetic checks positive equations, completed distinct-root sets and generic-v2 supplied source/filter/rank certificates; excludes UNSAT, total cost, full group orders, construction completeness and fall degrees.\n\
Fetch reads the canonical S3 bucket and does not create a compute worker.\n";

fn run() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        print!("{HELP}");
        return Ok(());
    };
    if command == "--help" || command == "-h" {
        print!("{HELP}");
        return Ok(());
    }
    let allowed: &[&str] = match command.as_str() {
        "audit" => &[
            "--run",
            "--arithmetic",
            "--expected-run-sha256",
            "--native-verifier",
        ],
        "export" => &[
            "--run",
            "--arithmetic",
            "--expected-run-sha256",
            "--native-verifier",
            "--out",
            "--allow-unreplayed-preview",
        ],
        "fetch" => &["--receipt", "--expected-sha256", "--out"],
        "certificate" => &["--public", "--base", "--config", "--certificate", "--role"],
        _ => bail!("unknown command {command}"),
    };
    let mut options = BTreeMap::new();
    while let Some(key) = args.next() {
        if key == "--help" {
            print!("{HELP}");
            return Ok(());
        }
        ensure!(allowed.contains(&key.as_str()), "unknown option {key}");
        let value = if matches!(key.as_str(), "--arithmetic" | "--allow-unreplayed-preview") {
            String::new()
        } else {
            args.next()
                .with_context(|| format!("missing value for {key}"))?
        };
        ensure!(
            options.insert(key.clone(), value).is_none(),
            "duplicate option {key}"
        );
    }
    let required = |key: &str| -> Result<&str> {
        options
            .get(key)
            .map(String::as_str)
            .with_context(|| format!("missing {key}"))
    };
    let report = if command == "fetch" {
        transport::fetch_cell(
            Path::new(required("--receipt")?),
            required("--expected-sha256")?,
            Path::new(required("--out")?),
            20 * 1024 * 1024 * 1024,
        )?
    } else if command == "certificate" {
        ml_dataset_independent_consumer::arithmetic::verify_certificate(
            &common::load(Path::new(required("--public")?))?,
            &common::load(Path::new(required("--base")?))?,
            &common::load(Path::new(required("--config")?))?,
            &common::load(Path::new(required("--certificate")?))?,
            required("--role")?,
        )?
    } else {
        let root = Path::new(required("--run")?);
        let (mut report, rows) = audit::audit(
            root,
            options.get("--expected-run-sha256").map(String::as_str),
            options.contains_key("--arithmetic"),
        )?;
        if let Some(binary) = options.get("--native-verifier") {
            transport::replay_native(root, Path::new(binary), &mut report, &rows)?;
        }
        if command == "export" {
            ensure!(
                report["producer_native_replay_passed"] == true
                    || options.contains_key("--allow-unreplayed-preview"),
                "export requires producer replay or explicit unreplayed preview"
            );
            let output = Path::new(required("--out")?);
            audit::export_rows(&rows, output)?;
            report["export_sha256"] = common::hash_file(output)?.into();
            report["export_rows"] = rows.len().into();
        }
        report
    };
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("consumer rejected dataset: {error:#}");
        std::process::exit(1);
    }
}
