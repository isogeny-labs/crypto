//! Native descriptive failure report. No missing counter is repriced or admitted.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::Path,
};
fn digest(path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(path)?;
    let mut hash = Sha256::new();
    let mut size = 0u64;
    let mut b = [0u8; 65536];
    loop {
        let n = file.read(&mut b)?;
        if n == 0 {
            break;
        }
        hash.update(&b[..n]);
        size += n as u64;
    }
    Ok(json!({"bytes":size,"sha256":format!("{:x}",hash.finalize())}))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() == 3 && args[0] == "--render" {
        let s: Value = serde_json::from_slice(&fs::read(&args[1])?)?;
        if s["qualified"] != false || s["qualifier_exit_code"] != 1 {
            return Err("failure report registration".into());
        }
        let dir = Path::new(&args[2]);
        fs::create_dir(dir)?;
        let mut rows = String::new();
        for (i, w) in s["workers"]
            .as_array()
            .ok_or("worker list")?
            .iter()
            .enumerate()
        {
            let y = 180 + i * 58;
            rows.push_str(&format!(r##"<rect x="35" y="{}" width="930" height="50" rx="5" fill="#f3f5f8"/><text x="50" y="{}">{}</text><text x="330" y="{}">{}</text><text x="505" y="{}">{}</text><text x="650" y="{}">{}</text><text x="830" y="{}" fill="#b42318">{}</text>"##,y-30,y,w["mode"].as_str().ok_or("mode")?,y,w["producer_reported_residuals"],y,w["producer_reported_partials"],y,w["producer_reported_matches"],y,w["foreign_thread_hooks"]));
        }
        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="560" viewBox="0 0 1000 560"><rect width="1000" height="560" fill="white"/><g font-family="sans-serif" font-size="19" fill="#172b4d"><text x="35" y="42" font-size="30" font-weight="700">Anchored descent42 — ownership gate rejected</text><text x="35" y="77" font-size="17">Unsigned producer record | identical seed401 | copied qualifier exit1</text><text x="50" y="130" font-weight="700">Worker</text><text x="330" y="130">Residuals</text><text x="505" y="130">Partials</text><text x="650" y="130">Matches</text><text x="800" y="130">Foreign hooks</text>{rows}<rect x="35" y="350" width="930" height="66" rx="7" fill="#fff0ed"/><text x="55" y="378" fill="#b42318" font-weight="700">Calling thread claimed; nested F4 ran on an unowned Rayon worker.</text><text x="55" y="403" font-size="17">Counter subtotals rejected. Geometry/yield columns above remain producer descriptions.</text><rect x="35" y="435" width="930" height="64" rx="7" fill="#edf5fc"/><text x="55" y="462">Next: install the whole call on one Rayon worker; retain the rejection gate.</text><text x="55" y="486" font-size="17">Repeat the same input/stream separately; never reprice or discard this failed capture.</text><text x="35" y="534" font-size="15">Source9c7ab02c1e | original archive hash-bound | collected before DELETE204 / GET40423:50:26Z</text></g></svg>"##
        );
        fs::write(dir.join("failure-overview.svg"), svg)?;
        fs::write(dir.join("report.typ"),"#set page(paper: \"a4\", flipped: true, margin: 13mm)\n#set text(size: 9pt)\n#image(\"failure-overview.svg\", width: 100%)\nUnsigned failure/custody assertion PCC-20261117-73eded. Scientific source, raw originals and copied refusal remain unchanged. Complete costs and ratios are not admitted.\n")?;
        return Ok(());
    }
    if args.len() != 2 {
        return Err("usage: REPORT TRIAL42_STAGE NEW_SUMMARY".into());
    }
    let stage = Path::new(&args[0]);
    let run = stage.join("returned/task/run");
    let error = fs::read_to_string(run.join("analysis-original.log"))?;
    if error.trim() != "Error: \"component cost/scope\""
        || fs::read_to_string(run.join("analysis-original.exit-code.txt"))?.trim() != "1"
    {
        return Err("expected original qualifier refusal".into());
    }
    let mut workers = Vec::new();
    for mode in ["original-nagao", "full-only", "anchored"] {
        let path = run.join(format!("{mode}.raw.json"));
        let raw: Value = serde_json::from_slice(&fs::read(&path)?)?;
        if raw["source_revision"] != "9c7ab02c1e20cd5d0fe93b3b49a53c087681a78f"
            || raw["mode"] != mode
            || raw["seed"] != 401
            || raw["foreign_thread_hooks"] != 1
            || fs::read_to_string(run.join(format!("{mode}.exit-code.txt")))?.trim() != "0"
        {
            return Err("registered failure worker".into());
        }
        workers.push(json!({"mode":mode,"worker_exit_code":0,"raw":digest(&path)?,"producer_reported_residuals":raw["trials"].as_array().map(Vec::len),"producer_reported_rows":raw["rows"].as_array().map(Vec::len),"producer_reported_rank":raw["rank"],"producer_reported_verified":raw["verified"],"producer_reported_partials":raw["partial_rows"],"producer_reported_matches":raw["matched_partials"],"foreign_thread_hooks":1,"cost_admitted":false}));
    }
    let summary = json!({"schema":"cover.anchored-descent.failed-capture/v1","source_revision":"9c7ab02c1e20cd5d0fe93b3b49a53c087681a78f","seed":401,"qualified":false,"qualifier_exit_code":1,"original_qualifier_error":error.trim(),"workers":workers,"archive":digest(&stage.join("artifacts.tar.gz"))?,"collection_timestamp":fs::read_to_string(stage.join("collected-at.txt"))?.trim(),"deletion_timestamp":fs::read_to_string(stage.join("deleted-at.txt"))?.trim(),"delete_http":fs::read_to_string(stage.join("delete.status"))?.trim(),"absence_http":fs::read_to_string(stage.join("absence.status"))?.trim(),"primitive_costs_admitted":false,"complete_total":null,"ratios":null,"retries_of_this_capture":0,"excluded_from_history":false,"interpretation":"Copied checker correctly refused foreign-thread work. Producer geometry/yield fields are descriptive, not independent mathematical or cost qualification. A changed-source ownership-controlled repeat must preserve this failed attempt."});
    let mut out = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&args[1])?;
    serde_json::to_writer_pretty(&mut out, &summary)?;
    out.write_all(b"\n")?;
    Ok(())
}
