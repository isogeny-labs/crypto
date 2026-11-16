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
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/walk_impl_uncached.rs"
    ));
    type Pair = ([u64; 4], [u64; 4]);
    type StepResult = (Option<Pair>, u64, [Option<u64>; 3]);
    pub fn step(
        k: [u64; 4],
        kp: [u64; 4],
        kg: [u64; 4],
        k0: [u64; 4],
        t: [u64; 4],
        acts: &[(u64, [[u64; 4]; 4])],
        p: u64,
    ) -> StepResult {
        let choice = branch(k, p);
        // Proposed arm: only the selected double is evaluated.
        // None in the unused repair slot means not evaluated, not repair failure.
        if choice == 0 {
            let (kd, r0) = xadd_repaired(k, k, k0, t, acts, p);
            let (km, r1) = xadd_repaired(k, kp, kg, t, acts, p);
            (kd.zip(km), choice, [r0, r1, None])
        } else {
            let (km, r1) = xadd_repaired(k, kp, kg, t, acts, p);
            let (kpp, r2) = xadd_repaired(kp, kp, k0, t, acts, p);
            (km.zip(kpp), choice, [None, r1, r2])
        }
    }
    pub fn project_divisor(d: Divisor, t: &serde_json::Value, p: u64) -> Option<[u64; 4]> {
        project(d, t, p)
    }
    pub fn project_any(
        d: Divisor,
        j: &Jacobian,
        t: &serde_json::Value,
        _acts: &[(u64, [[u64; 4]; 4])],
        p: u64,
    ) -> Option<[u64; 4]> {
        // CCS §6.3 and §7.4 give the classical Kummer vector q followed by
        // the invertible C1 fast-theta map. This handles degree 0/1/2,
        // including the a0=0 zero-numerator locus of the older generic law.
        let q = match d.degree {
            0 => [0, 0, 0, 1],
            1 => {
                let x = sub(0, d.u[0], p);
                [0, 1, x, mul(x, x, p)]
            }
            2 => {
                let (a0, a1, b1) = (d.u[0], d.u[1], d.v[1]);
                let f = &j.polynomial;
                let z = sub(mul(a1, a1, p), a0, p);
                let xi4 = sub(
                    add(
                        add(mul(b1, b1, p), mul(z, sub(mul(f[5], a1, p), 0, p), p), p),
                        mul(a1, sub(f[3], mul(f[4], a1, p), p), p),
                        p,
                    ),
                    f[2],
                    p,
                );
                [1, sub(0, a1, p), a0, xi4]
            }
            _ => return None,
        };
        let (l, mu, nu) = (
            t["lambda"].as_u64().unwrap(),
            t["mu"].as_u64().unwrap(),
            t["nu"].as_u64().unwrap(),
        );
        let m0 = [
            mul(mu, add(l, nu, p), p),
            mul(mul(nu, l, p), add(1, mu, p), p),
            mul(nu, add(l, mu, p), p),
            mul(mul(mu, l, p), add(1, nu, p), p),
        ];
        let m1 = [
            sub(0, mu, p),
            sub(0, mul(nu, l, p), p),
            sub(0, nu, p),
            sub(0, mul(mu, l, p), p),
        ];
        let m2 = [add(mu, 1, p), add(l, nu, p), add(nu, 1, p), add(l, mu, p)];
        let scale = ["a", "b", "c", "d"].map(|key| t[key].as_u64().unwrap());
        let out = std::array::from_fn(|i| {
            let row = [m0[i], m1[i], m2[i], p - 1];
            mul(
                scale[i],
                (0..4).fold(0, |v, k| add(v, mul(row[k], q[k], p), p)),
                p,
            )
        });
        (out != [0; 4]).then_some(out)
    }
    pub fn same_k(x: [u64; 4], y: [u64; 4], p: u64) -> bool {
        same(x, y, p)
    }
}
mod signed {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/recover_impl_uncached.rs"
    ));
    pub fn curve_coeff(t: &Value, p: u64) -> Vec<u64> {
        curve(t, p)
    }
    pub fn general_coords(k: [u64; 4], t: &Value, p: u64) -> Option<[u64; 4]> {
        general(k, t, p)
    }
    #[allow(clippy::too_many_arguments)] // The unchanged N11 Recover wrapper.
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

struct Advance {
    next: Option<([u64; 4], [u64; 4])>,
    choice: u64,
    roots: [Option<u64>; 3],
    used_fallback: bool,
    fallback_validity_checks: u64,
    transition_checks: u64,
    validation_failed: bool,
}

#[allow(clippy::too_many_arguments)]
fn signed_fallback(
    j: &Jacobian,
    t: &Value,
    acts: &[(u64, [[u64; 4]; 4])],
    p: u64,
    r: u64,
    g: Divisor,
    q: Divisor,
    a: u64,
    b: u64,
    choice: u64,
) -> Option<([u64; 4], [u64; 4])> {
    let aa = (2 * a + choice) % r;
    let bb = (2 * b) % r;
    let d = j.add(j.mul(g, aa), j.mul(q, bb));
    let dplus = j.add(d, g);
    let d_valid = j.valid(d);
    let plus_valid = j.valid(dplus);
    if !d_valid || !plus_valid {
        return None;
    }
    walk::project_any(d, j, t, acts, p).zip(walk::project_any(dplus, j, t, acts, p))
}

#[allow(clippy::too_many_arguments)]
fn advance(
    k: [u64; 4],
    kp: [u64; 4],
    kg: [u64; 4],
    k0: [u64; 4],
    constants: [u64; 4],
    acts: &[(u64, [[u64; 4]; 4])],
    j: &Jacobian,
    t: &Value,
    p: u64,
    r: u64,
    g: Divisor,
    q: Divisor,
    a: u64,
    b: u64,
    validate_transition: bool,
) -> Advance {
    let (next, choice, roots) = walk::step(k, kp, kg, k0, constants, acts, p);
    if let Some((nk, nkp)) = next {
        if validate_transition {
            let aa = (2 * a + choice) % r;
            let bb = (2 * b) % r;
            let d = j.add(j.mul(g, aa), j.mul(q, bb));
            let dplus = j.add(d, g);
            let expected =
                walk::project_any(d, j, t, acts, p).zip(walk::project_any(dplus, j, t, acts, p));
            let valid = j.valid(d)
                && j.valid(dplus)
                && expected
                    .is_some_and(|(e, ep)| walk::same_k(nk, e, p) && walk::same_k(nkp, ep, p));
            return Advance {
                next: valid.then_some((nk, nkp)),
                choice,
                roots,
                used_fallback: false,
                fallback_validity_checks: 0,
                transition_checks: 2,
                validation_failed: !valid,
            };
        }
        return Advance {
            next,
            choice,
            roots,
            used_fallback: false,
            fallback_validity_checks: 0,
            transition_checks: 0,
            validation_failed: false,
        };
    }
    // The labels determine the signed successor without a discrete-log
    // oracle. Arithmetic here is native Jacobian arithmetic and is charged.
    let next = signed_fallback(j, t, acts, p, r, g, q, a, b, choice);
    Advance {
        next,
        choice,
        roots,
        used_fallback: true,
        fallback_validity_checks: 2,
        transition_checks: 0,
        validation_failed: false,
    }
}

pub fn run(output_dir: &str) {
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
    let validate_transitions =
        std::env::var("KUMMER_VALIDATE_TRANSITIONS").ok().as_deref() == Some("1");
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
                mut fallback_steps,
                mut fallback_failures,
                mut fallback_checks,
                mut transition_checks,
            ) = (
                0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
            );
            let mut recovered = None;
            let mut status = "candidate_cap";
            let cap = 100 * p + 10000;
            while attempts < cap {
                let Some(general) = signed::general_coords(k, t, p) else {
                    general_skips += 1;
                    let step = advance(
                        k,
                        kp,
                        kg,
                        k0,
                        consts,
                        acts,
                        &j,
                        t,
                        p,
                        r,
                        g,
                        q,
                        a,
                        b,
                        validate_transitions,
                    );
                    repairs += step.roots.iter().filter(|x| x.is_some()).count() as u64;
                    attempts += 1;
                    if step.used_fallback {
                        fallback_steps += 1;
                        fallback_checks += step.fallback_validity_checks;
                    }
                    transition_checks += step.transition_checks;
                    let Some((nk, nkp)) = step.next else {
                        if step.validation_failed {
                            status = "transition_mismatch";
                        } else {
                            fallback_failures += 1;
                            status = "fallback_projection_fail";
                        }
                        break;
                    };
                    a = (2 * a + step.choice) % r;
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
                let step = advance(
                    k,
                    kp,
                    kg,
                    k0,
                    consts,
                    acts,
                    &j,
                    t,
                    p,
                    r,
                    g,
                    q,
                    a,
                    b,
                    validate_transitions,
                );
                repairs += step.roots.iter().filter(|x| x.is_some()).count() as u64;
                attempts += 1;
                if step.used_fallback {
                    fallback_steps += 1;
                    fallback_checks += step.fallback_validity_checks;
                }
                transition_checks += step.transition_checks;
                let Some((nk, nkp)) = step.next else {
                    if step.validation_failed {
                        status = "transition_mismatch";
                    } else {
                        fallback_failures += 1;
                        status = "fallback_projection_fail";
                    }
                    break;
                };
                a = (2 * a + step.choice) % r;
                b = 2 * b % r;
                k = nk;
                kp = nkp;
            }
            if status == "verified" {
                solved += 1;
            }
            completed += 1;
            out.push_str(&format!("{}\n",json!({"schema":"kummer-labelled-fallback/v1","bits":bits,"p":p,"r":r,"target_index":v["target_index"],"workload_id":v["workload_id"],"planted":v["planted"],"source_scalar_verified":v["source_scalar_verified"],"status":status,"recovered":recovered,"source_verified":status=="verified","attempts":attempts,"split_relations":splits,"recover_success":recovery_ok,"recover_fail":recovery_fail,"general_conversion_skips":general_skips,"xadd_repairs":repairs,"walk_restarts":restarts,"fallback_steps":fallback_steps,"fallback_failures":fallback_failures,"fallback_validity_checks":fallback_checks,"transition_checks":transition_checks,"transition_validation_enabled":validate_transitions,"graph_cycles":graph.cycles,"graph_operations":graph.operations,"walk_ops":walk::read_ops(),"recover_ops":signed::read_ops(),"field_counts":j.f.counts()})));
        }
        fs::create_dir_all(output_dir).unwrap();
        fs::write(format!("{output_dir}/collector-{bits}.jsonl"), out).unwrap();
        if completed == max_targets {
            break;
        }
    }
    println!("completed={completed} solved={solved}");
}
fn main() {
    run("research/kummer_differential_collector_20261022/n19-labelled-fallback/results/collector");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c2_oracle(d: Divisor, t: &Value, f: &[u64], p: u64) -> [u64; 4] {
        assert_eq!(d.degree, 2);
        let mm = |x: u64, y: u64| x * y % p;
        let aa = |x: u64, y: u64| (x + y) % p;
        let ss = |x: u64, y: u64| (x + p - y) % p;
        let (a0, a1, b1) = (d.u[0], d.u[1], d.v[1]);
        let (l, mu, nu) = (
            t["lambda"].as_u64().unwrap(),
            t["mu"].as_u64().unwrap(),
            t["nu"].as_u64().unwrap(),
        );
        let mut c = 1;
        let mut lin = 0;
        let mut prev_lin = 0;
        let mut rem0_over_a0 = 0;
        for (n, &coef) in f.iter().enumerate() {
            if n >= 2 {
                rem0_over_a0 = ss(rem0_over_a0, mm(coef, prev_lin));
            }
            let nc = ss(0, mm(a0, lin));
            let nl = ss(c, mm(a1, lin));
            prev_lin = lin;
            (c, lin) = (nc, nl);
        }
        let terms = [
            mm(ss(mu, a0), aa(aa(l, a1), nu)),
            mm(ss(mm(nu, l), a0), aa(aa(1, a1), mu)),
            mm(ss(nu, a0), aa(aa(l, a1), mu)),
            mm(ss(mm(mu, l), a0), aa(aa(1, a1), nu)),
        ];
        let scale = ["a", "b", "c", "d"].map(|key| t[key].as_u64().unwrap());
        std::array::from_fn(|i| mm(scale[i], ss(ss(terms[i], mm(b1, b1)), rem0_over_a0)))
    }

    #[test]
    fn all_models_force_zero_xadd_and_reconstruct_signed_successor() {
        std::env::set_current_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
            .unwrap();
        let theta_rows =
            fs::read_to_string("research/kummer_theta_20261022/results/theta-models.jsonl")
                .unwrap();
        let theta: BTreeMap<u64, Value> = theta_rows
            .lines()
            .map(|line| {
                let v: Value = serde_json::from_str(line).unwrap();
                (v["p"].as_u64().unwrap(), v["selected"]["constants"].clone())
            })
            .collect();
        let mut seen = HashSet::new();
        for bits in [16, 20, 24, 28, 32, 36, 40] {
            let input = fs::read_to_string(format!(
                "research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl"
            ))
            .unwrap();
            for line in input.lines() {
                let v: Value = serde_json::from_str(line).unwrap();
                let p = v["p"].as_u64().unwrap();
                if !seen.insert(p) {
                    continue;
                }
                let t = &theta[&p];
                let f = signed::curve_coeff(t, p);
                let j = Jacobian {
                    f: Field::new(p, 1, 0),
                    polynomial: f[..6].to_vec(),
                };
                let cases = v["cases"].as_array().unwrap();
                let g: Divisor = serde_json::from_value(
                    cases[1]["coordinate_witness"]["jacobian_expected"].clone(),
                )
                .unwrap();
                let q: Divisor = serde_json::from_value(
                    cases[2]["coordinate_witness"]["jacobian_expected"].clone(),
                )
                .unwrap();
                let kg = parse_vec4(&cases[1]["coordinate_witness"]["k_expected"]);
                let k = parse_vec4(&cases[2]["coordinate_witness"]["k_expected"]);
                let kp = parse_vec4(&cases[3]["coordinate_witness"]["k_expected"]);
                let constants = ["A", "B", "C", "D"].map(|key| t[key].as_u64().unwrap());
                let r = v["r"].as_u64().unwrap();
                // A zero difference forces the cleared xADD vector to zero
                // in both doubling calls, hence at least one chosen output.
                let forced = advance(
                    k,
                    kp,
                    kg,
                    [0; 4],
                    constants,
                    &[],
                    &j,
                    t,
                    p,
                    r,
                    g,
                    q,
                    0,
                    1,
                    true,
                );
                assert!(forced.used_fallback, "p={p}");
                let (out, out_plus) = forced.next.expect("signed fallback projection");
                let aa = forced.choice % r;
                let bb = 2 % r;
                let d = j.add(j.mul(g, aa), j.mul(q, bb));
                let dplus = j.add(d, g);
                let independent_d = if forced.choice == 0 {
                    j.add(q, q)
                } else {
                    j.add(j.add(q, q), g)
                };
                assert_eq!(d, independent_d, "signed label invariant p={p}");
                assert!(j.valid(d) && j.valid(dplus));
                if d.degree == 2 {
                    assert!(walk::same_k(out, c2_oracle(d, t, &f, p), p));
                }
                if dplus.degree == 2 {
                    assert!(walk::same_k(out_plus, c2_oracle(dplus, t, &f, p), p));
                }
                assert!(walk::same_k(
                    walk::project_any(g, &j, t, &[], p).unwrap(),
                    kg,
                    p
                ));
                assert!(walk::same_k(
                    walk::project_any(q, &j, t, &[], p).unwrap(),
                    k,
                    p
                ));
                assert!(walk::same_k(
                    walk::project_any(j.add(q, g), &j, t, &[], p).unwrap(),
                    kp,
                    p
                ));
                let k0 = ["a", "b", "c", "d"].map(|key| t[key].as_u64().unwrap());
                let (mut regular_k, mut regular_kp) = (k, kp);
                let (mut label_a, mut label_b) = (0, 1);
                for _ in 0..3 {
                    let step = advance(
                        regular_k,
                        regular_kp,
                        kg,
                        k0,
                        constants,
                        &[],
                        &j,
                        t,
                        p,
                        r,
                        g,
                        q,
                        label_a,
                        label_b,
                        true,
                    );
                    assert!(!step.validation_failed, "regular transition p={p}");
                    assert_eq!(
                        step.transition_checks,
                        if step.used_fallback { 0 } else { 2 }
                    );
                    (regular_k, regular_kp) = step.next.expect("validated regular pair");
                    label_a = (2 * label_a + step.choice) % r;
                    label_b = 2 * label_b % r;
                }
                assert!(j.f.counts().jac_adds > 0);
                assert!(walk::project_any(Divisor::O, &j, t, &[], p).is_some());
                let zero_numerator = Divisor {
                    degree: 2,
                    u: [0, p - 1, 1],
                    v: [0, 0],
                };
                assert!(j.valid(zero_numerator));
                assert!(walk::project_divisor(zero_numerator, t, p).is_none());
                let repaired = walk::project_any(zero_numerator, &j, t, &[], p).unwrap();
                assert!(walk::same_k(
                    repaired,
                    c2_oracle(zero_numerator, t, &f, p),
                    p
                ));
                for x in [
                    0,
                    1,
                    t["lambda"].as_u64().unwrap(),
                    t["mu"].as_u64().unwrap(),
                    t["nu"].as_u64().unwrap(),
                ] {
                    let branch = Divisor {
                        degree: 1,
                        u: [sub_mod(0, x, p), 1, 0],
                        v: [0, 0],
                    };
                    assert!(j.valid(branch));
                    assert!(walk::project_any(branch, &j, t, &[], p).is_some());
                }
            }
        }
        assert_eq!(seen.len(), 21);
    }
}

pub fn fixture_check(input:&Value, forced:bool)->Value {
 let v=&input["target"]; let th=&input["theta"]["selected"]; let t=&th["constants"]; let p=v["p"].as_u64().unwrap(); let r=v["r"].as_u64().unwrap();
 let r0=th["r0"].as_u64().unwrap(); let delta=th["delta"].as_u64().unwrap();
 let acts:Vec<_>=input["model"]["torsion_actions"].as_array().unwrap().iter().map(|z|(mul_mod(sub_mod(z["root"].as_u64().unwrap(),r0,p),inv_mod(delta,p),p),parse_matrix(&z["action"]["matrix"]))).collect();
 let f=signed::curve_coeff(t,p);let j=Jacobian{f:Field::new(p,1,0),polynomial:f[..6].to_vec()};
 let c=v["cases"].as_array().unwrap();let g:Divisor=serde_json::from_value(c[1]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();let q:Divisor=serde_json::from_value(c[2]["coordinate_witness"]["jacobian_expected"].clone()).unwrap();
 let kg=parse_vec4(&c[1]["coordinate_witness"]["k_expected"]);let mut k=parse_vec4(&c[2]["coordinate_witness"]["k_expected"]);let mut kp=parse_vec4(&c[3]["coordinate_witness"]["k_expected"]);
 let k0=if forced{[0;4]}else{["a","b","c","d"].map(|z|t[z].as_u64().unwrap())};let constants=["A","B","C","D"].map(|z|t[z].as_u64().unwrap());
 let(mut a,mut b)=(0,1);let mut rows=vec![];
 for step_index in 0..if forced{1}else{3}{
 walk::reset_ops();signed::reset_ops();let x=advance(k,kp,kg,k0,constants,&acts,&j,t,p,r,g,q,a,b,true);let ops=walk::read_ops();
 let labels=((2*a+x.choice)%r,2*b%r);rows.push(json!({"p":p,"forced":forced,"step":step_index,"next":x.next,"choice":x.choice,"labels":labels,"fallback":x.used_fallback,"fallback_checks":x.fallback_validity_checks,"transition_checks":x.transition_checks,"validation_failed":x.validation_failed,"repairs":x.roots,"ops":ops}));
 assert!(!x.validation_failed);let(nk,nkp)=x.next.expect("fixture successor");k=nk;kp=nkp;(a,b)=labels;
 }
 json!(rows)
}
