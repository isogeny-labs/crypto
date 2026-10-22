//! Native, source-linked normalization of four independently verified studies.
//! This is a reporting converter, not a collector or a substitute for source validators.

use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

type Row = BTreeMap<String, String>;

const GRID_FIELDS: &[&str] = &[
    "lead",
    "arm",
    "slice",
    "targets",
    "verified",
    "candidate_failures",
    "rho_failures",
    "S_per_M",
    "I_per_M",
    "native_GAE_per_logical",
    "candidate_GAE",
    "external_GAE",
    "rho_GAE",
    "candidate_over_external",
    "candidate_over_rho",
    "baseline_GAE",
    "candidate_over_baseline",
    "status",
    "source_grid",
];
const TARGET_FIELDS: &[&str] = &[
    "lead",
    "arm",
    "bits",
    "p",
    "target_index",
    "workload_id",
    "source_verified",
    "candidate_dlp_verified",
    "rho_verified",
    "candidate_GAE",
    "rho_GAE",
    "candidate_over_rho",
    "coordinate_nonzero_matches",
    "coordinate_zero_outputs",
    "coordinate_failures",
    "signed_recover_verified",
    "recover_method",
    "recover_record",
    "status",
    "source_record",
    "failure",
    "solver_mode",
    "order",
];
const PENCIL: &[(u64, u32)] = &[
    (739, 16),
    (1151, 16),
    (1187, 16),
    (3371, 20),
    (3527, 20),
    (3931, 20),
    (11731, 24),
    (11887, 24),
    (12143, 24),
    (48383, 28),
];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .to_path_buf()
}
fn s(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::Bool(true) => "True".into(),
        Value::Bool(false) => "False".into(),
        Value::String(x) => x.clone(),
        _ => v.to_string(),
    }
}
fn js<'a>(v: &'a Value, key: &str) -> &'a Value {
    v.get(key).unwrap_or(&Value::Null)
}
fn put(row: &mut Row, key: &str, value: impl ToString) {
    row.insert(key.into(), value.to_string());
}
fn row(pairs: &[(&str, String)]) -> Row {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect()
}
fn val<'a>(r: &'a Row, key: &str) -> &'a str {
    r.get(key).map(String::as_str).unwrap_or("")
}
fn num(s: &str) -> f64 {
    s.parse().unwrap_or_else(|_| panic!("invalid number: {s}"))
}
fn close(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0),
        "{a} != {b}"
    );
}

fn csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut value = String::new();
    let mut quoted = false;
    let mut chars = line.trim_end_matches('\r').chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                value.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                fields.push(std::mem::take(&mut value));
            }
            _ => value.push(ch),
        }
    }
    assert!(!quoted, "unterminated quoted CSV field");
    fields.push(value);
    fields
}
fn read_csv(path: &Path) -> Vec<Row> {
    let mut iter =
        BufReader::new(File::open(path).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
            .lines();
    let head = csv_line(&iter.next().unwrap().unwrap());
    iter.map(|line| {
        let values = csv_line(&line.unwrap());
        assert_eq!(values.len(), head.len(), "{}", path.display());
        head.iter().cloned().zip(values).collect()
    })
    .collect()
}
fn escape_csv(x: &str) -> String {
    if x.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", x.replace('"', "\"\""))
    } else {
        x.into()
    }
}
fn write_csv(path: &Path, fields: &[&str], rows: &[Row]) {
    let mut out = File::create(path).unwrap();
    writeln!(out, "{}", fields.join(",")).unwrap();
    for row in rows {
        let values: Vec<_> = fields
            .iter()
            .map(|field| escape_csv(val(row, field)))
            .collect();
        writeln!(out, "{}", values.join(",")).unwrap();
    }
}
fn read_jsonl(path: &Path) -> Vec<Value> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .collect()
}

fn price_rows(repo: &Path) -> Vec<Row> {
    let mut out = Vec::new();
    let sources = [
        (
            "factor_base",
            "full_16",
            "research/four_followups_20261021/results/matched/price-grid.csv",
            "fb1-j16",
        ),
        (
            "factor_base",
            "full_8",
            "research/four_followups_20261021/results/matched/price-grid.csv",
            "fb1-j8",
        ),
        (
            "factor_base",
            "full_32",
            "research/four_followups_20261021/results/matched/price-grid.csv",
            "fb1-j32",
        ),
        (
            "factor_base",
            "half_16",
            "research/four_followups_20261021/results/matched/price-grid.csv",
            "fb2-j16",
        ),
        (
            "factor_base",
            "quarter_16",
            "research/four_followups_20261021/results/matched/price-grid.csv",
            "fb4-j16",
        ),
        (
            "projective_law",
            "wrapper",
            "research/projective_law_followup_20261022/results/price-grid-projective.csv",
            "",
        ),
        (
            "projective_law",
            "retained_state",
            "research/projective_law_followup_20261022/results/price-grid-state.csv",
            "",
        ),
    ];
    for (lead, arm, source, source_arm) in sources {
        for old in read_csv(&repo.join(source)) {
            if !source_arm.is_empty() && val(&old, "arm") != source_arm {
                continue;
            }
            let candidate = num(val(&old, "candidate_GAE"));
            close(
                candidate / num(val(&old, "rho_GAE")),
                num(val(&old, "candidate_over_rho")),
            );
            close(
                candidate / num(val(&old, "external_GAE")),
                num(val(&old, "candidate_over_external")),
            );
            let mut r = Row::new();
            for key in GRID_FIELDS {
                put(&mut r, key, val(&old, key));
            }
            for key in [
                "targets",
                "verified",
                "S_per_M",
                "I_per_M",
                "native_GAE_per_logical",
                "candidate_GAE",
                "external_GAE",
                "rho_GAE",
                "candidate_over_external",
                "candidate_over_rho",
            ] {
                put(&mut r, key, val(&old, key));
            }
            put(
                &mut r,
                "candidate_failures",
                if val(&old, "candidate_failures").is_empty() {
                    val(&old, "failures")
                } else {
                    val(&old, "candidate_failures")
                },
            );
            put(
                &mut r,
                "rho_failures",
                if val(&old, "rho_failures").is_empty() {
                    val(&old, "failures")
                } else {
                    val(&old, "rho_failures")
                },
            );
            for (k, v) in [
                ("lead", lead),
                ("arm", arm),
                ("source_grid", source),
                ("status", "priced_source_verified"),
            ] {
                put(&mut r, k, v);
            }
            out.push(r);
        }
    }
    for (p, _bits) in PENCIL {
        let source = format!("research/pencil_sparse_20261022/results/p{p}-peel-vs-online.csv");
        for old in read_csv(&repo.join(&source)) {
            let base = num(val(&old, "baseline_GAE"));
            let cand = num(val(&old, "candidate_GAE"));
            let rho = num(val(&old, "rho_GAE"));
            close(cand / base, num(val(&old, "candidate_over_baseline")));
            close(base / rho, num(val(&old, "baseline_over_rho")));
            close(cand / rho, num(val(&old, "candidate_over_rho")));
            for (arm, price, over_rho, over_base) in [
                ("online_identity", "baseline_GAE", "baseline_over_rho", ""),
                (
                    "degree_peel",
                    "candidate_GAE",
                    "candidate_over_rho",
                    "candidate_over_baseline",
                ),
            ] {
                let mut r = Row::new();
                for key in [
                    "targets",
                    "S_per_M",
                    "I_per_M",
                    "native_GAE_per_logical",
                    "rho_GAE",
                    "baseline_GAE",
                ] {
                    put(&mut r, key, val(&old, key));
                }
                put(&mut r, "verified", val(&old, "targets"));
                put(&mut r, "candidate_failures", "0");
                put(&mut r, "rho_failures", "0");
                put(&mut r, "lead", "pencil_collector");
                put(&mut r, "arm", arm);
                put(&mut r, "slice", format!("p{p}"));
                put(&mut r, "candidate_GAE", val(&old, price));
                put(&mut r, "candidate_over_rho", val(&old, over_rho));
                put(
                    &mut r,
                    "candidate_over_baseline",
                    if over_base.is_empty() {
                        "1"
                    } else {
                        val(&old, over_base)
                    },
                );
                put(&mut r, "status", "priced_source_verified");
                put(&mut r, "source_grid", &source);
                out.push(r);
            }
        }
    }
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        out.push(row(&[
            ("lead", "kummer_walk".into()),
            ("arm", "coordinate_identity_gate".into()),
            ("slice", bits.to_string()),
            ("targets", "180".into()),
            ("verified", "180".into()),
            ("candidate_failures", "0".into()),
            (
                "status",
                "coordinate_and_recover_gate_no_dlp_collector".into(),
            ),
            (
                "source_grid",
                format!("research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl"),
            ),
        ]));
    }
    assert_eq!(out.len(), 2743);
    out
}

fn priced_target(source: &Value, lead: &str, arm: &str, p_key: &str, path: &str) -> Row {
    let verified = if source.get("candidate_verified").is_some() {
        js(source, "candidate_verified")
    } else {
        js(source, "verified")
    };
    row(&[
        ("lead", lead.into()),
        ("arm", arm.into()),
        ("bits", s(js(source, "bits"))),
        ("p", s(js(source, p_key))),
        ("target_index", s(js(source, "target_index"))),
        ("workload_id", s(js(source, "workload_id"))),
        ("source_verified", s(verified)),
        ("candidate_dlp_verified", s(verified)),
        (
            "rho_verified",
            if source.get("rho_verified").is_some() {
                s(js(source, "rho_verified"))
            } else {
                s(verified)
            },
        ),
        ("candidate_GAE", s(js(source, "candidate_GAE"))),
        ("rho_GAE", s(js(source, "rho_GAE"))),
        ("candidate_over_rho", s(js(source, "candidate_over_rho"))),
        ("status", "priced_source_verified".into()),
        (
            "source_record",
            if source.get("raw_record").is_some() {
                s(js(source, "raw_record"))
            } else {
                path.into()
            },
        ),
    ])
}

fn target_rows(repo: &Path) -> Vec<Row> {
    let mut out = Vec::new();
    let factor = "research/four_followups_20261021/results/matched/targets.jsonl";
    for source in read_jsonl(&repo.join(factor)) {
        let arm = match js(&source, "arm").as_str().unwrap() {
            "fb1-j16" => "full_16",
            "fb1-j8" => "full_8",
            "fb1-j32" => "full_32",
            "fb2-j16" => "half_16",
            "fb4-j16" => "quarter_16",
            _ => panic!("unknown factor arm"),
        };
        out.push(priced_target(
            &source,
            "factor_base",
            arm,
            "curve_p",
            factor,
        ));
    }
    for (arm, path) in [
        (
            "wrapper",
            "research/projective_law_followup_20261022/results/targets-projective.jsonl",
        ),
        (
            "retained_state",
            "research/projective_law_followup_20261022/results/targets-state.jsonl",
        ),
    ] {
        for source in read_jsonl(&repo.join(path)) {
            out.push(priced_target(&source, "projective_law", arm, "p", path));
        }
    }
    let pencil = "research/pencil_sparse_20261022/results/targets.jsonl";
    for source in read_jsonl(&repo.join(pencil)) {
        let verified = js(&source, "candidate_verified").as_bool().unwrap();
        let priced = js(&source, "representative");
        let mut r = row(&[
            ("lead", "pencil_collector".into()),
            (
                "arm",
                format!(
                    "{}_{}",
                    s(js(&source, "solver_mode")),
                    s(js(&source, "order"))
                ),
            ),
            ("bits", s(js(&source, "bits"))),
            ("p", s(js(&source, "curve_p"))),
            ("target_index", s(js(&source, "target_index"))),
            ("workload_id", s(js(&source, "workload_id"))),
            ("source_verified", s(js(&source, "rho_verified"))),
            (
                "candidate_dlp_verified",
                s(js(&source, "candidate_verified")),
            ),
            ("rho_verified", s(js(&source, "rho_verified"))),
            ("candidate_GAE", s(js(priced, "pencil_GAE"))),
            ("rho_GAE", s(js(priced, "rho_GAE"))),
            ("candidate_over_rho", s(js(priced, "pencil_over_rho"))),
            (
                "status",
                if verified {
                    "priced_source_verified".into()
                } else {
                    "incomplete_cap_unpriced".into()
                },
            ),
            ("source_record", s(js(&source, "raw_path"))),
            ("failure", s(js(&source, "failure"))),
            ("solver_mode", s(js(&source, "solver_mode"))),
            ("order", s(js(&source, "order"))),
        ]);
        if !verified {
            assert!(val(&r, "candidate_GAE").is_empty());
        }
        out.push(std::mem::take(&mut r));
    }
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        let path = format!("research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl");
        let recover_path =
            format!("research/kummer_recover_20261022/results/recover-target-{bits}.jsonl");
        let recover: BTreeMap<(String, String), Value> = read_jsonl(&repo.join(&recover_path))
            .into_iter()
            .map(|v| ((s(js(&v, "p")), s(js(&v, "target_index"))), v))
            .collect();
        for source in read_jsonl(&repo.join(&path)) {
            let signed = &recover[&(s(js(&source, "p")), s(js(&source, "target_index")))];
            assert_eq!(js(signed, "workload_id"), js(&source, "workload_id"));
            let cases = js(&source, "cases").as_array().unwrap();
            let passed = cases
                .iter()
                .filter(|v| js(v, "coordinate_status") == "pass")
                .count();
            let zero = cases
                .iter()
                .filter(|v| js(v, "coordinate_status") == "zero_output")
                .count();
            let failures = cases.len() - passed - zero;
            assert_eq!(
                cases.len(),
                js(&source, "case_count").as_u64().unwrap() as usize
            );
            out.push(row(&[
                ("lead", "kummer_walk".into()),
                ("arm", "coordinate_identity_gate".into()),
                ("bits", bits.to_string()),
                ("p", s(js(&source, "p"))),
                ("target_index", s(js(&source, "target_index"))),
                ("workload_id", s(js(&source, "workload_id"))),
                (
                    "source_verified",
                    if js(&source, "all_source_checks") == true
                        && js(&source, "source_scalar_verified") == true
                    {
                        "True".into()
                    } else {
                        "False".into()
                    },
                ),
                ("coordinate_nonzero_matches", passed.to_string()),
                ("coordinate_zero_outputs", zero.to_string()),
                ("coordinate_failures", failures.to_string()),
                (
                    "signed_recover_verified",
                    if js(signed, "correct") == true && js(signed, "source_scalar_verified") == true
                    {
                        "True".into()
                    } else {
                        "False".into()
                    },
                ),
                ("recover_method", s(js(signed, "status"))),
                ("recover_record", recover_path.clone()),
                (
                    "status",
                    "coordinate_and_recover_gate_no_dlp_collector".into(),
                ),
                ("source_record", path.clone()),
            ]));
        }
    }
    assert_eq!(out.len(), 11528);
    out
}

fn main() {
    let repo = root();
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out = here.join("results");
    fs::create_dir_all(&out).unwrap();
    let grids = price_rows(&repo);
    let targets = target_rows(&repo);
    let peel: Vec<_> = grids
        .iter()
        .filter(|r| val(r, "lead") == "pencil_collector" && val(r, "arm") == "degree_peel")
        .collect();
    assert_eq!(peel.len(), 360);
    assert_eq!(
        peel.iter()
            .filter(|r| num(val(r, "candidate_over_baseline")) < 1.0)
            .count(),
        240
    );
    assert_eq!(
        peel.iter()
            .filter(|r| num(val(r, "candidate_over_baseline")) > 1.0)
            .count(),
        120
    );
    assert_eq!(
        peel.iter()
            .filter(|r| num(val(r, "candidate_over_rho")) < 1.0)
            .count(),
        0
    );
    let mut keys: BTreeMap<(String, String, String), BTreeSet<String>> = BTreeMap::new();
    let mut pencil_verified = BTreeSet::new();
    let mut pencil_present = BTreeSet::new();
    let mut pencil_rows = 0;
    let mut pencil_complete_rows = 0;
    let mut pencil_cap_rows = 0;
    let mut zero = 0;
    let mut coordinate_fail = 0;
    for r in &targets {
        let key = (
            val(r, "bits").to_string(),
            val(r, "p").to_string(),
            val(r, "target_index").to_string(),
        );
        keys.entry(key.clone())
            .or_default()
            .insert(val(r, "workload_id").into());
        assert_eq!(val(r, "source_verified").to_lowercase(), "true");
        if val(r, "lead") == "pencil_collector" {
            pencil_rows += 1;
            pencil_present.insert(key.clone());
            if val(r, "candidate_dlp_verified") == "True" {
                pencil_verified.insert(key);
                pencil_complete_rows += 1;
            } else {
                assert_eq!(val(r, "status"), "incomplete_cap_unpriced");
                pencil_cap_rows += 1;
            }
        }
        if val(r, "lead") == "kummer_walk" {
            zero += num(val(r, "coordinate_zero_outputs")) as usize;
            coordinate_fail += num(val(r, "coordinate_failures")) as usize;
            assert_eq!(val(r, "signed_recover_verified").to_lowercase(), "true");
        }
        if !val(r, "candidate_GAE").is_empty() {
            close(
                num(val(r, "candidate_GAE")) / num(val(r, "rho_GAE")),
                num(val(r, "candidate_over_rho")),
            );
        }
    }
    assert_eq!(keys.len(), 1260);
    assert!(keys.values().all(|v| v.len() == 1));
    assert_eq!(
        (pencil_rows, pencil_present.len(), pencil_verified.len()),
        (1448, 544, 543)
    );
    assert_eq!((pencil_complete_rows, pencil_cap_rows), (1446, 2));
    assert_eq!((zero, coordinate_fail), (7, 0));
    write_csv(&out.join("standard-cost-grid.csv"), GRID_FIELDS, &grids);
    write_csv(&out.join("standard-targets.csv"), TARGET_FIELDS, &targets);
    let mut representative = Vec::new();
    for r in &grids {
        if val(r, "status") != "priced_source_verified" {
            continue;
        }
        if (
            val(r, "S_per_M"),
            val(r, "I_per_M"),
            val(r, "native_GAE_per_logical"),
        ) != ("0.5", "10", "0.1")
        {
            continue;
        }
        representative.push(json!({
            "lead": val(r, "lead"), "arm": val(r, "arm"), "slice": val(r, "slice"),
            "targets": num(val(r, "targets")) as usize,
            "candidate_over_rho": num(val(r, "candidate_over_rho")),
            "candidate_over_external": if val(r, "candidate_over_external").is_empty() { Value::Null } else { json!(num(val(r, "candidate_over_external"))) },
            "candidate_GAE": num(val(r, "candidate_GAE")),
            "rho_GAE": num(val(r, "rho_GAE")), "source_grid": val(r, "source_grid"),
        }));
    }
    let summary = json!({
        "schema": "four-followups-review/v1", "price_unit": "GAE",
        "representative_price": {"S_per_M": 0.5, "I_per_M": 10, "native_GAE_per_logical": 0.1},
        "scope": "Frozen 21-model, 1,260-public-target cohort; 60 targets/model; seven order sizes.",
        "rows": grids.len(), "representative_rows": representative,
        "target_rows": targets.len(), "distinct_target_keys": keys.len(),
        "requirements": [
            {"lead":"factor_base","status":"verified_complete_for_five_frozen_arms","candidate_targets_per_arm":1260,"rho_targets_per_arm":1260,"source":"research/four_followups_20261021/REPORT.md"},
            {"lead":"projective_law","status":"verified_complete_for_two_frozen_arms","candidate_targets_per_arm":1260,"rho_targets_per_arm":1260,"source":"research/projective_law_followup_20261022/REPORT.md"},
            {"lead":"pencil_collector","status":"partial","candidate_distinct_targets":543,"rho_distinct_targets":544,"failed_distinct_targets":1,"open_distinct_targets":717,"remaining":"717 original target workloads have no completed candidate solve; one p=186379 target has two cap failures and source-verified rho.","source":"research/pencil_sparse_20261022/REPORT.md"},
            {"lead":"kummer_walk","status":"partial","coordinate_label_cases":8820,"nonzero_matches":8813,"zero_outputs":7,"wrong_nonzero_outputs":0,"candidate_dlp_targets":0,"candidate_over_rho":null,"signed_recoveries_verified":1260,"zero_coordinate_repairs_verified":175,"remaining":"A closed labeled DLP collector with complete cost.","source":"research/kummer_recover_20261022/REPORT.md"},
        ],
    });
    fs::write(
        out.join("summary.json"),
        format!("{}\n", serde_json::to_string_pretty(&summary).unwrap()),
    )
    .unwrap();
    draw_graphs(here, &grids);
    println!(
        "wrote {} normalized price rows and {} target rows",
        grids.len(),
        targets.len()
    );
}

fn draw_graphs(here: &Path, grids: &[Row]) {
    let arms = [
        ("full_16", "#1764aa"),
        ("full_8", "#16a0a0"),
        ("full_32", "#3946a5"),
        ("half_16", "#b2691b"),
        ("quarter_16", "#a03543"),
        ("wrapper", "#75832a"),
        ("retained_state", "#8647a3"),
    ];
    let x = |bits: f64| 90.0 + (bits - 16.0) / 24.0 * 600.0;
    let y = |ratio: f64| 465.0 - ratio.log10() / 2.0 * 425.0;
    let mut svg = vec![r#"<svg xmlns="http://www.w3.org/2000/svg" width="920" height="560" viewBox="0 0 920 560">"#.to_string(),
        r#"<rect width="100%" height="100%" fill="white"/>"#.into(),
        r#"<text x="90" y="25" font-size="17" font-family="sans-serif">Cold candidate / matched rho, representative price</text>"#.into()];
    for ratio in [1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0] {
        let yy = y(ratio);
        svg.push(format!(
            r##"<line x1="90" y1="{yy:.1}" x2="690" y2="{yy:.1}" stroke="#d8dfe7"/>"##
        ));
        svg.push(format!(r#"<text x="78" y="{:.1}" text-anchor="end" font-size="12" font-family="sans-serif">{ratio}</text>"#, yy+4.0));
    }
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        let xx = x(bits as f64);
        svg.push(format!(
            r##"<line x1="{xx:.1}" y1="40" x2="{xx:.1}" y2="465" stroke="#e9edf1"/>"##
        ));
        svg.push(format!(r#"<text x="{xx:.1}" y="487" text-anchor="middle" font-size="12" font-family="sans-serif">{bits}</text>"#));
    }
    for (i, (arm, color)) in arms.iter().enumerate() {
        let mut data: Vec<_> = grids
            .iter()
            .filter(|r| {
                val(r, "arm") == *arm
                    && val(r, "slice") != "all"
                    && val(r, "S_per_M") == "0.5"
                    && val(r, "I_per_M") == "10"
                    && val(r, "native_GAE_per_logical") == "0.1"
            })
            .collect();
        data.sort_by_key(|r| val(r, "slice").parse::<u32>().unwrap());
        assert_eq!(data.len(), 7);
        let points = data
            .iter()
            .map(|r| {
                format!(
                    "{:.1},{:.1}",
                    x(num(val(r, "slice"))),
                    y(num(val(r, "candidate_over_rho")))
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        svg.push(format!(
            r#"<polyline points="{points}" fill="none" stroke="{color}" stroke-width="2.4"/>"#
        ));
        for r in data {
            svg.push(format!(
                r#"<circle cx="{:.1}" cy="{:.1}" r="3.5" fill="{color}"/>"#,
                x(num(val(r, "slice"))),
                y(num(val(r, "candidate_over_rho")))
            ));
        }
        svg.push(format!(
            r#"<line x1="715" y1="{}" x2="743" y2="{}" stroke="{color}" stroke-width="3"/>"#,
            75 + i * 27,
            75 + i * 27
        ));
        svg.push(format!(
            r#"<text x="753" y="{}" font-size="13" font-family="sans-serif">{arm}</text>"#,
            79 + i * 27
        ));
    }
    svg.push(r#"<text x="390" y="518" text-anchor="middle" font-size="13" font-family="sans-serif">Order bits</text>"#.into());
    svg.push(r#"<text x="90" y="544" font-size="11" font-family="sans-serif">Kummer DLP has no priced collector and is intentionally absent. Source: standard-cost-grid.csv.</text>"#.into());
    svg.push("</svg>".into());
    fs::write(
        here.join("cost-ratios.svg"),
        format!("{}\n", svg.join("\n")),
    )
    .unwrap();

    let py = |ratio: f64| 270.0 - (ratio - 0.2) * 215.0;
    let pencil: Vec<_> = grids
        .iter()
        .filter(|r| val(r, "lead") == "pencil_collector" && val(r, "arm") == "degree_peel")
        .collect();
    assert_eq!(pencil.len(), 360);
    let mut psvg = vec![r#"<svg xmlns="http://www.w3.org/2000/svg" width="920" height="355" viewBox="0 0 920 355">"#.to_string(),
        r#"<rect width="100%" height="100%" fill="white"/>"#.into(),
        r#"<text x="100" y="27" font-family="sans-serif" font-size="17">Degree-peel / matched online: 36-cell range and native price</text>"#.into()];
    for ratio in [0.2, 0.5, 1.0, 1.2] {
        let yy = py(ratio);
        psvg.push(format!(
            r##"<line x1="100" y1="{yy:.1}" x2="815" y2="{yy:.1}" stroke="#d8dfe7"/>"##
        ));
        let label = if ratio == 1.0 {
            "1.0".to_string()
        } else {
            ratio.to_string()
        };
        psvg.push(format!(r#"<text x="88" y="{:.1}" text-anchor="end" font-family="sans-serif" font-size="12">{label}</text>"#, yy+4.0));
    }
    for (i, (p, _)) in PENCIL.iter().enumerate() {
        let slice = format!("p{p}");
        let data: Vec<_> = pencil.iter().filter(|r| val(r, "slice") == slice).collect();
        assert_eq!(data.len(), 36);
        let ratios: Vec<_> = data
            .iter()
            .map(|r| num(val(r, "candidate_over_baseline")))
            .collect();
        let low = ratios.iter().copied().fold(f64::INFINITY, f64::min);
        let high = ratios.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let native = data
            .iter()
            .find(|r| {
                val(r, "S_per_M") == "0.5"
                    && val(r, "I_per_M") == "10"
                    && val(r, "native_GAE_per_logical") == "0.1"
            })
            .unwrap();
        let xx = 100 + i * 76;
        psvg.push(format!(r##"<line x1="{xx}" y1="{:.1}" x2="{xx}" y2="{:.1}" stroke="#426b9d" stroke-width="4"/>"##, py(low), py(high)));
        psvg.push(format!(
            r##"<circle cx="{xx}" cy="{:.1}" r="5" fill="#bd5b24"/>"##,
            py(num(val(native, "candidate_over_baseline")))
        ));
        psvg.push(format!(r#"<text x="{xx}" y="304" text-anchor="middle" font-family="sans-serif" font-size="11">{p}</text>"#));
    }
    psvg.push(r#"<text x="460" y="329" text-anchor="middle" font-family="sans-serif" font-size="12">Curve prime p; last column is one target, others are 60 targets</text>"#.into());
    psvg.push("</svg>".into());
    fs::write(
        here.join("pencil-ratios.svg"),
        format!("{}\n", psvg.join("\n")),
    )
    .unwrap();
}
