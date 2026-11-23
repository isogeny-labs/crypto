//! Presentation of frozen native estimates; no additional statistics.
use serde_json::Value;
use std::fmt::Write;
pub fn tsv(s: &Value) -> String {
    let mut out=String::from("group\treference\tphase\tn\tmedian\tmin\tmax\tdescriptive95_low\tdescriptive95_high\tstanding\n");
    for g in s["groups"].as_array().unwrap() {
        for reference in ["school", "newton", "lookup", "half", "half-double"] {
            let den = if reference == "half" {
                "half-aa"
            } else {
                "half"
            };
            for phase in ["query_ns", "cold_ns", "query_muls", "cold_muls"] {
                let key = format!("{reference}/{den}:{phase}");
                let v = &g["paired_ratios"][&key];
                writeln!(
                    out,
                    "{}\t{reference}/{den}\t{phase}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    g["group"].as_str().unwrap(),
                    v["n"],
                    v["median"],
                    v["min"],
                    v["max"],
                    v["bootstrap_median95"][0],
                    v["bootstrap_median95"][1],
                    if phase.ends_with("ns") {
                        "local_diagnostic_belowL2"
                    } else {
                        "counted_producer_unverified"
                    }
                )
                .unwrap();
            }
        }
    }
    out
}
pub fn markdown(s: &Value, controls: &Value) -> String {
    let mut out = String::from("# Frozen Vieta speed panel: recorded paired costs\n\n");
    writeln!(out,"Primary plan504 calls; attempted {}, successful {}, failed {}, warmup calls {}. Calibration72 is separate. Exact per-target values, ranges and descriptive4096-bootstrap median intervals remain in `run/summary.json`; no populations or configurations are pooled. All observations are producer-owned [pcc:unverified].\n",s["attempted"],s["successful"],s["failed"],s["warmup_calls"]).unwrap();
    out.push_str("Ratios are reference/half-GCD, so above1 favors half-GCD and below1 favors the reference. S=schoolbook; N=Euclid/Newton; L=lookup. Elapsed ratios are local belowL2 diagnostics, never admitted speed claims. These tables reproduce precomputed native medians, without refitting. Full construction is charged to every arm; query cost remains separate.\n\n");
    out.push_str("| Exact group | n | S/H query time | N/H query time | L/H query time | S/H cold time | N/H cold time | L/H cold time |\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for g in s["groups"].as_array().unwrap() {
        write!(
            out,
            "| {} | {} ",
            g["group"].as_str().unwrap(),
            g["arms"]["half"]["successful"]
        )
        .unwrap();
        for phase in ["query_ns", "cold_ns"] {
            for arm in ["school", "newton", "lookup"] {
                let v = &g["paired_ratios"][format!("{arm}/half:{phase}")];
                if let Some(m) = v["median"].as_f64() {
                    write!(out, "| {m:.4} ").unwrap();
                } else {
                    out.push_str("| missing ");
                }
            }
        }
        out.push_str("|\n");
    }
    out.push_str("\n| Exact group | S/H query muls | N/H query muls | L/H query muls | S/H cold muls | N/H cold muls | L/H cold muls |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for g in s["groups"].as_array().unwrap() {
        write!(out, "| {} ", g["group"].as_str().unwrap()).unwrap();
        for phase in ["query_muls", "cold_muls"] {
            for arm in ["school", "newton", "lookup"] {
                let v = &g["paired_ratios"][format!("{arm}/half:{phase}")];
                if let Some(m) = v["median"].as_f64() {
                    write!(out, "| {m:.4} ").unwrap();
                } else {
                    out.push_str("| missing ");
                }
            }
        }
        out.push_str("|\n");
    }
    out.push_str("\n| Exact group | Exact count controls | Local timing separation | L2 admission |\n|---|---|---|---|\n");
    for g in controls["groups"].as_array().unwrap() {
        writeln!(
            out,
            "| {} | {} | {} | false |",
            g["group"].as_str().unwrap(),
            g["count_controls_pass"],
            g["positive_time_above_frozen_1_25_and_observed_aa_max"]
        )
        .unwrap();
    }
    out.push_str("\nThe Lean gate checks21 theorems/0trusted exceptions, including the conditional normal-form implication and finite projector/full-splitting/nilpotent certificates. The arbitrary-depth tower NF instance and independent signed standing remain unfinished; the full degree bound remains unverified. Shared crypto-lean PR26 CI did not start because GitHub reports an account billing lock. See `ASSUMPTION_AUDIT.md`, `PROOF_MAP.md`, `REQUIREMENTS.md`, `COST_MODEL.md`, raw runs, protocol/publication receipts and the report sources.\n");
    out
}
