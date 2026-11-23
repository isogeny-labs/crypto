//! Scoring arithmetic, tier enforcement, schema validation and the parsers:
//! the 33 tests of the tool's first version (test_optlevel.py), one for one.
//! No PMU, GPU or compiler is needed: measurements are synthetic documents.
use optlevel::cmd::{self, SCHEMA_TEXT};
use optlevel::score::{self as S, Extras, Interp};
use optlevel::{plot, run};
use serde_json::{json, Map, Value};

fn schema() -> Value {
    serde_json::from_str(SCHEMA_TEXT).unwrap()
}
fn st(v: f64) -> Value {
    json!({"median": v, "min": v, "max": v, "n": 5})
}
fn st3(v: f64, lo: f64, hi: f64) -> Value {
    json!({"median": v, "min": lo, "max": hi, "n": 5})
}
fn calib() -> Value {
    json!({"schema": S::CALIBRATION_SCHEMA, "created_utc": "2026-11-17T00:00:00Z", "flags": [],
           "host": {"fingerprint": "ffffffffffffffff", "cpu_model": "test cpu", "logical_cpus": 4, "caches_bytes": {"L2": 1 << 20, "L3": 32 << 20}},
           "compute": {"modmuladd_p31": {"total_ops_per_s": {"1": st(2e9), "4": st(8e9)}, "scalar_ops_per_s": 5e8, "simd_ops_per_s": 2e9},
                       "ptr_hop": {"total_ops_per_s": {"1": st(5e8), "4": st(2e9)}, "scalar_ops_per_s": 5e8, "simd_ops_per_s": 5e8},
                       "fma_f64": {"total_ops_per_s": {"1": st(4e10), "4": st(1.6e11)}, "scalar_ops_per_s": 2e9, "simd_ops_per_s": 4e10}},
           "memory": {"bandwidth_bytes_per_s": {"1": st(1e10), "4": st(2e10)}, "llc_bytes": 32 << 20, "latency_ns": st(100.0)}})
}
struct M {
    rn: Option<f64>,
    bytes_per_op: Option<f64>,
    tier: &'static str,
    ws: Option<i64>,
}
impl Default for M {
    fn default() -> Self {
        M {
            rn: None,
            bytes_per_op: None,
            tier: "T-timing",
            ws: None,
        }
    }
}
fn meas(cls: &str, r1: f64, m: M) -> Value {
    let noise = 0.02;
    let mut pts = json!({"1": {"ops_per_s": st3(r1, r1 * 0.98, r1 * 1.01), "ops": st(1e9)}});
    if let Some(rn) = m.rn {
        pts["4"] = json!({"ops_per_s": st3(rn, rn * 0.97, rn * 1.02), "ops": st(4e9)});
    }
    json!({"schema": S::MEASUREMENT_SCHEMA, "tier": m.tier, "op_class": cls, "host": calib()["host"], "command": ["k"],
           "ops_source": "test", "points": pts, "label": cls, "created_utc": "2026-11-17T00:00:01Z",
           "noise_floor": {"aa_ratio_deviation": noise, "rel_range": noise, "value": noise, "pairs": 5, "sufficient": true},
           "declared_bytes_per_op": m.bytes_per_op, "working_set_bytes": m.ws, "flags": [], "conditions": null})
}
fn build(c: &Value, m: &Value) -> Value {
    S::build_record(c, m, Extras::default()).unwrap()
}
fn build_diag(c: &Value, m: &Value, diag: Value) -> Value {
    S::build_record(
        c,
        m,
        Extras {
            extra_diag: diag.as_object().cloned().unwrap_or_default(),
            ..Default::default()
        },
    )
    .unwrap()
}
/// Every record a test builds must pass the schema and the tier rules.
fn check(rec: Value) -> Value {
    assert_eq!(S::validate(&rec, &schema()), Vec::<String>::new());
    S::enforce_tier(&rec).unwrap();
    rec
}
fn close(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-7 * b.abs().max(1e-300) + 1e-12,
        "{a} != {b}"
    );
}
fn num(v: &Value) -> f64 {
    v.as_f64().unwrap_or_else(|| panic!("not a number: {v}"))
}
fn text(v: &Value) -> &str {
    v.as_str().unwrap_or_else(|| panic!("not a string: {v}"))
}
fn has_flag(r: &Value, flag: &str) -> bool {
    r["flags"].as_array().unwrap().iter().any(|x| x == flag)
}

// ---- Arithmetic (8) ------------------------------------------------------------
#[test]
fn stats() {
    assert_eq!(
        S::stats(&[3.0, 1.0, 2.0]),
        json!({"median": 2.0, "min": 1.0, "max": 3.0, "n": 3})
    );
    assert_eq!(S::stats(&[1.0, 2.0, 3.0, 4.0])["median"], 2.5);
    assert!(S::stats_opt([None]).is_null());
}

#[test]
fn interp() {
    let pts = [(1, 10.0), (4, 34.0)];
    assert_eq!(
        S::interp_scale(&pts, 4).unwrap(),
        (
            34.0,
            Interp {
                interpolated: false,
                extrapolated: false
            }
        )
    );
    let (v, meta) = S::interp_scale(&pts, 2).unwrap();
    close(v, 18.0);
    assert!(meta.interpolated);
    let (v, meta) = S::interp_scale(&pts, 8).unwrap();
    assert_eq!(v, 34.0); // never extrapolates a ceiling upward
    assert!(meta.extrapolated);
}

#[test]
fn roofline() {
    let r = S::roofline(2e9, 1e10, Some(24.0)).unwrap(); // memory roof 4.17e8 < peak
    assert_eq!(r["binding"], "memory");
    close(num(&r["bound_ops_per_s"]), 1e10 / 24.0);
    close(num(&r["next_roof_ratio"]), 2e9 / (1e10 / 24.0));
    let r = S::roofline(2e9, 1e10, Some(1.0)).unwrap(); // memory roof 1e10 > peak
    assert_eq!(
        (text(&r["binding"]), num(&r["bound_ops_per_s"])),
        ("compute", 2e9)
    );
    let r = S::roofline(2e9, 1e10, None).unwrap();
    assert_eq!(r["binding"], "compute (traffic unknown)");
    assert!(r["memory_roof_ops_per_s"].is_null());
    assert_eq!(
        S::roofline(2e9, 1e10, Some(0.0)).unwrap()["binding"],
        "compute"
    );
    assert!(S::roofline(0.0, 1e10, Some(1.0)).is_err());
}

#[test]
fn attainment_interval() {
    let a = S::attainment(&st3(50.0, 40.0, 60.0), 100.0, Some(80.0));
    assert_eq!(
        (num(&a["value"]), num(&a["low"]), num(&a["high"])),
        (0.5, 0.4, 0.75)
    );
}

#[test]
fn noise_floor() {
    let nf = S::noise_floor(
        &[100.0, 101.0, 99.0, 100.0, 100.0],
        &[102.0, 103.0, 101.0, 102.0, 102.0],
    );
    close(
        num(&nf["aa_ratio_deviation"]),
        (100.0f64 / 102.0 - 1.0).abs(),
    );
    close(num(&nf["rel_range"]), (103.0 - 99.0) / 101.0);
    assert_eq!(
        num(&nf["value"]),
        num(&nf["aa_ratio_deviation"]).max(num(&nf["rel_range"]))
    );
    assert_eq!(nf["sufficient"], true);
    assert_eq!(
        S::noise_floor(&[1.0, 1.0], &[1.0, 1.0])["sufficient"],
        false
    );
}

#[test]
fn letters() {
    let got: Vec<&str> = [0.95, 0.70, 0.69, 0.40, 0.2, 0.08, 0.01]
        .iter()
        .map(|x| S::letter(Some(*x)).unwrap())
        .collect();
    assert_eq!(got.join(""), "AABBCDE");
    assert_eq!(S::letter_range(Some(0.68), 0.05).unwrap(), "A/B");
    assert_eq!(S::letter_range(Some(0.55), 0.05).unwrap(), "B");
}

#[test]
fn parallel() {
    let p = S::parallel_block(100.0, 200.0, 4, 1000.0, 2000.0, Value::Null);
    close(num(&p["raw_efficiency"]), 0.5);
    close(num(&p["roofline_normalised_efficiency"]), 1.0); // the machine itself only scales 2x
    close(num(&p["machine_scaling"]), 0.5);
}

#[test]
fn grade_rule() {
    let g = S::summary_grade(Some(0.9), Some(0.4), 0.0, &[]);
    close(num(&g["score"]), 0.36f64.sqrt());
    assert_eq!(
        (text(&g["letter"]), text(&g["scope"])),
        ("B", "single+parallel")
    );
    assert_eq!(
        S::summary_grade(Some(0.9), None, 0.0, &[])["scope"],
        "single-thread only"
    );
    assert!(
        S::summary_grade(Some(0.9), Some(0.9), 0.0, &["too short".to_string()])["score"].is_null()
    );
}

// ---- Records (16) ----------------------------------------------------------------
#[test]
fn positive_control_near_compute_roof() {
    let r = check(build(
        &calib(),
        &meas(
            "modmuladd_p31",
            1.9e9,
            M {
                rn: Some(7.4e9),
                bytes_per_op: Some(12.0),
                ws: Some(16384),
                ..Default::default()
            },
        ),
    ));
    assert_eq!(r["roofline"]["single"]["binding"], "compute");
    assert_eq!(r["intensity"]["in_cache"], true);
    close(num(&r["attainment"]["single"]["value"]), 0.95);
    assert_eq!(r["grade"]["letter"], "A");
    assert!(text(&r["grade"]["limiting_factor"]).contains("near the roofline"));
    assert!(text(&r["grade"]["limiting_factor"]).contains("algorithmic"));
}

#[test]
fn scalar_control_is_core_bound() {
    let m = meas(
        "modmuladd_p31",
        5.2e8,
        M {
            rn: Some(2.0e9),
            bytes_per_op: Some(12.0),
            ws: Some(16384),
            ..Default::default()
        },
    );
    let r = check(build_diag(
        &calib(),
        &m,
        json!({"vector_share_static": 0.0}),
    ));
    close(num(&r["attainment"]["single"]["value"]), 0.26);
    assert!(text(&r["grade"]["limiting_factor"]).contains("not vectorized"));
    assert_eq!(r["grade"]["letter"], "C");
    let r2 = check(build(&calib(), &m));
    assert!(text(&r2["grade"]["limiting_factor"]).contains("inferred from timing"));
}

#[test]
fn stream_on_bandwidth_slope() {
    let r = check(build(
        &calib(),
        &meas(
            "fma_f64",
            4.0e8,
            M {
                rn: Some(7.9e8),
                bytes_per_op: Some(24.0),
                ws: Some(1 << 30),
                ..Default::default()
            },
        ),
    ));
    assert_eq!(r["roofline"]["single"]["binding"], "memory");
    close(
        num(&r["attainment"]["single"]["value"]),
        4.0e8 / (1e10 / 24.0),
    );
    close(num(&r["parallel"]["raw_efficiency"]), 7.9e8 / 1.6e9);
    assert!(num(&r["parallel"]["roofline_normalised_efficiency"]) > 0.95);
    assert_eq!(r["grade"]["letter"], "A");
    assert!(text(&r["grade"]["limiting_factor"]).contains("memory-bandwidth roof"));
}

#[test]
fn pointer_chase_is_latency_bound() {
    let r = check(build(
        &calib(),
        &meas(
            "ptr_hop",
            1.0e7,
            M {
                rn: Some(4.0e7),
                bytes_per_op: Some(64.0),
                ws: Some(1 << 30),
                ..Default::default()
            },
        ),
    ));
    assert!(num(&r["attainment"]["single"]["value"]) < 0.08);
    close(num(&r["diagnosis"]["latency_attainment"]), 1.0);
    assert!(text(&r["grade"]["limiting_factor"]).contains("latency bound"));
    assert_eq!(r["grade"]["letter"], "D"); // sqrt(0.064 * 0.128)
}

#[test]
fn parallel_limits() {
    let r = check(build(
        &calib(),
        &meas(
            "modmuladd_p31",
            1.9e9,
            M {
                rn: Some(2.4e9),
                bytes_per_op: Some(12.0),
                ws: Some(16384),
                ..Default::default()
            },
        ),
    ));
    assert!(text(&r["grade"]["limiting_factor"]).contains("parallel scaling"));
}

#[test]
fn contended_full_point_is_reported_but_not_graded() {
    let mut m = meas(
        "modmuladd_p31",
        1.9e9,
        M {
            rn: Some(4.0e9),
            bytes_per_op: Some(12.0),
            ws: Some(16384),
            ..Default::default()
        },
    );
    m["points"]["4"]["other_cpu_cores"] = st(2.0);
    let r = check(build(&calib(), &m));
    assert!(has_flag(&r, "FULL_POINT_CONTENDED"));
    assert_eq!(r["grade"]["scope"], "single-thread only");
    close(num(&r["grade"]["score"]), 0.95);
    close(num(&r["attainment"]["full"]["value"]), 0.5); // still reported
    assert!(!text(&r["grade"]["limiting_factor"]).contains("parallel scaling"));
}

#[test]
fn unknown_intensity_is_flagged() {
    let r = check(build(&calib(), &meas("fma_f64", 1e10, M::default())));
    assert!(has_flag(&r, "INTENSITY_UNKNOWN"));
    assert_eq!(
        r["roofline"]["single"]["binding"],
        "compute (traffic unknown)"
    );
    assert_eq!(r["grade"]["scope"], "single-thread only");
    assert!(r["parallel"].is_null());
}

#[test]
fn ceiling_exceeded_invalidates() {
    let r = check(build(
        &calib(),
        &meas(
            "modmuladd_p31",
            3.0e9,
            M {
                bytes_per_op: Some(0.0),
                ..Default::default()
            },
        ),
    ));
    assert!(has_flag(&r, "CEILING_EXCEEDED"));
    assert!(r["grade"]["score"].is_null());
    assert_eq!(r["grade"]["scope"], "invalid");
    assert_eq!(num(&r["attainment"]["single"]["value"]), 1.5); // reported, never clamped
}

#[test]
fn too_short_invalidates() {
    let mut m = meas(
        "modmuladd_p31",
        1e9,
        M {
            bytes_per_op: Some(0.0),
            ..Default::default()
        },
    );
    m["flags"] = json!(["TOO_SHORT"]);
    let r = check(build(&calib(), &m));
    assert!(r["grade"]["letter"].is_null());
}

#[test]
fn timing_tier_carries_no_pmu_values() {
    let mut m = meas(
        "modmuladd_p31",
        1e9,
        M {
            bytes_per_op: Some(0.0),
            ..Default::default()
        },
    );
    m["pmu"] = json!({"ipc": 2.5, "llc_misses_per_op": 0.1});
    m["memory_traffic"] = json!({"source": "llc-miss", "bytes_per_op": 6.4});
    let r = check(build(&calib(), &m));
    assert!(r["diagnosis"]["ipc"].is_null());
    assert!(r["diagnosis"]["llc_misses_per_op"].is_null());
    assert_eq!(r["intensity"]["source"], "declared");
    assert!(r["diagnosis"]["unavailable"].get("ipc").is_some());
    let mut bad = r.clone();
    bad["diagnosis"]["ipc"] = json!(2.5);
    assert!(S::enforce_tier(&bad).is_err());
    let mut bad = r.clone();
    bad["intensity"]["source"] = json!("llc-miss");
    assert!(S::enforce_tier(&bad).is_err());
}

#[test]
fn pmu_tier_uses_measured_traffic_and_topdown() {
    let mut m = meas(
        "modmuladd_p31",
        4e8,
        M {
            bytes_per_op: Some(12.0),
            tier: "T-pmu",
            ws: Some(1 << 30),
            ..Default::default()
        },
    );
    m["pmu"] = json!({"ipc": 0.8, "cycles_per_op": 7.0, "instructions_per_op": 5.6, "llc_misses_per_op": 0.01, "dtlb_misses_per_op": 0.0});
    m["memory_traffic"] = json!({"source": "llc-miss", "bytes_per_op": 0.64});
    let td = json!({"frontend_bound": 0.05, "bad_speculation": 0.05, "backend_bound": 0.6, "retiring": 0.3, "memory_bound": 0.1, "core_bound": 0.5});
    let r = check(build_diag(&calib(), &m, json!({"topdown": td})));
    assert_eq!(r["intensity"]["source"], "llc-miss");
    close(num(&r["intensity"]["declared_over_measured"]), 12.0 / 0.64);
    assert_eq!(num(&r["diagnosis"]["ipc"]), 0.8);
    assert!(text(&r["grade"]["limiting_factor"]).contains("core bound"));
    assert_eq!(r["grade"]["limiting_basis"], "pmu top-down");
}

#[test]
fn gpu_tiers() {
    let mut m = meas(
        "fma_f64",
        1e10,
        M {
            bytes_per_op: Some(0.0),
            tier: "T-gpu-timing",
            ..Default::default()
        },
    );
    m["gpu"] = json!({"nvidia_smi": {"samples": 3}});
    let r = check(build(&calib(), &m));
    assert_eq!(r["device"], "gpu");
    let mut bad = r.clone();
    bad["diagnosis"]["gpu"]["ncu"] = json!({"sm_throughput_pct": 90});
    assert!(S::enforce_tier(&bad).is_err());
    let mut cpu = check(build(
        &calib(),
        &meas(
            "fma_f64",
            1e10,
            M {
                bytes_per_op: Some(0.0),
                ..Default::default()
            },
        ),
    ));
    cpu["diagnosis"]["gpu"] = json!({});
    assert!(S::enforce_tier(&cpu).is_err());
}

#[test]
fn graded_record_needs_noise_floor() {
    let mut m = meas(
        "fma_f64",
        1e10,
        M {
            bytes_per_op: Some(0.0),
            ..Default::default()
        },
    );
    m["noise_floor"] = Value::Null;
    assert!(S::build_record(&calib(), &m, Extras::default()).is_err());
}

#[test]
fn host_mismatch_and_algorithmic_axis() {
    let mut m = meas(
        "fma_f64",
        3.9e10,
        M {
            bytes_per_op: Some(0.0),
            ..Default::default()
        },
    );
    m["host"]["fingerprint"] = json!("0000000000000000");
    let alg = json!({"counted_ops": 1e12, "best_known_ops": 1e9, "ratio": 1e3});
    let r = check(
        S::build_record(
            &calib(),
            &m,
            Extras {
                algorithmic: Some(alg),
                ..Default::default()
            },
        )
        .unwrap(),
    );
    assert!(has_flag(&r, "CALIBRATION_HOST_MISMATCH"));
    assert_eq!(r["grade"]["letter"], "A"); // a tuned bad algorithm still scores high
    assert_eq!(num(&r["algorithmic"]["ratio"]), 1e3);
}

#[test]
fn schema_rejects() {
    let r = build(
        &calib(),
        &meas(
            "fma_f64",
            1e10,
            M {
                bytes_per_op: Some(0.0),
                ..Default::default()
            },
        ),
    );
    let mutations: [fn(&mut Value); 4] = [
        |d| {
            d.as_object_mut().unwrap().remove("noise_floor");
        },
        |d| d["tier"] = json!("T-guess"),
        |d| d["grade"]["letter"] = json!("S"),
        |d| d["attainment"]["single"]["value"] = json!("high"),
    ];
    for mutate in mutations {
        let mut bad = r.clone();
        mutate(&mut bad);
        assert_ne!(S::validate(&bad, &schema()), Vec::<String>::new());
    }
}

#[test]
fn benchrec_units() {
    let r = build(
        &calib(),
        &meas(
            "fma_f64",
            1e10,
            M {
                rn: Some(3e10),
                bytes_per_op: Some(0.0),
                ..Default::default()
            },
        ),
    );
    let mut units: Vec<String> = S::benchrec_metrics(&r)
        .iter()
        .map(|m| text(&m["unit"]).to_string())
        .collect();
    units.sort();
    units.dedup();
    assert_eq!(units, ["ops_per_s", "ratio"]);
    // and the printed lines quote like a shell would need
    let lines = cmd::benchrec_lines(&r);
    assert_eq!(lines.len(), 4);
    assert!(lines[0].starts_with("python3 tools/benchrec.py metric --name optlevel:fma_f64:single:ops_per_s --value 10000000000.0 --unit ops_per_s --better higher --reference-kind none --note 'optlevel T-timing; not a speed claim' --n 5 "));
}

// ---- Parsers (9) --------------------------------------------------------------------
#[test]
fn ops_protocol() {
    let out = "noise\n{\"schema\":\"optlevel.ops/v1\",\"ops\":10,\"kernel_seconds\":1.0,\"bytes_declared\":80,\"op_class\":\"x\",\"working_set_bytes\":64}\n\
               {\"schema\":\"optlevel.ops/v1\",\"ops\":5,\"kernel_seconds\":0.5,\"bytes_declared\":40}\n{\"schema\":\"other\",\"ops\":99}\n";
    let o = run::parse_ops(out, "", None, None).unwrap();
    assert_eq!(
        (
            o.ops,
            o.kernel_seconds,
            o.bytes_declared,
            o.op_class.as_deref(),
            o.working_set_bytes
        ),
        (15, Some(1.5), Some(120.0), Some("x"), Some(64))
    );
    let re = regex::Regex::new(r"walk_steps=([\d,]+)").unwrap();
    assert_eq!(
        run::parse_ops("walk_steps=1,000\n", "walk_steps=24", None, Some(&re))
            .unwrap()
            .ops,
        1024
    );
    assert_eq!(run::parse_ops("", "", Some(7), None).unwrap().ops, 7);
    assert!(run::parse_ops("nothing", "", None, None).is_none());
}

#[test]
fn perf_csv() {
    let text = "# started\n1000,,cycles:u,5,100.00,,\n2500,,instructions:u,5,100.00,2.5,insn per cycle\n40,,cache-misses:u,5,62.50,,\n<not supported>,,dTLB-load-misses:u,0,100.00,,\n";
    let (c, mux, nc) = run::parse_perf_csv(text);
    assert_eq!(
        (num(&c["cycles"]), num(&c["instructions"])),
        (1000.0, 2500.0)
    );
    assert_eq!(
        (mux, nc),
        (
            vec!["cache-misses".to_string()],
            vec!["dTLB-load-misses".to_string()]
        )
    );
}

#[test]
fn pmu_summary() {
    let core = [
        json!({"ops": 100, "counters": {"cycles": 400.0, "instructions": 1000.0, "branches": 100.0, "branch-misses": 2.0}}),
    ];
    let mem = [
        json!({"ops": 100, "counters": {"cycles": 400.0, "instructions": 1000.0, "cache-misses": 50.0}, "multiplexed": ["cache-misses"]}),
    ];
    let (pmu, mux) = cmd::summarise_pmu(&core, &mem);
    assert_eq!(
        (
            num(&pmu["ipc"]),
            num(&pmu["cycles_per_op"]),
            num(&pmu["branch_miss_rate"]),
            num(&pmu["llc_misses_per_op"])
        ),
        (2.5, 4.0, 0.02, 0.5)
    );
    assert_eq!(mux, ["cache-misses"]);
    assert!(pmu["dtlb_misses_per_op"].is_null());
}

#[test]
fn topdown_normalise() {
    let doc = json!({"metrics": [{"metric": "tma_frontend_bound", "value": 12.0}, {"metric": "Bad Speculation", "value": 3.0},
                                 {"metric": "tma_backend_bound", "value": 55.0}, {"metric": "tma_retiring", "value": 30.0},
                                 {"metric": "tma_memory_bound", "value": 40.0}, {"metric": "tma_core_bound", "value": 15.0}], "multiplexed": []});
    let td = cmd::normalise_topdown(&doc);
    close(num(&td["backend_bound"]), 0.55);
    close(num(&td["bad_speculation"]), 0.03);
    assert!(S::topdown_limit(&td).unwrap().contains("memory bound"));
    assert!(S::topdown_limit(&json!({"frontend_bound": 0.1, "bad_speculation": 0.0, "backend_bound": 0.2, "retiring": 0.7})).unwrap().contains("retiring"));
}

#[test]
fn smi_samples() {
    let s = cmd::parse_smi_samples(
        "98, 40, 310.5, 71, 0x0000000000000000\n97, 41, 300.0, 72, 0x0000000000000004\nbad line\n",
    );
    assert_eq!(s["samples"], 2);
    assert_eq!(num(&s["utilization_gpu_pct"]["median"]), 97.5);
    assert_eq!(num(&s["throttled_sample_share"]), 0.5);
}

#[test]
fn scale_spec() {
    assert_eq!(
        cmd::parse_scale(Some("copies:4,1")).unwrap(),
        ("copies".to_string(), None, vec![1, 4])
    );
    assert_eq!(
        cmd::parse_scale(Some("env:RAYON_NUM_THREADS:1,2,4")).unwrap(),
        (
            "env".to_string(),
            Some("RAYON_NUM_THREADS".to_string()),
            vec![1, 2, 4]
        )
    );
    assert!(cmd::parse_scale(Some("threads:4")).is_err());
}

#[test]
fn gpu_path_refuses_without_gpu() {
    if let Err(why) = cmd::gpu_list() {
        assert!(!why.is_empty());
    }
}

/// Tag balance and attribute quoting: enough to catch a broken SVG without an XML dependency.
fn assert_wellformed(svg: &str) {
    let mut stack: Vec<String> = Vec::new();
    let mut rest = svg;
    while let Some(i) = rest.find('<') {
        assert!(!rest[..i].contains('>'), "stray > in text: {}", &rest[..i]);
        let j = i + rest[i..].find('>').expect("unterminated tag");
        let tag = &rest[i + 1..j];
        assert_eq!(
            tag.matches('"').count() % 2,
            0,
            "unbalanced quotes in <{tag}>"
        );
        if let Some(name) = tag.strip_prefix('/') {
            assert_eq!(stack.pop().as_deref(), Some(name), "mismatched </{name}>");
        } else if !tag.ends_with('/') {
            stack.push(tag.split_whitespace().next().unwrap().to_string());
        }
        rest = &rest[j + 1..];
    }
    assert!(stack.is_empty(), "unclosed: {stack:?}");
    let body = svg
        .replace("&amp;", "")
        .replace("&lt;", "")
        .replace("&gt;", "")
        .replace("&quot;", "")
        .replace("&#x27;", "");
    assert!(!body.contains('&'), "unescaped ampersand");
}

#[test]
fn plot_is_wellformed_and_two_surface() {
    let c = calib();
    let recs = [
        build(
            &c,
            &meas(
                "modmuladd_p31",
                1.9e9,
                M {
                    rn: Some(7.4e9),
                    bytes_per_op: Some(12.0),
                    ws: Some(16384),
                    ..Default::default()
                },
            ),
        ),
        build(
            &c,
            &meas(
                "fma_f64",
                4.0e8,
                M {
                    rn: Some(7.9e8),
                    bytes_per_op: Some(24.0),
                    ws: Some(1 << 30),
                    ..Default::default()
                },
            ),
        ),
        build(
            &c,
            &meas(
                "ptr_hop",
                1.0e7,
                M {
                    rn: Some(4.0e7),
                    bytes_per_op: Some(64.0),
                    ws: Some(1 << 30),
                    ..Default::default()
                },
            ),
        ),
    ];
    let text = plot::roofline_svg(&c, &recs, "t & <t>").unwrap();
    assert_wellformed(&text);
    assert!(text.contains("prefers-color-scheme: dark"));
    assert!(text.contains("t &amp; &lt;t&gt;"));
    assert_eq!(text.matches("<circle").count(), 6);
}

#[test]
fn registry() {
    let reg: Value = serde_json::from_str(include_str!("../registry.json")).unwrap();
    assert_eq!(reg["schema"], "optlevel.registry/v1");
    let kernels = reg["kernels"].as_array().unwrap();
    let mut ids: Vec<&str> = kernels.iter().map(|k| text(&k["id"])).collect();
    let n = ids.len();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), n);
    for k in kernels {
        let k: &Map<String, Value> = k.as_object().unwrap();
        for field in [
            "id",
            "repository",
            "path",
            "device",
            "op_class",
            "ops_counter",
            "measure",
            "status",
        ] {
            assert!(k.contains_key(field), "{:?} lacks {field}", k.get("id"));
        }
        assert!(["cpu", "gpu"].contains(&text(&k["device"])));
        assert!([
            "measured here",
            "needs PMU host",
            "needs GPU",
            "needs build"
        ]
        .contains(&text(&k["status"])));
        assert!(["emitted", "derivable", "none"].contains(&text(&k["ops_counter"]["kind"])));
        // no command line names the removed Python entry point
        assert!(
            !text(&k["measure"]).contains("optlevel.py"),
            "{} still calls optlevel.py",
            text(&k["id"])
        );
    }
}
