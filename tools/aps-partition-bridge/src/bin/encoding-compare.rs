use aps_partition_bridge::encoding_panel::{self, Panel};
use serde::Serialize;
use std::{fs::OpenOptions, io::Write};

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    source_revision: String,
    source_manifest_sha256: String,
    transport_control: bool,
    performance_trial: bool,
    execution_attested: bool,
    cost_complete: bool,
    panel: Panel,
}

fn hex(s: &str, n: usize) -> bool {
    s.len() == n
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn run() -> Result<bool, String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 8
        || !matches!(args[1].as_str(), "structural" | "transport-control")
        || args[2] != "--source-revision"
        || args[4] != "--source-manifest-sha256"
        || args[6] != "--out"
        || !hex(&args[3], 40)
        || !hex(&args[5], 64)
    {
        return Err("usage: encoding-compare structural|transport-control --source-revision HEX40 --source-manifest-sha256 HEX64 --out NEW_JSON".into());
    }
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&args[7])
        .map_err(|e| e.to_string())?;
    let control = args[1] == "transport-control";
    let panel = if control {
        encoding_panel::collect(|_| {
            Err("intentional transport-only failure; no solver called".into())
        })
    } else {
        encoding_panel::run()
    };
    let passed = if control {
        panel.flow.planned == 8
            && panel.flow.attempted == 8
            && panel.flow.failed_or_incomplete == 8
            && panel.flow.excluded_by_rule == 0
            && panel
                .arms
                .iter()
                .all(|a| a.error.is_some() && a.report.is_none())
    } else {
        panel.complete_and_equal
    };
    let report = Report {
        schema: "aps.quartic-encoding-panel/v1",
        source_revision: args[3].clone(),
        source_manifest_sha256: args[5].clone(),
        transport_control: control,
        performance_trial: false,
        execution_attested: false,
        cost_complete: false,
        panel,
    };
    serde_json::to_writer(&mut file, &report).map_err(|e| e.to_string())?;
    file.write_all(b"\n").map_err(|e| e.to_string())?;
    Ok(passed)
}

fn main() {
    match run() {
        Ok(true) => {}
        Ok(false) => std::process::exit(2),
        Err(error) => {
            eprintln!("encoding-compare: {error}");
            std::process::exit(1);
        }
    }
}
