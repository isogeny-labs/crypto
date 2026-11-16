//! Analyze a transcript; never launches workers, CAS tools or fixtures.
use std::io::{self, Read};
fn run() -> Result<bool, String> {
    if std::env::args_os().len() != 1 {
        return Err("usage: compact-speed-analysis < transcript.json".into());
    }
    const MAX: u64 = 16 << 20;
    let mut bytes = Vec::new();
    io::stdin()
        .lock()
        .take(MAX + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX {
        return Err("transcript exceeds 16 MiB".into());
    }
    let case = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let report = compact_speed_analysis::analyze(&case);
    let admitted = report.status == "calculated_exploratory_case";
    println!(
        "{}",
        serde_json::to_string(&report).map_err(|e| e.to_string())?
    );
    Ok(admitted)
}
fn main() {
    let code = match run() {
        Ok(true) => 0,
        Ok(false) => 2,
        Err(reason) => {
            println!(
                "{}",
                serde_json::json!({"schema":"compact-speed-analysis/v1", "status":"input_error", "reason":reason, "numerical_rule_passes":false, "execution_admission_verified":false, "family_confirmed":false})
            );
            2
        }
    };
    std::process::exit(code);
}
