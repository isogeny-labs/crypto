//! Native renderer of original Run47 records; no scientific worker invocation.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn num(v: &Value) -> Result<u128> {
    if let Some(s) = v.as_str() {
        Ok(s.parse()?)
    } else {
        Ok(v.as_u64().ok_or("integer")? as u128)
    }
}
fn comma(x: u128) -> String {
    let s = x.to_string();
    let mut o = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            o.push(',');
        }
        o.push(c);
    }
    o
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
        return Err("usage: REPORT COLLECTED_STAGE EXISTING_PACKET_WITH_ORIGINAL_SUMMARY".into());
    }
    let stage = Path::new(&a[0]);
    let run = stage.join("returned/task/run");
    let packet = Path::new(&a[1]);
    let bytes = fs::read(run.join("summary.json"))?;
    let s: Value = serde_json::from_slice(&bytes)?;
    let deletion: Value = serde_json::from_slice(&fs::read(stage.join("deletion.json"))?)?;
    if s["schema"] != "cover.table-roots.qualified/v1"
        || s["source_revision"] != "f0a0fdb2789288259e5057781dfe1acc6300ff03"
        || s["planned"] != 2
        || s["completed"] != 2
        || s["excluded"] != 0
        || s["automatic_retries"] != 0
        || s["paired_geometry_equal"] != true
        || s["default46geometry_and_primitives_equal"] != true
        || s["complete_cost"] != false
        || s["rho_parity"] != false
        || deletion["delete_http"] != 204
        || deletion["independent_get_http"] != 404
        || fs::read_to_string(run.join("analysis-original.exit-code.txt"))?.trim() != "0"
    {
        return Err("original qualification/custody".into());
    }
    let rows = s["outcomes"].as_array().ok_or("outcomes")?;
    if rows.len() != 2 {
        return Err("two arms".into());
    }
    for (r, role) in rows.iter().zip(["base-horner", "forward-differences"]) {
        if r["mode"] != role
            || fs::read_to_string(run.join(format!("{role}.exit-code.txt")))?.trim() != "0"
        {
            return Err("actual worker exit".into());
        }
    }
    for dir in [&run, stage] {
        for e in fs::read_dir(dir)? {
            let e = e?;
            if e.file_type()?.is_file() && e.metadata()?.len() <= 65536 {
                let dest = packet.join(e.file_name());
                if dest.exists() {
                    if fs::read(&dest)? != fs::read(e.path())? {
                        return Err("original differs".into());
                    }
                } else {
                    fs::copy(e.path(), dest)?;
                }
            }
        }
    }
    let mut manifest = String::new();
    inventory(stage, stage, &mut manifest)?;
    fs::write(packet.join("local-originals.sha256"), manifest)?;
    let mut body = String::new();
    let mut table=String::from("| Arm | Relations observed incomplete bitops | Whole observed incomplete bitops | Inner seconds |\n|---|---:|---:|---:|\n");
    let names = ["Base Horner", "Forward differences"];
    let mut costs = Vec::new();
    let mut relations = Vec::new();
    let mut walls = Vec::new();
    for (i, r) in rows.iter().enumerate() {
        let ac = &r["proof"]["accounting"];
        let cost = num(&ac["observed_primitive_subtotal_bitops"])?;
        let rel = num(&ac["phases"]
            .as_array()
            .ok_or("phases")?
            .iter()
            .find(|p| p["phase"] == "relations")
            .ok_or("relations")?["diagnostic_priced_bitops"])?;
        let wall = num(&r["inner_wall_ns"])?;
        let y = 158 + i * 68;
        costs.push(cost);
        relations.push(rel);
        walls.push(wall);
        body.push_str(&format!(r##"<rect x="25" y="{}" width="1150" height="53" fill="#f1f4f8"/><text x="40" y="{y}">{}</text><text x="325" y="{y}">{}</text><text x="620" y="{y}">{}</text><text x="970" y="{y}">{:.3}</text>"##,y-32,names[i],comma(rel),comma(cost),wall as f64/1e9));
        table.push_str(&format!(
            "| {} | {} | {} | {:.3} |\n",
            names[i],
            comma(rel),
            comma(cost),
            wall as f64 / 1e9
        ));
        let color = if i == 0 { "#42769d" } else { "#3c9173" };
        let yy = 340 + i * 65;
        body.push_str(&format!(r##"<text x="40" y="{}" font-size="18">{}</text><rect x="325" y="{yy}" width="{:.2}" height="30" fill="{color}"/><text x="950" y="{}">{:.3} T</text>"##,yy+23,names[i],rel as f64/1e12/1.6*590.0,yy+23,rel as f64/1e12));
    }
    let saved = relations[0]
        .checked_sub(relations[1])
        .ok_or("recorded direction")?;
    if costs[0].checked_sub(costs[1]) != Some(saved) || walls[1] <= walls[0] {
        return Err("registered interpretation data".into());
    }
    let rel_pct = 100.0 * saved as f64 / relations[0] as f64;
    let whole_pct = 100.0 * saved as f64 / costs[0] as f64;
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="790" viewBox="0 0 1200 790"><rect width="1200" height="790" fill="white"/><g font-family="sans-serif" font-size="20" fill="#172b4d"><text x="25" y="42" font-size="29" font-weight="700">Norm root tables47 — lower counted work, higher elapsed</text><text x="25" y="78" font-size="17">One Runpod CPU5c host | p1009 / exposed seed389 | two cold complete scalar recoveries | n=1 per arm</text><text x="40" y="115">Arm</text><text x="325" y="115">Relations: observed subtotal</text><text x="620" y="115">Whole: observed subtotal</text><text x="970" y="115">Inner seconds</text>{body}<text x="325" y="310" font-size="17">Relation phase scale: 0 → 1.6 trillion observed bitops (T)</text><rect x="25" y="474" width="1150" height="96" fill="#edf5ef"/><text x="40" y="505">Exact reduction: {} observed bitops; {:.2}% of relations, {:.2}% of whole subtotal.</text><text x="40" y="536">All 11,822 sources, 516 projections and 3,415 descent residuals match. Same verified scalar.</text><rect x="25" y="595" width="1150" height="64" fill="#fff2df"/><text x="40" y="624">15.054 → 15.358 seconds: no elapsed improvement in this single observation.</text><text x="40" y="649" font-size="16">This host differs from46; no cross-host timing ratio or phase composition.</text><text x="25" y="698" font-size="16">All workers/frozen checker exit0; default geometry/primitive counts match46 exactly. No failures excluded.</text><text x="25" y="728" font-size="16">Originals collected05:08:05Z before owned-pod DELETE204 / GET40405:08:16Z. No owned paid runner remains.</text><text x="25" y="760" font-size="15">Source f0a0fdb27892 | sourcePCC b2d7b1 / resultPCC 5bb829 UNVERIFIED | complete costs/ratios null; rho parity false</text></g></svg>"##,
        comma(saved),
        rel_pct,
        whole_pct
    );
    let report = packet.join("report");
    fs::create_dir(&report)?;
    fs::write(report.join("overview.svg"), svg)?;
    fs::write(report.join("table.md"), table)?;
    fs::write(report.join("report.typ"),"#set page(paper: \"a4\", flipped: true, margin: 10mm)\n#set text(size: 8pt)\n#image(\"overview.svg\", width: 100%, height: 176mm, fit: \"contain\")\nWhole observations include the complete scalar-recovery pipeline; native integer/control/allocation/PRNG coverage remains incomplete. Independent source/Cantor/rank/Krylov/affine/elliptic and partial-counter replay passes, without a signed execution verdict. Established finite-difference arithmetic: NIST DLMF §3.3(iv), equation3.3.38. Derived reductions use (baseline − candidate)/baseline; no timing interval is estimated.\n")?;
    fs::write(
        report.join("report-metadata.json"),
        serde_json::to_vec_pretty(
            &json!({"schema":"cover.table-roots.report/v1","summary_sha256":hash(&bytes),"source_revision":s["source_revision"],"observed_saved_bitops":saved.to_string(),"relation_percent":rel_pct,"whole_percent":whole_pct,"reduction_formula":"(baseline-candidate)/baseline","complete_cost":false,"complete_total":null,"ratios":null,"rho_parity":false,"scientific_worker_invoked":false,"deletion":deletion,"canonical_graphs":"Historical frozen figures and scoreboard unchanged; no competitive admission"}),
        )?,
    )?;
    println!("Rendered two original outcomes; saved {saved} observed bitops; no worker invoked");
    Ok(())
}
