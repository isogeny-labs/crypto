//! Validator harness for genus4_prime_order_20261118 (not part of the frozen instrument).
//! Copied into the validator's git-archive tree of ca72267899 as src/bin/val_po.rs and built only
//! after that tree's `genus_ladder` was built and hashed, so the library is the one the frozen binary
//! links.  Reads commands on stdin, one per line:
//!   irr P LO HI          f_irreducible(P, random_f(P, 4, cs)) for cs = LO..=HI (the binary's prefilter call)
//!   firr P c0 .. c9      f_irreducible(P, [c0..c9]) on an arbitrary monic f (differential test)
//!   ord P CS             JCurve::new(P, 4, random_f) and jacobian_order(CS) (the search's order computation)
//!   inst P CS TS ORDER H R [ARMS CC WMAX FILL SEEDS]
//!                        instance_from(...) as the binary builds it (f, G, T, t in Mumford form), then per
//!                        arm (x or Q) and seed the recovered logarithm and the counted fields, made by the
//!                        same calls and parameters as src/bin/genus_ladder.rs at ca72267899.
//! The validator's own arithmetic (jac.py) then checks G, T, t, [r]G = 0 and [recovered]G = T.
use crypto_lib::cryptanalysis::genus_ladder as gl;
use crypto_lib::cryptanalysis::genus_ladder::{Arm, IcParams};
use crypto_lib::cryptanalysis::ic_boundary::{RhoWalk, WalkPolicy};
use serde_json::json;
use std::io::BufRead;

fn main() {
    let g = 4u32;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.is_empty() {
            continue;
        }
        let n = |i: usize| f[i].parse::<u64>().unwrap();
        match f[0] {
            "irr" => {
                let p = n(1);
                for cs in n(2)..=n(3) {
                    let fx = gl::random_f(p, g, cs);
                    println!("{}", json!({"kind": "irr", "p": p, "cs": cs, "f": fx, "irr": gl::f_irreducible(p, &fx)}));
                }
            }
            "firr" => {
                let p = n(1);
                let fx: Vec<u64> = (2..f.len()).map(n).collect();
                println!("{}", json!({"kind": "firr", "p": p, "f": fx, "irr": gl::f_irreducible(p, &fx)}));
            }
            "ord" => {
                let (p, cs) = (n(1), n(2));
                let rec = match gl::JCurve::new(p, g, &gl::random_f(p, g, cs)) {
                    Err(e) => json!({"kind": "ord", "p": p, "cs": cs, "err": e}),
                    Ok(c) => match c.jacobian_order(cs) {
                        Ok(o) => json!({"kind": "ord", "p": p, "cs": cs, "f": c.f_coeffs(), "order": o}),
                        Err(e) => json!({"kind": "ord", "p": p, "cs": cs, "f": c.f_coeffs(), "err": e}),
                    },
                };
                println!("{rec}");
            }
            "inst" => {
                let (p, cs, ts, order, h, r) = (n(1), n(2), n(3), n(4), n(5), n(6));
                let curve = gl::JCurve::new(p, g, &gl::random_f(p, g, cs)).expect("curve");
                let inst = gl::instance_from(curve, order, h, r, ts).expect("instance");
                let c = &inst.curve;
                println!("{}", json!({"kind": "instance", "p": p, "curve_seed": cs, "target_seed": ts,
                    "f": c.f_coeffs(), "G": [c.u_coeffs(&inst.g), c.v_coeffs(&inst.g)],
                    "T": [c.u_coeffs(&inst.target), c.v_coeffs(&inst.target)], "t": inst.t_secret,
                    "order": inst.order, "h": inst.h, "r": inst.r}));
                if f.len() < 12 {
                    continue;
                }
                let cc: f64 = f[8].parse().unwrap();
                let wmax: usize = f[9].parse().unwrap();
                let fill: f64 = f[10].parse().unwrap();
                for arm in f[7].chars() {
                    for seed in f[11].split(',').map(|s| s.parse::<u64>().unwrap()) {
                        match arm {
                            'x' => {
                                let ext = gl::IcExt { coord_square: true, unit_t: true, ferrari: true, sge_coord: true };
                                let sge = Some(gl::SgeParams { w_max: wmax, fill_max: fill });
                                let o = gl::run_ic_ext(&inst, IcParams {
                                    arm: Arm::DoubleLp, nb: gl::fb_size_2lp(p, g, cc), seed, max_trials: 50_000_000,
                                    restart: gl::RESTART, prefilter: true, verify_relations: false, sge }, ext);
                                println!("{}", json!({"kind": "run", "arm": "x", "cc": cc, "w_max": wmax, "fill_max": fill,
                                    "p": p, "curve_seed": cs, "target_seed": ts, "seed": seed, "recovered": o.recovered,
                                    "certified": o.certified, "ok": o.recovered == Some(inst.t_secret),
                                    "group_ops": o.group_ops(), "rel_mul": o.rel.mul, "rel_add": o.rel.add,
                                    "la_mul": o.la.mul, "la_add": o.la.add, "trials": o.trials,
                                    "relations": o.relations, "fb_cols": o.fb_cols}));
                            }
                            'Q' => {
                                let mut walk = RhoWalk::plain();
                                walk.cycle_window = 0;
                                let o = gl::run_rho_policy(&inst, seed, walk, WalkPolicy::legacy());
                                println!("{}", json!({"kind": "run", "arm": "Q", "p": p, "curve_seed": cs,
                                    "target_seed": ts, "seed": seed, "recovered": o.recovered, "certified": o.certified,
                                    "ok": o.recovered == Some(inst.t_secret), "group_ops": o.group_ops(),
                                    "steps": o.steps, "compares": o.compares, "native": o.native}));
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
