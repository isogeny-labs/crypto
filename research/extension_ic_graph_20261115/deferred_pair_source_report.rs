//! Descriptive report of the hash-bound recorded-input graph projection.
use serde_json::Value;
use std::{fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 1 {
        return Err("usage: REPORT EXISTING_COMPONENT_PACKET".into());
    }
    let p = Path::new(&a[0]);
    let v: Value = serde_json::from_slice(&fs::read(p.join("recorded49-graph-replay.json"))?)?;
    if v["schema"] != "cover.deferred-pair.graph-replay/v1"
        || v["original_sources_consumed"] != 108058
        || v["independent_full_incidences_checked"] != 865
        || v["baseline_full_rows"] != 522
        || v["first_516_rows"]["complete_batch_b_count"] != 155648
        || v["final_actual_rank"] != 508
    {
        return Err("recorded graph result binding".into());
    }
    let out = p.join("report");
    fs::create_dir_all(&out)?;
    let mut svg = String::from(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="350" viewBox="0 0 1000 350"><rect width="1000" height="350" fill="white"/><g font-family="DejaVu Sans" font-size="18" fill="#203747"><path d="M95 20V280H950" fill="none" stroke="#607080"/><path d="M95 124H950" stroke="#8996a0" stroke-dasharray="6 5"/><text x="98" y="117">516-row sparse-check threshold</text><text x="35" y="25">rows</text><text x="95" y="305">0</text><text x="950" y="305" text-anchor="end">185344 enumerated B</text>"##,
    );
    for (key, color) in [("baseline_rows", "#8996a0"), ("retained_rows", "#2c7395")] {
        let mut points = String::from("95,280 ");
        for t in v["trajectory"].as_array().ok_or("trajectory")? {
            let x = 95. + 855. * t["b_count"].as_u64().ok_or("B")? as f64 / 185344.;
            let y = 280. - 260. * t[key].as_u64().ok_or("rows")? as f64 / 865.;
            points.push_str(&format!("{x:.2},{y:.2} "));
        }
        svg.push_str(&format!(
            r#"<polyline points="{points}" stroke="{color}" stroke-width="4" fill="none"/>"#
        ));
    }
    svg.push_str(r##"<text x="400" y="341" fill="#8996a0">Grounded discard:522</text><text x="670" y="341" fill="#2c7395">Deferred retention:865</text></g></svg>"##);
    fs::write(out.join("recorded-projections.svg"), svg)?;
    let typ = r##"#set page(paper:"a4",margin:(x:15mm,y:13mm))
#set text(font:"DejaVu Sans",size:9pt)
#set par(leading:0.5em)
= Retaining ungrounded edges produces more exact relations
*Deferred graph50 · recorded-input mathematics · 17 November 2026*

The new bounded Rust adjacency queue retains valid two-class edges until an endpoint gains an exact unit anchor. On the same108058validated Runpod49sources, it yields865full integer cancellation witnesses versus522for the existing grounded-discard graph. The recorded prefix reaches516rows at complete B batch155648, compared with185344for the baseline. This is a mathematical projection of retained sources; no new natural-yield, timing or DLP experiment ran. PCC-20261117-e9bd73 is UNVERIFIED. [pcc:unverified]

#image("recorded-projections.svg",width:100%)

== What the queue preserves
Every accepted source has zero, one or two distinct signed classes. An anchor is a source combination with large part exactly +D_v. A newly anchored vertex wakes its pending edges: one known endpoint propagates a signed anchor; two known endpoints produce an integer-zero large part. Unanchored cycles emit nothing. Coefficients are integers, including2when paths overlap; there is no modulo2or modular-inverse shortcut. [pcc:unverified]

Source ids remain original and consecutive. The queue processes pending edge ids in insertion order. Every deferred edge is activated at most once, and a bounded refusal consumes its original source id. Caps cover lifetime sources/vertices/deferred edges/witness support; malformed shapes leave state unchanged. [pcc:unverified]

== Checked scope and observed state
Thirteen combined Rust controls pass:32sign orientations, unanchored odd-cycle refusal, shared-path coefficient2, explicit vertex/edge/witness/source bounds, malformed inputs and64×257synthetic streams with independent integer incidence checks. Strict Clippy and optimized builds pass. [pcc:unverified]

Recorded replay hashes the original screened49raw, reproduces every baseline graph decision and checkpoint, and independently multiplies32408anchor/865full incidence witnesses. Every projected row annihilates the original normalized kernel. Dense u128row reduction reaches actual rank508at B154037/complete batch154624; this is not a new sparse-kernel timing or success claim. [pcc:unverified]

Final state:141145observed classes,32408anchors,90610deferred edges,15825activated and74785still live. Maximum stored/emitted witness support27/25; no vertex, edge or witness refusal. Default bounds262144sources/classes/deferred edges and256witness terms. Longer witnesses, memory and queue cost remain to be measured. [pcc:unverified]

== Prior art and next experiment
Read Gaudry–Thériault–Thomé corr2004-29,§3.1–3.3, primary URL https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf. Their full LP graph retains unrooted edges;§3.3discards them for simpler analysis. Their non-F2multiplicity warning motivates exact signed cancellation. This queue is a conservative retention variant, not a novelty or complexity claim. KB tools/wrapper unavailable; existing modular large_prime_filter.rs reviewed and not substituted for integer evidence. [pcc:unverified]

Next integrate a separate screened deferred collector and complete independent replay, preserving older schemas and consumed calls. New projections must record their activation source explicitly: a balanced cycle can cancel its latest anchor from the final witness, so witness.max_source alone cannot bind the emission checkpoint. Freeze a new source/protocol before Runpod measurement. [pcc:unverified]

Original49source bd69a76db876fa1da120de0edf1033489327d937; raw SHAe767e8a175a475645b3a899a32c2f0643ce20a750693aaa8797697934eb4bc56. Original complete mathematical replay is separately recorded at PCC3319ad, also UNVERIFIED. Original failures/results stay unchanged; all owned pods through49deleted. No complete-cost or scoreboard admission. [pcc:unverified]
"##;
    fs::write(out.join("source-report.typ"), typ)?;
    fs::write(out.join("figure.typ"),"#set page(width:750pt,height:262.5pt,margin:0pt)\n#image(\"recorded-projections.svg\",width:750pt,height:262.5pt)\n")?;
    Ok(())
}
