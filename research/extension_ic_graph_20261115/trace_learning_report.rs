//! Native bound arithmetic/report; does not execute an algebraic solver.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fmt::Write as _, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn n(v: &Value, key: &str) -> Result<u128> {
    Ok(v[key].as_str().ok_or("decimal bound")?.parse()?)
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 3 {
        return Err("usage: LEARNING_REPORT AUDIT COHORT NEW_DIR".into());
    }
    let bytes = fs::read(&a[0])?;
    let audit: Value = serde_json::from_slice(&bytes)?;
    let prior_bytes = fs::read(&a[1])?;
    let prior: Value = serde_json::from_slice(&prior_bytes)?;
    if audit["schema"] != "cover.unit-trace.learning-contract-audit/v1"
        || prior["schema"] != "cover.unit-trace.saved-cohort-counters/v1"
        || audit["original_input_sha256"] != prior["arms"][0]["input_sha256"]
        || audit["passed"] != true
        || audit["total_f4_calls"] != 8
        || audit["residual_index"] != 0
        || audit["p"] != 1009
        || prior["systems"] != 3415
        || prior["roots"] != 3419
        || prior["every_coordinate_multiset_equal"] != true
    {
        return Err("audit/cohort binding".into());
    }
    let mut rows = Vec::new();
    let mut table = String::new();
    let mut details = String::new();
    let mut mdtable = String::new();
    let mut svg = String::from(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="940" height="235" viewBox="0 0 940 235"><rect width="940" height="235" fill="white"/><g font-family="DejaVu Sans" fill="#17334a"><text x="18" y="25" font-size="17">Reported counter sum + learning-multiplication bound · full saved cohort</text>"##,
    );
    let axis = |x: f64| 190.0 + (x - 1_740_000_000.0) / 90_000_000.0 * 650.0;
    for tick in [
        1_740_000_000u64,
        1_760_000_000,
        1_780_000_000,
        1_800_000_000,
        1_820_000_000,
    ] {
        let x = axis(tick as f64);
        write!(
            svg,
            r##"<line x1="{x:.2}" x2="{x:.2}" y1="48" y2="148" stroke="#e2e8ed"/><text x="{x:.2}" y="173" font-size="13" text-anchor="middle">{:.2}B</text>"##,
            tick as f64 / 1e9
        )?;
    }
    for (i, (arity, id, label)) in [
        (5, "unit-traced", "Five with trace"),
        (6, "six-traced", "Six with trace"),
    ]
    .iter()
    .enumerate()
    {
        let r = audit["arms"]
            .as_array()
            .ok_or("audit arms")?
            .iter()
            .find(|r| r["arity"] == *arity)
            .ok_or("arity")?;
        let b = prior["arms"]
            .as_array()
            .ok_or("cohort arms")?
            .iter()
            .find(|r| r["arm"] == *id)
            .ok_or("prior arm")?;
        if b["recording"]["systems"] != 1
            || b["recording"]["f4_muls"]
                .as_str()
                .ok_or("record muls")?
                .parse::<u64>()?
                != r["record"]["field_ops"].as_u64().ok_or("record counter")?
        {
            return Err("same single learning run".into());
        }
        let base = n(&b["total"], "reported_sum")?;
        let bound = &r["per_step_learning_upper"];
        let learning = n(bound, "multiplications_including_fermat_upper")?;
        let upper = base.checked_add(learning).ok_or("upper overflow")?;
        if !(1_740_000_000..=1_830_000_000).contains(&base) || upper > 1_830_000_000 {
            return Err("figure domain".into());
        }
        writeln!(table, "[{label}],[{base}],[{learning}],[{upper}],")?;
        writeln!(mdtable, "| {label} | {base} | {learning} | {upper} |")?;
        writeln!(
            details,
            "[{label}],[{}],[{}],[{}],",
            n(bound, "field_additions_upper")?,
            n(bound, "inverse_calls_upper")?,
            n(bound, "contributor_word_ors_upper")?
        )?;
        let y = 73 + i * 54;
        let lo = axis(base as f64);
        let hi = axis(upper as f64);
        write!(
            svg,
            r##"<text x="175" y="{}" font-size="15" text-anchor="end">{label}</text><line x1="{lo:.2}" x2="{hi:.2}" y1="{y}" y2="{y}" stroke="#367da5" stroke-width="8"/><line x1="{lo:.2}" x2="{lo:.2}" y1="{}" y2="{}" stroke="#17334a"/><line x1="{hi:.2}" x2="{hi:.2}" y1="{}" y2="{}" stroke="#17334a"/>"##,
            y + 5,
            y - 9,
            y + 9,
            y - 9,
            y + 9
        )?;
        rows.push(json!({"arity":arity,"reported_counter_sum":base.to_string(),"learning_multiplication_upper":learning.to_string(),"included_multiplication_upper":upper.to_string(),"other_learning_bounds":bound}));
    }
    svg.push_str(r##"<text x="18" y="204" font-size="13">Intervals are deterministic arithmetic bounds, not statistical confidence intervals.</text><text x="18" y="225" font-size="13">Other field/native/pipeline work remains outside this submetric · no timing or parity inference</text></g></svg>"##);
    let ordered =
        n(&rows[1], "included_multiplication_upper")? < n(&rows[0], "reported_counter_sum")?;
    let derived = json!({"schema":"cover.unit-trace.learning-bound-comparison/v1","audit_sha256":format!("{:x}",Sha256::digest(bytes)),"cohort_sha256":format!("{:x}",Sha256::digest(prior_bytes)),"arms":rows,
        "six_upper_below_five_reported_lower":ordered,"single_recorded_learning_run_per_backend":true,"complete_cost":false,"ratios":null,"rho_parity":false,"independent_signed_verdict":false,
        "interpretation":"conditional bound for the named multiplication counters plus omitted tracked-RREF multiplications only; no full-stage or whole-pipeline ranking"});
    let md=format!("# Trace-learning bound and replay-contract audit\n\nEight fixed F4calls and six linear-stage controls on the first recorded system pass. Both arities reproduce the captured root under learning, self replay and ordinary reference. Three arithmetic-bound controls and strict scoped Clippy pass. This is recorded-input development mathematics, not a throughput trial. Unsigned PCC-20261123-ac5831. [pcc:unverified]\n\n![Included multiplication bounds](learning-bounds.svg)\n\n| Path | Recorded F4+linear sum | Missing learning multiplication upper | Included multiplication upper |\n|---|---:|---:|---:|\n{mdtable}\nThe six-coordinate upper bound is below the five-coordinate recorded lower bound for this particular arithmetic subtotal. Each full saved stream has exactly one learning run, whose counter/matrix maxima match the new fixed control. Other operations and phases remain outside this comparison; no full-cost or runtime winner follows. [pcc:unverified]\n\nFor each learned matrix, let R be the pair-row upper count, C the full matrix column count, and k=min(R,C). Recorded rows/free columns form a subset. Bounds include R*C*k row multiplications, (R-1)*(C+1)*k additions including p-f, k inverse calls, 19*k Fermat multiplications at p1009, and (R-1)*k*ceil(R/64) contributor word ORs. Zero-sized matrices contribute zero. Sum each unit separately with checked u128. Inverse-call and Fermat-multiplication columns are alternative accounting, never charged twice. Allocation/copies/scans/bookkeeping remain missing.\n\nAll saved66five/33six mismatch cases meet the existing inconsistency-or-staircase<=64 predicate. Deliberately corrupted stored row indices return mismatch without that condition and are rejected by the audit predicate; they are not passed to the linear stage. This is synthetic API-contract evidence, not a naturally observed production failure. The stopping predicate is the existing rule, not a general Buchberger certificate or completeness theorem. [pcc:unverified]\n\nNo production source change is justified solely by this injected corruption. Dedicated worker traces remain internally generated and arity-separated, and all captured root sets match. Broader exceptional-input guarantees remain separate. New Runpod trials still require restored funds/checkpoint access and a new isolated cold protocol.\n\nSources: [original audit](../audit-original.json), [derived comparison](derived-comparison.json), [fixed plan](../PLAN.md), [native audit](../../trace_learning_audit.rs), [prior saved counters](../../trace-cohort-20261123/proof-bound-counter-summary.json). Source/immutable binary were published and hash-checked before the sole control. No elapsed values exported; library budget clocks remain internal.\n");
    let typ = format!(
        r###"#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9pt)
#set par(leading:0.5em)
= Trace-learning bounds and replay acceptance
Rust recorded-input mathematics · 23 November 2026

Eight fixed F4 calls and six linear-stage checks pass on the first saved system. Learning, self replay and ordinary reference produce the same captured root for both arities. Producer evidence: PCC-20261123-ac5831, *UNVERIFIED*. [pcc:unverified]

#image("learning-bounds.svg",width:100%)
#table(columns:(1.5fr,1.25fr,1.1fr,1.3fr),align:(left,right,right,right),[Path],[Recorded sum],[Learning mul upper],[Included mul upper],{table})
The six-coordinate upper bound remains below the five-coordinate recorded lower bound for this *limited multiplication subtotal*. Every saved stream has one learning run; the new fixed control matches its emitted F4 counter and matrix maxima. This is not a complete F4/descent/pipeline cost or runtime ranking. [pcc:unverified]

== Where the missing bound comes from
For each step, recorded rows and free columns are subsets of the pair rows R and full columns C; rank k is at most min(R,C). The tracked dense reduction performs at most `R*C*k` row multiplications and `(R-1)*(C+1)*k` additions, including p-f. Add k pivot inversions, each using 19 Fermat multiplications at p1009. Contributor updates use at most `(R-1)*k*ceil(R/64)` word ORs. Empty matrices contribute zero; every sum/product is checked in u128.

#table(columns:(1.5fr,1.3fr,1fr,1.3fr),align:(left,right,right,right),[Path],[Additions upper],[Inverse calls upper],[Word ORs upper],{details})
Inverse calls and their expanded Fermat multiplications are alternative units, never added twice. Bounds are upper limits, not actual event counts. Allocation, row copies, pivot/contributor scans and other bookkeeping remain outside them.

== Acceptance control
All saved 66 five-coordinate and 33 six-coordinate mismatch cases meet the existing inconsistency-or-staircase-at-most64 condition. Injected invalid stored row indices return mismatch without that condition; the audit rejects them before linear extraction. That synthetic corruption is not a naturally observed worker failure. The stopping rule is not a general Buchberger certificate or completeness theorem. [pcc:unverified]

The fixed captured controls agree; no production change is made solely from injected corruption. Dedicated workers retain internally learned, arity-separated traces. Broader exceptional-input guarantees stay open.

== Scope and next step
This is a once-only recorded control from a published immutable native binary, not a timed benchmark. No new target or natural relation search was generated. No elapsed data is exported; internal budget clocks remain. Runpod funds/checkpoint access and complete isolated cold accounting are still required for the requested comparison and parity conclusion.

Sources: audit-original.json, PLAN.md, derived-comparison.json; trace_learning_audit.rs and trace_learning_report.rs; retained Root41 source and full saved-cohort counters. #link("https://github.com/isogeny-labs/crypto/pull/4986")[PR 4986].
"###
    );
    let out = Path::new(&a[2]);
    fs::create_dir(out)?;
    for (file, text) in [
        ("REPORT.md", md),
        ("report.typ", typ),
        ("learning-bounds.svg", svg),
    ] {
        fs::write(out.join(file), text)?;
    }
    fs::write(
        out.join("derived-comparison.json"),
        serde_json::to_vec_pretty(&derived)?,
    )?;
    Ok(())
}
