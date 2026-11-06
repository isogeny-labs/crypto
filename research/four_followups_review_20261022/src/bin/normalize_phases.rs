//! Reporting-only, source-reconciled phase accounting for the frozen cohort.
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Default)]
struct C {
    m: f64,
    s: f64,
    i: f64,
    logical: f64,
    base_m: f64,
    base_i: f64,
}
impl C {
    fn add(&mut self, x: C) {
        self.m += x.m;
        self.s += x.s;
        self.i += x.i;
        self.logical += x.logical;
        self.base_m += x.base_m;
        self.base_i += x.base_i;
    }
    fn subtract(self, x: C) -> C {
        C {
            m: self.m - x.m,
            s: self.s - x.s,
            i: self.i - x.i,
            logical: self.logical - x.logical,
            base_m: self.base_m - x.base_m,
            base_i: self.base_i - x.base_i,
        }
    }
    fn cost(self, sq: f64, inv: f64, native: f64, unit: [f64; 3], p: u64) -> f64 {
        let inv_bits = (64 - (p - 2).leading_zeros() + (p - 2).count_ones()) as f64;
        let bm = self.base_m - self.base_i * inv_bits;
        assert!(bm >= -1e-6, "negative base multiplication residual");
        (self.m + bm + sq * self.s + inv * (self.i + self.base_i))
            / (unit[0] + sq * unit[1] + inv * unit[2])
            + native * self.logical
    }
    fn nonnegative(self) {
        for x in [
            self.m,
            self.s,
            self.i,
            self.logical,
            self.base_m,
            self.base_i,
        ] {
            assert!(x >= -1e-6, "negative counter residual {x}");
        }
    }
}
fn n(v: &Value, k: &str) -> f64 {
    v.get(k).and_then(Value::as_u64).unwrap_or(0) as f64
}
fn cnt(v: &Value) -> C {
    C {
        m: n(v, "m"),
        s: n(v, "s"),
        i: n(v, "i"),
        logical: n(v, "field_adds") + n(v, "bit_ops") + n(v, "a"),
        ..C::default()
    }
}
fn logic(v: &Value) -> f64 {
    v.as_object()
        .map(|m| {
            m.iter()
                .filter(|(k, _)| k.ends_with("_uncharged"))
                .map(|(_, x)| x.as_u64().unwrap() as f64)
                .sum()
        })
        .unwrap_or(0.)
}
fn arith(v: &Value) -> C {
    C {
        base_m: n(v, "base_muls"),
        base_i: n(v, "base_inversions"),
        logical: n(v, "base_adds")
            + n(v, "scalar_adds")
            + n(v, "scalar_muls")
            + n(v, "scalar_inversions"),
        ..C::default()
    }
}
fn read(path: &Path) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}
fn close(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 2e-9 * a.abs().max(b.abs()).max(1.),
        "{a} != {b}"
    );
}
fn key(v: &Value) -> (u64, u64) {
    (
        v["p"].as_u64().unwrap(),
        v["target_index"].as_u64().unwrap(),
    )
}
fn raw<'a>(
    cache: &'a mut BTreeMap<String, BTreeMap<(u64, u64), Value>>,
    root: &Path,
    path: &str,
    p: u64,
    index: u64,
) -> &'a Value {
    cache
        .entry(path.to_owned())
        .or_insert_with(|| {
            read(&root.join(path))
                .into_iter()
                .filter(|v| v.get("target_index").is_some())
                .map(|v| (key(&v), v))
                .collect()
        })
        .get(&(p, index))
        .unwrap_or_else(|| panic!("missing {path} {p}:{index}"))
}
fn phases(v: &Value) -> Vec<(String, C)> {
    v["phases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| (x["name"].as_str().unwrap().to_owned(), cnt(&x["counts"])))
        .collect()
}
fn standard(v: &Value) -> Vec<(String, C)> {
    let mut out = phases(v);
    out.push((
        "native_logical_unallocated".into(),
        C {
            logical: logic(&v["native"]),
            ..C::default()
        },
    ));
    out
}
fn pencil(v: &Value) -> Vec<(String, C)> {
    let mut out: Vec<_> = v["phases"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, x)| {
            let mut c = cnt(&x["field"]);
            c.add(arith(&x["arithmetic"]));
            (name.clone(), c)
        })
        .collect();
    let mut total = cnt(&v["counts"]);
    total.add(arith(&v["arithmetic"]));
    total.logical += n(v, "solver_row_updates")
        + n(v, "solver_index_ops")
        + n(v, "solver_node_order_ops")
        + order_extra(v) as f64;
    let mut allocated = C::default();
    for (_, c) in &out {
        allocated.add(*c);
    }
    let residual = total.subtract(allocated);
    residual.nonnegative();
    out.push(("unallocated_setup_and_solver".into(), residual));
    out
}
fn gcd(mut a: u64, mut b: u64) -> (u64, u64) {
    let mut steps = 0;
    while b != 0 {
        (a, b) = (b, a % b);
        steps += 1;
    }
    (a, steps)
}
fn order_extra(v: &Value) -> u64 {
    if v["solver_node_order"] != "affine" {
        return 0;
    }
    let size = v["solver_nodes"].as_u64().unwrap();
    let mut stride = 0x9e37_79b9_7f4a_7c15_u64 % size;
    let mut ops = 2;
    loop {
        let (g, steps) = gcd(stride, size);
        ops += steps;
        if g == 1 {
            break;
        }
        stride = (stride + 1) % size;
        ops += 2;
    }
    ops
}
fn rho_pencil(v: &Value) -> Vec<(String, C)> {
    let r = &v["rho"];
    let mut out = phases(r);
    let mut allocated = C::default();
    for (_, c) in &out {
        allocated.add(*c);
    }
    let mut residual = cnt(&r["counts"]).subtract(allocated);
    residual.nonnegative();
    residual.logical += logic(&r["native"]);
    out.push(("rho_unallocated_verification_and_logical".into(), residual));
    out
}
fn kummer_count(v: &Value) -> C {
    let mut c = cnt(v);
    c.logical = n(v, "a") + n(v, "field_adds");
    c
}
fn kummer(
    root: &Path,
    bits: u64,
    p: u64,
    index: u64,
    models: &BTreeMap<u64, Value>,
    targets: &BTreeMap<(u64, u64), Value>,
    cache: &mut BTreeMap<String, BTreeMap<(u64, u64), Value>>,
) -> (Vec<(String, C)>, Vec<(String, C)>, [f64; 3]) {
    let path=format!("research/kummer_differential_collector_20261022/results/paired/uncached/collector-{bits}.jsonl");
    let rp = format!(
        "research/kummer_differential_collector_20261022/results/paired/rho/rho-{bits}.jsonl"
    );
    let v = raw(cache, root, &path, p, index).clone();
    let r = raw(cache, root, &rp, p, index);
    let m = &models[&p];
    let t = &targets[&(p, index)];
    let mut out = Vec::new();
    for name in [
        "root_ops",
        "root_extension_counts",
        "root_base_counts",
        "theta_ops",
        "torsion_ops",
        "torsion_extension_counts",
        "torsion_base_counts",
    ] {
        out.push((name.into(), kummer_count(&m[name])));
    }
    for name in ["transfer_ops", "extension_counts", "base_counts"] {
        out.push((name.into(), kummer_count(&t[name])));
    }
    for name in ["walk_ops", "recover_ops", "field_counts"] {
        out.push((name.into(), kummer_count(&v[name])));
    }
    out.push((
        "collector_logical_unallocated".into(),
        C {
            logical: n(&v, "graph_operations")
                + 3. * n(&v, "attempts")
                + n(&v, "recover_success")
                + n(&v, "walk_restarts"),
            ..C::default()
        },
    ));
    let rc = &r["counters"];
    let rho = vec![(
        "rho_total_unphased".into(),
        C {
            m: n(rc, "field_multiplications"),
            s: n(rc, "field_squarings"),
            i: n(rc, "field_inversions"),
            logical: logic(rc),
            ..C::default()
        },
    )];
    (
        out,
        rho,
        [
            n(rc, "source_add_multiplications"),
            n(rc, "source_add_squarings"),
            n(rc, "source_add_inversions"),
        ],
    )
}
fn csv(path: &Path) -> Vec<BTreeMap<String, String>> {
    let input = fs::read_to_string(path).unwrap();
    let mut lines = input.lines();
    let fields: Vec<_> = lines.next().unwrap().split(',').collect();
    lines
        .map(|line| {
            let values: Vec<_> = line.split(',').collect();
            assert_eq!(fields.len(), values.len());
            fields
                .iter()
                .zip(values)
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect()
        })
        .collect()
}
fn get<'a>(r: &'a BTreeMap<String, String>, k: &str) -> &'a str {
    r.get(k).unwrap()
}
fn class(lead: &str, phase: &str) -> &'static str {
    match phase {
        "destination_construction"
        | "rho_initialization"
        | "root_ops"
        | "root_extension_counts"
        | "root_base_counts"
        | "theta_ops"
        | "torsion_ops"
        | "torsion_extension_counts"
        | "torsion_base_counts" => "setup",
        "transfer" | "transfer_ops" | "extension_counts" | "base_counts" => "transfer",
        "walk_initialization" | "walk_generation" | "walk_ops" | "rho_search" => "walk",
        "relations_and_signed_elimination" | "candidate_verification" | "elimination" | "sieve" => {
            "relation_processing"
        }
        "recover_ops" => "recovery",
        "source_verification" | "recovery_check" => "verification",
        "field_counts" if lead == "kummer_walk" => "source_unallocated",
        "native_logical_unallocated"
        | "collector_logical_unallocated"
        | "unallocated_setup_and_solver"
        | "rho_unallocated_verification_and_logical"
        | "rho_total_unphased" => "source_unallocated",
        _ => panic!("unmapped phase {lead}: {phase}"),
    }
}
fn write(root: &Path) {
    let dir = root.join("research/four_followups_review_20261022/phase-accounting");
    fs::create_dir_all(&dir).unwrap();
    let target_rows =
        csv(&root.join("research/four_followups_review_20261022/results/standard-targets.csv"));
    let kroot = "research/kummer_differential_collector_20261022/results";
    let models: BTreeMap<_, _> = read(&root.join(format!("{kroot}/cold-model-setup.jsonl")))
        .into_iter()
        .map(|v| (v["p"].as_u64().unwrap(), v))
        .collect();
    let targets: BTreeMap<_, _> = read(&root.join(format!("{kroot}/cold-target-setup.jsonl")))
        .into_iter()
        .map(|v| (key(&v), v))
        .collect();
    let mut cache = BTreeMap::new();
    let mut per=String::from("lead,arm,bits,p,target_index,workload_id,method,phase,phase_class,status,cost_GAE,phase_fraction,source_record\n");
    let mut agg: BTreeMap<
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (usize, f64, f64),
    > = BTreeMap::new();
    let mut checked = 0usize;
    let mut nulls = 0usize;
    for row in &target_rows {
        let lead = get(row, "lead");
        let arm = get(row, "arm");
        let bits: u64 = get(row, "bits").parse().unwrap();
        let p: u64 = get(row, "p").parse().unwrap();
        let index: u64 = get(row, "target_index").parse().unwrap();
        let src = get(row, "source_record");
        if arm == "coordinate_identity_gate" {
            nulls += 1;
            per.push_str(&format!(
                "{lead},{arm},{bits},{p},{index},{},candidate,,,coordinate_only_no_cost,,,{src}\n",
                get(row, "workload_id")
            ));
            continue;
        }
        let (candidate, rho, unit) = match lead {
            "factor_base" | "projective_law" => {
                let v = raw(&mut cache, root, src, p, index);
                let u = &v["source_add"];
                (
                    standard(v),
                    standard(&v["rho"]),
                    [n(u, "m"), n(u, "s"), n(u, "i")],
                )
            }
            "pencil_collector" => {
                let v = raw(&mut cache, root, src, p, index);
                let u = &v["source_add"];
                (
                    if v["verified"] == true {
                        pencil(v)
                    } else {
                        Vec::new()
                    },
                    rho_pencil(v),
                    [n(u, "m"), n(u, "s"), n(u, "i")],
                )
            }
            "kummer_walk" => kummer(root, bits, p, index, &models, &targets, &mut cache),
            _ => panic!("unknown lead {lead}"),
        };
        assert!(unit.iter().sum::<f64>() > 0.);
        for (method, list, recorded) in [
            ("candidate", candidate, get(row, "candidate_GAE")),
            ("rho", rho, get(row, "rho_GAE")),
        ] {
            let phase_source = if lead == "kummer_walk" {
                format!(
                    "research/kummer_differential_collector_20261022/results/paired/{}/{}-{bits}.jsonl",
                    if method == "candidate" { "uncached" } else { "rho" },
                    if method == "candidate" { "collector" } else { "rho" }
                )
            } else {
                src.to_owned()
            };
            if recorded.is_empty() {
                assert_eq!(method, "candidate");
                nulls += 1;
                per.push_str(&format!("{lead},{arm},{bits},{p},{index},{},candidate,,,incomplete_candidate_no_cost,,,{src}\n",get(row,"workload_id")));
                continue;
            }
            let expected: f64 = recorded.parse().unwrap();
            for sq in [0., 0.5, 1.] {
                for inv in [1., 10., 100., 1000.] {
                    for native in [0., 0.1, 1.] {
                        let costs: Vec<_> = list
                            .iter()
                            .map(|(name, c)| (name, c.cost(sq, inv, native, unit, p)))
                            .collect();
                        let total: f64 = costs.iter().map(|(_, x)| x).sum();
                        if (sq, inv, native) == (0.5, 10., 0.1) {
                            if (total - expected).abs() > 1e-6 {
                                eprintln!("mismatch {lead} {arm} {bits} {p} {index} {method} {total} {expected} unit={unit:?} phases={:?}",list.iter().map(|(n,c)|(n,c.m,c.s,c.i,c.logical)).collect::<Vec<_>>());
                            }
                            close(total, expected);
                            checked += 1;
                        }
                        let slice = bits.to_string();
                        for (name, cost) in costs {
                            let k = (
                                lead.to_owned(),
                                arm.to_owned(),
                                slice.clone(),
                                method.to_owned(),
                                name.clone(),
                                format!("{sq}"),
                                format!("{inv}"),
                                format!("{native}"),
                            );
                            let entry = agg.entry(k).or_default();
                            entry.0 += 1;
                            entry.1 += cost;
                            entry.2 += total;
                            let k = (
                                lead.to_owned(),
                                arm.to_owned(),
                                "all".into(),
                                method.to_owned(),
                                name.clone(),
                                format!("{sq}"),
                                format!("{inv}"),
                                format!("{native}"),
                            );
                            let entry = agg.entry(k).or_default();
                            entry.0 += 1;
                            entry.1 += cost;
                            entry.2 += total;
                            if lead == "pencil_collector" {
                                let k = (
                                    lead.to_owned(),
                                    arm.to_owned(),
                                    format!("p{p}"),
                                    method.to_owned(),
                                    name.clone(),
                                    format!("{sq}"),
                                    format!("{inv}"),
                                    format!("{native}"),
                                );
                                let entry = agg.entry(k).or_default();
                                entry.0 += 1;
                                entry.1 += cost;
                                entry.2 += total;
                            }
                            if (sq, inv, native) == (0.5, 10., 0.1) {
                                per.push_str(&format!("{lead},{arm},{bits},{p},{index},{},{method},{name},{},priced,{cost:.12},{:.12},{phase_source}\n",get(row,"workload_id"),class(lead,name),cost/total));
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(checked, 2 * (6300 + 2520 + 1506 + 1260) + 4);
    assert_eq!(nulls, 1264);
    let mut totals: BTreeMap<(String, String, String, String, String, String, String), f64> =
        BTreeMap::new();
    for ((lead, arm, slice, method, _phase, sq, inv, native), (_, cost, _)) in &agg {
        *totals
            .entry((
                lead.clone(),
                arm.clone(),
                slice.clone(),
                method.clone(),
                sq.clone(),
                inv.clone(),
                native.clone(),
            ))
            .or_default() += cost;
    }
    let grids =
        csv(&root.join("research/four_followups_review_20261022/results/standard-cost-grid.csv"));
    let mut grid_checks = 0;
    for row in &grids {
        if get(row, "candidate_GAE").is_empty() {
            continue;
        }
        for (method, col) in [("candidate", "candidate_GAE"), ("rho", "rho_GAE")] {
            let arm = if get(row, "arm") == "degree_peel" {
                "peel_identity"
            } else {
                get(row, "arm")
            };
            let key = (
                get(row, "lead").to_owned(),
                arm.to_owned(),
                get(row, "slice").to_owned(),
                method.to_owned(),
                get(row, "S_per_M").to_owned(),
                get(row, "I_per_M").to_owned(),
                get(row, "native_GAE_per_logical").to_owned(),
            );
            let got = *totals
                .get(&key)
                .unwrap_or_else(|| panic!("missing aggregate {key:?}"));
            close(got, get(row, col).parse().unwrap());
            grid_checks += 1;
        }
    }
    fs::write(dir.join("phase-targets.csv"), per).unwrap();
    let mut grid=String::from("lead,arm,slice,method,phase,phase_class,S_per_M,I_per_M,native_GAE_per_logical,targets,cost_GAE,cold_total_GAE,phase_fraction\n");
    for ((lead, arm, slice, method, phase, sq, inv, native), (count, cost, total)) in agg {
        grid.push_str(&format!("{lead},{arm},{slice},{method},{phase},{},{sq},{inv},{native},{count},{cost:.12},{total:.12},{:.12}\n",class(&lead,&phase),cost/total));
    }
    fs::write(dir.join("phase-grid.csv"), grid).unwrap();
    fs::write(dir.join("summary.json"),format!("{{\"schema\":\"phase-accounting/v1\",\"standard_target_rows\":{},\"reconciled_method_rows\":{checked},\"aggregate_price_checks\":{grid_checks},\"explicit_null_rows\":{nulls},\"price_cells\":36,\"representative_cell\":{{\"S_per_M\":0.5,\"I_per_M\":10,\"native_GAE_per_logical\":0.1}}}}\n",target_rows.len())).unwrap();
}
fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .to_path_buf();
    write(&root);
}
