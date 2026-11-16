//! One-target IC using only polynomially peeled mixed-line quartic relations.
#![recursion_limit = "512"]
#[allow(dead_code)]
#[path = "quartic_twist_tagged_s5.rs"]
mod tagged;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

use crypto_lib::cryptanalysis::gaudry_quartic::{Curve4, Pt4, E4};
use crypto_lib::cryptanalysis::quartic_folds::{construct_twist24, rho_twist24, Twist24Instance};
use crypto_lib::cryptanalysis::residual_walk::{inv_mod, pow_mod};
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::{json, Value};
use tagged::{collect_mixed_pattern, SymS4, Terms};

fn u(v: &Value) -> u64 {
    v.as_u64().expect("u64")
}
fn e(v: &Value) -> E4 {
    let a = v.as_array().expect("F4 element");
    E4([u(&a[0]), u(&a[1]), u(&a[2]), u(&a[3])])
}
fn point(v: &Value) -> Pt4 {
    Pt4 {
        x: e(&v[0]),
        y: e(&v[1]),
        inf: false,
    }
}
fn mm(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn sub(a: u64, b: u64, n: u64) -> u64 {
    (a + n - b) % n
}

struct OrbitMap {
    col: Vec<usize>,
    coef: Vec<u64>,
    reps: Vec<usize>,
}
fn orbit_map(inst: &Twist24Instance, c: &Curve4, base: &[Pt4]) -> OrbitMap {
    let n = inst.core.n;
    let by_x: HashMap<E4, usize> = base.iter().enumerate().map(|(i, pt)| (pt.x, i)).collect();
    let mut col = vec![usize::MAX; base.len()];
    let mut coef = vec![0u64; base.len()];
    let mut reps = Vec::new();
    for (i, &rep) in base.iter().enumerate() {
        if col[i] != usize::MAX {
            continue;
        }
        let ci = reps.len();
        reps.push(i);
        for j in 0..4 {
            let mut frob = rep;
            for _ in 0..j {
                frob = inst.frob(c, &frob);
            }
            let fj = pow_mod(inst.lambda_frob, j, n);
            for k in 0..3 {
                let transformed = inst.core.psi(c, &frob, k as u32);
                let scalar = mm(fj, pow_mod(inst.core.lambda, k, n), n);
                assert_eq!(c.mul(&rep, scalar), transformed);
                let &idx = by_x.get(&transformed.x).expect("orbit left base");
                let signed = if base[idx].y == transformed.y {
                    scalar
                } else {
                    assert_eq!(base[idx].y, c.f.neg(&transformed.y));
                    sub(0, scalar, n)
                };
                if col[idx] == usize::MAX {
                    col[idx] = ci;
                    coef[idx] = signed;
                } else {
                    assert_eq!((col[idx], coef[idx]), (ci, signed));
                }
            }
        }
    }
    assert!(col.iter().all(|&x| x != usize::MAX));
    assert_eq!(reps.len(), 4);
    assert_eq!(reps[0], 0);
    OrbitMap { col, coef, reps }
}

struct Matrix {
    basis: [Option<[u64; 5]>; 4],
    rank: usize,
    modular_muls: u64,
    modular_inversions: u64,
    n: u64,
}
impl Matrix {
    fn new(n: u64) -> Self {
        Self {
            basis: [None; 4],
            rank: 0,
            modular_muls: 0,
            modular_inversions: 0,
            n,
        }
    }
    fn insert(&mut self, row: [u64; 4], rhs: u64) -> bool {
        let mut v = [row[0], row[1], row[2], row[3], rhs];
        for col in 0..4 {
            if v[col] == 0 {
                continue;
            }
            if let Some(pivot) = &self.basis[col] {
                let factor = v[col];
                for j in col..5 {
                    v[j] = sub(v[j], mm(factor, pivot[j], self.n), self.n);
                    self.modular_muls += 1;
                }
            } else {
                let inv = inv_mod(v[col], self.n);
                self.modular_inversions += 1;
                for item in v.iter_mut().skip(col) {
                    *item = mm(*item, inv, self.n);
                    self.modular_muls += 1;
                }
                self.basis[col] = Some(v);
                self.rank += 1;
                return true;
            }
        }
        assert_eq!(v[4], 0, "inconsistent relation");
        false
    }
    fn solution(&mut self) -> Option<[u64; 4]> {
        if self.rank != 4 {
            return None;
        }
        let mut out = [0u64; 4];
        for i in (0..4).rev() {
            let row = self.basis[i].unwrap();
            let mut rhs = row[4];
            for j in i + 1..4 {
                rhs = sub(rhs, mm(row[j], out[j], self.n), self.n);
                self.modular_muls += 1;
            }
            out[i] = rhs;
        }
        Some(out)
    }
}

fn relation_row(terms: &Terms, map: &OrbitMap, a: u64, b: u64, n: u64) -> ([u64; 4], u64) {
    let mut sum = [0u64; 4];
    for &(id, sign) in terms {
        let coef = if sign == 1 {
            map.coef[id]
        } else {
            sub(0, map.coef[id], n)
        };
        sum[map.col[id]] = (sum[map.col[id]] + coef) % n;
    }
    ([sum[1], sum[2], sum[3], sub(0, b, n)], sub(a, sum[0], n))
}

fn run(raw: &str, enabled_patterns: &[usize]) -> Value {
    assert!(matches!(enabled_patterns, [1] | [1, 2] | [1, 2, 3]));
    let frozen: Vec<Value> = fs::read_to_string(raw)
        .expect("frozen trace")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let rec = frozen
        .iter()
        .find(|v| u(&v["p"]) == 37 && u(&v["seed"]) == 0 && v["line_powers"] == json!([0, 2]))
        .unwrap();
    let overall = Instant::now();
    let inst = construct_twist24(37, 2, 0).expect("canonical twist instance");
    let n = inst.core.n;
    assert_eq!(n, 1_873_633);
    assert_eq!(inst.core.g.x, e(&rec["generator_x"]));
    assert_eq!(inst.core.g.y, e(&rec["generator_y"]));
    assert_eq!(inst.core.q.x, e(&rec["target_x"]));
    assert_eq!(inst.core.q.y, e(&rec["target_y"]));
    let c = inst.core.curve();
    let t2 = E4([0, 0, 1, 0]);
    c.f.reset_muls();
    c.reset_ops();
    let base_started = Instant::now();
    let mut rng_line0 = StdRng::seed_from_u64(0x1A4F_01D5);
    let mut base: Vec<Pt4> = (1..37)
        .filter_map(|x| c.lift_x(&c.f.from_fp(x), &mut rng_line0))
        .collect();
    let mut rng_line2 = StdRng::seed_from_u64(0x1A4F_01D5 ^ 2);
    base.extend((1..37).filter_map(|x| c.lift_x(&c.f.scale(&t2, x), &mut rng_line2)));
    let prior_base: Vec<Pt4> = rec["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    assert_eq!(base, prior_base);
    assert_eq!(base.len(), 48);
    assert_eq!(base[0], inst.core.g);
    assert!(base.iter().all(|pt| c.on_curve(pt)));
    let base_fp_muls = c.f.muls();
    let base_group_ops = c.ops();
    let base_ms_l0 = base_started.elapsed().as_secs_f64() * 1e3;
    c.f.reset_muls();
    c.reset_ops();
    let map_started = Instant::now();
    let map = orbit_map(&inst, &c, &base);
    let map_fp_muls = c.f.muls();
    let map_group_ops = c.ops();
    let map_ms_l0 = map_started.elapsed().as_secs_f64() * 1e3;
    let by_x: HashMap<E4, usize> = base.iter().enumerate().map(|(i, pt)| (pt.x, i)).collect();
    let mut rng_solver = StdRng::seed_from_u64(0x4D49_5845_4450_4F4C);
    c.f.reset_muls();
    c.reset_ops();
    let pre_started = Instant::now();
    let s4 = SymS4::precompute(&c, &mut rng_solver);
    let precompute_fp_muls = c.f.muls();
    let precompute_ms_l0 = pre_started.elapsed().as_secs_f64() * 1e3;
    assert_eq!(s4.precompute_muls, precompute_fp_muls);
    let mut matrix = Matrix::new(n);
    let mut selected = Vec::new();
    let mut traces = Vec::new();
    let mut stream_external_muls = 0u64;
    let mut stream_internal_muls = 0u64;
    let mut stream_group_ops = 0u64;
    let mut failed_systems = 0u64;
    for (query_index, z) in rec["query_trace"]
        .as_array()
        .unwrap()
        .iter()
        .take(61)
        .enumerate()
    {
        let (a, b) = (u(&z[0]), u(&z[1]));
        c.f.reset_muls();
        c.reset_ops();
        let residual_started = Instant::now();
        let residual = c.add(&c.mul(&inst.core.g, a), &c.mul(&inst.core.q, b));
        let residual_fp_muls = c.f.muls();
        let residual_group_ops = c.ops();
        let residual_ms_l0 = residual_started.elapsed().as_secs_f64() * 1e3;
        stream_external_muls += residual_fp_muls;
        stream_group_ops += residual_group_ops;
        let mut patterns = Vec::new();
        for &k in enabled_patterns {
            let result =
                collect_mixed_pattern(&c, &base, &by_x, &residual, k, &s4, &mut rng_solver);
            stream_external_muls += result.external_fp_muls;
            stream_internal_muls += result.internal_fp_muls;
            stream_group_ops += result.group_ops;
            failed_systems += result.system_failures;
            for terms in &result.relations {
                if matrix.rank == 4 {
                    continue;
                }
                let (row, rhs) = relation_row(terms, &map, a, b, n);
                if matrix.insert(row, rhs) {
                    selected.push(json!({"query_index":query_index,"k":k,"a":a,"b":b,
                        "terms":terms,"row":row,"rhs":rhs,"rank":matrix.rank}));
                }
            }
            patterns.push(json!({"k":k,"signed_relations":result.relations,
                "systems":result.systems,"system_failures":result.system_failures,
                "field_solutions":result.field_solutions,"split_failures":result.split_failures,
                "foreign_or_repeat":result.foreign_or_repeat,
                "external_fp_muls":result.external_fp_muls,
                "internal_fp_muls":result.internal_fp_muls,"group_ops":result.group_ops,
                "wall_ms_l0":result.wall_ms_l0}));
        }
        eprintln!(
            "query {query_index}: anchored rank {}, selected {}",
            matrix.rank,
            selected.len()
        );
        traces.push(json!({"query_index":query_index,"a":a,"b":b,
            "recorded_all_pattern_hits":u(&z[2]),"patterns":patterns,
            "residual_fp_muls":residual_fp_muls,"residual_group_ops":residual_group_ops,
            "residual_ms_l0":residual_ms_l0,"rank_after":matrix.rank}));
        if matrix.rank == 4 {
            break;
        }
    }
    let solution = matrix.solution();
    c.f.reset_muls();
    c.reset_ops();
    let verify_started = Instant::now();
    let mut factor_logs_verified = false;
    let mut recovered = None;
    let mut physical_factor_logs = Vec::new();
    if let Some(logs) = solution {
        let target_log = logs[3];
        let reps_logs = [1, logs[0], logs[1], logs[2]];
        let reps_ok = map
            .reps
            .iter()
            .enumerate()
            .all(|(j, &id)| c.mul(&inst.core.g, reps_logs[j]) == base[id]);
        let points_ok = base.iter().enumerate().all(|(id, pt)| {
            c.mul(&inst.core.g, mm(reps_logs[map.col[id]], map.coef[id], n)) == *pt
        });
        factor_logs_verified = reps_ok && points_ok;
        physical_factor_logs = (0..base.len())
            .map(|id| mm(reps_logs[map.col[id]], map.coef[id], n))
            .collect();
        if factor_logs_verified && c.mul(&inst.core.g, target_log) == inst.core.q {
            recovered = Some(target_log);
        }
    }
    let verify_fp_muls = c.f.muls();
    let verify_group_ops = c.ops();
    let verify_ms_l0 = verify_started.elapsed().as_secs_f64() * 1e3;
    let ic_wall_ms_l0 = overall.elapsed().as_secs_f64() * 1e3;
    let rho_started = Instant::now();
    let rho = rho_twist24(&inst, 0);
    let rho_wall_ms_l0 = rho_started.elapsed().as_secs_f64() * 1e3;
    assert!(rho.correct && rho.recovered == Some(271));
    let status = if recovered == Some(271) && failed_systems == 0 {
        "pass"
    } else {
        "partial"
    };
    let counted_ic_fp_muls = inst.model_fp_muls
        + inst.twist_geometry_fp_muls
        + inst.cubic_eigen_fp_muls
        + inst.frob_eigen_fp_muls
        + base_fp_muls
        + map_fp_muls
        + precompute_fp_muls
        + stream_external_muls
        + stream_internal_muls
        + verify_fp_muls;
    let counted_ic_group_ops = inst.cubic_eigen_group_ops
        + inst.frob_eigen_group_ops
        + base_group_ops
        + map_group_ops
        + stream_group_ops
        + verify_group_ops;
    json!({"status":status,"p":37,"b0":2,"n":n,"seed":0,
        "enabled_patterns":enabled_patterns,
        "public_target_scalar_for_postcheck":271,"recovered":recovered,
        "solved_factor_and_target_logs":solution,"physical_factor_logs":physical_factor_logs,
        "base_size":base.len(),"folded_columns":map.reps.len(),
        "base_coordinates":base.iter().map(|pt|json!([pt.x.0,pt.y.0])).collect::<Vec<_>>(),
        "generator_x":inst.core.g.x.0,"generator_y":inst.core.g.y.0,
        "target_x":inst.core.q.x.0,"target_y":inst.core.q.y.0,
        "lambda_cubic":inst.core.lambda,"lambda_frob":inst.lambda_frob,
        "orbit_column":map.col,"orbit_coefficient":map.coef,"orbit_representatives":map.reps,
        "fixture_input_fp_muls_excluded":inst.input_fp_muls,
        "fixture_input_group_ops_excluded":inst.input_group_ops,
        "model_fp_muls":inst.model_fp_muls,"twist_geometry_fp_muls":inst.twist_geometry_fp_muls,
        "eigen_fp_muls":inst.cubic_eigen_fp_muls+inst.frob_eigen_fp_muls,
        "eigen_group_ops":inst.cubic_eigen_group_ops+inst.frob_eigen_group_ops,
        "base_fp_muls":base_fp_muls,"base_group_ops":base_group_ops,"base_ms_l0":base_ms_l0,
        "map_fp_muls":map_fp_muls,"map_group_ops":map_group_ops,"map_ms_l0":map_ms_l0,
        "s4_precompute_fp_muls":precompute_fp_muls,"s4_precompute_ms_l0":precompute_ms_l0,
        "stream_external_fp_muls":stream_external_muls,"stream_internal_fp_muls":stream_internal_muls,
        "stream_group_ops":stream_group_ops,"failed_systems":failed_systems,
        "matrix_modular_muls":matrix.modular_muls,"matrix_modular_inversions":matrix.modular_inversions,
        "anchored_rank":matrix.rank,"selected_relations":selected,"trace":traces,
        "factor_logs_verified":factor_logs_verified,
        "verify_fp_muls":verify_fp_muls,"verify_group_ops":verify_group_ops,"verify_ms_l0":verify_ms_l0,
        "counted_ic_fp_muls":counted_ic_fp_muls,"counted_ic_group_ops":counted_ic_group_ops,
        "ic_wall_ms_l0":ic_wall_ms_l0,"rho":rho,"rho_wall_ms_l0":rho_wall_ms_l0,
        "hash_allocator_cpu_unpriced":true,"rho_native_hash_cpu_unpriced":true,
        "missing_native_counters":["polynomial-solver hash probes and inserts",
            "relation-set tree operations","allocator CPU and requested peak bytes",
            "rho partition hashes, table lookups and inserts"]})
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!((2..=3).contains(&args.len()), "usage: quartic_twist_mixed_polynomial_ic MIXED_NDJSON [--patterns=1|--patterns=1,2|--patterns=1,2,3]");
    let patterns = if args.len() == 3 {
        args[2]
            .strip_prefix("--patterns=")
            .unwrap()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    } else {
        vec![1, 2, 3]
    };
    println!("{}", run(&args[1], &patterns));
}
