//! Read-only audit/report for the retained A/A rejection, never a timing retry.
use compact_speed_analysis::paired_statistics;
use plotters::prelude::*;
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn ns(v: &Value) -> Result<u64> {
    v.as_u64()
        .filter(|x| *x > 0)
        .ok_or_else(|| "positive cost required".into())
}
fn fixture_label(s: &Value) -> Result<&'static str> {
    match s["input_sha256"].as_str() {
        Some("63f5d4bcaaf41f57ed81aba878bab41900c552c8ebbfc3956b67b0d016e4350d") => {
            Ok("first-star")
        }
        Some("e4d22ea93228f207b87640813fb925e15973d8ee43d5f966a4d98a63917cbaf3") => Ok("last-star"),
        _ => Err("paired input outside known frozen fixtures".into()),
    }
}
#[test]
fn fixture_label_rejects_unknown_and_binds_each_frozen_hash() {
    assert_eq!(fixture_label(&json!({"input_sha256":"63f5d4bcaaf41f57ed81aba878bab41900c552c8ebbfc3956b67b0d016e4350d"})).unwrap(),"first-star");
    assert_eq!(fixture_label(&json!({"input_sha256":"e4d22ea93228f207b87640813fb925e15973d8ee43d5f966a4d98a63917cbaf3"})).unwrap(),"last-star");
    assert!(fixture_label(&json!({})).is_err());
    assert!(fixture_label(&json!({"input_sha256":"a".repeat(64)})).is_err());
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 6 {
        return Err(
            "usage: kernel-rejection-report RAW MONITOR NEW_REPORT RESULT_PCC EXPECTED_KERNEL"
                .into(),
        );
    }
    let raw = Path::new(&args[1]);
    let out = Path::new(&args[3]);
    let conditions = fs::read_to_string(&args[2])?;
    if conditions.lines().count() != 1 {
        return Err("single monitor record".into());
    }
    let monitor: Value = serde_json::from_str(&conditions)?;
    if monitor["schema"] != "isolated-bench/1"
        || monitor["contended_samples"] != 0
        || monitor["exit_status"] != 0
    {
        return Err("isolation did not admit".into());
    }
    if ![
        "legacy",
        "ordered",
        "cached",
        "both",
        "both_updates",
        "both_updates_sum",
        "both_updates_lead",
        "both_updates_fused",
        "both_updates_frontier",
    ]
    .contains(&args[5].as_str())
    {
        return Err("known kernel".into());
    }
    let s: Value = serde_json::from_slice(&fs::read(raw.join("summary.json"))?)?;
    let fixture_name = fixture_label(&s)?;
    if s["failure"] != "A/A rejected; main blocks not run"
        || !s["result"].is_null()
        || s["identities_unchanged"] != true
        || s["qualification"] != false
        || !s["blocks"].as_array().ok_or("blocks")?.is_empty()
    {
        return Err("not the retained rejection transcript".into());
    }
    let pairs = s["aa_pairs"]
        .as_array()
        .filter(|p| p.len() == 7)
        .ok_or("seven pairs")?;
    let names = [
        "f4-regeneration",
        "expanded-regeneration",
        "root-sparse-both",
    ];
    let mut count = 0;
    for panel in std::iter::once(&s["warmup"]).chain(
        pairs
            .iter()
            .flat_map(|p| p.as_array().into_iter().flatten()),
    ) {
        if panel["all_complete_bases_equal"] != true {
            return Err("basis mismatch".into());
        }
        let answer = &panel["samples"][2]["answer"];
        if answer["kernel_mode"] != args[5]
            || answer["completion_exports"] != 0
            || answer["root_api"] != true
            || answer["native_product_criterion"] != true
            || answer["constructor"]["subset_succeeded"] != true
        {
            return Err("selected candidate mode/acceptance".into());
        }
        let basis = panel["samples"][0]["basis_sha256"]
            .as_str()
            .filter(|b| b.len() == 64)
            .ok_or("basis hash")?;
        let rows = panel["samples"]
            .as_array()
            .filter(|r| r.len() == 3)
            .ok_or("roster")?;
        for (i, row) in rows.iter().enumerate() {
            let sum = ns(&row["pipeline_work_ns"])?
                .checked_add(ns(&panel[if i == 2 {
                    "preparation_ns"
                } else {
                    "regeneration_setup_ns"
                }])?)
                .and_then(|x| x.checked_add(row["cross_arm_equality_ns"].as_u64()?))
                .ok_or("cost overflow")?;
            if row["pipeline"] != names[i]
                || row["status"] != "complete"
                || row["basis_sha256"] != basis
                || row["paid_wall_ns"] != sum
            {
                return Err("invalid answer/cost sum".into());
            }
            count += 1;
        }
    }
    if count != 45 {
        return Err("expected45 complete answers".into());
    }
    let mut md = String::from("# Paired development test: A/A rejection\n\nThe frozen noise gate rejects this run. No main blocks ran and no10x/15x comparison is available. This is a retained rejection, with no replacement or same-source retry. RESULT_PCC_PLACEHOLDER, local L0 unsigned evidence. [pcc:unverified]\n\nExactly one warmup panel and seven labelled A/A pairs contain45 complete matching workflow answers. Executable/runtime identities remain unchanged. This Rust report checks each paid sum and recomputes all six A/A intervals from the raw labelled pairs with the frozen calculator. Recorded JSON statistics are compared within four machine epsilons (decimal parse rounding); gate decisions are compared exactly. [pcc:unverified]\n\n| Arm | Scope | Median A/B | Ordinary95% interval | Includes1 |\n|---|---|---:|---|---|\n");
    md = md.replace("RESULT_PCC_PLACEHOLDER", &args[4]);
    let mut points = Vec::new();
    let mut checks = Vec::new();
    for (arm, name) in names.iter().enumerate() {
        let ratios = pairs
            .iter()
            .map(|p| -> Result<[f64; 2]> {
                let a = &p[0]["samples"][arm];
                let b = &p[1]["samples"][arm];
                Ok([
                    ns(&a["pipeline_work_ns"])? as f64 / ns(&b["pipeline_work_ns"])? as f64,
                    ns(&a["paid_wall_ns"])? as f64 / ns(&b["paid_wall_ns"])? as f64,
                ])
            })
            .collect::<Result<Vec<_>>>()?;
        let stats = paired_statistics(&ratios)?;
        let mut admitted = true;
        for (scope, stat) in ["pipeline_work", "full_paid"].iter().zip(stats) {
            let recomputed = serde_json::to_value(&stat)?;
            let recorded = &s["aa"][arm][scope];
            if recomputed["samples"] != recorded["samples"] {
                return Err("sample count mismatch".into());
            }
            for (a, b) in [
                (
                    &recomputed["median_paired_ratio"],
                    &recorded["median_paired_ratio"],
                ),
                (
                    &recomputed["adjusted_lower_1_over_720"],
                    &recorded["adjusted_lower_1_over_720"],
                ),
                (
                    &recomputed["percentile_95"][0],
                    &recorded["percentile_95"][0],
                ),
                (
                    &recomputed["percentile_95"][1],
                    &recorded["percentile_95"][1],
                ),
            ] {
                let a = a.as_f64().ok_or("computed statistic")?;
                let b = b.as_f64().ok_or("recorded statistic")?;
                // serde_json without float_roundtrip can parse one ULP differently.
                if (a - b).abs() > 4. * f64::EPSILON * a.abs().max(1.) {
                    return Err(format!("calculator disagreement {name}/{scope}").into());
                }
            }
            let [lo, hi] = stat.percentile_95;
            let includes = lo <= 1. && hi >= 1.;
            admitted &= includes;
            md += &format!(
                "| {name} | {scope} | {:.6} | [{lo:.6}, {hi:.6}] | {includes} |\n",
                stat.median_paired_ratio
            );
            points.push((
                format!("{name} / {scope}"),
                lo,
                stat.median_paired_ratio,
                hi,
            ));
        }
        if s["aa"][arm]["admitted"] != admitted {
            return Err("gate disagreement".into());
        }
        checks.push(json!({"arm":name,"admitted":admitted}));
    }
    md += "\nThe table retains every ordinary A/A interval and exact gate decision. At least one scope rejects this run; the observations do not establish a cause for the labelled difference. The selected kernel mode and zero exports are checked in every candidate answer, alongside native/root acceptance flags. No main speed ratio is computed. [pcc:unverified]\n\nThe computation is the previously selected p2147483647,n16,first-star,seed7 known fixture. Every repeated workflow pays its original measured preparation charge; each panel obtains fresh three-route references charged in full to regeneration arms. Candidate pays construction, serialized proof, separate root API checking and output consumption. These are distinct validation algorithms for the same complete reduced basis. This is not a fresh-target, fastest-control or kernel comparison. [pcc:unverified]\n\nNo fresh seed was generated. The earlier single-call screen remains separate. Certificate/output degrees and quotient dimensions do not establish a reduction in intrinsic regularity. The15x objective remains open. [pcc:unverified]\n\n![Labelled A/A ratios](aa.svg){width=95%}\n\nPoints are medians, bars are ordinary95% percentile intervals. Vertical line marks1. No interval or sample was discarded.\n";
    md = md.replace("first-star", fixture_name).replace(
        "sixteen-workflow screen results",
        "retained full-control screen results",
    );
    fs::create_dir(out)?;
    fs::write(out.join("report.md"), md)?;
    fs::write(
        out.join("audit.json"),
        serde_json::to_vec_pretty(
            &json!({"complete_matching_answers":count,"main_blocks":0,"gate":checks,"all_paid_sums_checked":true,"expected_kernel_mode":args[5],"completion_exports":0,"zero_monitor_contention":true,"all_aa_statistics_recomputed":true,"input_sha256":s["input_sha256"],"fixture":fixture_name,"standing":"unsigned producer audit"}),
        )?,
    )?;
    let path = out.join("aa.svg");
    let root = SVGBackend::new(&path, (1250, 640)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "A/A rejection: at least one interval excludes1",
            ("sans-serif", 24),
        )
        .margin(20)
        .x_label_area_size(55)
        .y_label_area_size(390)
        .build_cartesian_2d(
            (points.iter().map(|p| p.1).fold(1f64, f64::min) * 0.98)
                ..(points.iter().map(|p| p.3).fold(1f64, f64::max) * 1.02),
            0usize..7usize,
        )?;
    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_desc("Labelled A / B (ordinary95% interval)")
        .y_labels(8)
        .y_label_formatter(&|y| {
            points
                .get(y.saturating_sub(1))
                .filter(|_| *y > 0)
                .map(|p| p.0.clone())
                .unwrap_or_default()
        })
        .draw()?;
    chart.draw_series(std::iter::once(PathElement::new(
        vec![(1., 0), (1., 7)],
        BLACK.mix(0.4),
    )))?;
    for (i, (_, lo, mid, hi)) in points.iter().enumerate() {
        let color = if i % 2 == 0 { BLUE } else { RED };
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(*lo, i + 1), (*hi, i + 1)],
            color.stroke_width(2),
        )))?;
        chart.draw_series(std::iter::once(Circle::new(
            (*mid, i + 1),
            5,
            color.filled(),
        )))?;
    }
    root.present()?;
    Ok(())
}
