//! Native compact publication and report from retained Runpod49 analysis/replay.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn n(v: &Value) -> Result<u128> {
    Ok(v.as_str().ok_or("subtotal string")?.parse()?)
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
fn write_new(p: &Path, b: &[u8]) -> Result<()> {
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)?
        .write_all(b)?;
    Ok(())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 1 {
        return Err("usage: REPORT EXISTING_PACKET".into());
    }
    let packet = Path::new(&a[0]);
    let raw = fs::read(packet.join("run-study.json"))?;
    let mut s: Value = serde_json::from_slice(&raw)?;
    let replay_bytes = fs::read(packet.join("postexecution-replay.json"))?;
    let q: Value = serde_json::from_slice(&replay_bytes)?;
    if s["schema"] != "cover.discriminant-pair.run-study/v1"
        || q["schema"] != "cover.discriminant-pair.postexecution-replay/v1"
        || q["original_frozen_qualifier_exit_code"] != 1
        || q["matched_pair"]["rank_comparison_excludes_only_native_inner_wall_ns"] != true
        || q["completed"] != 4
    {
        return Err("study/replay binding".into());
    }
    let mut rows = String::new();
    for (r, name) in s["outcomes"]
        .as_array_mut()
        .ok_or("outcomes")?
        .iter_mut()
        .zip([
            "Original IC",
            "Quadratic table",
            "Grounded pairs",
            "Screened pairs",
        ])
    {
        rows.push_str(&format!(
            "[{name}],[{}],[{:.3}],\n",
            comma(n(&r["observed_incomplete_bitops"])?),
            r["inner_wall_ns_practicality_only"]
                .as_u64()
                .ok_or("timer")? as f64
                / 1e9
        ));
        if let Some(checks) = r["rank_checks"].as_array_mut() {
            for c in checks {
                *c = json!({"b_count":c["b_count"],"rows":c["rows"],
                    "rank_certificate":c["result"]["rank_certificate"],
                    "inner_wall_ns":c["native"]["inner_wall_ns"],
                    "observed_incomplete_subtotal":c["native"]["observed_incomplete_subtotal"]});
            }
        }
        let phases:Vec<Value>=r["phases"].as_array().ok_or("phases")?.iter()
            .map(|p|json!({"phase":p["phase"],"observed_incomplete_bitops":p["diagnostic_priced_bitops"]})).collect();
        r["phases"] = json!(phases);
    }
    s["schema"] = json!("cover.discriminant-pair.run-study-compact/v1");
    s["full_analysis_local_sha256"] = json!(format!("{:x}", Sha256::digest(&raw)));
    s["full_replay_sha256"] = json!(format!("{:x}", Sha256::digest(&replay_bytes)));
    let o = s["outcomes"].as_array().ok_or("outcomes")?;
    let before = n(&o[2]["observed_incomplete_bitops"])?;
    let after = n(&o[3]["observed_incomplete_bitops"])?;
    let saved = before.checked_sub(after).ok_or("screen loss")?;
    s["screen_observed_subtotal_reduction"] = json!(saved.to_string());
    s["screen_observed_subtotal_reduction_percent"] = json!(100.0 * saved as f64 / before as f64);
    write_new(
        &packet.join("run-study-compact.json"),
        &serde_json::to_vec_pretty(&s)?,
    )?;
    let out = packet.join("report");
    fs::create_dir_all(&out)?;
    let mut svg = String::from(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="255" viewBox="0 0 1000 255"><rect width="1000" height="255" fill="white"/><g font-family="DejaVu Sans" fill="#203747">"##,
    );
    for (i, (r, name)) in s["outcomes"]
        .as_array()
        .ok_or("outcomes")?
        .iter()
        .zip([
            "Original IC",
            "Quadratic table",
            "Grounded pairs",
            "Screened pairs",
        ])
        .enumerate()
    {
        let y = 15 + i * 51;
        let val = n(&r["observed_incomplete_bitops"])?;
        let width = val as f64 / 7e12 * 590.;
        svg.push_str(&format!(r##"<text x="230" y="{}" font-size="20" text-anchor="end">{name}</text><rect x="245" y="{y}" width="{width:.2}" height="29" fill="#347a9e"/><text x="975" y="{}" font-size="20" text-anchor="end">{:.3}e12</text>"##,y+23,y+23,val as f64/1e12));
    }
    svg.push_str("<text x=\"245\" y=\"244\" font-size=\"18\">Observed incomplete work; smaller is better</text></g></svg>");
    write_new(&out.join("observed-work.svg"), svg.as_bytes())?;
    let typ=r##"#set page(paper: "a4", margin: (x:15mm,y:13mm))
#set text(font:"DejaVu Sans",size:9pt)
#set par(leading:0.55em)
= Discriminant screening reduces graph-IC relation work
*Runpod49 · 17 November 2026 · four cold Rust workers*

Both grounded-pair collectors recover the expected elliptic scalar after raising the declared split-history cap to524288. The screened collector removes198848 provably ineligible quartics while preserving every source, graph projection, sparse kernel, descent and affine solve. Its observed work subtotal falls @PERCENT@% against unscreened pairs; both remain above the original IC and quadratic-table baselines in this unit. [pcc:unverified]

#table(columns:(1.4fr,1.6fr,0.7fr),inset:5pt,stroke:0.4pt,
 [*Arm*],[*Observed work subtotal*],[*Inner s*],
 @ROWS@
)
#image("observed-work.svg",width:100%)

All subtotals are incomplete native bitops, not complete solve costs. Inner elapsed time is a practicality observation: one fixed-order cold call per arm on the same host/CPU/binary, no warmup, retry or exclusion. Full ratios and primary rho comparison remain unreported. [pcc:unverified]

== Exact geometry and cheaper algebraic refusals
Both pairs stop at B185344 with108058 validated sources,522 emitted rows and certified homogeneous rank508. Sources comprise250 full,9535 one-class and98273 two-class relations. The current graph discards93640 ungrounded two-class sources. This motivates a separately bounded signed-forest retention study on recorded data. [pcc:unverified]

Screen298778 quartic calls; refuse198848 by zero/nonsquare discriminant character; retain99930 calls to the original bounded splitter. Both pairs have identical retained random polynomials and uniform events; the necessary screen never substitutes for full factor/principal checks. Their3415 descent residuals and final affine scalar are identical. [pcc:unverified]

== Frozen check fails; separate replay preserves it
The frozen qualifier exits1at “paired exact geometry rank_checks”: it erroneously requires measured checkpoint times to agree (42529515 versus42358498ns). Every other checkpoint field matches. A separately sourced postexecution Rust replay excludes only that timer from pair equality, validates timer encoding, and independently checks all original arithmetic, source/graph/rank/flow/RNG/affine records. Original records and failed qualification remain unchanged. Replay controls150pass; no scientific call is repeated. [pcc:unverified]

== Execution and evidence scope
Execution source bd69a76db876fa1da120de0edf1033489327d937, same152scientific paths as7e574194, published before allocation. Runpod CPU5c4CPU/8GiB0.14USD/hour; owned pod89omx6a1a0ryra created09:28:06Z, checked live guardianPID134219, sole dispatch09:31:01Z. Archive collected/hash-checked/private-scanned before DELETE204/independentGET404at09:41:25Z. Large originals and full study remain hash-bound LOCAL; compact study and separate replay are published. [pcc:unverified]

PCC-20261117-208002/bba5e6 and replay/report claims remain UNVERIFIED producer evidence. Complete native cost, fresh targets, scaling and signed independent admission remain open. Swan1962, Corollary1/§4 (credits Stickelberger), supports the necessary finite-field discriminant test; no novelty claim. [pcc:unverified]
"##.replace("@ROWS@",&rows).replace("@PERCENT@",&format!("{:.3}",100.0*saved as f64/before as f64));
    write_new(&out.join("report.typ"), typ.as_bytes())?;
    Ok(())
}
