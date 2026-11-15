// VALIDATOR TOOL (not producer code). research/validation_20261113.
//
// Rebuilds the instances of `negation_remeasure` with the committed library
// functions and re-runs every solver of every arm with the seeds that
// `src/bin/negation_remeasure.rs` uses, printing one JSON line per instance
// with the instance data (curve coefficients, points, Mumford divisors) and
// every solver's returned log and counted operations.  The validator's Python
// (`check_nr_certs.py`, on top of `research/validation_20261112/scripts/vlib.py`)
// then certifies every returned log with its own arithmetic and recomputes the
// means stored in `remeasure_*.json`, so that this dump is tied to the
// measured runs.
//
// usage: val_nr_dump v3 | cover | tower PRIMES [INSTANCES] [OLD_SEEDS] [SEEDS] [RHO_SEEDS] [FIRST_INSTANCE]
use crypto_lib::cryptanalysis::chained_transfer::bielliptic::Quotient;
use crypto_lib::cryptanalysis::chained_transfer::ec::Pt;
use crypto_lib::cryptanalysis::chained_transfer::hyper64::{build64, crosscheck, ic, H64Div};
use crypto_lib::cryptanalysis::chained_transfer::ic_control::Arm as IcArm;
use crypto_lib::cryptanalysis::chained_transfer::instrument::{
    convert_div, genus2_to_h64, ic_on_rung, instance_for_rung, solve_linear_counted,
};
use crypto_lib::cryptanalysis::chained_transfer::measure::{
    genus2_collect_rows, genus2_index_calculus, rho_on_ec, EcGroup,
};
use crypto_lib::cryptanalysis::chained_transfer::negation_remeasure::{
    cover_select, rho_full_ec, v3_first,
};
use crypto_lib::cryptanalysis::chained_transfer::pushforward::{
    collect_jc, e1_birthday_first_collision, e1_fibre_ic, e1_two_term_ic, jc_first_collision,
    solve_rows,
};
use crypto_lib::cryptanalysis::genus2_fp::Divisor;
use crypto_lib::cryptanalysis::ic_boundary::rho_reference;
use serde_json::{json, Value};

fn pt(q: &Pt) -> Value {
    match q {
        None => Value::Null,
        Some((x, y)) => json!([x.a, y.a, x.b, y.b]),
    }
}
fn dv(d: &Divisor) -> Value {
    json!({"u": d.u_coeffs(), "v": d.v_coeffs()})
}
fn hv(d: &H64Div) -> Value {
    json!({"u": d.u_coeffs(), "v": d.v_coeffs()})
}
fn arm_of(l: &str) -> IcArm {
    match l {
        "A" => IcArm::Full,
        "B" => IcArm::SingleLp,
        _ => IcArm::DoubleLp,
    }
}

fn v3() {
    for (i, &p) in [211u64, 419, 821, 1619].iter().enumerate() {
        let (cand, v) = v3_first(p, i).expect("candidate");
        let c = crypto_lib::cryptanalysis::chained_transfer::bielliptic::Bielliptic::new(
            p,
            crypto_lib::cryptanalysis::chained_transfer::negation_remeasure::candidate_coeffs(
                p, cand,
            ),
        )
        .expect("bielliptic");
        let mut runs = vec![];
        let ro = rho_on_ec(&v.e1, v.gen, v.target, v.r, v.seed);
        runs.push(json!({"arm": "old_rho", "group": "E1", "seed": v.seed, "k": ro.recovered,
            "ops": ro.group_ops.adds + ro.group_ops.doubles}));
        let io = genus2_index_calculus(&v.jac, &v.dg, &v.dt, v.r, p as usize, 50_000_000, v.seed);
        runs.push(json!({"arm": "recorded", "group": "J", "seed": v.seed, "k": io.recovered,
            "ops": io.group_ops, "la": io.la_field_ops, "trials": io.trials}));
        let rl = genus2_collect_rows(&v.jac, &v.dg, &v.dt, v.r, p as usize, 50_000_000, v.seed);
        let (lx, lnat) = solve_linear_counted(&rl.rows, rl.fb_size, v.r);
        runs.push(json!({"arm": "L", "group": "J", "seed": v.seed, "k": lx, "ops": rl.group_ops, "la": lnat.mul}));
        let h = genus2_to_h64(&v.jac).expect("word Jacobian");
        let hg = convert_div(&h, &v.dg).expect("G");
        let ht = convert_div(&h, &v.dt).expect("T");
        let inst = instance_for_rung(&h, v.jac_order, v.r, hg, ht, v.x).expect("instance");
        for l in ["A", "B", "C"] {
            for s in 0..8u64 {
                let res = ic_on_rung(&inst, arm_of(l), 500 + s, 40_000_000);
                runs.push(json!({"arm": l, "group": "H", "seed": 500 + s, "k": res.recovered,
                    "ops": res.group_ops(), "rel_mul": res.rel.mul, "la_mul": res.la.mul}));
            }
        }
        for neg in [true, false] {
            for s in 0..32u64 {
                let r = rho_full_ec(&v.e1, v.gen, v.target, v.r, 1000 + s, neg);
                runs.push(json!({"arm": if neg {"rho_neg"} else {"rho_plain"}, "group": "E1",
                    "seed": 1000 + s, "k": r.recovered, "ops": r.total_ops,
                    "native": r.native()}));
            }
        }
        println!("{}", json!({"target": "v3", "p": p, "candidate": cand, "c": c.c, "x0": c.x0,
            "e1": [v.e1.a2, v.e1.a4, v.e1.a6], "r": v.r, "x": v.x, "e1_order": v.e1_order,
            "jac_order": v.jac_order, "gen": pt(&v.gen), "target_pt": pt(&v.target),
            "jac_f": v.jac.f(), "dg": dv(&v.dg), "dt": dv(&v.dt),
            "h_f": h.f_coeffs(), "hg": hv(&hg), "ht": hv(&ht), "runs": runs}));
    }
}

fn cover(primes: &[u64]) {
    let all = [211u64, 419, 821, 1619, 3203, 6421, 12841];
    for &p in primes {
        let i = all.iter().position(|&q| q == p).expect("cover prime index");
        let ci = cover_select(p, i).expect("cover curve");
        let b = &ci.b;
        let (ge, r, e1) = (ci.gen, ci.r, b.e1);
        let jac_order = e1.order() * b.e2.order();
        let mut seeds = vec![];
        for k in 0..3u64 {
            let seed = ci.seed0 ^ (k * 0x1000);
            let x = (seed.wrapping_mul(2_246_822_519).wrapping_add(7)) % r;
            let te = e1.mul(&ge, x as u128);
            let dg = b.pull(Quotient::E1, &ge);
            let dt = b.pull(Quotient::E1, &te);
            let mut runs = vec![];
            let ro = rho_on_ec(&e1, ge, te, r, seed);
            runs.push(json!({"arm": "rho_e1_recorded", "group": "E1", "k": ro.recovered,
                "ops": ro.group_ops.adds + ro.group_ops.doubles}));
            let jc = collect_jc(&b.jac, &dg, &dt, r, p as usize, 50_000_000, seed);
            let (jr, jla) = solve_rows(&jc.rels, jc.nfb, r);
            runs.push(json!({"arm": "ic_jc", "group": "J", "k": jr, "ops": jc.ops, "la": jla}));
            let rows_l: Vec<(Vec<i64>, u64, u64)> =
                jc.rels.iter().map(|q| (q.row.clone(), q.alpha, q.beta)).collect();
            let (lx, lnat) = solve_linear_counted(&rows_l, jc.nfb, r);
            runs.push(json!({"arm": "ic_jc_L", "group": "J", "k": lx, "ops": jc.ops, "la": lnat.mul}));
            if let Some(c) = jc_first_collision(b, &jc.rels, r) {
                runs.push(json!({"arm": "ic_jc_first_collision", "group": "J", "k": c.log, "ops": c.ops}));
            }
            let (a4, _t, a4c) = e1_fibre_ic(b, &ge, &te, r, jc.rels.len(), 50_000_000, seed);
            runs.push(json!({"arm": "ic_e1_fibre_full", "group": "E1", "k": a4.recovered,
                "ops": a4.group_ops, "la": a4.la_mulmods, "lookups": a4.lookups}));
            if let Some(c) = a4c {
                runs.push(json!({"arm": "ic_e1_fibre_first_collision", "group": "E1", "k": c.log, "ops": c.ops}));
            }
            if let Some(c) = e1_birthday_first_collision(b, &ge, &te, r, 50_000_000, seed) {
                runs.push(json!({"arm": "birthday_e1_first_collision", "group": "E1", "k": c.log, "ops": c.ops}));
            }
            let a5 = e1_two_term_ic(b, &ge, &te, r, 50_000_000, seed);
            runs.push(json!({"arm": "ic_e1_two_term", "group": "E1", "k": a5.recovered,
                "ops": a5.group_ops, "la": a5.la_mulmods, "lookups": a5.lookups}));
            seeds.push(json!({"k": k, "seed": seed, "x": x, "target_pt": pt(&te),
                "dg": dv(&dg), "dt": dv(&dt), "runs": runs}));
        }
        // seed-0 instance: matched rho and arms A, B, C
        let x0 = (ci.seed0.wrapping_mul(2_246_822_519).wrapping_add(7)) % r;
        let te0 = e1.mul(&ge, x0 as u128);
        let h = genus2_to_h64(&b.jac).expect("word Jacobian");
        let dgs = b.pull(Quotient::E1, &ge);
        let dts = b.pull(Quotient::E1, &te0);
        let hg = convert_div(&h, &dgs).expect("G");
        let ht = convert_div(&h, &dts).expect("T");
        let inst = instance_for_rung(&h, jac_order, r, hg, ht, x0).expect("instance");
        let mut runs0 = vec![];
        for neg in [true, false] {
            for s in 0..32u64 {
                let rr = rho_full_ec(&e1, ge, te0, r, 2000 + s, neg);
                runs0.push(json!({"arm": if neg {"rho_neg"} else {"rho_plain"}, "group": "E1",
                    "seed": 2000 + s, "k": rr.recovered, "ops": rr.total_ops, "native": rr.native()}));
            }
        }
        for l in ["A", "B", "C"] {
            for s in 0..8u64 {
                let res = ic_on_rung(&inst, arm_of(l), 500 + s, 40_000_000);
                runs0.push(json!({"arm": l, "group": "H", "seed": 500 + s, "k": res.recovered,
                    "ops": res.group_ops(), "rel_mul": res.rel.mul, "la_mul": res.la.mul}));
            }
        }
        println!("{}", json!({"target": "cover_pushforward", "p": p, "candidate": ci.candidate,
            "c": b.c, "x0": b.x0, "seed0": ci.seed0, "e1": [e1.a2, e1.a4, e1.a6], "r": r,
            "e1_order": e1.order(), "jac_order": jac_order, "gen": pt(&ge),
            "jac_f": b.jac.f(), "h_f": h.f_coeffs(), "seeds": seeds,
            "inst0": {"x": x0, "target_pt": pt(&te0), "dg": dv(&dgs), "dt": dv(&dts),
                      "hg": hv(&hg), "ht": hv(&ht), "runs": runs0}}));
    }
}

fn tower(primes: &[u64], instances: usize, old_seeds: u64, seeds: u64, rho_seeds: u64, first: usize) {
    let params = crosscheck::ic_params();
    for &p in primes {
        for inst in first..first + instances {
            let seed = 1000 + 17 * inst as u64 + p % 997;
            let Some(t) = build64(p, seed, p / 4, 64) else {
                println!("{}", json!({"target": "tower_large", "p": p, "instance": inst, "none": true}));
                continue;
            };
            let eg = EcGroup(t.e);
            let mut rho = vec![];
            for s in 0..rho_seeds {
                let o = rho_reference(&eg, t.gen, t.target, t.r, 100 + s, 1 << 26);
                rho.push(json!({"arm": "old_rho", "seed": 100 + s, "k": o.recovered,
                    "ops": o.group_ops.adds + o.group_ops.doubles}));
                for neg in [true, false] {
                    let rr = rho_full_ec(&t.e, t.gen, t.target, t.r, 100 + s, neg);
                    rho.push(json!({"arm": if neg {"rho_neg"} else {"rho_plain"}, "seed": 100 + s,
                        "k": rr.recovered, "ops": rr.total_ops, "native": rr.native()}));
                }
            }
            let mut rungs = vec![];
            for rg in &t.rungs {
                let cur = &rg.cover.curve;
                let mut sol = vec![];
                for s in 0..old_seeds {
                    match ic::solve(cur, rg.d, rg.t, t.r, rg.jac_order, 300 + s, &params) {
                        Ok(o) => sol.push(json!({"arm": "recorded", "seed": 300 + s, "k": o.recovered,
                            "ops": o.group_ops(), "la_mulmods": o.la_mulmods()})),
                        Err(e) => sol.push(json!({"arm": "recorded", "seed": 300 + s, "error": format!("{e}")})),
                    }
                }
                match instance_for_rung(cur, rg.jac_order, t.r, rg.d, rg.t, t.x) {
                    Ok(ins) => {
                        for l in ["A", "B", "C"] {
                            for s in 0..seeds {
                                let res = ic_on_rung(&ins, arm_of(l), 300 + s, 40_000_000);
                                sol.push(json!({"arm": l, "seed": 300 + s, "k": res.recovered,
                                    "ops": res.group_ops(), "rel_mul": res.rel.mul, "la_mul": res.la.mul}));
                            }
                        }
                    }
                    Err(e) => sol.push(json!({"arm": "instance", "error": e})),
                }
                rungs.push(json!({"genus": rg.cover.genus, "f": cur.f_coeffs(), "jac_order": rg.jac_order,
                    "d": hv(&rg.d), "t": hv(&rg.t), "runs": sol}));
            }
            println!("{}", json!({"target": "tower_large", "p": p, "instance": inst, "seed": seed,
                "e": [t.e.a2, t.e.a4, t.e.a6], "n_e": t.e.order(), "r": t.r, "x": t.x,
                "gen": pt(&t.gen), "target_pt": pt(&t.target), "j_invariant": t.e.j_invariant(),
                "rho": rho, "rungs": rungs}));
        }
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let primes = |i: usize| -> Vec<u64> {
        a.get(i)
            .map(|s| s.split(',').map(|t| t.trim().parse().unwrap()).collect())
            .unwrap_or_default()
    };
    let num = |i: usize, d: u64| a.get(i).and_then(|s| s.parse().ok()).unwrap_or(d);
    match a.get(1).map(String::as_str) {
        Some("v3") => v3(),
        Some("cover") => cover(&primes(2)),
        Some("tower") => tower(
            &primes(2),
            num(3, 3) as usize,
            num(4, 8),
            num(5, 4),
            num(6, 64),
            num(7, 0) as usize,
        ),
        _ => eprintln!("usage: val_nr_dump v3 | cover PRIMES | tower PRIMES [INST] [OLD] [SEEDS] [RHO]"),
    }
}
