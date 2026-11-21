//! Descriptive Runpod50 report from original qualified records and native study.
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn comma(n: u128) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 1 {
        return Err("usage: REPORT EXISTING_RESULT_PACKET".into());
    }
    let p = Path::new(&a[0]);
    let s: Value = serde_json::from_slice(&fs::read(p.join("run-study.json"))?)?;
    let qb = fs::read(p.join("qualified-original.json"))?;
    let q: Value = serde_json::from_slice(&qb)?;
    let d: Value = serde_json::from_slice(&fs::read(p.join("deletion.json"))?)?;
    if s["schema"] != "cover.deferred-pair.run-study/v1"
        || q["schema"] != "cover.deferred-pair.qualified/v1"
        || s["original_qualified_summary_sha256"] != format!("{:x}", Sha256::digest(&qb))
        || s["source_revision"] != q["source_revision"]
        || s["completed"] != 8
        || q["completed"] != 8
        || d["delete_http"] != 204
        || d["independent_get_http"] != 404
    {
        return Err("original study/qualification/custody binding".into());
    }
    let outcomes = s["outcomes"].as_array().ok_or("outcomes")?;
    let labels = [
        "Original IC",
        "Quadratic table",
        "Grounded screened",
        "Deferred screened",
        "Rho plain batch1",
        "Rho negation batch1",
        "Rho plain batch16",
        "Rho negation batch16",
    ];
    let out = p.join("report");
    fs::create_dir_all(&out)?;
    let mut rows = String::new();
    let mut svg = String::from(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="330" viewBox="0 0 1000 330"><rect width="1000" height="330" fill="white"/><g font-family="DejaVu Sans" font-size="18" fill="#203747">"##,
    );
    for (i, (r, name)) in outcomes.iter().zip(labels).enumerate() {
        let cost: u128 = r["observed_incomplete_bitops"]
            .as_str()
            .ok_or("subtotal")?
            .parse()?;
        let seconds = r["inner_wall_ns_practicality_only"]
            .as_u64()
            .ok_or("inner time")? as f64
            / 1e9;
        rows.push_str(&format!("[{name}],[{}],[{seconds:.3}],\n", comma(cost)));
        let y = 9 + i * 34;
        let width = seconds / 130. * 620.;
        let color = if i == 3 {
            "#2c7395"
        } else if i >= 4 {
            "#8996a0"
        } else {
            "#628d9f"
        };
        svg.push_str(&format!(r#"<text x="245" y="{}" text-anchor="end">{name}</text><rect x="260" y="{y}" width="{width:.3}" height="23" fill="{color}"/><text x="975" y="{}" text-anchor="end">{seconds:.3}s</text>"#,y+18,y+18));
    }
    svg.push_str(r##"<path d="M260 285H880" stroke="#607080"/><text x="260" y="308">0</text><text x="880" y="308" text-anchor="end">130 seconds</text><text x="260" y="329">Run50 · N=1 cold process per arm · inner elapsed observation</text></g></svg>"##);
    fs::write(out.join("elapsed.svg"), svg)?;
    let typ=r##"#set page(paper:"a4",margin:(x:15mm,y:13mm))
#set text(font:"DejaVu Sans",size:9pt)
#set par(leading:0.45em)
= Deferred graph IC reaches rank earlier
*Runpod50 · matched IC/rho portfolio · 17 November 2026*

All eight Rust workers recover the expected elliptic scalar; the original complete qualifier exits0. Retaining ungrounded edges lowers the graph solver's observed work subtotal5.165%against grounded screened pairs. Deferred inner time is24.727s; the fastest registered rho arm is36.902s. These unsigned, exposed-target N=1observations do not establish complete-cost or statistical rho parity. PCC7defbb/6224b9are UNVERIFIED. [pcc:unverified]

#table(columns:(1.4fr,1.6fr,0.6fr),inset:4pt,stroke:0.4pt,
 [*Arm*],[*Observed incomplete bitops*],[*Inner s*],
 @ROWS@
)
#image("elapsed.svg",width:100%)

== What changed, and what now dominates
Deferred collection stops at B155648instead of185344, with90877validated originals and518rows of certified rank508. It retains125491classes/20510anchors and activates8145previously ungrounded edges; no resource refusal. Maximum stored/emitted witness support16/13. [pcc:unverified]

Relations save385877619300observed bitops; longer graph rows add63764686550in linear algebra. Net saving322112932750,5.165%. Both graph arms keep the same3415descent residuals and final affine solve. Descent alone costs3426280619436observed bitops, making a cheaper descent finder the next technical lead. Existing original IC/table remain stronger than this graph arm in the recorded subtotal; the table also has the lowest elapsed time. [pcc:unverified]

== Matched scope, checks and custody
Public p1009/seed389extension-field fixture, subgroup L263807419614139553; one fixed-order cold process per arm, same binary/host/first allowed CPU, zero warmups/retries/exclusions. Rho portfolio: plain/negation×batch1/16, r256/dp16; fastest and lowest observed subtotal are plain batch16. Whole-process elapsed is27.829sfor deferred versus36.908sfor that rho arm, including capture output. No uncertainty interval or global-optimal rho claim. Complete native totals/ratios remain null; fresh/scaling/signed admission remains open. [pcc:unverified]

The original qualifier checks all source/queue/integer incidence/activation/rank/flow/RNG and affine/elliptic data, exact common prefixes and all four rho accounts. Prior source controls45worker/158qualifier pass. Small originals, all rho raw records and60793Boriginal qualification are published; large ICwitnesses/archive stay hash-bound LOCAL. Source11c9a0312f0b0a8ae85be325e72cbf9602d9d777was published before allocation. [pcc:unverified]

Owned CPU5c4CPU8GiB0.14USD/hour podqp4dpp4peu66x9created10:37:16Z; guardianPID138494checked live; sole dispatch10:38:40Z. Originals collected/hash/private-scanned before DELETE204/independentGET404at10:49:49Z. A later export-format correction preserves measured refs/archive/binaries and all scientific code. Primary graph source: Gaudry–Thériault–Thomé corr2004-29§3.1–3.3; necessary screen: Swan1962Corollary1/§4. No scoreboard promotion. [pcc:unverified]
"##.replace("@ROWS@",&rows);
    fs::write(out.join("report.typ"), typ)?;
    fs::write(out.join("figure.typ"),"#set page(width:750pt,height:247.5pt,margin:0pt)\n#image(\"elapsed.svg\",width:750pt,height:247.5pt)\n")?;
    Ok(())
}
