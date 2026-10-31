//! Render descriptive, per-cell costs from a verified native tournament round.
//!
//! The runner's `tournament verify` command checks the cryptographic receipts
//! and re-derives its decision. Run that first; this tool only groups its
//! immutable receipt numbers for a report and site table.

use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Default)]
struct Totals {
    count: u64,
    ir: u128,
    s: f64,
    floor_ratio: f64,
    floor_count: u64,
    phases: BTreeMap<String, u128>,
    base_sizes: BTreeSet<u64>,
    ranks: BTreeSet<u64>,
}

fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn number(v: &Value, key: &str) -> Result<u64> {
    v.get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing integer {key}").into())
}

fn string<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    v.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {key}").into())
}

fn receipts(root: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            receipts(&path, out)?;
        } else if path.file_name().is_some_and(|n| n == "receipt.json") {
            out.push(path);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let usage = "usage: ic_tournament_round_report ROUND REGISTRY OUTPUT";
    let round = PathBuf::from(args.next().ok_or(usage)?);
    let registry_path = PathBuf::from(args.next().ok_or(usage)?);
    let output = PathBuf::from(args.next().ok_or(usage)?);
    if args.next().is_some() {
        return Err(usage.into());
    }
    let decision = read_json(&round.join("decision.json"))?;
    let contract = read_json(&round.join("contract.json"))?;
    let audit = read_json(&round.join("audit.json"))?;
    let fixtures = read_json(&round.join("fixtures.json"))?;
    let registry = read_json(&registry_path)?;
    let mut curve_slugs = BTreeMap::new();
    for stage_cases in fixtures
        .as_object()
        .ok_or("fixtures is not an object")?
        .values()
    {
        for case in stage_cases
            .as_array()
            .ok_or("stage fixtures is not an array")?
        {
            let cell = string(case, "cell")?;
            let fixture = case.get("fixture").ok_or("case has no fixture")?;
            let degree = number(fixture, "degree")?;
            let a = number(fixture, "curve_a")?;
            let order = string(fixture, "subgroup_order")?;
            let matches: Vec<&Value> = registry["curves"]
                .as_array()
                .ok_or("registry has no curves")?
                .iter()
                .filter(|curve| {
                    curve["family"] == "koblitz"
                        && curve["params"]["n"].as_u64() == Some(degree)
                        && curve["params"]["a"].as_u64() == Some(a)
                        && curve["representations"].as_array().is_some_and(|rs| {
                            rs.iter().any(|r| r["curve"]["subgroup_order"] == order)
                        })
                })
                .collect();
            if matches.len() != 1 {
                return Err(format!(
                    "{cell}: expected one exact ICV1 registry match, found {}",
                    matches.len()
                )
                .into());
            }
            let slug = string(matches[0], "slug")?.to_owned();
            if curve_slugs
                .insert(cell.to_owned(), slug.clone())
                .is_some_and(|old| old != slug)
            {
                return Err(format!("{cell}: conflicting ICV1 registry identities").into());
            }
        }
    }
    if string(&audit, "status")? != "VERIFIED" {
        return Err("native audit is not VERIFIED".into());
    }
    let mut files = Vec::new();
    receipts(&round.join("runs"), &mut files)?;
    files.sort();
    if files.len() as u64 != number(&audit, "trial_receipts")? {
        return Err("receipt count differs from native audit".into());
    }
    let mut totals: BTreeMap<(String, String, String), Totals> = BTreeMap::new();
    let mut stage_counts: BTreeMap<String, u64> = BTreeMap::new();
    for path in files {
        let r = read_json(&path)?;
        if string(&r, "status")? != "VERIFIED" {
            return Err(format!("unverified receipt: {}", path.display()).into());
        }
        let stage = string(&r, "stage")?.to_owned();
        let cell = string(&r, "cell")?.to_owned();
        let arm = string(&r, "arm")?.to_owned();
        let ir = number(&r, "total_operations")? as u128;
        let phases = r["phase_costs"].as_object().ok_or("missing phase costs")?;
        let mut phase_sum = 0u128;
        let entry = totals.entry((stage.clone(), cell, arm)).or_default();
        for (name, value) in phases {
            let n = value.as_u64().ok_or("noninteger phase cost")? as u128;
            phase_sum += n;
            *entry.phases.entry(name.clone()).or_default() += n;
        }
        if phase_sum != ir {
            return Err(format!("exclusive phases do not sum in {}", path.display()).into());
        }
        entry.count += 1;
        entry.ir += ir;
        entry.s += r["normalized_S"].as_f64().ok_or("missing S")?;
        if let Some(x) = r["ratio_to_floor"].as_f64() {
            entry.floor_ratio += x;
            entry.floor_count += 1;
        }
        if let Some(x) = r["certificate"]["signed_base_size"].as_u64() {
            entry.base_sizes.insert(x);
        }
        if let Some(x) = r["certificate"]["rank"].as_u64() {
            entry.ranks.insert(x);
        }
        *stage_counts.entry(stage).or_default() += 1;
    }
    for (stage, count) in &stage_counts {
        let summary = read_json(&round.join("summaries").join(format!("{stage}.json")))?;
        if number(&summary, "verified_runs")? != *count || number(&summary, "runs")? != *count {
            return Err(format!("{stage} summary and receipt counts differ").into());
        }
    }
    let mut rows = Vec::new();
    for ((stage, cell, arm), a) in &totals {
        let mean_ir = a.ir as f64 / a.count as f64;
        let ratio_to = |other: &str| {
            totals
                .get(&(stage.clone(), cell.clone(), other.to_owned()))
                .map(|reference| mean_ir / (reference.ir as f64 / reference.count as f64))
        };
        let mut phase_means = Map::new();
        let mut phase_shares = Map::new();
        for (phase, sum) in &a.phases {
            phase_means.insert(phase.clone(), json!(*sum as f64 / a.count as f64));
            phase_shares.insert(phase.clone(), json!(*sum as f64 / a.ir as f64));
        }
        rows.push(json!({
            "stage": stage,
            "cell": cell,
            "curve_slug": curve_slugs.get(cell).ok_or("receipt cell is absent from fixtures")?,
            "arm": arm,
            "verified": a.count,
            "mean_ir": mean_ir,
            "log2_mean_ir": mean_ir.log2(),
            "mean_s_ir_per_sqrt_r": a.s / a.count as f64,
            "ratio_to_incumbent_mean_ir": ratio_to("incumbent"),
            "ratio_to_rho_mean_ir": ratio_to("rho"),
            "mean_ratio_to_rank_floor": if a.floor_count == a.count { Some(a.floor_ratio / a.count as f64) } else { None },
            "signed_base_sizes": a.base_sizes,
            "ranks": a.ranks,
            "phase_mean_ir": phase_means,
            "phase_share": phase_shares,
        }));
    }
    let report = json!({
        "schema": "ic-native-tournament-measurements/1",
        "round": round.file_name().ok_or("round has no name")?.to_string_lossy(),
        "unit": string(&contract, "unit")?,
        "target_count_per_cold_job": number(&contract, "target_count")?,
        "audit": audit,
        "decision": decision,
        "stage_counts": stage_counts,
        "rows": rows,
    });
    fs::write(
        output,
        format!("{}\n", serde_json::to_string_pretty(&report)?),
    )?;
    Ok(())
}
