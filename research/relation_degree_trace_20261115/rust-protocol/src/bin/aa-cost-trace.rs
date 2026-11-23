//! Retrospective exact cost decomposition; never invokes a native solver.
use compact_speed_analysis::{paired_statistics, Statistics};
use plotters::prelude::*;
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
const ARMS: [&str; 3] = [
    "f4-regeneration",
    "expanded-regeneration",
    "root-sparse-both",
];
fn number(v: &Value) -> Result<u64> {
    v.as_u64().ok_or_else(|| "unsigned cost".into())
}
fn sum(v: &[u64]) -> Result<u64> {
    v.iter().try_fold(0u64, |a, b| {
        a.checked_add(*b).ok_or_else(|| "cost overflow".into())
    })
}
#[derive(Clone, Debug, Serialize)]
struct Cost {
    prep: u64,
    setup: u64,
    process: [u64; 3],
    route_parent: u64,
    reference_parent: u64,
    work: [u64; 3],
    equality: [u64; 3],
    paid: [u64; 3],
    control_work_layers: Vec<[u64; 4]>,
    control_native_phases: Vec<[u64; 5]>,
    f4_observations_sha256: String,
}
fn decompose(
    prep: u64,
    setup: u64,
    routes: [u64; 3],
    process: [u64; 3],
    work: [u64; 3],
    equality: [u64; 3],
    paid: [u64; 3],
) -> Result<Cost> {
    let mut route_parent = 0u64;
    for i in 0..3 {
        route_parent = route_parent
            .checked_add(
                routes[i]
                    .checked_sub(process[i])
                    .ok_or("route shorter than process")?,
            )
            .ok_or("parent overflow")?;
    }
    let reference_parent = setup
        .checked_sub(sum(&[prep, sum(&routes)?])?)
        .ok_or("setup shorter than routes/preparation")?;
    for i in 0..3 {
        if sum(&[work[i], if i == 2 { prep } else { setup }, equality[i]])? != paid[i] {
            return Err("paid sum mismatch".into());
        }
    }
    if sum(&[prep, sum(&process)?, route_parent, reference_parent])? != setup {
        return Err("setup sum mismatch".into());
    }
    Ok(Cost {
        prep,
        setup,
        process,
        route_parent,
        reference_parent,
        work,
        equality,
        paid,
        control_work_layers: Vec::new(),
        control_native_phases: Vec::new(),
        f4_observations_sha256: String::new(),
    })
}
fn work_layers(total: u64, worker: u64, validation: u64, process: u64) -> Result<[u64; 4]> {
    let parent = worker
        .checked_sub(process)
        .ok_or("worker shorter than process")?;
    let remaining = total
        .checked_sub(sum(&[worker, validation])?)
        .ok_or("work shorter than worker/validation")?;
    let layers = [process, parent, validation, remaining];
    if sum(&layers)? != total {
        return Err("work layers mismatch".into());
    }
    Ok(layers)
}
fn native_layers(process: u64, before_print: u64, phases: [u64; 3]) -> Result<[u64; 5]> {
    let rest = before_print
        .checked_sub(sum(&phases)?)
        .ok_or("worker phases overlap")?;
    let launch = process
        .checked_sub(before_print)
        .ok_or("worker exceeds process wall")?;
    let layers = [phases[0], phases[1], phases[2], rest, launch];
    if sum(&layers)? != process {
        return Err("native layers mismatch".into());
    }
    Ok(layers)
}
fn read(path: &Path, pins: &mut BTreeMap<String, String>) -> Result<Value> {
    let bytes = fs::read(path)?;
    pins.insert(
        path.display().to_string(),
        format!("{:x}", Sha256::digest(&bytes)),
    );
    Ok(serde_json::from_slice(&bytes)?)
}
fn panel(
    raw: &Path,
    p: &Value,
    pins: &mut BTreeMap<String, String>,
    input: &Value,
) -> Result<Cost> {
    let saved = read(&raw.join("panel.json"), pins)?;
    if &saved != p || p["input_sha256"] != *input || p["all_complete_bases_equal"] != true {
        return Err("panel identity mismatch".into());
    }
    let samples = p["samples"]
        .as_array()
        .filter(|v| v.len() == 3)
        .ok_or("three arms")?;
    let mut work = [0; 3];
    let mut eq = [0; 3];
    let mut paid = [0; 3];
    for i in 0..3 {
        let s = &samples[i];
        if s["pipeline"] != ARMS[i]
            || s["status"] != "complete"
            || s["basis_sha256"] != samples[0]["basis_sha256"]
        {
            return Err("arm/answer mismatch".into());
        }
        work[i] = number(&s["pipeline_work_ns"])?;
        eq[i] = number(&s["cross_arm_equality_ns"])?;
        paid[i] = number(&s["paid_wall_ns"])?;
    }
    let refs = read(&raw.join("references.json"), pins)?;
    if refs["reference_basis_sha256"] != samples[0]["basis_sha256"] {
        return Err("reference basis differs".into());
    }
    let rows = refs["routes"]
        .as_array()
        .filter(|v| v.len() == 3)
        .ok_or("three references")?;
    let mut routes = [0; 3];
    let mut process = [0; 3];
    for (i, name) in ["std", "slimgb", "msolve"].into_iter().enumerate() {
        if rows[i]["route"] != name {
            return Err("reference order".into());
        }
        routes[i] = number(&rows[i]["whole_route_ns"])?;
        let r = read(&raw.join(format!("setup-{name}.process.json")), pins)?;
        if r["success"] != true || r["exit_code"] != 0 {
            return Err("reference process failed".into());
        }
        process[i] = number(&r["parent_process_wall_ns"])?;
    }
    let mut cost = decompose(
        number(&p["preparation_ns"])?,
        number(&p["regeneration_setup_ns"])?,
        routes,
        process,
        work,
        eq,
        paid,
    )?;
    for arm in 0..2 {
        let process = read(&raw.join(format!("{}.process.json", ARMS[arm])), pins)?;
        let call = read(&raw.join(format!("{}.call.json", ARMS[arm])), pins)?;
        if process["success"] != true || process["exit_code"] != 0 {
            return Err("control process failed".into());
        }
        let sample = &samples[arm]["sample"];
        for key in ["worker_wall_ns", "validation_wall_ns", "basis_sha256"] {
            if sample[key] != call[key] {
                return Err("control nested observation mismatch".into());
            }
        }
        let output = &call["worker_output"];
        let phases = [
            number(&output["clock_ns"]["input_parse_admission_conversion"])?,
            number(&output["clock_ns"]["construction_and_internal_checks"])?,
            number(&output["clock_ns"]["canonical_output"])?,
        ];
        cost.control_native_phases.push(native_layers(
            number(&process["parent_process_wall_ns"])?,
            number(&output["worker_wall_before_print_ns"])?,
            phases,
        )?);
        if arm == 0 {
            if !output["f4_observations"].is_object() {
                return Err("F4 observations required".into());
            }
            cost.f4_observations_sha256 = format!(
                "{:x}",
                Sha256::digest(serde_json::to_vec(&output["f4_observations"])?)
            );
        }
        cost.control_work_layers.push(work_layers(
            work[arm],
            number(&sample["worker_wall_ns"])?,
            number(&sample["validation_wall_ns"])?,
            number(&process["parent_process_wall_ns"])?,
        )?);
    }
    Ok(cost)
}
fn agrees(s: &Statistics, raw: &Value) -> Result<()> {
    if raw["samples"] != s.samples {
        return Err("AA sample count".into());
    }
    for (x, y) in [
        (s.median_paired_ratio, &raw["median_paired_ratio"]),
        (s.percentile_95[0], &raw["percentile_95"][0]),
        (s.percentile_95[1], &raw["percentile_95"][1]),
        (
            s.adjusted_lower_1_over_720,
            &raw["adjusted_lower_1_over_720"],
        ),
    ] {
        let y = y.as_f64().ok_or("AA statistic")?;
        if (x - y).abs() > 4. * f64::EPSILON * x.abs().max(1.) {
            return Err("AA recomputation differs".into());
        }
    }
    Ok(())
}
fn median(v: &mut [f64]) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn diff(a: u64, b: u64) -> i128 {
    a as i128 - b as i128
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: aa-cost-trace RECORDS_CONFIG NEW_OUTPUT".into());
    }
    let config = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    let files: Vec<PathBuf> = serde_json::from_slice(&fs::read(config)?)?;
    if files.len() != 5 {
        return Err("all five studies required".into());
    }
    let mut pins = BTreeMap::new();
    let mut studies = Vec::new();
    let mut chart_rows = Vec::new();
    let mut work_table=String::from("\n## Existing control-work subintervals\n\n| Study | Arm | Process A-B ms | Worker parent A-B ms | Validation A-B ms | Remaining parent A-B ms |\n|---|---|---:|---:|---:|---:|\n");
    let mut native_table=String::from("\n## Recorded worker phase deltas\n\n| Study | Arm | Admission A-B ms | Construct/check A-B ms | Canonical output A-B ms | Worker remainder A-B ms | Launch/printing A-B ms | Same F4 observations across14 |\n|---|---|---:|---:|---:|---:|---:|---|\n");
    let mut md=String::from("# Retrospective A/A cost layers\n\nFive retained studies, passing and rejected; no new native call, timing retry, pooled speed estimate, fifteen-fold flag or changed gate. Raw summaries are bound to closed-record hashes; every accessed raw file is pinned in audit.json. Counts overlap with the original studies. This descriptive trace identifies measured cost contributions, not a causal source of noise. [pcc:unverified]\n\nEach control paid interval equals its work plus shared fresh three-route setup and equality cost. Setup includes charged original preparation, child process intervals, route parsing/metadata and outer reference overhead. Candidate pays its own work plus preparation and equality. All nesting residuals and sums are checked as integers; ratios/differences are never substituted for old gates. [pcc:unverified]\n\n| Study | Original all-AA pass | Median setup A/B | Median setup first/second | Sum F4 paid A-B ms | Work contribution ms | Setup contribution ms | Equality contribution ms |\n|---|---|---:|---:|---:|---:|---:|---:|\n");
    for record_path in &files {
        let record = read(record_path, &mut pins)?;
        if record["status"] != "measured" {
            return Err("closed measured record required".into());
        }
        let command = record["commands"][0]["argv"]
            .as_array()
            .ok_or("record command")?;
        let at = command
            .iter()
            .position(|s| s == "root-paired" || s == "update-paired")
            .ok_or("paired command")?;
        let raw = Path::new(
            command
                .get(at + 2)
                .and_then(Value::as_str)
                .ok_or("raw path")?,
        );
        let summary_path = raw.join("summary.json");
        let s = read(&summary_path, &mut pins)?;
        let expected = record["raw"]
            .as_array()
            .ok_or("record raw")?
            .iter()
            .find(|r| {
                r["path"]
                    .as_str()
                    .is_some_and(|p| p.ends_with("/summary.json"))
            })
            .and_then(|r| r["sha256"].as_str())
            .ok_or("record summary hash")?;
        if pins[&summary_path.display().to_string()] != expected
            || s["qualification"] != false
            || s["identities_unchanged"] != true
        {
            return Err("frozen summary binding".into());
        }
        let pairs = s["aa_pairs"]
            .as_array()
            .filter(|p| p.len() == 7)
            .ok_or("seven pairs")?;
        let mut costs = Vec::new();
        let mut rows = Vec::new();
        let mut label = Vec::new();
        let mut physical = Vec::new();
        let mut totals = [0i128; 4];
        let mut work_totals = [[0i128; 4]; 2];
        let mut native_totals = [[0i128; 5]; 2];
        for (pair, p) in pairs.iter().enumerate() {
            if p.as_array().is_none_or(|p| p.len() != 2) {
                return Err("AB shape".into());
            }
            let a = panel(
                &raw.join(format!("aa-{pair:02}-a")),
                &p[0],
                &mut pins,
                &s["input_sha256"],
            )?;
            let b = panel(
                &raw.join(format!("aa-{pair:02}-b")),
                &p[1],
                &mut pins,
                &s["input_sha256"],
            )?;
            for arm in 0..2 {
                for i in 0..4 {
                    work_totals[arm][i] +=
                        diff(a.control_work_layers[arm][i], b.control_work_layers[arm][i]);
                }
            }
            for arm in 0..2 {
                for i in 0..5 {
                    native_totals[arm][i] += diff(
                        a.control_native_phases[arm][i],
                        b.control_native_phases[arm][i],
                    );
                }
            }
            let mut layered = Vec::new();
            for arm in 0..3 {
                let work = diff(a.work[arm], b.work[arm]);
                let equality = diff(a.equality[arm], b.equality[arm]);
                let preparation = diff(a.prep, b.prep);
                let child: Vec<_> = (0..3).map(|i| diff(a.process[i], b.process[i])).collect();
                let route_parent = diff(a.route_parent, b.route_parent);
                let outer = diff(a.reference_parent, b.reference_parent);
                let setup = if arm == 2 {
                    preparation
                } else {
                    preparation + child.iter().sum::<i128>() + route_parent + outer
                };
                let paid = diff(a.paid[arm], b.paid[arm]);
                if work + setup + equality != paid {
                    return Err("signed cost reconciliation".into());
                }
                layered.push(json!({"arm":ARMS[arm],"paid_delta_ns":paid,"work_delta_ns":work,"setup_or_prep_delta_ns":setup,"equality_delta_ns":equality,"preparation_delta_ns":preparation,"reference_child_deltas_ns":child,"route_parent_delta_ns":route_parent,"reference_parent_delta_ns":outer}));
                if arm == 0 {
                    for (i, v) in [paid, work, setup, equality].into_iter().enumerate() {
                        totals[i] += v;
                    }
                }
            }
            let ratio = a.setup as f64 / b.setup as f64;
            label.push(ratio);
            physical.push(if pair % 2 == 0 { ratio } else { 1. / ratio });
            rows.push(json!({"pair":pair,"execution_order":if pair%2==0{"AB"}else{"BA"},"a":a,"b":b,"layers":layered}));
            costs.push([a, b]);
        }
        let mut all_admitted = true;
        for (arm, name) in ARMS.into_iter().enumerate() {
            let ratios: Vec<_> = costs
                .iter()
                .map(|c| {
                    [
                        c[0].work[arm] as f64 / c[1].work[arm] as f64,
                        c[0].paid[arm] as f64 / c[1].paid[arm] as f64,
                    ]
                })
                .collect();
            let stats = paired_statistics(&ratios)?;
            let mut admitted = true;
            if s["aa"][arm]["arm"] != name {
                return Err("AA arm".into());
            }
            for (scope, stat) in ["pipeline_work", "full_paid"].into_iter().zip(&stats) {
                agrees(stat, &s["aa"][arm][scope])?;
                admitted &= stat.percentile_95[0] <= 1. && stat.percentile_95[1] >= 1.;
            }
            if s["aa"][arm]["admitted"] != admitted {
                return Err("gate changed".into());
            }
            all_admitted &= admitted;
        }
        if totals[0] != totals[1] + totals[2] + totals[3] {
            return Err("total delta reconciliation".into());
        }
        let id = record["id"].as_str().ok_or("id")?;
        let short = id.get(9..13).ok_or("timestamp")?;
        let lm = median(&mut label);
        let pm = median(&mut physical);
        md += &format!(
            "| {short} | {all_admitted} | {lm:.6} | {pm:.6} | {:.3} | {:.3} | {:.3} | {:.3} |\n",
            totals[0] as f64 / 1e6,
            totals[1] as f64 / 1e6,
            totals[2] as f64 / 1e6,
            totals[3] as f64 / 1e6
        );
        for arm in 0..2 {
            let w = work_totals[arm];
            work_table += &format!(
                "| {short} | {} | {:.3} | {:.3} | {:.3} | {:.3} |\n",
                ARMS[arm],
                w[0] as f64 / 1e6,
                w[1] as f64 / 1e6,
                w[2] as f64 / 1e6,
                w[3] as f64 / 1e6
            );
        }
        let first_hash = &costs[0][0].f4_observations_sha256;
        let same_observations = costs
            .iter()
            .flatten()
            .all(|c| &c.f4_observations_sha256 == first_hash);
        for arm in 0..2 {
            let n = native_totals[arm];
            if n.iter().sum::<i128>() != work_totals[arm][0] {
                return Err("native signed reconciliation".into());
            }
            native_table += &format!(
                "| {short} | {} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {same_observations} |\n",
                ARMS[arm],
                n[0] as f64 / 1e6,
                n[1] as f64 / 1e6,
                n[2] as f64 / 1e6,
                n[3] as f64 / 1e6,
                n[4] as f64 / 1e6
            );
        }
        chart_rows.push((short.to_string(), totals));
        studies.push(json!({"record":record_path,"raw":raw,"input_sha256":s["input_sha256"],"all_aa_admitted":all_admitted,"all14_f4_observations_equal":same_observations,"control_native_phase_delta_totals_ns":native_totals,"control_work_layer_delta_totals_ns":work_totals,"sum_f4_delta_ns":{"paid":totals[0],"work":totals[1],"setup":totals[2],"equality":totals[3]},"median_setup_a_over_b":lm,"median_setup_first_over_second":pm,"pairs":rows}));
    }
    md += &work_table;
    md += &native_table;
    md+="\nWorker phase clocks are elapsed walls too; admission/construction/canonical/remaining intervals plus launch/printing reconstruct the recorded process wall exactly. Identical F4 observation hashes cover the recorded degree/step/max-row/max-column counters, not every instruction or physical cause. [pcc:unverified]\n";
    md+="\nSigned totals sum the seven labelled differences; they are not independent samples, confidence intervals or adjusted results. Positive means A cost more than B; first/second orientation follows the frozen alternating execution schedule. Separate inputs and algorithms are retained per study, never pooled to estimate speed. No causal hardware/cache/scheduler/clock diagnosis follows. All original successes and rejections remain in force. [pcc:unverified]\n\nSources checked (KB unavailable): official Rust Instant/Command::status contracts and recorded Runner/Panel/reference code, with the existing paired protocols. Recorded process walls include wrappers, scheduling and stdout consumption; outer costs include parsing and metadata. Missing CPU-time/temperature/counter evidence prevents attributing a physical cause. No native measurement or timing code is changed. [pcc:unverified]\n\n![Exact F4 paid delta contributions](layers.svg){width=90%}\n";
    fs::create_dir(out)?;
    fs::write(out.join("report.md"), md)?;
    fs::write(
        out.join("audit.json"),
        serde_json::to_vec_pretty(
            &json!({"studies":studies,"accessed_file_sha256":pins,"all_integer_sums_and_old_aa_gates_recomputed":true,"native_calls":0,"scope":"retrospective decomposition only; original results unchanged"}),
        )?,
    )?;
    let extent = chart_rows
        .iter()
        .flat_map(|(_, v)| v.iter())
        .map(|v| (*v as f64).abs() / 1e6)
        .fold(1f64, f64::max)
        * 1.25;
    let path = out.join("layers.svg");
    let root = SVGBackend::new(&path, (1300, 700)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Retained A/A: exact F4 paid A-minus-B contributions",
            ("sans-serif", 24),
        )
        .margin(25)
        .x_label_area_size(60)
        .y_label_area_size(100)
        .build_cartesian_2d(0f64..chart_rows.len() as f64, -extent..extent)?;
    chart
        .configure_mesh()
        .x_desc("Study timestamp; positive means A more costly")
        .x_labels(chart_rows.len())
        .x_label_formatter(&|x| {
            chart_rows
                .get(x.floor() as usize)
                .map(|r| r.0.clone())
                .unwrap_or_default()
        })
        .y_desc("Sum over seven paired differences, milliseconds")
        .draw()?;
    for (j, (name, color)) in [
        ("paid", BLACK),
        ("work", BLUE),
        ("setup", RED),
        ("equality", GREEN),
    ]
    .into_iter()
    .enumerate()
    {
        chart
            .draw_series(chart_rows.iter().enumerate().map(|(i, (_, v))| {
                Circle::new(
                    (i as f64 + 0.15 + j as f64 * 0.17, v[j] as f64 / 1e6),
                    6,
                    color.filled(),
                )
            }))?
            .label(name)
            .legend(move |(x, y)| Circle::new((x + 8, y), 5, color.filled()));
    }
    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.9))
        .border_style(BLACK)
        .draw()?;
    root.present()?;
    Ok(())
}
#[test]
fn decomposition_rejects_bad_nesting_paid_sums_and_overflow() {
    let c = decompose(
        10,
        110,
        [25; 3],
        [20; 3],
        [30, 40, 5],
        [2; 3],
        [142, 152, 17],
    )
    .unwrap();
    assert_eq!(c.route_parent, 15);
    assert_eq!(c.reference_parent, 25);
    assert!(decompose(
        10,
        80,
        [25; 3],
        [20; 3],
        [30, 40, 5],
        [2; 3],
        [112, 122, 17]
    )
    .is_err());
    assert!(decompose(
        10,
        110,
        [25; 3],
        [26; 3],
        [30, 40, 5],
        [2; 3],
        [142, 152, 17]
    )
    .is_err());
    assert!(decompose(
        10,
        110,
        [25; 3],
        [20; 3],
        [30, 40, 5],
        [2; 3],
        [143, 152, 17]
    )
    .is_err());
    assert!(sum(&[u64::MAX, 1]).is_err());
}

#[test]
fn nested_work_rejects_omitted_or_overlapping_intervals() {
    assert_eq!(work_layers(100, 80, 5, 70).unwrap(), [70, 10, 5, 15]);
    assert!(work_layers(80, 80, 5, 70).is_err());
    assert!(work_layers(100, 80, 5, 90).is_err());
}

#[test]
fn native_phase_nesting_is_exact_and_rejects_bad_clocks() {
    assert_eq!(
        native_layers(100, 90, [10, 60, 5]).unwrap(),
        [10, 60, 5, 15, 10]
    );
    assert!(native_layers(80, 90, [10, 60, 5]).is_err());
    assert!(native_layers(100, 90, [10, 80, 5]).is_err());
}
