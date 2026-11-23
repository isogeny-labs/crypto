//! Validator dumper (not part of the frozen instrument).  Built from the
//! a99e7267f1 git-archive tree after its frozen `genus_ladder` was built and
//! hashed.  Reads lines `p cs ts order h r [arms seeds]` on stdin and prints,
//! per line, the producer's instance (f, G, T, t_secret in Mumford form) and,
//! for each listed arm and seed, the recovered logarithm with the counted
//! fields, so the validator can (a) compare G, T, t with its own arithmetic
//! and (b) tie each recovered log to the frozen binary's row by equal counts.
use crypto_lib::cryptanalysis::genus_ladder as gl;
use crypto_lib::cryptanalysis::genus_ladder::{Arm, IcParams};
use crypto_lib::cryptanalysis::ic_boundary::{rho_reference_walk, RhoWalk};
use serde_json::json;

/// Gate G3 of study 1, re-executed outside `cargo test`: the same calls and the
/// same assertions as `ext_default_is_run_ic_count_for_count` and
/// `ext_arms_certify_at_genus_3_and_4` (genus_ladder.rs 2036-2125 at bce0eb2148),
/// printed instead of asserted.
fn g3() {
    let first_instance = |p: u64, g: u32| -> gl::Instance {
        for cs in 1..400u64 {
            if let Ok(i) = gl::make_instance(p, g, cs, 7, 64) {
                return i;
            }
        }
        panic!("no instance at p = {p}, g = {g}");
    };
    let mut all = true;
    for (p, g) in [(131u64, 3u32), (131, 4), (257, 3)] {
        let inst = first_instance(p, g);
        let prm = IcParams {
            arm: Arm::DoubleLp,
            nb: gl::fb_size_2lp(p, g, 2.0),
            seed: 5,
            max_trials: 5_000_000,
            restart: gl::restart_sqrt(inst.r),
            prefilter: true,
            verify_relations: false,
            sge: None,
        };
        let a = gl::run_ic(&inst, prm);
        let b = gl::run_ic_ext(&inst, prm, gl::IcExt::default());
        let ok = (a.trials, a.relations, a.group_ops(), a.rel, a.la, a.verify_ops, a.recovered)
            == (b.trials, b.relations, b.group_ops(), b.rel, b.la, b.verify_ops, b.recovered);
        all &= ok;
        println!("{}", json!({"kind": "G3", "test": "ext_default_is_run_ic_count_for_count", "p": p, "g": g, "pass": ok}));
    }
    for (p, g) in [(131u64, 3u32), (131, 4), (257, 4)] {
        let inst = first_instance(p, g);
        let prm = IcParams {
            arm: Arm::DoubleLp,
            nb: gl::fb_size_2lp(p, g, 2.0),
            seed: 9,
            max_trials: 5_000_000,
            restart: gl::restart_sqrt(inst.r),
            prefilter: true,
            verify_relations: true,
            sge: None,
        };
        let base = gl::run_ic(&inst, prm);
        for (cs, ut) in [(true, false), (false, true), (true, true)] {
            let ext = gl::IcExt { coord_square: cs, unit_t: ut };
            let x = gl::run_ic_ext(&inst, prm, ext);
            let mut ok = x.certified
                && x.bad_relations == 0
                && x.recovered.map(|v| v % inst.r) == Some(inst.t_secret % inst.r);
            if cs && !ut {
                ok &= x.relations == base.relations;
                if x.la_fallbacks == 0 {
                    ok &= x.la_wied.mul < base.la_wied.mul;
                }
            }
            all &= ok;
            println!("{}", json!({"kind": "G3", "test": "ext_arms_certify_at_genus_3_and_4", "p": p, "g": g,
                "coord_square": cs, "unit_t": ut, "pass": ok, "bad_relations": x.bad_relations}));
        }
    }
    println!("{}", json!({"kind": "G3", "all_pass": all}));
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).expect("stdin");
    if input.trim() == "G3" {
        g3();
        return;
    }
    for line in input.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();
        if v.len() < 6 {
            continue;
        }
        let n = |i: usize| v[i].parse::<u64>().expect("number");
        let (p, cs, ts, order, h, r) = (n(0), n(1), n(2), n(3), n(4), n(5));
        let arms = v.get(6).copied().unwrap_or("");
        let seeds: Vec<u64> = v
            .get(7)
            .map(|s| s.split(',').map(|x| x.parse().expect("seed")).collect())
            .unwrap_or_default();
        let curve = gl::JCurve::new(p, 4, &gl::random_f(p, 4, cs)).expect("curve");
        let inst = gl::instance_from(curve, order, h, r, ts).expect("instance");
        let c = &inst.curve;
        println!(
            "{}",
            json!({"kind": "instance", "p": p, "curve_seed": cs, "target_seed": ts,
                "f": c.f_coeffs(), "t": inst.t_secret,
                "G": [c.u_coeffs(&inst.g), c.v_coeffs(&inst.g)],
                "T": [c.u_coeffs(&inst.target), c.v_coeffs(&inst.target)]})
        );
        for ch in arms.chars() {
            for &seed in &seeds {
                match ch {
                    'c' | 'v' | 'w' => {
                        let ext = gl::IcExt {
                            coord_square: ch != 'c',
                            unit_t: ch == 'w',
                        };
                        let o = gl::run_ic_ext(
                            &inst,
                            IcParams {
                                arm: Arm::DoubleLp,
                                nb: gl::fb_size_2lp(p, 4, 2.0),
                                seed,
                                max_trials: 50_000_000,
                                restart: gl::RESTART,
                                prefilter: true,
                                verify_relations: false,
                                sge: None,
                            },
                            ext,
                        );
                        println!(
                            "{}",
                            json!({"kind": "run", "arm": ch.to_string(), "p": p, "curve_seed": cs,
                                "target_seed": ts, "seed": seed, "recovered": o.recovered,
                                "certified": o.certified, "group_ops": o.group_ops(),
                                "rel_mul": o.rel.mul, "rel_add": o.rel.add, "la_mul": o.la.mul,
                                "la_add": o.la.add, "trials": o.trials, "relations": o.relations,
                                "cantor_mul": o.cantor_mul, "cantor_inv": o.cantor_inv})
                        );
                    }
                    'R' | 'P' | 'Q' => {
                        // Q: the validator's sensitivity walk, RhoWalk::plain() with the
                        // short-cycle window off (cycle_window = 0); not a registered arm.
                        let walk = match ch {
                            'P' => RhoWalk::plain(),
                            'Q' => RhoWalk {
                                cycle_window: 0,
                                ..RhoWalk::plain()
                            },
                            _ => RhoWalk::negation(),
                        };
                        // The walk's own counters and work_native, by a second direct call
                        // with run_rho_walk's arguments (same seed, same cap).
                        let max_steps = (60.0 * (inst.r as f64).sqrt()) as u64 + 10_000;
                        let rr = rho_reference_walk(&inst.curve, inst.g, inst.target, inst.r, seed, max_steps, walk);
                        let o = gl::run_rho_walk(&inst, seed, walk);
                        println!(
                            "{}",
                            json!({"kind": "run", "arm": ch.to_string(), "p": p, "curve_seed": cs,
                                "target_seed": ts, "seed": seed, "recovered": o.recovered,
                                "certified": o.certified, "group_ops": o.group_ops(),
                                "native": o.native, "compares": o.compares, "steps": o.steps,
                                "cantor_mul": o.cantor_mul, "cantor_inv": o.cantor_inv,
                                "walk_counters": rr.counters, "walk_work_native": rr.work_native,
                                "direct_steps": rr.steps, "direct_recovered": rr.recovered})
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}
