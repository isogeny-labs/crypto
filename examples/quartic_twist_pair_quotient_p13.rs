//! Quotient the p=13 twist pair table under its exact 12 abscissa actions.
//! Frozen protocol: research/twist_index_calculus_20261020/QUARTIC_TWIST_LINE_PROTOCOL.md.
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::env;
use std::fs;
use std::time::Instant;

use crypto_lib::cryptanalysis::gaudry_quartic::{mitm_signed, pair_table, Curve4, Pt4, E4};
use crypto_lib::cryptanalysis::residual_walk::pow_mod;
use serde_json::{json, Value};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pair {
    i: usize,
    j: usize,
    s: i8,
}

#[derive(Clone, Copy)]
struct SignedIndex {
    index: usize,
    sign: i8,
}

fn u(v: &Value) -> u64 {
    v.as_u64().unwrap()
}
fn e(v: &Value) -> E4 {
    let a = v.as_array().unwrap();
    E4([u(&a[0]), u(&a[1]), u(&a[2]), u(&a[3])])
}
fn point(v: &Value) -> Pt4 {
    Pt4 {
        x: e(&v[0]),
        y: e(&v[1]),
        inf: false,
    }
}
fn psi(c: &Curve4, pt: Pt4, z: u64, h2: E4, h3: E4) -> Pt4 {
    if pt.inf {
        return pt;
    }
    Pt4 {
        x: c.f.mul(&h2, &c.f.frob_p(&pt.x, z)),
        y: c.f.mul(&h3, &c.f.frob_p(&pt.y, z)),
        inf: false,
    }
}
fn action(c: &Curve4, mut pt: Pt4, j: usize, k: usize, z: u64, h2: E4, h3: E4, omega: u64) -> Pt4 {
    for _ in 0..j {
        pt = psi(c, pt, z, h2, h3);
    }
    if !pt.inf {
        pt.x = c.f.scale(&pt.x, pow_mod(omega, k as u64, c.f.p));
    }
    pt
}

fn canonical_x(c: &Curve4, pt: Pt4, z: u64, h2: E4, h3: E4, omega: u64) -> E4 {
    assert!(!pt.inf);
    (0..4)
        .flat_map(|j| (0..3).map(move |k| action(c, pt, j, k, z, h2, h3, omega).x))
        .min_by_key(|x| x.0)
        .unwrap()
}

fn transformed_pair(pair: Pair, map: &[SignedIndex]) -> Pair {
    let a = map[pair.i];
    let b = map[pair.j];
    let sign = a.sign * pair.s * b.sign;
    assert_ne!(a.index, b.index);
    Pair {
        i: a.index.min(b.index),
        j: a.index.max(b.index),
        s: sign,
    }
}

fn quotient_decompositions(
    c: &Curve4,
    base: &[Pt4],
    r: &Pt4,
    reps: &HashMap<E4, Vec<(Pair, Pt4)>>,
    maps: &[Vec<SignedIndex>],
    z: u64,
    h2: E4,
    h3: E4,
    omega: u64,
) -> Vec<Vec<(usize, i64)>> {
    let mut out = BTreeSet::new();
    for k in 0..base.len() {
        for sk in [1i64, -1] {
            let pk = if sk == 1 { base[k] } else { c.neg(&base[k]) };
            let tk = c.sub(r, &pk);
            for l in (k + 1)..base.len() {
                for sl in [1i64, -1] {
                    let pl = if sl == 1 { base[l] } else { c.neg(&base[l]) };
                    let needed = c.sub(&tk, &pl);
                    if needed.inf {
                        continue;
                    }
                    let key = canonical_x(c, needed, z, h2, h3, omega);
                    let Some(bucket) = reps.get(&key) else {
                        continue;
                    };
                    for &(pair, pair_sum) in bucket {
                        for j in 0..4 {
                            for m in 0..3 {
                                let tag = j * 3 + m;
                                let image = action(c, pair_sum, j, m, z, h2, h3, omega);
                                let overall = if image == needed {
                                    1i64
                                } else if image == c.neg(&needed) {
                                    -1i64
                                } else {
                                    continue;
                                };
                                let a = maps[tag][pair.i];
                                let b = maps[tag][pair.j];
                                if [a.index, b.index].contains(&k)
                                    || [a.index, b.index].contains(&l)
                                {
                                    continue;
                                }
                                let mut terms = vec![
                                    (k, sk),
                                    (l, sl),
                                    (a.index, overall * a.sign as i64),
                                    (b.index, overall * pair.s as i64 * b.sign as i64),
                                ];
                                terms.sort_unstable();
                                out.insert(terms);
                            }
                        }
                    }
                }
            }
        }
    }
    out.into_iter().collect()
}

fn run(raw_path: &str) -> Value {
    let raw: Value = serde_json::from_str(&fs::read_to_string(raw_path).unwrap()).unwrap();
    assert_eq!((u(&raw["p"]), u(&raw["b0"]), u(&raw["n"])), (13, 4, 28_753));
    let c = Curve4::with_params(13, E4::ZERO, E4([0, 0, 0, 4]));
    let base: Vec<_> = raw["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    let g = Pt4 {
        x: e(&raw["generator_x"]),
        y: e(&raw["generator_y"]),
        inf: false,
    };
    let q = Pt4 {
        x: e(&raw["target_x"]),
        y: e(&raw["target_y"]),
        inf: false,
    };
    assert_eq!(c.mul(&g, 271), q);
    let trace = raw["query_trace"].as_array().unwrap();
    assert_eq!(trace.len(), 7);
    let residuals: Vec<_> = trace
        .iter()
        .map(|item| c.add(&c.mul(&g, u(&item[0])), &c.mul(&q, u(&item[1]))))
        .collect();
    let t = E4([0, 1, 0, 0]);
    let z = c.f.pow(&t, 12).0[0];
    let h = c.f.pow(&t, (13u128.pow(4)) - 1 - 6);
    let h2 = c.f.sq(&h);
    let h3 = c.f.mul(&h2, &h);
    let omega = (2..13)
        .map(|x| pow_mod(x, 4, 13))
        .find(|&x| x != 1)
        .unwrap();
    let index: HashMap<_, _> = base.iter().enumerate().map(|(i, p)| (p.x, i)).collect();
    assert_eq!(index.len(), base.len());
    let full_c = Curve4::with_params(13, E4::ZERO, E4([0, 0, 0, 4]));
    full_c.reset_ops();
    full_c.f.reset_muls();
    let started_full = Instant::now();
    let full_table = pair_table(&full_c, &base);
    let full_setup_ms_l0 = started_full.elapsed().as_secs_f64() * 1e3;
    let full_setup_m = full_c.f.muls();
    let full_setup_g = full_c.ops();
    full_c.reset_ops();
    full_c.f.reset_muls();
    let started_full_query = Instant::now();
    let full: Vec<_> = residuals
        .iter()
        .map(|r| mitm_signed(&full_c, &base, &full_table, r))
        .collect();
    let full_query_ms_l0 = started_full_query.elapsed().as_secs_f64() * 1e3;
    let full_query_m = full_c.f.muls();
    let full_query_g = full_c.ops();

    let qc = Curve4::with_params(13, E4::ZERO, E4([0, 0, 0, 4]));
    qc.reset_ops();
    qc.f.reset_muls();
    let started_quotient = Instant::now();
    let mut maps = Vec::new();
    for j in 0..4 {
        for k in 0..3 {
            let mut map = Vec::new();
            for &pt in &base {
                let image = action(&qc, pt, j, k, z, h2, h3, omega);
                assert!(qc.on_curve(&image));
                let &at = index.get(&image.x).expect("base closure");
                let sign = if image.y == base[at].y {
                    1
                } else {
                    assert_eq!(image.y, qc.f.neg(&base[at].y));
                    -1
                };
                map.push(SignedIndex { index: at, sign });
            }
            maps.push(map);
        }
    }
    let mut seen = HashSet::new();
    let mut orbit_hist = BTreeMap::new();
    let mut reps: HashMap<E4, Vec<(Pair, Pt4)>> = HashMap::new();
    for i in 0..base.len() {
        for j in (i + 1)..base.len() {
            for s in [-1i8, 1] {
                let pair = Pair { i, j, s };
                if seen.contains(&pair) {
                    continue;
                }
                let orbit: BTreeSet<_> =
                    maps.iter().map(|map| transformed_pair(pair, map)).collect();
                assert!(orbit.contains(&pair));
                for &entry in &orbit {
                    assert!(seen.insert(entry));
                }
                *orbit_hist.entry(orbit.len()).or_insert(0u64) += 1;
                let representative = *orbit.iter().next().unwrap();
                let second = if representative.s == 1 {
                    base[representative.j]
                } else {
                    qc.neg(&base[representative.j])
                };
                let sum = qc.add(&base[representative.i], &second);
                assert!(!sum.inf);
                let key = canonical_x(&qc, sum, z, h2, h3, omega);
                reps.entry(key).or_default().push((representative, sum));
            }
        }
    }
    let quotient_setup_ms_l0 = started_quotient.elapsed().as_secs_f64() * 1e3;
    let quotient_setup_m = qc.f.muls();
    let quotient_setup_g = qc.ops();
    assert_eq!(seen.len(), base.len() * (base.len() - 1));
    let representatives: usize = reps.values().map(Vec::len).sum();
    assert_eq!(
        orbit_hist
            .iter()
            .map(|(size, count)| *size * *count as usize)
            .sum::<usize>(),
        seen.len()
    );
    qc.reset_ops();
    qc.f.reset_muls();
    let started_quotient_query = Instant::now();
    let quotient: Vec<_> = residuals
        .iter()
        .map(|r| quotient_decompositions(&qc, &base, r, &reps, &maps, z, h2, h3, omega))
        .collect();
    let quotient_query_ms_l0 = started_quotient_query.elapsed().as_secs_f64() * 1e3;
    let quotient_query_m = qc.f.muls();
    let quotient_query_g = qc.ops();
    let mut rows = Vec::new();
    for (index, (a, b)) in full.iter().zip(&quotient).enumerate() {
        assert_eq!(
            a, b,
            "full signed decomposition set differs at query {index}"
        );
        assert_eq!(a.len() as u64, u(&trace[index][2]));
        let selected = &raw["selected_relations"][index];
        let terms: Vec<_> = selected["terms"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| (u(&v[0]) as usize, v[1].as_i64().unwrap()))
            .collect();
        assert!(a.contains(&terms));
        rows.push(
            json!({"query_index":index,"a":u(&trace[index][0]),"b":u(&trace[index][1]),
            "decompositions":a.len(),"selected_present":true,"sets_equal":true}),
        );
    }
    json!({"p":13,"b0":4,"n":28753,"base_points":base.len(),
        "full_pair_entries":base.len()*(base.len()-1),
        "quotient_representatives":representatives,"quotient_hash_keys":reps.len(),
        "pair_orbit_hist":orbit_hist,"queries":rows,
        "full_setup_fp_muls":full_setup_m,"full_setup_group_ops":full_setup_g,
        "full_query_fp_muls":full_query_m,"full_query_group_ops":full_query_g,
        "quotient_setup_fp_muls":quotient_setup_m,"quotient_setup_group_ops":quotient_setup_g,
        "quotient_query_fp_muls":quotient_query_m,"quotient_query_group_ops":quotient_query_g,
        "full_setup_ms_l0":full_setup_ms_l0,"full_query_ms_l0":full_query_ms_l0,
        "quotient_setup_ms_l0":quotient_setup_ms_l0,"quotient_query_ms_l0":quotient_query_ms_l0,
        "status":"pass"})
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert_eq!(
        args.len(),
        2,
        "usage: quartic_twist_pair_quotient_p13 PILOT_RECORD"
    );
    println!("{}", run(&args[1]));
}
