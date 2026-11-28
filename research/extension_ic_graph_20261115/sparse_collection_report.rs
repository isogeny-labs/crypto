//! Export and render recorded qualified data. Never invokes a scientific worker.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn number(v: &Value) -> Result<u128> {
    if let Some(s) = v.as_str() {
        Ok(s.parse()?)
    } else {
        Ok(v.as_u64().ok_or("integer")? as u128)
    }
}
fn comma(x: u128) -> String {
    let s = x.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}
fn inventory(root: &Path, path: &Path, out: &mut String) -> Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?.collect::<std::io::Result<_>>()?;
    entries.sort_by_key(|e| e.path());
    for e in entries {
        let p = e.path();
        if e.file_type()?.is_dir() {
            inventory(root, &p, out)?;
        } else if e.file_type()?.is_file() {
            out.push_str(&format!(
                "{}  {}\n",
                hash(&fs::read(&p)?),
                p.strip_prefix(root)?.display()
            ));
        }
    }
    Ok(())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: REPORT COLLECTED_STAGE NEW_PACKET".into());
    }
    let stage = Path::new(&a[0]);
    let run = stage.join("returned/task/run");
    let original = fs::read(run.join("summary.json"))?;
    let s: Value = serde_json::from_slice(&original)?;
    let deletion: Value = serde_json::from_slice(&fs::read(stage.join("deletion.json"))?)?;
    if s["schema"] != "cover.extension-sparse.qualified/v1"
        || s["source_revision"] != "ed5ca5ab8a4f41f89165a40e7efefc21aa76a399"
        || s["planned"] != 9
        || s["completed"] != 9
        || s["excluded"] != 0
        || s["automatic_retries"] != 0
        || s["complete_cost"] != false
        || s["rho_parity"] != false
        || s["independent_signed_verdict"] != false
        || deletion["delete_http"] != 204
        || deletion["independent_get_http"] != 404
        || fs::read_to_string(run.join("analysis-original.exit-code.txt"))?.trim() != "0"
    {
        return Err("original qualifier and custody gates".into());
    }
    let modes = [
        "ic",
        "extension-full-smooth",
        "extension-remainders",
        "extension-sparse-full",
        "extension-sparse-remainders",
        "rho-plain-1",
        "rho-negation-1",
        "rho-plain-16",
        "rho-negation-16",
    ];
    let rows = s["outcomes"].as_array().ok_or("outcomes")?;
    if rows.len() != modes.len() {
        return Err("nine outcomes".into());
    }
    for (r, m) in rows.iter().zip(modes) {
        if r["mode"] != m
            || fs::read_to_string(run.join(format!("{m}.exit-code.txt")))?.trim() != "0"
        {
            return Err("mode/exit".into());
        }
    }
    let compact = serde_json::to_vec(&s)?;
    if compact.len() > 65536 || serde_json::from_slice::<Value>(&compact)? != s {
        return Err("lossless compact export".into());
    }
    let packet = Path::new(&a[1]);
    fs::create_dir(packet)?;
    fs::write(packet.join("qualified-summary.compact.json"), &compact)?;
    fs::write(
        packet.join("summary-export.json"),
        serde_json::to_vec_pretty(&json!({
            "schema":"cover.lossless-json-export/v1","original_path":run.join("summary.json"),
            "original_bytes":original.len(),"original_sha256":hash(&original),
            "export_file":"qualified-summary.compact.json","export_bytes":compact.len(),
            "export_sha256":hash(&compact),"serde_value_equal":true,
            "original_not_modified":true,"scientific_measurement":false
        }))?,
    )?;
    // Copy exact small originals. The complete originals stay LOCAL and hash-bound.
    for dir in [&run, stage] {
        for e in fs::read_dir(dir)? {
            let e = e?;
            let p = e.path();
            if e.file_type()?.is_file() && e.metadata()?.len() <= 65536 {
                let dest = packet.join(e.file_name());
                if dest.exists() {
                    if fs::read(&dest)? != fs::read(&p)? {
                        return Err("duplicate original name differs".into());
                    }
                } else {
                    fs::copy(p, dest)?;
                }
            }
        }
    }
    let mut manifest = String::new();
    inventory(stage, stage, &mut manifest)?;
    fs::write(packet.join("local-originals.sha256"), manifest)?;
    let report = packet.join("report");
    fs::create_dir(&report)?;
    let mut table=String::from("| Arm | Observed incomplete primitive subtotal (bitops model) | Inner seconds |\n|---|---:|---:|\n");
    let mut body = String::new();
    let names = [
        "Original IC",
        "Graph: full smooth",
        "Graph: quadratic",
        "Sparse: full smooth",
        "Sparse: quadratic",
        "Rho: plain / 1",
        "Rho: negation / 1",
        "Rho: plain / 16",
        "Rho: negation / 16",
    ];
    let mut derived = Vec::new();
    for (i, r) in rows.iter().enumerate() {
        let cost = number(&r["proof"]["accounting"]["observed_primitive_subtotal_bitops"])?;
        let ns = number(&r["inner_wall_ns"])?;
        let y = 145 + i * 49;
        let color = if i < 3 {
            "#376b97"
        } else if i < 5 {
            "#389476"
        } else {
            "#956aab"
        };
        body.push_str(&format!(r##"<rect x="25" y="{}" width="1150" height="43" fill="#f2f5f8"/><text x="35" y="{y}">{}</text><text x="285" y="{y}" font-size="16">{}</text><rect x="555" y="{}" width="{:.2}" height="17" fill="{color}"/><text x="805" y="{y}">{:.3}</text><rect x="905" y="{}" width="{:.2}" height="17" fill="{color}"/>"##,
            y-28,names[i],comma(cost),y-17,cost as f64/1e12/300.0*220.0,ns as f64/1e9,y-17,ns as f64/1e9/300.0*220.0));
        table.push_str(&format!(
            "| {} | {} | {:.3} |\n",
            modes[i],
            comma(cost),
            ns as f64 / 1e9
        ));
        derived.push(json!({"mode":modes[i],"observed_incomplete_subtotal":cost.to_string(),"inner_wall_ns":ns.to_string()}));
    }
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="820" viewBox="0 0 1200 820"><rect width="1200" height="820" fill="white"/><g font-family="sans-serif" font-size="18" fill="#172b4d"><text x="25" y="40" font-size="29" font-weight="700">Sparse graph collection46 — nine complete solves replay</text><text x="25" y="75" font-size="17">Runpod CPU | p1009 / exposed seed389 | one cold round per arm | n=1, no interval estimated</text><text x="35" y="110">Arm</text><text x="285" y="110">Observed incomplete subtotal</text><text x="555" y="110">0 → 300 trillion bitops</text><text x="805" y="110">Inner s</text><text x="905" y="110">0 → 300 seconds</text>{body}<rect x="25" y="615" width="1150" height="80" fill="#fff3df"/><text x="40" y="644">Sparse checkpoints lower the observed LA and whole primitive subtotals in both matched pairs.</text><text x="40" y="674">Elapsed times rise slightly: 46.219 → 46.384 s; 34.887 → 34.929 s. No elapsed improvement inferred.</text><text x="25" y="730" font-size="16">All workers/checker exit0; common sources/witnesses, kernel, descent stream and affine endpoints agree.</text><text x="25" y="759" font-size="16">Unsigned software replay. Complete counts/ratios null; rho parity false. Four rho roles are the registered portfolio.</text><text x="25" y="789" font-size="15">Source ed5ca5ab8a4f | sole POST 2026-11-17T04:17:53Z | owned pod deleted 04:37:15Z after collection</text></g></svg>"##
    );
    fs::write(report.join("overview.svg"), svg)?;
    fs::write(report.join("table.md"), table)?;
    let phases = ["relations", "linear_algebra", "descent"];
    let colors = ["#4386aa", "#b98743", "#58926c"];
    let mut body = String::new();
    let mut phase_table =
        String::from("| Arm | Relations | Linear algebra | Descent |\n|---|---:|---:|---:|\n");
    for (i, r) in rows.iter().take(5).enumerate() {
        let y = 154 + i * 76;
        let mut x = 330.0;
        body.push_str(&format!(
            r##"<text x="25" y="{}">{}</text>"##,
            y + 23,
            names[i]
        ));
        phase_table.push_str(&format!("| {}", modes[i]));
        for (phase, color) in phases.iter().zip(colors) {
            let p = r["proof"]["accounting"]["phases"]
                .as_array()
                .ok_or("phases")?
                .iter()
                .find(|p| p["phase"] == *phase)
                .ok_or("phase")?;
            let c = number(&p["diagnostic_priced_bitops"])?;
            let width = c as f64 / 1e12 / 7.0 * 800.0;
            let font = if *phase == "linear_algebra" { 10 } else { 16 };
            body.push_str(&format!(r##"<rect x="{x:.2}" y="{y}" width="{width:.2}" height="38" fill="{color}"/><text x="{:.2}" y="{}" fill="white" font-size="{font}">{:.3}</text>"##,x+3.0,y+25,c as f64/1e12));
            x += width;
            phase_table.push_str(&format!(" | {}", comma(c)));
        }
        phase_table.push_str(" |\n");
    }
    fs::write(report.join("phases.md"), phase_table)?;
    fs::write(
        report.join("phases.svg"),
        format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="750" viewBox="0 0 1200 750"><rect width="1200" height="750" fill="white"/><g font-family="sans-serif" font-size="20" fill="#172b4d"><text x="25" y="40" font-size="29" font-weight="700">Replacing collection RREF saves counted work; descent dominates</text><text x="25" y="75" font-size="17">Recorded exclusive phases, trillion observed bitops; n=1 per arm. Native coverage incomplete.</text><rect x="330" y="100" width="18" height="18" fill="#4386aa"/><text x="358" y="117">Relations</text><rect x="530" y="100" width="18" height="18" fill="#b98743"/><text x="558" y="117">Linear algebra</text><rect x="780" y="100" width="18" height="18" fill="#58926c"/><text x="808" y="117">Descent</text>{body}<text x="25" y="585">LA: 0.672 → 0.282 T (full); 0.693 → 0.327 T (quadratic). One certified checkpoint each.</text><text x="25" y="620">Surplus collection adds seven/six projected rows. All 3,415 descent residuals stay identical.</text><text x="25" y="655">Quadratic sparse descent: 3.426 T; relations: 1.539 T; LA: 0.327 T.</text><text x="25" y="694" font-size="17">Next discriminating lead: cheaper algebraic relation/decomposition construction with exact witness replay.</text><text x="25" y="727" font-size="15">Run46 | same public target, source ed5ca5ab8a4f | remaining setup/verification phases included in whole totals</text></g></svg>"##
        ),
    )?;
    fs::write(report.join("report.typ"),"#set page(paper: \"a4\", flipped: true, margin: 10mm)\n#set text(size: 8pt)\n#image(\"overview.svg\", width: 100%, height: 176mm, fit: \"contain\")\nObserved subtotals are independently repriced partial primitive counts, not complete machine work. All setup remains charged to the cold pipeline; admission, serialization, verifier and uncovered native work are not fully counted. This exposed target is development evidence, not fresh confirmation.\n#pagebreak()\n#image(\"phases.svg\", width: 100%, height: 176mm, fit: \"contain\")\nSource methods: Joux–Vitse cover construction; signed quadratic-class graph elimination; Eberly–Kaltofen (ISSAC 1997, doi:10.1145/258726.258776) full randomized Lanczos rank/kernel certificates. Exact source and trace verification is recorded by the frozen Rust checker; no independently signed execution verdict.\n")?;
    fs::write(
        report.join("report-metadata.json"),
        serde_json::to_vec_pretty(
            &json!({"schema":"cover.extension-sparse.report/v1","source_revision":s["source_revision"],"summary_original_sha256":hash(&original),"rows":derived,"deletion":deletion,"complete_cost":false,"complete_total":null,"ratios":null,"rho_parity":false,"scientific_worker_invoked":false,"canonical_graphs":"Historical frozen figures unchanged; no scoreboard promotion"}),
        )?,
    )?;
    println!(
        "Exported {} original bytes to {} lossless JSON bytes; nine rows; no worker invoked",
        original.len(),
        compact.len()
    );
    Ok(())
}
