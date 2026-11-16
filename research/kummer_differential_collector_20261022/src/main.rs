#![allow(dead_code)]
use five_leads_study::three_leads::{Divisor, Field, Jacobian, Ring, Rng, SignedGraph};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
};
fn parse_vec4(v: &Value) -> [u64; 4] {
    std::array::from_fn(|i| v[i].as_u64().unwrap())
}
fn parse_matrix(v: &Value) -> [[u64; 4]; 4] {
    std::array::from_fn(|i| parse_vec4(&v[i]))
}
fn sub_mod(x: u64, y: u64, p: u64) -> u64 {
    (x + p - y) % p
}
fn mul_mod(x: u64, y: u64, p: u64) -> u64 {
    x * y % p
}
fn inv_mod(x: u64, p: u64) -> u64 {
    let mut base = x;
    let mut exp = p - 2;
    let mut z = 1;
    while exp > 0 {
        if exp & 1 != 0 {
            z = mul_mod(z, base, p);
        }
        base = mul_mod(base, base, p);
        exp >>= 1;
    }
    z
}

mod walk {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/walk_impl.rs"));
    pub fn step(
        k: [u64; 4],
        kp: [u64; 4],
        kg: [u64; 4],
        k0: [u64; 4],
        t: [u64; 4],
        acts: &[(u64, [[u64; 4]; 4])],
        p: u64,
    ) -> (Option<([u64; 4], [u64; 4])>, u64, [Option<u64>; 3]) {
        let choice = branch(k, p);
        let (kd, r0) = xadd_repaired(k, k, k0, t, acts, p);
        let (km, r1) = xadd_repaired(k, kp, kg, t, acts, p);
        let (kpp, r2) = xadd_repaired(kp, kp, k0, t, acts, p);
        let pair = if choice == 0 { kd.zip(km) } else { km.zip(kpp) };
        (pair, choice, [r0, r1, r2])
    }
    pub fn project_divisor(d: Divisor, t: &serde_json::Value, p: u64) -> Option<[u64; 4]> {
        project(d, t, p)
    }
    pub fn project_any(
        d: Divisor,
        j: &Jacobian,
        t: &serde_json::Value,
        acts: &[(u64, [[u64; 4]; 4])],
        p: u64,
    ) -> Option<[u64; 4]> {
        project_full(d, j, t, acts, p)
    }
}
mod signed {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/recover_impl.rs"));
    pub fn curve_coeff(t: &Value, p: u64) -> Vec<u64> {
        curve(t, p)
    }
    pub fn general_coords(k: [u64; 4], t: &Value, p: u64) -> Option<[u64; 4]> {
        general(k, t, p)
    }
    pub fn lift(
        anchor: Divisor,
        kq: [u64; 4],
        kplus: [u64; 4],
        kp: [u64; 4],
        t: &Value,
        acts: &[(u64, [[u64; 4]; 4])],
        j: &Jacobian,
        p: u64,
    ) -> (Option<Divisor>, &'static str, usize) {
        let constants = ["A", "B", "C", "D"].map(|z| t[z].as_u64().unwrap());
        let (km, _) = xadd_repaired(kq, kp, kplus, constants, acts, p);
        let (q, plus, minus) = (
            general(kq, t, p),
            general(kplus, t, p),
            km.and_then(|x| general(x, t, p)),
        );
        if let (Some(q), Some(plus), Some(minus)) = (q, plus, minus) {
            let f = curve(t, p);
            let (mut out, mut status) = recover(anchor, q, plus, minus, &f, p);
            let mut tested = 0;
            if out.is_none() && status == "D_zero" {
                let (candidate, n) = exceptional_lift(anchor, q, plus, &f, j, p);
                out = candidate;
                tested = n;
                if out.is_some() {
                    status = "enumerated_exception_lift";
                }
            }
            (out, status, tested)
        } else {
            (None, "general_conversion_inapplicable", 0)
        }
    }
}

fn main() {
    let theta_rows =
        fs::read_to_string("research/kummer_theta_20261022/results/theta-models.jsonl").unwrap();
    let mut theta = BTreeMap::new();
    let mut normal = BTreeMap::new();
    for line in theta_rows.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let p = v["p"].as_u64().unwrap();
        theta.insert(p, v["selected"]["constants"].clone());
        normal.insert(
            p,
            (
                v["selected"]["r0"].as_u64().unwrap(),
                v["selected"]["delta"].as_u64().unwrap(),
            ),
        );
    }
    let model_rows =
        fs::read_to_string("research/kummer_coordinates_20261022/results/generic-models.jsonl")
            .unwrap();
    let mut actions = BTreeMap::new();
    for line in model_rows.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let p = v["p"].as_u64().unwrap();
        let (r0, delta) = normal[&p];
        let acts: Vec<_> = v["torsion_actions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|z| {
                (
                    mul_mod(
                        sub_mod(z["root"].as_u64().unwrap(), r0, p),
                        inv_mod(delta, p),
                        p,
                    ),
                    parse_matrix(&z["action"]["matrix"]),
                )
            })
            .collect();
        actions.insert(p, acts);
    }
    let max_targets = std::env::var("KUMMER_TARGET_CAP")
        .ok()
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(1260);
    let start_index = std::env::var("KUMMER_START_INDEX")
        .ok()
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(0);
    let mut completed = 0;
    let mut solved = 0;
    let mut seen_targets = 0;
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        let input = fs::read_to_string(format!(
            "research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl"
        ))
        .unwrap();
        let mut out = String::new();
        for line in input.lines() {
            let global_index = seen_targets;
            seen_targets += 1;
            if global_index < start_index {
                continue;
            }
            if completed == max_targets {
                break;
            }
            let v: Value = serde_json::from_str(line).unwrap();
            let p = v["p"].as_u64().unwrap();
            let r = v["r"].as_u64().unwrap();
            walk::reset_ops();
            signed::reset_ops();
            let t = &theta[&p];
            let acts = &actions[&p];
            let f = signed::curve_coeff(t, p);
            let j = Jacobian {
                f: Field::new(p, 1, 0),
                polynomial: f[..6].to_vec(),
            };
            let cases = v["cases"].as_array().unwrap();
            let g: Divisor =
                serde_json::from_value(cases[1]["coordinate_witness"]["jacobian_expected"].clone())
                    .unwrap();
            let q: Divisor =
                serde_json::from_value(cases[2]["coordinate_witness"]["jacobian_expected"].clone())
                    .unwrap();
            let kg = parse_vec4(&cases[1]["coordinate_witness"]["k_expected"]);
            let mut k = parse_vec4(&cases[2]["coordinate_witness"]["k_expected"]);
            let mut kp = parse_vec4(&cases[3]["coordinate_witness"]["k_expected"]);
            let k0 = ["a", "b", "c", "d"].map(|key| t[key].as_u64().unwrap());
            let consts = ["A", "B", "C", "D"].map(|key| t[key].as_u64().unwrap());
            walk::setup_constants(consts, p);
            signed::setup_constants(consts, p);
            let ring = Ring::new(r);
            let mut graph = SignedGraph::new(ring.clone());
            let mut nodes = HashMap::new();
            let mut relation_seen = HashSet::new();
            let mut walk_seen = HashSet::new();
            let mut rng = Rng(v["algorithm_seed"]
                .as_str()
                .unwrap()
                .parse::<u64>()
                .unwrap()
                ^ 0x6b756d6d6572);
            let (mut a, mut b) = (0u64, 1u64);
            let (
                mut attempts,
                mut splits,
                mut recovery_ok,
                mut recovery_fail,
                mut repairs,
                mut restarts,
                mut general_skips,
            ) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
            let mut recovered = None;
            let mut status = "candidate_cap";
            let cap = 100 * p + 10000;
            while attempts < cap {
                let Some(general) = signed::general_coords(k, t, p) else {
                    general_skips += 1;
                    let (next, choice, roots) = walk::step(k, kp, kg, k0, consts, acts, p);
                    repairs += roots.iter().filter(|x| x.is_some()).count() as u64;
                    attempts += 1;
                    let Some((nk, nkp)) = next else {
                        status = "xadd_inapplicable";
                        break;
                    };
                    a = (2 * a + choice) % r;
                    b = 2 * b % r;
                    k = nk;
                    kp = nkp;
                    continue;
                };
                let key = (general[1], general[2], general[3]);
                if !walk_seen.insert(key) {
                    restarts += 1;
                    walk_seen.clear();
                    a = rng.next() % r;
                    b = rng.next() % r;
                    let d = j.add(j.mul(g, a), j.mul(q, b));
                    let dplus = j.add(d, g);
                    let Some(kk) = walk::project_any(d, &j, t, acts, p) else {
                        status = "restart_projection_fail";
                        break;
                    };
                    let Some(kpp) = walk::project_any(dplus, &j, t, acts, p) else {
                        status = "restart_projection_fail";
                        break;
                    };
                    k = kk;
                    kp = kpp;
                    continue;
                }
                let a0 = general[2];
                let a1 = sub_mod(0, general[1], p);
                let disc = j.f.sub(j.f.sq(a1), j.f.mul(4, a0));
                if let Some(root) = j.f.sqrt(disc) {
                    let x0 = j.f.mul(j.f.sub(root, a1), p.div_ceil(2));
                    let x1 = j.f.mul(j.f.sub(j.f.neg(root), a1), p.div_ceil(2));
                    if x0 != x1 {
                        let (signed_d, rec_status, _tested) =
                            signed::lift(g, k, kp, kg, t, acts, &j, p);
                        if let Some(d) = signed_d {
                            recovery_ok += 1;
                            if !relation_seen.insert(d.min(j.neg(d))) {
                                // Preserve one relation per signed divisor, as in baseline.
                            } else {
                                let y0 = j.f.add(d.v[0], j.f.mul(d.v[1], x0));
                                let y1 = j.f.add(d.v[0], j.f.mul(d.v[1], x1));
                                let n0 = *nodes.entry(x0).or_insert_with(|| graph.node());
                                let n1 = *nodes.entry(x1).or_insert_with(|| graph.node());
                                splits += 1;
                                recovered = graph.edge(
                                    n0,
                                    if y0 <= p / 2 { 1 } else { -1 },
                                    n1,
                                    if y1 <= p / 2 { 1 } else { -1 },
                                    a,
                                    b,
                                );
                                if let Some(ans) = recovered {
                                    if j.mul(g, ans) == q && ans == v["planted"].as_u64().unwrap() {
                                        status = "verified";
                                    } else {
                                        status = "wrong_scalar";
                                    }
                                    break;
                                }
                            }
                        } else {
                            recovery_fail += 1;
                            if rec_status == "D_zero" {
                                status = "recover_D_zero";
                            }
                        }
                    }
                }
                let (next, choice, roots) = walk::step(k, kp, kg, k0, consts, acts, p);
                repairs += roots.iter().filter(|x| x.is_some()).count() as u64;
                attempts += 1;
                let Some((nk, nkp)) = next else {
                    status = "xadd_inapplicable";
                    break;
                };
                a = (2 * a + choice) % r;
                b = 2 * b % r;
                k = nk;
                kp = nkp;
            }
            if status == "verified" {
                solved += 1;
            }
            completed += 1;
            out.push_str(&format!("{}\n",json!({"schema":"kummer-differential-collector/v1","bits":bits,"p":p,"r":r,"target_index":v["target_index"],"workload_id":v["workload_id"],"planted":v["planted"],"source_scalar_verified":v["source_scalar_verified"],"status":status,"recovered":recovered,"source_verified":status=="verified","attempts":attempts,"split_relations":splits,"recover_success":recovery_ok,"recover_fail":recovery_fail,"general_conversion_skips":general_skips,"xadd_repairs":repairs,"walk_restarts":restarts,"graph_cycles":graph.cycles,"graph_operations":graph.operations,"walk_ops":walk::read_ops(),"recover_ops":signed::read_ops(),"field_counts":j.f.counts()})));
        }
        fs::create_dir_all("research/kummer_differential_collector_20261022/results/cached")
            .unwrap();
        fs::write(
            format!(
                "research/kummer_differential_collector_20261022/results/cached/collector-{bits}.jsonl"
            ),
            out,
        )
        .unwrap();
        if completed == max_targets {
            break;
        }
    }
    println!("completed={completed} solved={solved}");
}
