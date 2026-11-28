//! Render retained symbolic control evidence; no relation finder or benchmark.
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: REPORT CONTROL_DIRECTORY NEW_REPORT_DIRECTORY".into());
    }
    let p = Path::new(&a[0]);
    let out = Path::new(&a[1]);
    let tests = fs::read(p.join("tests-exact-witness.log"))?;
    if !String::from_utf8_lossy(&tests).contains("test result: ok. 6 passed; 0 failed")
        || !fs::read(p.join("clippy-exact-witness.log"))?.is_empty()
    {
        return Err("native control receipts".into());
    }
    fs::create_dir(out)?;
    let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="780" viewBox="0 0 1200 780"><rect width="1200" height="780" fill="white"/><g font-family="sans-serif" fill="#172b4d"><text x="25" y="40" font-size="29" font-weight="700">Grounded signed cofactor graph48 — integer witnesses</text><text x="25" y="74" font-size="17">Generic algebraic core | six control groups pass | 64 synthetic signed streams | no natural-yield or elapsed claim</text><rect x="25" y="103" width="1150" height="290" rx="6" fill="#f1f5f8"/><circle cx="165" cy="275" r="34" fill="#d5e5f3"/><circle cx="480" cy="205" r="36" fill="#d8eddf"/><circle cx="890" cy="205" r="36" fill="#d8eddf"/><g stroke="#467e65" stroke-width="4" fill="none"><path d="M200 268 L444 214"/><path d="M517 205 L853 205"/></g><path d="M510 238 Q690 340 860 238" fill="none" stroke="#456eab" stroke-width="4" stroke-dasharray="10 5"/><text x="143" y="281" font-size="20">root</text><text x="464" y="212" font-size="22">D0</text><text x="874" y="212" font-size="22">D1</text><text x="258" y="210" font-size="19">R0: +D0</text><text x="595" y="173" font-size="19">R1: +D0 +D1</text><text x="580" y="319" font-size="19" fill="#456eab">R2: +D0 −D1</text><text x="365" y="362" font-size="18">A0 = R0</text><text x="790" y="362" font-size="18">A1 = R1 −R0</text><rect x="25" y="414" width="1150" height="110" fill="#edf5ef"/><text x="40" y="447" font-size="23">Emit W = R2 −A0 +A1 = −2R0 +R1 +R2</text><text x="40" y="480" font-size="21">Large incidence: −2(1, 0) +(1, 1) +(1, −1) = (0, 0)</text><text x="40" y="509" font-size="17">Coefficient 2 is retained exactly. Witness support and source ids are bounded; caps preserve prior anchors.</text><rect x="25" y="545" width="1150" height="78" fill="#fff2df"/><text x="40" y="576" font-size="21">An ungrounded all-plus triangle leaves (2, 2, 2); it does not justify a full row.</text><text x="40" y="605" font-size="18">Graph rows are not rank certificates. Integer incidence replay checks every emitted/stored witness.</text><text x="25" y="660" font-size="18">Next dependency: bounded quartic splitting, two principal/Mumford certificates, full IC collector integration.</text><text x="25" y="693" font-size="17">Source: Gaudry–Thériault–Thomé, 2005 author preprint §§2.3 / 3.1 / 3.3; grounded-tree variant.</text><text x="25" y="725" font-size="16">Caller must check actual curve relations. Uniform-class assumptions and the paper's complexity bound are not imported.</text><text x="25" y="758" font-size="15">PCC-20261117-602228 UNVERIFIED | native unit controls only | no Runpod48call/allocation | no Lean-checked theorem</text></g></svg>"##;
    fs::write(out.join("witness.svg"), svg)?;
    fs::write(out.join("report.typ"),"#set page(paper: \"a4\", flipped: true, margin: 10mm)\n#set text(size: 8pt)\n#image(\"witness.svg\", width: 100%, height: 177mm, fit: \"contain\")\nEach source relation has at most two distinct signed cofactor classes. A retained anchor has large incidence exactly one canonical class. For signs s,t in {−1,1}, t(R−s A_x) has incidence D_y; when both endpoints are anchored, R−s A_x−t A_y has zero large incidence. This is a symbolic derivation/control illustration; geometric relation admission and genuine performance remain separate dependencies.\n")?;
    fs::write(
        out.join("report-metadata.json"),
        serde_json::to_vec_pretty(
            &json!({"schema":"cover.grounded-pair-graph.report/v1","unit_log_sha256":format!("{:x}",Sha256::digest(tests)),"control_groups":6,"synthetic_streams":64,"source_relations_per_stream":257,"example_witness":[[0,-2],[1,1],[2,1]],"scientific_measurement":false,"geometric_relation_finder":false,"complete_dlp":false,"promotion_eligible":false,"claim":"PCC-20261117-602228","standing":"UNVERIFIED"}),
        )?,
    )?;
    Ok(())
}
