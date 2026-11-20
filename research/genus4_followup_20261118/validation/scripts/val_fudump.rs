//! Validator dumper for genus4_followup_20261118 (not part of the frozen instrument).
//! Copied into the validator's git-archive tree of ae36cdc6e4 as src/bin/val_fudump.rs and built
//! only after that tree's `genus_ladder` was built and hashed, so the library is the one the frozen
//! binary links.  Reads lines `p cs ts order h r arms seeds` on stdin (arms from {w, x, Q}; seeds
//! comma separated) and prints, per line, the producer's instance (f, G, T, t_secret in Mumford form),
//! then per arm and seed the recovered logarithm and the counted fields, made by the same calls and
//! parameters as `src/bin/genus_ladder.rs` at ae36cdc6e4 (cc = 2.0, max_trials 5e7, RESTART, SGE 4,1.5
//! for x).  The validator's own arithmetic (jac.py) then checks G, T, t and [recovered]G = T.
use crypto_lib::cryptanalysis::genus_ladder as gl;
use crypto_lib::cryptanalysis::genus_ladder::{Arm, IcParams};
use crypto_lib::cryptanalysis::ic_boundary::{RhoWalk, WalkPolicy};
use serde_json::json;
use std::io::BufRead;

fn main() {
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 6 {
            continue;
        }
        let n = |i: usize| f[i].parse::<u64>().unwrap();
        let (p, cs, ts, order, h, r) = (n(0), n(1), n(2), n(3), n(4), n(5));
        let g = 4u32;
        let curve = gl::JCurve::new(p, g, &gl::random_f(p, g, cs)).expect("curve");
        let inst = gl::instance_from(curve, order, h, r, ts).expect("instance");
        let c = &inst.curve;
        println!("{}", json!({"kind": "instance", "p": p, "curve_seed": cs, "target_seed": ts,
            "f": c.f_coeffs(), "G": [c.u_coeffs(&inst.g), c.v_coeffs(&inst.g)],
            "T": [c.u_coeffs(&inst.target), c.v_coeffs(&inst.target)], "t": inst.t_secret,
            "order": inst.order, "h": inst.h, "r": inst.r}));
        if f.len() < 8 {
            continue;
        }
        for arm in f[6].chars() {
            for seed in f[7].split(',').map(|s| s.parse::<u64>().unwrap()) {
                match arm {
                    'w' | 'x' => {
                        let x = arm == 'x';
                        let ext = gl::IcExt { coord_square: true, unit_t: true, ferrari: x, sge_coord: x };
                        let sge = if x { Some(gl::SgeParams { w_max: 4, fill_max: 1.5 }) } else { None };
                        let o = gl::run_ic_ext(&inst, IcParams {
                            arm: Arm::DoubleLp, nb: gl::fb_size_2lp(p, g, 2.0), seed, max_trials: 50_000_000,
                            restart: gl::RESTART, prefilter: true, verify_relations: false, sge }, ext);
                        println!("{}", json!({"kind": "run", "arm": arm.to_string(), "p": p, "curve_seed": cs,
                            "target_seed": ts, "seed": seed, "recovered": o.recovered, "certified": o.certified,
                            "ok": o.recovered == Some(inst.t_secret), "group_ops": o.group_ops(),
                            "rel_mul": o.rel.mul, "rel_add": o.rel.add, "la_mul": o.la.mul, "la_add": o.la.add,
                            "trials": o.trials, "relations": o.relations}));
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
}
