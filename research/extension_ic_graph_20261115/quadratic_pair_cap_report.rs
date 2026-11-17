//! Native report of original Runpod48 observations and separate component replay.
use serde_json::Value;
use std::{fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
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
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 1 {
        return Err("usage: REPORT EXISTING_PACKET".into());
    }
    let packet = Path::new(&a[0]);
    let read =
        |n: &str| -> Result<Value> { Ok(serde_json::from_slice(&fs::read(packet.join(n))?)?) };
    let s = read("cap-study.json")?;
    let q = read("cap-component-check.json")?;
    let deletion = read("deletion.json")?;
    if s["schema"] != "cover.quadratic-pair.cap-study/v1"
        || q["schema"] != "cover.quadratic-pair.cap-component-check/v1"
        || s["original_qualifier_exit_code"] != 1
        || q["original_frozen_qualifier_exit_code"] != 1
        || q["outcomes"][2]["proof"]["actual_independent_row_rank"] != 417
        || deletion["delete_http"] != 204
        || deletion["independent_get_http"] != 404
    {
        return Err("original cap/component/custody binding".into());
    }
    let out = packet.join("report");
    fs::create_dir_all(&out)?;
    let mut rows = String::new();
    for (r, name) in s["outcomes"].as_array().ok_or("three arms")?.iter().zip([
        "Original IC",
        "Quadratic table",
        "Grounded pairs",
    ]) {
        let n: u128 = r["observed_incomplete_bitops"]
            .as_str()
            .ok_or("subtotal")?
            .parse()?;
        let seconds = r["inner_wall_ns_practicality_only"]
            .as_u64()
            .ok_or("inner wall")? as f64
            / 1e9;
        let status = if r["verified"] == true {
            "scalar recovered"
        } else {
            "capped; no scalar"
        };
        rows.push_str(&format!(
            "[{name}],[{status}],[{}],[{seconds:.3}],\n",
            comma(n)
        ));
    }
    let h = &s["candidate"]["quartic_refusals"];
    let entries = [
        (
            "Not a degree-two product",
            "quartic is not a product of degree-two factors",
        ),
        ("Admitted two-factor split", "admitted two-factor split"),
        ("Fp-linear support", "quartic has Fp linear support"),
        ("Bounded attempt refusal", "quartic split attempt cap"),
        ("Repeated quartic", "repeated quartic factor"),
    ];
    let mut svg = String::from(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="300" viewBox="0 0 1000 300"><rect width="1000" height="300" fill="white"/><g font-family="DejaVu Sans" fill="#203747">"##,
    );
    for (i, (label, key)) in entries.iter().enumerate() {
        let n = h[key].as_u64().ok_or("refusal histogram")?;
        let y = 25 + i * 45;
        let width = n as f64 / 200000.0 * 590.0;
        let color = if i == 1 { "#347a9e" } else { "#7d8c96" };
        svg.push_str(&format!(r#"<text x="260" y="{}" font-size="18" text-anchor="end">{label}</text><rect x="275" y="{y}" width="{width:.3}" height="24" fill="{color}"/><text x="910" y="{}" font-size="18" text-anchor="end">{}</text>"#,y+19,y+19,comma(n as u128)));
    }
    svg.push_str(r#"<text x="275" y="280" font-size="17">0</text><text x="865" y="280" font-size="17" text-anchor="end">200,000 quartic requests</text></g></svg>"#);
    fs::write(out.join("quartic-refusals.svg"), svg)?;
    let typ=r##"#set page(paper: "a4", margin: (x: 15mm, y: 13mm))
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.55em)
= Two-quadratic graph IC: collection cap reached
*Runpod48 · 17 November 2026 · original failure retained*

The new Rust grounded collector reaches its 262144-quartic-history cap before sparse-rank admission. All three worker processes exit0, but the candidate returns no scalar and the frozen complete-pipeline qualifier exits1. The pod is deleted after artifact collection and checks. PCC-20261117-3cdb59/8c1742/a27dc9 remain UNVERIFIED producer evidence. [pcc:unverified]

#table(columns: (1fr,1.2fr,1.5fr,0.6fr), inset: 5pt, stroke: 0.4pt,
 [*Arm*],[*Outcome*],[*Observed work subtotal*],[*Inner s*],
 @ROWS@
)

All subtotals are incomplete native bitops. The candidate subtotal stops before descent; it cannot support a full-pipeline ratio. Elapsed observations are practicality notes: one fixed-order cold process per arm, same public point/host/binary/allowed CPU, zero warmups/retries/exclusions. [pcc:unverified]

== Where the quartic budget went
#image("quartic-refusals.svg", width: 100%)

At B index162203, the candidate has 94685 valid original sources:219 full,8373 one-class and86093 two-class. The graph grounds11670 classes, extends3449 two-class edges, discards82598 ungrounded sources and emits417 rows. It has no sparse checkpoint: the first is scheduled at516 rows. Rank417 from separate dense replay is below target508; the producer's certified lower bound remains0. [pcc:unverified]

== What the separate replay establishes
A new postexecution Rust component qualifier checks every principal/Cantor original, exact graph decision/witness and all417 projections. It reconstructs all309699 candidates,215013 refusals,262144 quartic calls, cohort hashes and the registered RNG stream (686060 sampled coefficients;696478 raw draws). Original IC and the table collector also pass complete mathematical replay. This component proof preserves the original frozen qualifier failure and supplies no candidate scalar or signed execution verdict. [pcc:unverified]

== Next test and execution record
Most quartics fail the degree-two factor condition, while only4 have repeated factors. The next lead is a cheaper discriminant screen, subject to separate proof/control/source/Runpod registration. This observed cap is a failure of the current bounded candidate, not evidence that graph elimination cannot work. [pcc:unverified]

Source f58a7eb96be9fe47c40f5ef5dcaabe74e2392854 (scientific paths equal253ddb); PR4537 and immutable source refs published before allocation. Runpod CPU5c,4CPU/8GiB,0.14 USD/hour; owned pod1juwnyxwvu3w98 created07:16:00Z; guardianPID131049 live before sole dispatch07:18:13Z. Original artifacts collected/hash-checked/private-scanned before DELETE204 and independentGET404 at07:21:23Z. Large witnesses/binaries/archive remain hash-bound LOCAL. [pcc:unverified]

Complete native accounting, fresh unknown-log/scaling targets, strongest matched rho and signed independent admission remain open. No primary rho-parity or scoreboard claim. Frozen46/47/core48 and consumed48 records remain preserved. Science, replay and report generation use Rust. [pcc:unverified]
"##.replace("@ROWS@",&rows);
    fs::write(out.join("report.typ"), typ)?;
    Ok(())
}
