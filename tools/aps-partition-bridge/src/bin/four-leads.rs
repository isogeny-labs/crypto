use aps_partition_bridge::leads::{aux_domains, label_design, pairings, quotient};
use serde_json::{json, Value};
use std::{fs::OpenOptions, io::Write};

const LEADS: [&str; 4] = ["aux_domains", "label_design", "quotient", "pairings"];

fn hex(s: &str, n: usize) -> bool {
    s.len() == n
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn dispatch(lead: &str) -> Result<Value, String> {
    let value = match lead {
        "aux_domains" => serde_json::to_value(aux_domains::run()?),
        "label_design" => serde_json::to_value(label_design::run()?),
        "quotient" => serde_json::to_value(quotient::run()?),
        "pairings" => serde_json::to_value(pairings::run()?),
        _ => return Err("unknown fixed lead".into()),
    };
    value.map_err(|e| e.to_string())
}

fn run() -> Result<bool, String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 8
        || (!LEADS.contains(&args[1].as_str()) && args[1] != "transport-control")
        || args[2] != "--source-revision"
        || args[4] != "--source-manifest-sha256"
        || args[6] != "--out"
        || !hex(&args[3], 40)
        || !hex(&args[5], 64)
    {
        return Err("usage: four-leads LEAD|transport-control --source-revision HEX40 --source-manifest-sha256 HEX64 --out NEW_JSON".into());
    }
    let mut out = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&args[7])
        .map_err(|e| e.to_string())?;
    // Cohort workers reserve independent outputs before the parent releases
    // their common start barrier. No scientific work runs before release.
    if let Ok(path) = std::env::var("APS_FOUR_LEADS_BARRIER") {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        while !std::path::Path::new(&path).is_file() {
            if std::time::Instant::now() >= deadline {
                return Err("parallel cohort start barrier was not released".into());
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
    let control = args[1] == "transport-control";
    let (result, error) = if control {
        let arms: Vec<_> = LEADS.iter().map(|lead| json!({"lead": lead, "error": "intentional transport failure; no solver called"})).collect();
        (
            Some(
                json!({"arms": arms, "flow": {"planned":4,"attempted":4,"completed":0,"failed":4,"excluded_by_rule":0}}),
            ),
            None,
        )
    } else {
        match dispatch(&args[1]) {
            Ok(result) => (Some(result), None),
            Err(error) => (None, Some(error)),
        }
    };
    let passed = if control {
        true
    } else {
        result
            .as_ref()
            .and_then(|r| r.get("complete_and_equal"))
            .and_then(Value::as_bool)
            == Some(true)
            && error.is_none()
    };
    let record = json!({
        "schema":"aps.four-leads/v1", "source_revision":args[3], "source_manifest_sha256":args[5],
        "lead":args[1], "transport_control":control, "performance_trial":false,
        "execution_attested":false,"cost_complete":false,"result":result,"error":error,
        "complete_and_equal":passed && !control,
    });
    serde_json::to_writer(&mut out, &record).map_err(|e| e.to_string())?;
    out.write_all(b"\n").map_err(|e| e.to_string())?;
    Ok(passed)
}

fn main() {
    match run() {
        Ok(true) => {}
        Ok(false) => std::process::exit(2),
        Err(error) => {
            eprintln!("four-leads: {error}");
            std::process::exit(1);
        }
    }
}
