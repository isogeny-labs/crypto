//! Independent recomputation of the frozen root-finding benchmark summary.
//!
//! This example reads only committed JSONL evidence. It does not import or
//! execute the benchmark producer. Its output remains `[pcc:unverified]`
//! until an independent signed verifier establishes standing.

use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const STUDY: &str = "research/rootfind_baseline_100x_20261117";
const BASELINES: [&str; 2] = ["baseline.jsonl", "baseline-2.jsonl"];
const PROBE: &str = "floor-probe.jsonl";
const RESULTS: &str = "RESULTS.md";

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Size {
    bits: Option<u64>,
    p: Option<u64>,
    #[serde(rename = "r")]
    _r: Option<u64>,
    #[serde(rename = "n")]
    _n: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
struct BaselineRow {
    schema: String,
    family: String,
    size: Size,
    abscissae: u64,
    arm: String,
    base_points: u64,
    targets: u64,
    rounds: u64,
    isolation: String,
    found: u64,
    adds: u64,
    build_adds: u64,
    solve_s_median: f64,
    build_s_median: f64,
    ns_per_add_median: f64,
    ns_per_target_median: f64,
    ns_per_build_add_median: f64,
}

#[derive(Clone, Debug, Deserialize)]
struct ProbeRow {
    schema: String,
    bits: u64,
    p: u64,
    abscissae: u64,
    targets: u64,
    table_keys: u64,
    filter_bytes: u64,
    abscissae_checked: u64,
    abscissa_mismatches: u64,
    exact_hits: u64,
    exact_hits_through_filter: u64,
    filter_positives: u64,
    ns_per_point_arithmetic: f64,
    ns_per_point_with_filter: f64,
}

#[derive(Clone, Debug)]
struct Cell {
    abscissae: u64,
    default_ns: f64,
    subtract_ns: f64,
    probe_ns: f64,
    default_to_probe: f64,
    subtract_to_probe: f64,
    target_50x_ns: f64,
    ratio_gap_to_50x: f64,
    exact_hits: u64,
}

fn fail(message: impl AsRef<str>) -> ! {
    panic!("root-finding baseline validation: {}", message.as_ref());
}

fn study_path(name: &str) -> PathBuf {
    Path::new(STUDY).join(name)
}

fn read_jsonl(path: &Path) -> Vec<Value> {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|error| fail(format!("{}: {error}", path.display())));
    text.lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(line_number, line)| {
            serde_json::from_str(line).unwrap_or_else(|error| {
                fail(format!(
                    "{}:{}: invalid JSON: {error}",
                    path.display(),
                    line_number + 1
                ))
            })
        })
        .collect()
}

fn sha256(path: &Path) -> String {
    let bytes = fs::read(path).unwrap_or_else(|error| fail(format!("{}: {error}", path.display())));
    hex::encode(Sha256::digest(bytes))
}

fn close(actual: f64, expected: f64, label: &str) {
    if !actual.is_finite() || !expected.is_finite() {
        fail(format!("{label}: non-finite value"));
    }
    let scale = actual.abs().max(expected.abs()).max(1.0);
    if (actual - expected).abs() > 1e-9 * scale {
        fail(format!("{label}: {actual} != {expected}"));
    }
}

fn validate_baselines() -> (Vec<BaselineRow>, usize) {
    let mut rows = Vec::new();
    let mut kernel_rows = 0;
    for name in BASELINES {
        let path = study_path(name);
        for (index, value) in read_jsonl(&path).into_iter().enumerate() {
            let schema = value
                .get("schema")
                .and_then(Value::as_str)
                .unwrap_or_else(|| fail(format!("{name}:{}: missing schema", index + 1)));
            match schema {
                "rootfind.baseline/v1" => {
                    let row: BaselineRow = serde_json::from_value(value)
                        .unwrap_or_else(|error| fail(format!("{name}:{}: {error}", index + 1)));
                    if row.schema != "rootfind.baseline/v1" {
                        fail(format!("{name}:{}: schema mismatch", index + 1));
                    }
                    if row.adds == 0 || row.targets == 0 {
                        fail(format!("{name}:{}: zero timing denominator", index + 1));
                    }
                    close(
                        row.ns_per_add_median,
                        row.solve_s_median * 1e9 / row.adds as f64,
                        &format!("{name}:{}: ns_per_add_median", index + 1),
                    );
                    close(
                        row.ns_per_target_median,
                        row.solve_s_median * 1e9 / row.targets as f64,
                        &format!("{name}:{}: ns_per_target_median", index + 1),
                    );
                    if row.build_adds > 0 {
                        close(
                            row.ns_per_build_add_median,
                            row.build_s_median * 1e9 / row.build_adds as f64,
                            &format!("{name}:{}: ns_per_build_add_median", index + 1),
                        );
                    }
                    rows.push(row);
                }
                "rootfind.baseline.kernel/v1" => kernel_rows += 1,
                other => fail(format!("{name}:{}: unknown schema {other}", index + 1)),
            }
        }
    }
    if rows.len() != 48 || kernel_rows != 5 {
        fail(format!(
            "expected 48 baseline rows and 5 kernel rows, found {} and {kernel_rows}",
            rows.len()
        ));
    }
    (rows, kernel_rows)
}

fn baseline_key(row: &BaselineRow) -> (u64, u64, u64, String) {
    (
        row.size
            .bits
            .unwrap_or_else(|| fail(format!("arm={}: prime row missing bits", row.arm))),
        row.size
            .p
            .unwrap_or_else(|| fail(format!("arm={}: prime row missing p", row.arm))),
        row.abscissae,
        row.arm.clone(),
    )
}

fn index_baselines(rows: &[BaselineRow]) -> HashMap<(u64, u64, u64, String), BaselineRow> {
    let mut indexed = HashMap::new();
    for row in rows {
        if row.family != "prime" {
            continue;
        }
        let key = baseline_key(row);
        if indexed.insert(key.clone(), row.clone()).is_some() {
            fail(format!(
                "duplicate row for bits={}, p={}, A={}, arm={}",
                key.0, key.1, key.2, key.3
            ));
        }
    }
    indexed
}

fn probe_rows() -> Vec<ProbeRow> {
    let path = study_path(PROBE);
    let rows: Vec<ProbeRow> = read_jsonl(&path)
        .into_iter()
        .enumerate()
        .map(|(index, value)| {
            serde_json::from_value(value)
                .unwrap_or_else(|error| fail(format!("{PROBE}:{}: {error}", index + 1)))
        })
        .collect();
    if rows.len() != 3 {
        fail(format!("{PROBE}: expected 3 rows, found {}", rows.len()));
    }
    rows
}

fn require_same_fixture(default: &BaselineRow, candidate: &BaselineRow) {
    if candidate.size != default.size
        || candidate.base_points != default.base_points
        || candidate.targets != default.targets
        || candidate.rounds != default.rounds
        || candidate.isolation != default.isolation
        || candidate.found != default.found
    {
        fail(format!(
            "A={}, arm={}: fixture differs from default",
            candidate.abscissae, candidate.arm
        ));
    }
}

fn validate_probe(probe: &ProbeRow, prime: u64, abscissae: u64) {
    if probe.schema != "rootfind.floor-probe/v1"
        || probe.bits != 32
        || probe.p != prime
        || probe.abscissae != abscissae
    {
        fail(format!("A={abscissae}: probe fixture mismatch"));
    }
    if probe.targets == 0
        || probe.table_keys == 0
        || probe.filter_bytes == 0
        || probe.abscissae_checked == 0
    {
        fail(format!("A={abscissae}: zero probe count"));
    }
    if probe.abscissa_mismatches > probe.abscissae_checked
        || probe.exact_hits > probe.abscissae_checked
        || probe.exact_hits_through_filter > probe.abscissae_checked
        || probe.filter_positives < probe.exact_hits
    {
        fail(format!("A={abscissae}: invalid probe count"));
    }
    if probe.abscissa_mismatches != 0 {
        fail(format!("A={abscissae}: recorded abscissa mismatch"));
    }
    if probe.exact_hits != probe.exact_hits_through_filter {
        fail(format!("A={abscissae}: filter dropped a recorded hit"));
    }
    if !probe.ns_per_point_arithmetic.is_finite()
        || probe.ns_per_point_arithmetic <= 0.0
        || !probe.ns_per_point_with_filter.is_finite()
        || probe.ns_per_point_with_filter <= 0.0
    {
        fail(format!("A={abscissae}: invalid probe timing"));
    }
}

fn require_reported_row(results: &str, label: &str, values: [f64; 3]) {
    let label_prefix = format!("| {label} |");
    let labeled_rows: Vec<&str> = results
        .lines()
        .filter(|line| line.starts_with(&label_prefix))
        .collect();
    if labeled_rows.len() != 1 {
        fail(format!(
            "RESULTS.md: expected one {label} row, found {}",
            labeled_rows.len()
        ));
    }
    let expected = format!(
        "| {label} | {:.1} ns | {:.1} ns | {:.1} ns |",
        values[0], values[1], values[2]
    );
    if !labeled_rows[0].starts_with(&expected) {
        fail(format!("RESULTS.md: missing rounded row {expected}"));
    }
}

fn headline_table(results: &str) -> String {
    let header = "| Arm | A = 64 | A = 256 | A = 1024 | vs default |";
    let mut lines = results.lines().skip_while(|line| *line != header);
    if lines.next() != Some(header) {
        fail("RESULTS.md: missing expected table header");
    }
    let mut table = String::from(header);
    table.push('\n');
    for line in lines.take_while(|line| !line.is_empty()) {
        table.push_str(line);
        table.push('\n');
    }
    table
}

fn build_validation() -> (Value, String) {
    let (baseline_rows, kernel_rows) = validate_baselines();
    let indexed = index_baselines(&baseline_rows);
    let baseline_isolation = baseline_rows
        .iter()
        .find(|row| row.family == "prime")
        .map(|row| row.isolation.as_str())
        .unwrap_or_else(|| fail("missing prime baseline rows"));
    if baseline_rows
        .iter()
        .filter(|row| row.family == "prime")
        .any(|row| row.isolation != baseline_isolation)
    {
        fail("prime baseline isolation changed across rows");
    }
    let probes = probe_rows();
    let probe_index: HashMap<u64, ProbeRow> = probes
        .iter()
        .map(|row| (row.abscissae, row.clone()))
        .collect();
    if probe_index.len() != probes.len() {
        fail("duplicate probe A value");
    }

    let reference_prime = 3_466_697_909_u64;
    let arms = [
        "rootfind-mitm",
        "rootfind-mitm-prime-batch",
        "rootfind-subtract",
        "rootfind-mitm-aa",
    ];
    let mut cells = Vec::new();
    let mut baseline_targets_per_cell = None;
    let mut probe_targets_per_cell = None;
    for abscissae in [64_u64, 256, 1024] {
        let key = |arm: &str| (32_u64, reference_prime, abscissae, arm.to_owned());
        let default = indexed
            .get(&key("rootfind-mitm"))
            .unwrap_or_else(|| fail(format!("A={abscissae}: missing default")));
        for arm in arms {
            let candidate = indexed
                .get(&key(arm))
                .unwrap_or_else(|| fail(format!("A={abscissae}: missing arm {arm}")));
            require_same_fixture(default, candidate);
        }
        if default.base_points != 2 * abscissae {
            fail(format!("A={abscissae}: default base_points mismatch"));
        }
        let subtract = indexed.get(&key("rootfind-subtract")).unwrap();
        let probe = probe_index
            .get(&abscissae)
            .unwrap_or_else(|| fail(format!("A={abscissae}: missing probe")));
        validate_probe(probe, reference_prime, abscissae);
        match baseline_targets_per_cell {
            None => baseline_targets_per_cell = Some(default.targets),
            Some(targets) if targets == default.targets => {}
            Some(_) => fail(format!("A={abscissae}: baseline target count changed")),
        }
        match probe_targets_per_cell {
            None => probe_targets_per_cell = Some(probe.targets),
            Some(targets) if targets == probe.targets => {}
            Some(_) => fail(format!("A={abscissae}: probe target count changed")),
        }

        let threshold = default.ns_per_add_median / 50.0;
        cells.push(Cell {
            abscissae,
            default_ns: default.ns_per_add_median,
            subtract_ns: subtract.ns_per_add_median,
            probe_ns: probe.ns_per_point_with_filter,
            default_to_probe: default.ns_per_add_median / probe.ns_per_point_with_filter,
            subtract_to_probe: subtract.ns_per_add_median / probe.ns_per_point_with_filter,
            target_50x_ns: threshold,
            ratio_gap_to_50x: probe.ns_per_point_with_filter / threshold,
            exact_hits: probe.exact_hits,
        });
    }

    let mut aa_ratios = Vec::new();
    for row in &baseline_rows {
        if row.family != "prime" || row.arm != "rootfind-mitm-aa" {
            continue;
        }
        let default_key = (
            row.size.bits.expect("prime row has bits"),
            row.size.p.expect("prime row has p"),
            row.abscissae,
            "rootfind-mitm".to_owned(),
        );
        let default = indexed
            .get(&default_key)
            .unwrap_or_else(|| fail(format!("A={}: missing A/A default", row.abscissae)));
        require_same_fixture(default, row);
        aa_ratios.push(row.ns_per_add_median / default.ns_per_add_median);
    }
    if aa_ratios.len() != 9 {
        fail(format!("expected 9 A/A cells, found {}", aa_ratios.len()));
    }

    let results = fs::read_to_string(study_path(RESULTS))
        .unwrap_or_else(|error| fail(format!("{RESULTS}: {error}")));
    if !results.contains("The table build costs as much as about 1,100 default targets") {
        fail("RESULTS.md: missing reported 1,100-target table-build claim");
    }
    let reported_table = headline_table(&results);
    let values = |field: fn(&Cell) -> f64| [field(&cells[0]), field(&cells[1]), field(&cells[2])];
    require_reported_row(
        &reported_table,
        "`rootfind-mitm` (default)",
        values(|cell| cell.default_ns),
    );
    require_reported_row(
        &reported_table,
        "`rootfind-mitm-prime-batch`",
        [
            indexed[&(
                32,
                reference_prime,
                64,
                "rootfind-mitm-prime-batch".to_owned(),
            )]
                .ns_per_add_median,
            indexed[&(
                32,
                reference_prime,
                256,
                "rootfind-mitm-prime-batch".to_owned(),
            )]
                .ns_per_add_median,
            indexed[&(
                32,
                reference_prime,
                1024,
                "rootfind-mitm-prime-batch".to_owned(),
            )]
                .ns_per_add_median,
        ],
    );
    require_reported_row(
        &reported_table,
        "`rootfind-subtract` (best existing CPU arm)",
        values(|cell| cell.subtract_ns),
    );
    require_reported_row(
        &reported_table,
        "Prototype (section 3)",
        values(|cell| cell.probe_ns),
    );

    let default_1024 = indexed
        .get(&(32, reference_prime, 1024, "rootfind-mitm".to_owned()))
        .unwrap();
    let build_target_equivalent =
        default_1024.build_s_median * 1e9 / default_1024.ns_per_target_median;

    let input_names = [BASELINES[0], BASELINES[1], PROBE, RESULTS];
    let inputs: serde_json::Map<String, Value> = input_names
        .iter()
        .map(|name| ((*name).to_owned(), json!(sha256(&study_path(name)))))
        .collect();
    let cell_values: Vec<Value> = cells
        .iter()
        .map(|cell| {
            json!({
                "abscissae": cell.abscissae,
                "default_ns_per_point": cell.default_ns,
                "subtract_ns_per_point": cell.subtract_ns,
                "probe_ns_per_point": cell.probe_ns,
                "timing_ratio_default_to_probe": cell.default_to_probe,
                "timing_ratio_subtract_to_probe": cell.subtract_to_probe,
                "target_50x_ns_per_point": cell.target_50x_ns,
                "ratio_gap_to_50x": cell.ratio_gap_to_50x,
                "recorded_exact_hits": cell.exact_hits,
            })
        })
        .collect();
    let abscissae_checked: u64 = probes.iter().map(|row| row.abscissae_checked).sum();
    let mismatches: u64 = probes.iter().map(|row| row.abscissa_mismatches).sum();
    let exact_hits: u64 = probes.iter().map(|row| row.exact_hits).sum();
    let through_filter: u64 = probes.iter().map(|row| row.exact_hits_through_filter).sum();
    let validation = json!({
        "schema": "rootfind.baseline-validation/v1",
        "status": "native producer-independent recomputation; pcc unverified",
        "inputs": inputs,
        "reference": {
            "family": "prime",
            "bits": 32,
            "p": reference_prime,
            "baseline_isolation": baseline_isolation,
            "baseline_targets_per_cell": baseline_targets_per_cell.expect("baseline targets set"),
            "probe_targets_per_cell": probe_targets_per_cell.expect("probe targets set"),
            "cells": cell_values,
        },
        "controls": {
            "aa_ratio_min": aa_ratios.iter().copied().fold(f64::INFINITY, f64::min),
            "aa_ratio_max": aa_ratios.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            "aa_cells": aa_ratios.len(),
            "abscissae_checked": abscissae_checked,
            "abscissa_mismatches": mismatches,
            "recorded_exact_hits": exact_hits,
            "recorded_exact_hits_through_filter": through_filter,
        },
        "errata": {
            "results_md_table_build_target_equivalent": {
                "reported_approximate_targets": 1100,
                "recomputed_targets": build_target_equivalent,
                "build_s_median": default_1024.build_s_median,
                "default_ns_per_target_median": default_1024.ns_per_target_median,
            }
        },
        "checks": {
            "baseline_derived_fields_recomputed": true,
            "baseline_rows_checked": baseline_rows.len(),
            "kernel_rows_schema_checked": kernel_rows,
            "headline_table_matches_rounded_raw_rows": true,
            "probe_prime_matches_baseline": true,
            "recorded_filter_hit_counts_match": true,
        }
    });

    let mut markdown = String::from(
        "# Native recomputation of the root-finding baseline\n\n\
Status: producer-independent recomputation from committed rows, `[pcc:unverified]`.\n\
This validator does not rerun the benchmark and does not establish an admitted\n\
performance measurement.\n\n\
| A | Default ns/point | Subtract arm ns/point | Probe ns/point | Default/probe timing ratio | Subtract/probe timing ratio | 50x threshold | Ratio gap to 50x |\n\
|---:|---:|---:|---:|---:|---:|---:|---:|\n",
    );
    for cell in &cells {
        markdown.push_str(&format!(
            "| {} | {:.3} | {:.3} | {:.3} | {:.3}x | {:.3}x | {:.3} ns | {:.3}x |\n",
            cell.abscissae,
            cell.default_ns,
            cell.subtract_ns,
            cell.probe_ns,
            cell.default_to_probe,
            cell.subtract_to_probe,
            cell.target_50x_ns,
            cell.ratio_gap_to_50x,
        ));
    }
    markdown.push_str(&format!(
        "\nThe committed baseline A/A rows span {:.3}x to {:.3}x across {} prime-field\n\
cells. That range describes historical baseline repeatability. It does not measure\n\
probe repeatability.\n\n\
The probe rows record {abscissae_checked} abscissa checks, {mismatches} mismatches,\n\
and {through_filter}/{exact_hits} aggregate exact hits through the filter. These\n\
are producer-recorded counts rather than freshly replayed per-hit witnesses.\n\n\
Baseline and subtract rows contain {} targets per cell. Probe rows contain {}\n\
targets per cell and time a smaller operation boundary. The displayed quotients are\n\
descriptive normalized timing ratios across those unmatched historical workloads.\n\
The 50x values are derived thresholds against each frozen default row, not measured\n\
arms. Every historical prime-field baseline row records\n\
`isolation: {baseline_isolation}`.\n\n\
## Erratum\n\n\
`RESULTS.md` says the A = 1024 table build equals about 1,100 default targets.\n\
The committed {:.3} ms build divided by {:.3} microseconds per target equals\n\
about {:.0} targets. The frozen results file remains unchanged. See\n\
[`ERRATA.md`](ERRATA.md).\n\n\
Recompute from the repository root with `cargo run --locked --release --manifest-path\n\
research/rootfind_baseline_100x_20261117/native/Cargo.toml -- --check`.\n",
        aa_ratios.iter().copied().fold(f64::INFINITY, f64::min),
        aa_ratios.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        aa_ratios.len(),
        baseline_targets_per_cell.expect("baseline targets set"),
        probe_targets_per_cell.expect("probe targets set"),
        default_1024.build_s_median * 1e3,
        default_1024.ns_per_target_median / 1e3,
        build_target_equivalent,
    ));
    (validation, markdown)
}

fn main() {
    let mode = env::args().nth(1).unwrap_or_else(|| {
        fail("choose exactly one of --write or --check");
    });
    if env::args().nth(2).is_some() || (mode != "--write" && mode != "--check") {
        fail("choose exactly one of --write or --check");
    }
    let (validation, markdown) = build_validation();
    let json_path = study_path("validation.json");
    let markdown_path = study_path("VALIDATION.md");
    let rendered_json = format!(
        "{}\n",
        serde_json::to_string_pretty(&validation).expect("validation JSON is serializable")
    );
    if mode == "--write" {
        fs::write(json_path, rendered_json).expect("write validation.json");
        fs::write(markdown_path, markdown).expect("write VALIDATION.md");
        return;
    }
    if fs::read(&json_path).expect("read validation.json") != rendered_json.as_bytes() {
        fail("validation.json is stale");
    }
    if fs::read(&markdown_path).expect("read VALIDATION.md") != markdown.as_bytes() {
        fail("VALIDATION.md is stale");
    }
    println!("root-finding baseline validation: PASS");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline(arm: &str) -> BaselineRow {
        BaselineRow {
            schema: "rootfind.baseline/v1".to_owned(),
            family: "prime".to_owned(),
            size: Size {
                bits: Some(32),
                p: Some(101),
                _r: Some(97),
                _n: None,
            },
            abscissae: 64,
            arm: arm.to_owned(),
            base_points: 128,
            targets: 256,
            rounds: 5,
            isolation: "none".to_owned(),
            found: 0,
            adds: 32_768,
            build_adds: 4_160,
            solve_s_median: 0.001,
            build_s_median: 0.001,
            ns_per_add_median: 1.0,
            ns_per_target_median: 1.0,
            ns_per_build_add_median: 1.0,
        }
    }

    fn probe() -> ProbeRow {
        ProbeRow {
            schema: "rootfind.floor-probe/v1".to_owned(),
            bits: 32,
            p: 101,
            abscissae: 64,
            targets: 2_048,
            table_keys: 4_096,
            filter_bytes: 8_192,
            abscissae_checked: 8_192,
            abscissa_mismatches: 0,
            exact_hits: 3,
            exact_hits_through_filter: 3,
            filter_positives: 5,
            ns_per_point_arithmetic: 9.0,
            ns_per_point_with_filter: 11.0,
        }
    }

    #[test]
    #[should_panic(expected = "fixture differs from default")]
    fn rejects_cross_fixture_comparison() {
        let default = baseline("rootfind-mitm");
        let mut candidate = baseline("rootfind-subtract");
        candidate.size.p = Some(103);
        require_same_fixture(&default, &candidate);
    }

    #[test]
    #[should_panic(expected = "zero probe count")]
    fn rejects_zero_probe_workload() {
        let mut row = probe();
        row.abscissae_checked = 0;
        validate_probe(&row, 101, 64);
    }

    #[test]
    #[should_panic(expected = "duplicate row")]
    fn rejects_duplicate_baseline_row() {
        let row = baseline("rootfind-mitm");
        index_baselines(&[row.clone(), row]);
    }

    #[test]
    #[should_panic(expected = "expected one default row, found 2")]
    fn rejects_duplicate_report_row() {
        let row = "| default | 1.0 ns | 2.0 ns | 3.0 ns |";
        let table = format!("{row}\n{row}\n");
        require_reported_row(&table, "default", [1.0, 2.0, 3.0]);
    }
}
