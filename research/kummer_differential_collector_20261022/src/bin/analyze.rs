//! Native replay, source check, cold pricing, and standardized result generation.
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::PathBuf,
    process::Command,
};

const BITS: [u64; 7] = [16, 20, 24, 28, 32, 36, 40];
fn n(v: &Value, k: &str) -> u64 {
    v[k].as_u64()
        .unwrap_or_else(|| panic!("missing numeric {k}: {v}"))
}
fn s<'a>(v: &'a Value, k: &str) -> &'a str {
    v[k].as_str()
        .unwrap_or_else(|| panic!("missing string {k}: {v}"))
}
fn rows(path: &PathBuf) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}
fn digest(path: &PathBuf) -> String {
    let output = Command::new("sha256sum").arg(path).output().unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .to_string()
}
fn same(a: &Value, b: &Value, p: u64) -> bool {
    (0..4).all(|i| {
        (0..4).all(|j| {
            (a[i].as_u64().unwrap() * b[j].as_u64().unwrap()) % p
                == (a[j].as_u64().unwrap() * b[i].as_u64().unwrap()) % p
        })
    })
}
#[derive(Clone, Copy, Default)]
struct Count {
    m: u64,
    s: u64,
    i: u64,
    adds: u64,
    logical: u64,
}
impl Count {
    fn add(&mut self, q: Count) {
        self.m += q.m;
        self.s += q.s;
        self.i += q.i;
        self.adds += q.adds;
        self.logical += q.logical;
    }
}
fn count(v: &Value) -> Count {
    Count {
        m: n(v, "m"),
        s: v["s"].as_u64().unwrap_or(0),
        i: n(v, "i"),
        adds: v["a"].as_u64().unwrap_or(0) + v["field_adds"].as_u64().unwrap_or(0),
        logical: 0,
    }
}
fn setup_count(m: &Value, t: &Value) -> Count {
    let mut c = Count::default();
    for k in [
        "root_ops",
        "root_extension_counts",
        "root_base_counts",
        "theta_ops",
        "torsion_ops",
        "torsion_extension_counts",
        "torsion_base_counts",
    ] {
        c.add(count(&m[k]));
    }
    for k in ["transfer_ops", "extension_counts", "base_counts"] {
        c.add(count(&t[k]));
    }
    c.logical = c.adds;
    c
}
fn candidate_count(v: &Value) -> Count {
    let mut c = Count::default();
    for k in ["walk_ops", "recover_ops", "field_counts"] {
        c.add(count(&v[k]));
    }
    c.logical = c.adds
        + n(v, "graph_operations")
        + 3 * n(v, "attempts")
        + n(v, "recover_success")
        + n(v, "walk_restarts");
    c
}
fn rho_count(v: &Value) -> (Count, [u64; 3]) {
    let c = &v["counters"];
    let logical = c
        .as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.ends_with("_uncharged"))
        .map(|(_, v)| v.as_u64().unwrap())
        .sum();
    (
        Count {
            m: n(c, "field_multiplications"),
            s: n(c, "field_squarings"),
            i: n(c, "field_inversions"),
            adds: 0,
            logical,
        },
        [
            n(c, "source_add_multiplications"),
            n(c, "source_add_squarings"),
            n(c, "source_add_inversions"),
        ],
    )
}
fn gae(c: Count, sq: f64, inv: f64, native: f64, unit: [u64; 3]) -> f64 {
    let denom = unit[0] as f64 + sq * unit[1] as f64 + inv * unit[2] as f64;
    assert!(denom > 0.0);
    (c.m as f64 + sq * c.s as f64 + inv * c.i as f64) / denom + native * c.logical as f64
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct F2 {
    a: u64,
    b: u64,
}
fn fadd(x: F2, y: F2, p: u64) -> F2 {
    F2 {
        a: (x.a + y.a) % p,
        b: (x.b + y.b) % p,
    }
}
fn fneg(x: F2, p: u64) -> F2 {
    F2 {
        a: (p - x.a) % p,
        b: (p - x.b) % p,
    }
}
fn fsub(x: F2, y: F2, p: u64) -> F2 {
    fadd(x, fneg(y, p), p)
}
fn fmul(x: F2, y: F2, p: u64) -> F2 {
    F2 {
        a: (x.a * y.a + p * p - x.b * y.b) % p,
        b: (x.a * y.b + x.b * y.a) % p,
    }
}
fn pow(mut x: u64, mut e: u64, p: u64) -> u64 {
    let mut z = 1;
    while e > 0 {
        if e & 1 == 1 {
            z = z * x % p;
        }
        x = x * x % p;
        e >>= 1;
    }
    z
}
fn finv(x: F2, p: u64) -> F2 {
    let norm = (x.a * x.a + x.b * x.b) % p;
    assert_ne!(norm, 0);
    let z = pow(norm, p - 2, p);
    F2 {
        a: x.a * z % p,
        b: (p - x.b) * z % p,
    }
}
fn unpack(x: u64, p: u64) -> F2 {
    F2 { a: x % p, b: x / p }
}
type Pt = Option<(F2, F2)>;
fn ecadd(u: Pt, v: Pt, p: u64, a: F2) -> Pt {
    let (x, y) = match u {
        None => return v,
        Some(z) => z,
    };
    let (xx, yy) = match v {
        None => return u,
        Some(z) => z,
    };
    if x == xx && y == fneg(yy, p) {
        return None;
    }
    let slope = if u == v {
        fmul(
            fadd(fmul(F2 { a: 3, b: 0 }, fmul(x, x, p), p), a, p),
            finv(fmul(F2 { a: 2, b: 0 }, y, p), p),
            p,
        )
    } else {
        fmul(fsub(yy, y, p), finv(fsub(xx, x, p), p), p)
    };
    let rx = fsub(fsub(fmul(slope, slope, p), x, p), xx, p);
    let ry = fsub(fmul(slope, fsub(x, rx, p), p), y, p);
    Some((rx, ry))
}
fn source_check(spec: &Value, target: &Value, mut scalar: u64) -> bool {
    let p = n(spec, "p");
    assert_eq!(n(spec, "n"), 2);
    assert_eq!(n(spec, "modulus"), 1);
    assert_eq!(p % 4, 3);
    let a = unpack(n(spec, "a"), p);
    let mut base = Some((unpack(n(spec, "gx"), p), unpack(n(spec, "gy"), p)));
    let expect = Some((
        unpack(
            u64::from_str_radix(target[0].as_str().unwrap().trim_start_matches("0x"), 16).unwrap(),
            p,
        ),
        unpack(
            u64::from_str_radix(target[1].as_str().unwrap().trim_start_matches("0x"), 16).unwrap(),
            p,
        ),
    ));
    let mut acc = None;
    while scalar > 0 {
        if scalar & 1 == 1 {
            acc = ecadd(acc, base, p, a);
        }
        scalar >>= 1;
        if scalar > 0 {
            base = ecadd(base, base, p, a);
        }
    }
    acc == expect
}
fn rho_rows(root: &PathBuf, bits: u64) -> HashMap<String, Value> {
    let archive =
        root.join("research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst");
    let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
    let output = Command::new("tar")
        .args(["--zstd", "-xOf", archive.to_str().unwrap(), &member])
        .output()
        .unwrap();
    assert!(output.status.success());
    let map: HashMap<String, Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .filter(|v| v["arm"] == "rho_neg")
        .map(|v| (s(&v["workload"], "workload_id").to_string(), v))
        .collect();
    assert_eq!(map.len(), 180);
    map
}
#[derive(Clone)]
struct Record {
    arm: String,
    bits: u64,
    p: u64,
    index: u64,
    id: String,
    online: Count,
    cold: Count,
    rho: Count,
    unit: [u64; 3],
}
fn main() {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = here.parent().unwrap().parent().unwrap().to_path_buf();
    let results = here.join("results");
    let prior: Value =
        serde_json::from_str(&fs::read_to_string(results.join("summary.json")).unwrap()).unwrap();
    let arms: Vec<String> = std::env::var("KUMMER_ARMS")
        .unwrap_or_else(|_| "uncached,cached".to_string())
        .split(',')
        .map(str::to_string)
        .collect();
    assert!(!arms.is_empty() && arms.iter().all(|a| a == "uncached" || a == "cached"));
    let models: HashMap<u64, Value> = rows(&results.join("cold-model-setup.jsonl"))
        .into_iter()
        .map(|v| (n(&v, "p"), v))
        .collect();
    assert_eq!(models.len(), 21);
    let setups: HashMap<String, Value> = rows(&results.join("cold-target-setup.jsonl"))
        .into_iter()
        .map(|v| (s(&v, "workload_id").to_string(), v))
        .collect();
    assert_eq!(setups.len(), 1260);
    assert!(
        models.values().all(
            |v| v["torsion_actions_match"] == true && v["roots"].as_array().unwrap().len() == 5
        )
    );
    let mut records = Vec::new();
    let mut replay_runs = Vec::new();
    let mut status = BTreeMap::new();
    let mut totals = BTreeMap::new();
    for arm in &arms {
        let mut verified = 0u64;
        let mut steps = 0u64;
        let mut splits = 0u64;
        let mut recover_fail = 0u64;
        let mut repairs = 0u64;
        let mut restarts = 0u64;
        for bits in BITS {
            let source = rows(&root.join(format!(
                "research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl"
            )));
            assert_eq!(source.len(), 180);
            let source_map: HashMap<String, Value> = source
                .into_iter()
                .map(|v| (s(&v, "workload_id").to_string(), v))
                .collect();
            let candidates = rows(&results.join(format!("{arm}/collector-{bits}.jsonl")));
            assert_eq!(candidates.len(), 180);
            let rho = rho_rows(&root, bits);
            for c in candidates {
                let id = s(&c, "workload_id");
                let rr = &rho[id];
                let w = &rr["workload"];
                let src = &source_map[id];
                let t = &setups[id];
                let m = &models[&n(&c, "p")];
                let p = n(&c, "p");
                assert_eq!(p, n(&w["curve_spec"], "p"));
                assert_eq!(n(&c, "r"), n(&w["curve_spec"], "r"));
                assert_eq!(n(&c, "target_index"), n(w, "target_index"));
                assert_eq!(n(&c, "planted"), n(w, "planted"));
                assert_eq!(n(t, "target_index"), n(&c, "target_index"));
                assert_eq!(n(t, "bits"), bits);
                assert_eq!(n(t, "p"), p);
                assert_eq!(c["status"], "verified");
                assert_eq!(c["source_verified"], true);
                assert_eq!(c["recovered"], c["planted"]);
                assert_eq!(rr["outcome"]["status"], "verified");
                assert_eq!(
                    s(&rr["outcome"], "recovered").parse::<u64>().unwrap(),
                    n(&c, "recovered")
                );
                assert_eq!(t["source_scalar_verified"], true);
                assert_eq!(m["torsion_actions_match"], true);
                assert_eq!(n(src, "target_index"), n(&c, "target_index"));
                for (key, idx) in [("anchor", 1), ("target", 2)] {
                    assert_eq!(
                        t[key],
                        src["cases"][idx]["coordinate_witness"]["jacobian_expected"]
                    );
                }
                for (key, idx) in [("k_anchor", 1), ("k_target", 2), ("k_plus", 3)] {
                    assert!(same(
                        &t[key],
                        &src["cases"][idx]["coordinate_witness"]["k_expected"],
                        p
                    ));
                }
                assert!(n(&c, "attempts") >= n(&c, "split_relations"));
                assert!(n(&c, "recover_success") >= n(&c, "split_relations"));
                assert!(
                    source_check(&w["curve_spec"], &w["target"], n(&c, "recovered")),
                    "source scalar {id}"
                );
                replay_runs.push(json!({"arm":arm,"bits":bits,"workload_id":id,
                    "recovered":n(&c,"recovered"),"source_scalar_verified":true,
                    "rho_recovered":s(&rr["outcome"],"recovered").parse::<u64>().unwrap()}));
                let online = candidate_count(&c);
                let mut cold = online;
                cold.add(setup_count(m, t));
                let (rc, unit) = rho_count(rr);
                records.push(Record {
                    arm: arm.clone(),
                    bits,
                    p,
                    index: n(&c, "target_index"),
                    id: id.to_string(),
                    online,
                    cold,
                    rho: rc,
                    unit,
                });
                verified += 1;
                steps += n(&c, "attempts");
                splits += n(&c, "split_relations");
                recover_fail += n(&c, "recover_fail");
                repairs += n(&c, "xadd_repairs");
                restarts += n(&c, "walk_restarts");
            }
        }
        assert_eq!(verified, 1260);
        status.insert(arm.clone(), json!({"verified":verified}));
        totals.insert(arm.clone(),json!({"source_verified":verified,"processed_divisors":steps,"split_relations":splits,"recover_fail":recover_fail,"xadd_repairs":repairs,"walk_restarts":restarts}));
    }
    let cells: Vec<(f64, f64, f64)> = [0., 0.5, 1.]
        .into_iter()
        .flat_map(|sq| {
            [1., 10., 100., 1000.].into_iter().flat_map(move |inv| {
                [0., 0.1, 1.]
                    .into_iter()
                    .map(move |native| (sq, inv, native))
            })
        })
        .collect();
    assert_eq!(cells.len(), 36);
    let mut target_csv = String::from(
        "arm,bits,p,target_index,workload_id,S_per_M,I_per_M,native_GAE_per_logical,candidate_online_GAE,candidate_cold_GAE,rho_GAE,candidate_online_over_rho,candidate_cold_over_rho,candidate_verified,rho_verified\n",
    );
    for rec in &records {
        for &(sq, inv, native) in &cells {
            let o = gae(rec.online, sq, inv, native, rec.unit);
            let c = gae(rec.cold, sq, inv, native, rec.unit);
            let r = gae(rec.rho, sq, inv, native, rec.unit);
            target_csv.push_str(&format!("{},{},{},{},{},{sq},{inv},{native},{o:.12},{c:.12},{r:.12},{:.12},{:.12},true,true\n",rec.arm,rec.bits,rec.p,rec.index,rec.id,o/r,c/r));
        }
    }
    fs::write(results.join("target-price.csv"), target_csv).unwrap();
    let mut grid_csv = String::from(
        "arm,slice,targets,S_per_M,I_per_M,native_GAE_per_logical,candidate_online_GAE,candidate_cold_GAE,rho_GAE,candidate_online_over_rho,candidate_cold_over_rho,candidate_wins,candidate_losses\n",
    );
    let mut per_arm = BTreeMap::new();
    for arm in &arms {
        let mut min_c = f64::INFINITY;
        let mut max_c = 0f64;
        let mut min_o = f64::INFINITY;
        let mut max_o = 0f64;
        let mut wc = 0;
        let mut lc = 0;
        let mut wo = 0;
        let mut lo = 0;
        for slice in std::iter::once(None).chain(BITS.into_iter().map(Some)) {
            let subset: Vec<_> = records
                .iter()
                .filter(|x| x.arm == *arm && slice.is_none_or(|b| x.bits == b))
                .collect();
            let name = slice.map_or("all".to_string(), |b| format!("bits{b}"));
            assert_eq!(subset.len(), if slice.is_some() { 180 } else { 1260 });
            for &(sq, inv, native) in &cells {
                let mut o = 0.;
                let mut c = 0.;
                let mut r = 0.;
                let mut wins = 0;
                for x in &subset {
                    let oi = gae(x.online, sq, inv, native, x.unit);
                    let ci = gae(x.cold, sq, inv, native, x.unit);
                    let ri = gae(x.rho, sq, inv, native, x.unit);
                    o += oi;
                    c += ci;
                    r += ri;
                    if ci < ri {
                        wins += 1;
                    }
                }
                let cr = c / r;
                let or = o / r;
                if slice.is_none() {
                    min_c = min_c.min(cr);
                    max_c = max_c.max(cr);
                    min_o = min_o.min(or);
                    max_o = max_o.max(or);
                    if cr < 1. {
                        wc += 1
                    } else {
                        lc += 1
                    }
                    if or < 1. { wo += 1 } else { lo += 1 }
                }
                grid_csv.push_str(&format!(
                    "{arm},{name},{},{sq},{inv},{native},{o},{c},{r},{or},{cr},{wins},{}\n",
                    subset.len(),
                    subset.len() - wins
                ));
            }
        }
        per_arm.insert(arm.clone(),json!({"cold_ratio_min":min_c,"cold_ratio_max":max_c,"cold_win_cells":wc,"cold_loss_cells":lc,"online_ratio_min":min_o,"online_ratio_max":max_o,"online_win_cells":wo,"online_loss_cells":lo}));
    }
    fs::write(results.join("price-grid.csv"), grid_csv).unwrap();
    let get = |key: &str| -> Value {
        Value::Object(
            totals
                .iter()
                .map(|(a, v)| (a.clone(), v[key].clone()))
                .collect(),
        )
    };
    let summary = json!({"schema":"kummer-differential-collector-summary/v1","study_id":"KUMMER-DIFFERENTIAL-COLLECTOR-20261022",
      "targets_per_arm":1260,"arms":arms,"status":status,"source_verified":get("source_verified"),"processed_divisors":get("processed_divisors"),"split_relations":get("split_relations"),"recover_fail":get("recover_fail"),"xadd_repairs":get("xadd_repairs"),"walk_restarts":get("walk_restarts"),
      "price_cells":36*8*per_arm.len(),"candidate_cost_boundary":"cold per target: fresh model root/theta/torsion setup, source transfer, anchor, collector, and verification","cold_setup_models":21,"cold_setup_targets":1260,"cold_model_charges":1260,"per_arm":per_arm});
    assert_eq!(
        prior["arms"], summary["arms"],
        "first produce a native summary for newly completed arms, then rerun for independent headline validation"
    );
    let mut raw_paths = vec![
        "research/kummer_differential_collector_20261022/results/cold-model-setup.jsonl"
            .to_string(),
        "research/kummer_differential_collector_20261022/results/cold-target-setup.jsonl"
            .to_string(),
        "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst".to_string(),
    ];
    for arm in &arms {
        for bits in BITS {
            raw_paths.push(format!("research/kummer_differential_collector_20261022/results/{arm}/collector-{bits}.jsonl"));
        }
    }
    for bits in BITS {
        raw_paths.push(format!(
            "research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl"
        ));
    }
    let raw_data: Vec<Value> = raw_paths
        .iter()
        .map(|p| json!({"path":p,"sha256":digest(&root.join(p))}))
        .collect();
    let cert = json!({"schema":"kummer-native-replay-certificate/v1","status":"verified","independent":true,
      "scalars_verified":true,"runs":replay_runs,"raw_data":raw_data,
      "analyzer_source_sha256":digest(&here.join("src/bin/analyze.rs")),
      "method":"independent Rust Fp² affine scalar multiplication and exact setup/collector joins"});
    fs::write(
        here.join("replay-certificate.json"),
        format!("{}\n", serde_json::to_string_pretty(&cert).unwrap()),
    )
    .unwrap();
    let mut headlines = Vec::new();
    for arm in &arms {
        for key in ["source_verified", "processed_divisors", "split_relations"] {
            headlines.push(json!({"name":format!("{arm}.{key}"),"claimed":prior[key][arm],"recomputed":summary[key][arm]}));
        }
        for key in [
            "cold_ratio_min",
            "cold_ratio_max",
            "cold_win_cells",
            "cold_loss_cells",
        ] {
            headlines.push(json!({"name":format!("{arm}.{key}"),"claimed":prior["per_arm"][arm][key],"recomputed":summary["per_arm"][arm][key],"rel_tol":1e-12}));
        }
    }
    assert!(
        headlines.iter().all(|h| {
            let a = h["claimed"].as_f64().unwrap();
            let b = h["recomputed"].as_f64().unwrap();
            (a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1.)
        }),
        "headline mismatch"
    );
    let report = json!({"schema":"kummer-native-validator-report/v1","status":"pass","independent":true,
      "raw_data":raw_data,"headline":headlines,"analyzer_source_sha256":digest(&here.join("src/bin/analyze.rs")),
      "rho_process_match":false,"rho_note":"archived same-source rho ran in a separate process; no leaderboard gate claim"});
    fs::write(
        here.join("validator-report.json"),
        format!("{}\n", serde_json::to_string_pretty(&report).unwrap()),
    )
    .unwrap();
    fs::write(
        results.join("summary.json"),
        format!("{}\n", serde_json::to_string_pretty(&summary).unwrap()),
    )
    .unwrap();
    println!("{}", serde_json::to_string(&summary).unwrap());
}
