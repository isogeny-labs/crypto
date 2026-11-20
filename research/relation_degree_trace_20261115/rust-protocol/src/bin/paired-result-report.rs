//! Read-only recomputation of a complete, admitted paired transcript.
use compact_speed_analysis::{paired_statistics, Statistics};
use plotters::prelude::*;
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
const NAMES: [&str; 3] = [
    "f4-regeneration",
    "expanded-regeneration",
    "root-sparse-both",
];
const CYCLE: [[usize; 3]; 6] = [
    [0, 1, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
    [1, 0, 2],
    [0, 2, 1],
];
fn cost(v: &Value) -> Result<u64> {
    v.as_u64()
        .filter(|x| *x > 0)
        .ok_or_else(|| "positive cost".into())
}
fn read_panel(p: &Value) -> Result<[[u64; 2]; 3]> {
    if p["all_complete_bases_equal"] != true {
        return Err("panel equality".into());
    }
    let rows = p["samples"]
        .as_array()
        .filter(|r| r.len() == 3)
        .ok_or("three arms")?;
    let hash = rows[0]["basis_sha256"]
        .as_str()
        .filter(|h| h.len() == 64)
        .ok_or("digest")?;
    let mut out = [[0; 2]; 3];
    for (i, r) in rows.iter().enumerate() {
        out[i] = [cost(&r["pipeline_work_ns"])?, cost(&r["paid_wall_ns"])?];
        let fee = cost(
            &p[if i == 2 {
                "preparation_ns"
            } else {
                "regeneration_setup_ns"
            }],
        )?;
        let paid = out[i][0]
            .checked_add(fee)
            .and_then(|x| x.checked_add(r["cross_arm_equality_ns"].as_u64()?))
            .ok_or("paid overflow")?;
        if r["pipeline"] != NAMES[i]
            || r["status"] != "complete"
            || r["basis_sha256"] != hash
            || out[i][1] != paid
        {
            return Err("answer/paid sum".into());
        }
    }
    let c = &rows[2]["answer"];
    if c["root_api"] != true
        || c["native_product_criterion"] != true
        || c["constructor"]["subset_succeeded"] != true
    {
        return Err("candidate fast path".into());
    }
    Ok(out)
}
fn agrees(stat: &Statistics, recorded: &Value) -> Result<()> {
    let computed = serde_json::to_value(stat)?;
    if computed["samples"] != recorded["samples"] {
        return Err("statistic sample count".into());
    }
    for (a, b) in [
        (
            &computed["median_paired_ratio"],
            &recorded["median_paired_ratio"],
        ),
        (&computed["percentile_95"][0], &recorded["percentile_95"][0]),
        (&computed["percentile_95"][1], &recorded["percentile_95"][1]),
        (
            &computed["adjusted_lower_1_over_720"],
            &recorded["adjusted_lower_1_over_720"],
        ),
    ] {
        let a = a.as_f64().ok_or("computed number")?;
        let b = b.as_f64().ok_or("recorded number")?;
        if (a - b).abs() > 4. * f64::EPSILON * a.abs().max(1.) {
            return Err("recomputed statistic disagrees".into());
        }
    }
    Ok(())
}
fn includes_one(s: &Statistics) -> bool {
    s.percentile_95[0] <= 1. && s.percentile_95[1] >= 1.
}
fn check_kernel_mode(answer: &Value, expected: &str) -> Result<()> {
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
    .contains(&expected)
        || answer["kernel_mode"] != expected
    {
        return Err("candidate kernel variant does not match protocol".into());
    }
    Ok(())
}
#[test]
fn kernel_guard_rejects_missing_wrong_and_unknown_modes() {
    for mode in [
        "legacy",
        "ordered",
        "cached",
        "both",
        "both_updates",
        "both_updates_sum",
        "both_updates_lead",
        "both_updates_fused",
        "both_updates_frontier",
    ] {
        assert!(check_kernel_mode(&json!({"kernel_mode":mode}), mode).is_ok());
        assert!(check_kernel_mode(&json!({}), mode).is_err());
        assert!(check_kernel_mode(&json!({"kernel_mode":"other"}), mode).is_err());
    }
    assert!(check_kernel_mode(&json!({"kernel_mode":"legacy"}), "both").is_err());
    assert!(check_kernel_mode(&json!({"kernel_mode":"unknown"}), "unknown").is_err());
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
    let a: Vec<_> = std::env::args().collect();
    if !(5..=7).contains(&a.len()) {
        return Err(
            "usage: paired-result-report RAW CONDITIONS NEW_REPORT RESULT_PCC [EXPECTED_EXPORTS [EXPECTED_KERNEL]]"
                .into(),
        );
    }
    let raw = Path::new(&a[1]);
    let out = Path::new(&a[3]);
    let s: Value = serde_json::from_slice(&fs::read(raw.join("summary.json"))?)?;
    let fixture_name = fixture_label(&s)?;
    let conditions = fs::read_to_string(&a[2])?;
    if conditions.lines().count() != 1 {
        return Err("single condition record".into());
    }
    let monitor: Value = serde_json::from_str(&conditions)?;
    if monitor["schema"] != "isolated-bench/1"
        || monitor["contended_samples"] != 0
        || monitor["exit_status"] != 0
    {
        return Err("isolation did not admit".into());
    }
    if s["qualification"] != false
        || s["identities_unchanged"] != true
        || !s["failure"].is_null()
        || s["result"].is_null()
    {
        return Err("incomplete paired transcript".into());
    }
    let pairs = s["aa_pairs"]
        .as_array()
        .filter(|p| p.len() == 7)
        .ok_or("seven A/A pairs")?;
    let blocks = s["blocks"]
        .as_array()
        .filter(|b| b.len() == 21)
        .ok_or("21 blocks")?;
    let expected_exports = a.get(5).map(|s| s.parse::<u64>()).transpose()?;
    let expected_kernel = a.get(6).map(String::as_str);
    if let Some(kernel) = expected_kernel {
        check_kernel_mode(&json!({"kernel_mode":kernel}), kernel)?;
        if expected_exports != Some(0) {
            return Err("kernel mode requires zero-export protocol".into());
        }
    }
    if let Some(expected) = expected_exports {
        for panel in std::iter::once(&s["warmup"])
            .chain(
                pairs
                    .iter()
                    .flat_map(|p| p.as_array().into_iter().flatten()),
            )
            .chain(blocks)
        {
            if let Some(kernel) = expected_kernel {
                check_kernel_mode(&panel["samples"][2]["answer"], kernel)?;
            }
            if panel["samples"][2]["answer"]["completion_exports"] != expected {
                return Err("candidate completion variant does not match protocol".into());
            }
        }
    }
    read_panel(&s["warmup"])?;
    let mut aa = Vec::new();
    for (arm, name) in NAMES.iter().enumerate() {
        let ratios = pairs
            .iter()
            .map(|p| -> Result<[f64; 2]> {
                if p.as_array().is_none_or(|p| p.len() != 2) {
                    return Err("A/B pair shape".into());
                }
                let x = read_panel(&p[0])?;
                let y = read_panel(&p[1])?;
                Ok([
                    x[arm][0] as f64 / y[arm][0] as f64,
                    x[arm][1] as f64 / y[arm][1] as f64,
                ])
            })
            .collect::<Result<Vec<_>>>()?;
        let stats = paired_statistics(&ratios)?;
        if !stats.iter().all(includes_one) || s["aa"][arm]["admitted"] != true {
            return Err("A/A rejection".into());
        }
        for (scope, stat) in ["pipeline_work", "full_paid"].iter().zip(stats) {
            agrees(&stat, &s["aa"][arm][scope])?;
            aa.push(json!({"arm":name,"scope":scope,"stat":stat}));
        }
    }
    let mut ratios = Vec::new();
    for (i, p) in blocks.iter().enumerate() {
        if p["order"] != json!(CYCLE[i % 6]) {
            return Err("counterbalanced order".into());
        }
        let v = read_panel(p)?;
        ratios.push([
            v[0][0] as f64 / v[2][0] as f64,
            v[1][0] as f64 / v[2][0] as f64,
            v[0][1] as f64 / v[2][1] as f64,
            v[1][1] as f64 / v[2][1] as f64,
        ]);
    }
    let stats = paired_statistics(&ratios)?;
    let flag15 =
        stats[2].adjusted_lower_1_over_720 > 15. && stats[3].adjusted_lower_1_over_720 > 15.;
    let flag10 =
        stats[2].adjusted_lower_1_over_720 > 10. && stats[3].adjusted_lower_1_over_720 > 10.;
    if s["result"]["paired_numeric_15x"] != flag15
        || s["result"]["paired_numeric_10x"] != flag10
        || s["result"]["family_confirmed"] != false
        || s["result"]["execution_admission_verified"] != false
    {
        return Err("decision flags disagree".into());
    }
    let mut md=format!("# Paired known-fixture development result\n\nRegistered numerical15x flag: **{flag15}**; separate10x partial flag: **{flag10}**. Both paid adjusted lower bounds must exceed the threshold. This is unsigned localL0 evidence, {}, not a fresh-family or signed execution verdict. [pcc:unverified]\n\nAll six A/A scope gates pass;21 counterbalanced main blocks finish, with108 complete matching workflow answers including warmup and A/A. Runtime identities are unchanged and the monitor records0 contended samples. PSI was unavailable and some service threads could not be moved. [pcc:unverified]\n\n| Named control | Scope | Median ratio | Ordinary95% interval | Adjusted lower1/720 |\n|---|---|---:|---|---:|\n",a[4]);
    let mut points = Vec::new();
    for (i, stat) in stats.iter().enumerate() {
        let control = i % 2;
        let scope = if i < 2 { "pipeline_work" } else { "full_paid" };
        agrees(stat, &s["result"]["comparisons"][control][scope])?;
        if s["result"]["comparisons"][control]["control"] != NAMES[control] {
            return Err("control label".into());
        }
        md += &format!(
            "| {} | {scope} | {:.6} | [{:.6}, {:.6}] | {:.6} |\n",
            NAMES[control],
            stat.median_paired_ratio,
            stat.percentile_95[0],
            stat.percentile_95[1],
            stat.adjusted_lower_1_over_720
        );
        points.push((format!("{} / {scope}", NAMES[control]), stat));
    }
    md+="\nRatios are named control/candidate. Pipeline-work includes launch, computation, checking and consumption; it is not a raw kernel metric. Full-paid adds the original measured preparation to the candidate and fresh three-route regeneration (including that preparation) to each original control. These are different validation algorithms for the same complete reduced basis. The candidate uses the actual root API and M31 arithmetic constructor. [pcc:unverified]\n\nOnly previously selected p2147483647,n16,first-star,seed7 is tested. No new seed, dimension or fresh-target generation. The strongest optimized controls and prior candidate retain their separate sixteen-workflow screen results; this three-arm study establishes no15x against the fastest control. Smaller-case regressions are retained in that screen. No intrinsic degree-of-regularity reduction follows from certificate/output degrees. [pcc:unverified]\n\n## A/A admission\n\n| Arm | Scope | Median A/B | Ordinary95% interval |\n|---|---|---:|---|\n";
    for row in &aa {
        let t = &row["stat"];
        md += &format!(
            "| {} | {} | {:.6} | [{:.6}, {:.6}] |\n",
            row["arm"].as_str().unwrap(),
            row["scope"].as_str().unwrap(),
            t["median_paired_ratio"].as_f64().unwrap(),
            t["percentile_95"][0].as_f64().unwrap(),
            t["percentile_95"][1].as_f64().unwrap()
        );
    }
    md+="\nThis Rust audit recomputes A/A and main statistics from integer costs with the frozen100000-draw joint bootstrap, SplitMix64 seed20261115, unbiased index rejection and nearest-rank endpoints. Every paid sum and block order is checked. Decimal JSON statistics compare within four machine epsilons; all gates compare exactly, without rounding. No trimming, replacement or estimator change. All raw panels/wires remain. [pcc:unverified]\n\n![Paired ratios](ratios.svg){width=95%}\n\nPoints: medians; bars: ordinary95% intervals; crosses: adjusted lower bounds. Lines mark10 and15.\n";
    md = md.replace("first-star", fixture_name).replace(
        "sixteen-workflow screen results",
        "retained full-control screen results",
    );
    fs::create_dir(out)?;
    fs::write(out.join("report.md"), md)?;
    fs::write(
        out.join("audit.json"),
        serde_json::to_vec_pretty(
            &json!({"complete_matching_workflow_answers":108,"main_blocks":21,"aa_all_pass":true,"paid_sums_checked":true,"statistics_recomputed":true,"expected_completion_exports":expected_exports,"expected_kernel_mode":expected_kernel,"paired_numeric_15x":flag15,"paired_numeric_10x":flag10,"main_statistics":stats,"input_sha256":s["input_sha256"],"fixture":fixture_name,"standing":"unsigned producer recomputation"}),
        )?,
    )?;
    let path = out.join("ratios.svg");
    let root = SVGBackend::new(&path, (1250, 650)).into_drawing_area();
    root.fill(&WHITE)?;
    let max = stats
        .iter()
        .map(|s| s.percentile_95[1])
        .fold(15f64, f64::max)
        * 1.2;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Paired known-fixture ratios: full paid and pipeline work",
            ("sans-serif", 22),
        )
        .margin(20)
        .x_label_area_size(55)
        .y_label_area_size(380)
        .build_cartesian_2d((1f64..max).log_scale(), 0usize..5usize)?;
    chart
        .configure_mesh()
        .disable_y_mesh()
        .y_labels(6)
        .y_label_formatter(&|y| {
            points
                .get(y.saturating_sub(1))
                .filter(|_| *y > 0)
                .map(|p| p.0.clone())
                .unwrap_or_default()
        })
        .x_desc("Named control / candidate (larger is faster)")
        .draw()?;
    for threshold in [10., 15.] {
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(threshold, 0), (threshold, 5)],
            BLACK.mix(0.3),
        )))?;
    }
    for (i, (_, stat)) in points.iter().enumerate() {
        let y = i + 1;
        let color = if i < 2 { BLUE } else { RED };
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(stat.percentile_95[0], y), (stat.percentile_95[1], y)],
            color.stroke_width(2),
        )))?;
        chart.draw_series(std::iter::once(Circle::new(
            (stat.median_paired_ratio, y),
            5,
            color.filled(),
        )))?;
        chart.draw_series(std::iter::once(Cross::new(
            (stat.adjusted_lower_1_over_720, y),
            6,
            color,
        )))?;
    }
    root.present()?;
    Ok(())
}
