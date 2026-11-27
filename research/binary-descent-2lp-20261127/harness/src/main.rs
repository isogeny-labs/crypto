//! b2lp_harness — arm D2, the double-large-prime variant of #5261's arm D
//! (Gaudry–Diem k = 3 Weil-descent decomposition oracle), on the #5253 binary
//! composite IC harness.  Study `b2lp-20261127/v1`.
//!
//! The field, curve, factor base, relation-target stream (A1), sign
//! resolution, echelon linear algebra (A2), rho and seed modules are the
//! #5253 files, and the descent oracle is #5261's `descent.rs`, all included
//! unchanged by `#[path]`: curves, targets `Q`, target streams, the oracle and
//! the counted unit are bitwise those of #5253 / #5261.
//!
//! ```text
//! b2lp_harness selftest <n> <n'>
//! b2lp_harness predict <out.jsonl> <beta> <theta> <n>...          (abstract-graph simulation, no field arithmetic)
//! b2lp_harness pilot <out.jsonl> <threads> <n>...                 (held-out theta pilot; grids in source)
//! b2lp_harness rho-pilot <out.jsonl> <threads> <n>...             (held-out rho-lane pilot)
//! b2lp_harness run <protocol.json> <out.jsonl> [threads]
//! b2lp_harness analyze <protocol.json> <runs.jsonl> <summary.json> <figure.svg>
//! ```
#![allow(dead_code, unused_imports, clippy::all)]
#[path = "../../../binary-composite-rootfind-20261126/harness/src/binary_ecc.rs"]
mod binary_ecc;
#[path = "../../../binary-composite-rootfind-20261126/harness/src/cryptanalysis.rs"]
mod cryptanalysis;
#[path = "../../../binary-composite-rootfind-20261126/harness/src/field.rs"]
mod field;
#[path = "../../../binary-composite-rootfind-20261126/harness/src/gen.rs"]
mod gen;
#[path = "../../../binary-composite-rootfind-20261126/harness/src/ic.rs"]
mod ic;
#[path = "../../../binary-composite-rootfind-20261126/harness/src/rho.rs"]
mod rho;
#[path = "../../../binary-composite-rootfind-20261126/harness/src/seeds.rs"]
mod seeds;
#[path = "../../../binary-weil-descent-20261126/harness/src/descent.rs"]
mod descent;

mod analysis;
mod graph;

use descent::Descent;
use field::{meq, Curve, Ops, Pt, CF};
use graph::{Comb, LpGraph};
use ic::*;
use seeds::Sm64;
use serde_json::json;
use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex};

// ── verbatim from the #5253 / #5261 main.rs (same curves and targets) ──
pub fn make_curve(domain: &str, n: u32, ci: u32) -> gen::Instance {
    let mut rng = Sm64::keyed(&[domain, &n.to_string(), &ci.to_string()]);
    gen::random_instance(n, &mut rng)
}
pub fn make_target(domain: &str, inst: &gen::Instance, n: u32, ci: u32, ti: u32) -> (Pt, u64) {
    let mut rng = Sm64::keyed(&[domain, &n.to_string(), &ci.to_string(), &ti.to_string()]);
    let c = &inst.curve;
    let d = 1 + rng.below(c.ell - 1);
    let cf = CF::new(&c.gf);
    (c.mul(&cf, d, inst.g), d)
}
fn phases_json(ph: &[(&str, Ops)]) -> serde_json::Value {
    let mut m = serde_json::Map::new();
    for (k, o) in ph {
        m.insert(k.to_string(), o.json());
    }
    serde_json::Value::Object(m)
}
fn total(ph: &[(&str, Ops)]) -> Ops {
    ph.iter().fold(Ops::default(), |a, (_, o)| a.add(o))
}
fn cost_json(o: &Ops, n: u32) -> serde_json::Value {
    json!({
        "meq_s1_u1": meq(o, n, 1.0, 1.0),
        "meq_s1_u01": meq(o, n, 1.0, 0.1),
        "meq_s1_u0": meq(o, n, 1.0, 0.0),
        "meq_s025_u1": meq(o, n, 0.25, 1.0),
        "meq_s025_u0": meq(o, n, 0.25, 0.0),
    })
}
fn curve_json(n: u32, np: u32, ci: u32, inst: &gen::Instance) -> serde_json::Value {
    let c = &inst.curve;
    json!({"n": n, "np": np, "ci": ci, "a": c.a, "b": format!("{:#x}", c.b), "irr_low_terms": c.irr_low,
           "order": c.order, "h": c.h, "ell": c.ell, "g": format!("{:?}", inst.g), "gen_attempts": inst.attempts})
}
pub fn rho_record(c: &Curve, g: Pt, q: Pt, w: usize, stream: &[&str]) -> serde_json::Value {
    let cf = CF::new(&c.gf);
    let mut rng = Sm64::keyed(stream);
    let max_steps = (200.0 * (c.ell as f64).sqrt()) as u64 + 100_000;
    let r = rho::rho(c, &cf, g, q, w, &mut rng, max_steps);
    let tot = total(&r.phases);
    json!({
        "arm": "RHO", "lanes": w, "d": r.d, "verified": r.verified, "steps": r.steps,
        "lookahead": r.lookahead, "cycles": r.cycles, "capped": r.capped,
        "fruitless_collisions": r.fruitless_collisions,
        "phases": phases_json(&r.phases), "total": tot.json(), "cost": cost_json(&tot, c.n),
        "wall_ns": r.wall_ns as u64,
    })
}
// ── end verbatim ──

/// Small-base size rule: `fs = min(F − 1, ⌈θ · F^β⌉)` (F counts T); θ < 0 means "all small" (D2 ≡ D).
pub fn fs_rule(f: usize, beta: f64, theta: f64) -> usize {
    if theta < 0.0 {
        return f - 1;
    }
    ((theta * (f as f64).powf(beta)).ceil() as usize).clamp(1, f - 1)
}

/// Group-law check of one relation over the small base (uncounted):
/// `Σ e_c·[h]P_c = [a]([h]G) + [b]([h]Q)` exactly in `E`.
fn check_comb(c: &Curve, cf: &CF, fb: &Fb, hp: &mut HashMap<usize, Pt>, hg: Pt, hq: Pt, y: &Comb) -> bool {
    let mut lhs = Pt::Inf;
    for &(col, e) in &y.cols {
        let p = *hp.entry(col).or_insert_with(|| c.mul(cf, c.h, fb.pts[col]));
        lhs = c.add(cf, lhs, c.mul(cf, e, p));
    }
    let rhs = c.joint(cf, y.a, hg, y.b, hq);
    lhs == rhs
}

/// One cold single-target solve with arm D2 on the A1 stream and the A2 echelon LA.
pub fn run_d2(c: &Curve, g: Pt, q: Pt, sub: &Sub, np: u32, stream: &[&str], beta: f64, theta: f64, check_yields: bool) -> serde_json::Value {
    let gf = &c.gf;
    let t0 = std::time::Instant::now();
    let (cf_sh, cf_set, cf_red, cf_rf, cf_lift, cf_gr) = (CF::new(gf), CF::new(gf), CF::new(gf), CF::new(gf), CF::new(gf), CF::new(gf));
    let cf_chk = CF::new(gf);
    let fb = build_fb(c, &cf_sh, sub);
    let o_fb = cf_sh.ops();
    let f = fb.xs.len();
    assert_eq!(fb.xs[0], 0, "T is column 0");
    let fs = fs_rule(f, beta, theta);
    let mut dsc = Descent::new(c, &cf_set, sub, np);
    let mut gr = LpGraph::new(c.ell, f, fs);
    let mut rng = Sm64::keyed(stream);
    let mut tg = Targets::new_ap(c, &cf_sh, g, q, &mut rng);
    let (mut o_targets, mut o_resolve, mut o_la) = (Ops::default(), Ops::default(), Ops::default());
    let mut ech = Echelon::new(c.ell);
    let mut yields: Vec<Comb> = vec![];
    let mut fed = 0usize;
    let mut set_d = vec![];
    let (mut unresolved, mut ntargets, mut nrel) = (0u64, 0u64, 0u64);
    let mut cols_all: std::collections::HashSet<usize> = Default::default();
    let mut cols_small: std::collections::HashSet<usize> = Default::default();
    let (mut y_checked, mut y_bad) = (0u64, 0u64);
    let mut hp: HashMap<usize, Pt> = HashMap::new();
    let (hg, hq) = (c.mul(&cf_chk, c.h, g), c.mul(&cf_chk, c.h, q));
    let mut w_chk = 0u128;
    let mut d = None;
    let cap = 400 * f as u64 + 2000;
    let mut rows_t: Vec<Row> = vec![];
    while ntargets < cap {
        let s0 = cf_sh.ops();
        let (r, ra, rb) = tg.next(c, &cf_sh);
        o_targets = o_targets.add(&cf_sh.ops().sub(&s0));
        ntargets += 1;
        let Pt::A(xr, _) = r else { continue };
        if xr == 0 {
            continue;
        }
        dsc.decompose(&cf_red, &cf_rf, &cf_lift, &fb, xr, &mut set_d);
        let s1 = cf_sh.ops();
        rows_t.clear();
        for t in &set_d {
            match resolve(c, &cf_sh, &fb, t, r, ra, rb) {
                Some(row) => rows_t.push(row),
                None => unresolved += 1,
            }
        }
        o_resolve = o_resolve.add(&cf_sh.ops().sub(&s1));
        for row in &rows_t {
            nrel += 1;
            let tc = std::time::Instant::now();
            for e in &row.cols {
                cols_all.insert(e.0);
            }
            w_chk += tc.elapsed().as_nanos();
            if let Some(y) = gr.add(&cf_gr, row) {
                let tc = std::time::Instant::now();
                assert!(y.cols.iter().all(|e| gr.is_small(e.0)), "yield free of large primes");
                for e in &y.cols {
                    cols_small.insert(e.0);
                }
                if check_yields {
                    y_checked += 1;
                    if !check_comb(c, &cf_chk, &fb, &mut hp, hg, hq, &y) {
                        y_bad += 1;
                    }
                }
                w_chk += tc.elapsed().as_nanos();
                yields.push(y);
            }
        }
        let s2 = cf_sh.ops();
        let mut dd = None;
        while fed < yields.len() {
            dd = ech.add(&cf_sh, &yields[fed].to_row());
            fed += 1;
            if dd.is_some() {
                break;
            }
        }
        o_la = o_la.add(&cf_sh.ops().sub(&s2));
        if let Some(dd) = dd {
            d = Some(dd);
            break;
        }
    }
    let s3 = cf_sh.ops();
    let verified = d.map(|dd| c.mul(&cf_sh, dd, g) == q).unwrap_or(false);
    let o_ver = cf_sh.ops().sub(&s3);
    let ph: Vec<(&str, Ops)> = vec![
        ("fb", o_fb),
        ("setup", cf_set.ops()),
        ("targets", o_targets),
        ("descent", cf_red.ops()),
        ("rootfind", cf_rf.ops()),
        ("lift", cf_lift.ops()),
        ("resolve", o_resolve),
        ("graph", cf_gr.ops()),
        ("la", o_la),
        ("verify", o_ver),
    ];
    let tot = total(&ph);
    let st = dsc.st;
    let gs = gr.st;
    let (comps, grounded, with_sv) = gr.census();
    let wall = t0.elapsed().as_nanos() - w_chk;
    json!({
        "d": d, "verified": verified, "targets": ntargets, "relations": nrel,
        "columns": cols_all.len(), "small_columns": cols_small.len(), "fb": f, "fs": fs, "beta": beta, "theta": theta,
        "rows_unresolved": unresolved, "capped": d.is_none() && ntargets >= cap, "target_cap": cap,
        "la": "echelon", "echelon_rows_in": ech.rows_in, "echelon_useless_dependencies": ech.useless_dependencies,
        "yields": yields.len(), "yields_fed": fed,
        "yield_checks": y_checked, "yield_check_failures": y_bad,
        "graph": {
            "rows": gs.rows, "full": gs.full, "one_lp": gs.one_lp, "two_lp": gs.two_lp, "discarded_3lp": gs.discarded_3lp,
            "yields_cycle": gs.yields_cycle, "yields_join": gs.yields_join, "groundings": gs.groundings,
            "tree_edges": gs.tree_edges, "vertices_touched": gs.vertices_touched,
            "reduce_steps": gs.reduce_steps, "max_reduce_steps": gs.max_reduce_steps, "reroot_steps": gs.reroot_steps,
            "yield_weight_mean": gs.yield_weight_sum as f64 / yields.len().max(1) as f64, "yield_weight_max": gs.yield_weight_max,
            "components": comps, "grounded_components": grounded, "components_with_sv": with_sv,
        },
        "phases": phases_json(&ph), "total": tot.json(), "cost": cost_json(&tot, c.n),
        "wall_ns": wall as u64, "check_wall_ns": w_chk as u64,
        "dstats": {
            "targets": st.targets, "deg_u_mean": st.deg_u_sum as f64 / st.targets.max(1) as f64, "deg_u_max": st.deg_u_max,
            "decompositions": st.decompositions,
            "fallback_rank": st.fallback_rank, "fallback_uzero": st.fallback_uzero,
            "fallback_quartic_zero": st.fallback_quartic_zero,
        },
        "note": "#5261 Gaudry-Diem k=3 descent oracle unchanged; relations with <=2 large primes; signed large-prime graph (union-find + spanning forest, grounded components); echelon LA on the small base",
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("selftest") => selftest(args[2].parse().unwrap(), args[3].parse().unwrap()),
        Some("predict") => predict(&args[2], args[3].parse().unwrap(), args[4].parse().unwrap(), args[5..].iter().map(|s| s.parse().unwrap()).collect()),
        Some("pilot") => pilot(&args[2], args[3].parse().unwrap(), args[4..].iter().map(|s| s.parse().unwrap()).collect()),
        Some("rho-pilot") => rho_pilot(&args[2], args[3].parse().unwrap(), args[4..].iter().map(|s| s.parse().unwrap()).collect()),
        Some("run") => run(&args[2], &args[3], args.get(4).map(|s| s.parse().unwrap()).unwrap_or(1)),
        Some("pilot-summary") => analysis::pilot_summary(&args[2], &args[3]),
        Some("predict-summary") => analysis::predict_summary(&args[2], &args[3]),
        Some("analyze") => analysis::analyze(&args[2], &args[3], &args[4], &args[5]),
        _ => eprintln!("usage: see source header"),
    }
}

/// Abstract relation stream with hidden logarithms: every relation and every
/// yield must satisfy `Σ e_c·L_c = a + b·d` for the planted `L`, `d`.
fn graph_selftest() {
    let ell: u64 = (1u64 << 61) - 1;
    let gf = cryptanalysis::semaev_decomp::Gf2::new(&gen::irreducible(13));
    let cf = CF::new(&gf);
    let mm = |a: u64, b: u64| ((a as u128 * b as u128) % ell as u128) as u64;
    for (f, fs, seed) in [(200usize, 20usize, "a"), (2000, 60, "b"), (500, 499, "c"), (1000, 5, "d")] {
        let mut rng = Sm64::keyed(&["b2lp-selftest-graph", seed]);
        let logs: Vec<u64> = (0..f).map(|_| rng.below(ell)).collect();
        let dl = rng.below(ell);
        let mut gr = LpGraph::new(ell, f, fs);
        let (mut ny, mut nrows) = (0u64, 0u64);
        for _ in 0..(6 * f) {
            let k = 1 + (rng.below(3) as usize);
            let mut cols: Vec<(usize, i64)> = vec![];
            for _ in 0..k {
                let col = 1 + rng.below((f - 1) as u64) as usize;
                let s = if rng.bits(1) == 1 { 1 } else { -1 };
                if let Some(e) = cols.iter_mut().find(|e| e.0 == col) {
                    e.1 += s;
                } else {
                    cols.push((col, s));
                }
            }
            cols.retain(|e| e.1 != 0);
            // a + b·d = Σ e·L
            let lhs = cols.iter().fold(0u64, |acc, &(c, e)| (acc + mm(((e % ell as i64 + ell as i64) as u64) % ell, logs[c])) % ell);
            let b = rng.below(ell);
            let a = (lhs + ell - mm(b, dl)) % ell;
            nrows += 1;
            if let Some(y) = gr.add(&cf, &Row { cols, a, b }) {
                ny += 1;
                assert!(y.cols.iter().all(|e| gr.is_small(e.0)));
                let l = y.cols.iter().fold(0u64, |acc, &(c, e)| (acc + mm(e, logs[c])) % ell);
                assert_eq!(l, (y.a + mm(y.b, dl)) % ell, "yield satisfies the planted logs");
            }
        }
        let s = gr.st;
        // rank identity: yields = full + (edges − rank)
        println!("graph selftest F={f} fs={fs}: rows={nrows} yields={ny} full={} 1lp={} 2lp={} 3lp={} cycle={} join={} groundings={} tree={}", s.full, s.one_lp, s.two_lp, s.discarded_3lp, s.yields_cycle, s.yields_join, s.groundings, s.tree_edges);
        assert_eq!(s.one_lp + s.two_lp, s.yields_cycle + s.yields_join + s.groundings + s.tree_edges);
    }
}

fn selftest(n: u32, np: u32) {
    graph_selftest();
    let inst = make_curve("b2lp-selftest-curve", n, 0);
    let c = &inst.curve;
    println!("{}", curve_json(n, np, 0, &inst));
    let basis = gen::subfield_basis(&c.gf, np);
    let sub = Sub::new(&basis);
    for ti in 0..6u32 {
        let (q, dtrue) = make_target("b2lp-selftest-target", &inst, n, 0, ti);
        let st = ["b2lp-selftest-stream", &n.to_string(), &ti.to_string()];
        for (beta, theta) in [(2.0 / 3.0, -1.0), (2.0 / 3.0, 1.0), (2.0 / 3.0, 3.0), (0.528, 2.0)] {
            let o = run_d2(c, inst.g, q, &sub, np, &st, beta, theta, true);
            println!(
                "D2 ti={ti} beta={beta:.3} theta={theta} fs={}/{} d_ok={} verified={} targets={} rels={} yields={} ychk={}/{} meq={:.3e} graph={}",
                o["fs"], o["fb"], o["d"].as_u64() == Some(dtrue), o["verified"], o["targets"], o["relations"], o["yields"],
                o["yield_checks"], o["yield_check_failures"], o["cost"]["meq_s1_u1"].as_f64().unwrap(), o["graph"]
            );
            assert_eq!(o["d"].as_u64(), Some(dtrue));
            assert_eq!(o["yield_check_failures"].as_u64(), Some(0));
        }
    }
    println!("selftest PASS");
}

/// Abstract simulation of the D2 graph and LA (prediction only): F classes,
/// Poisson(λ = 1/6) decompositions per target of 3 uniform classes with
/// uniform signs (λ is #5261's measured 4F³/(3N)), the same LpGraph and
/// Echelon code, random (a, b).  Reports targets, yields and counted graph/LA ops.
fn predict(out: &str, beta: f64, theta: f64, ns: Vec<u32>) {
    let mut fo = std::fs::OpenOptions::new().create(true).append(true).open(out).unwrap();
    let ell: u64 = (1u64 << 61) - 1;
    let gf = cryptanalysis::semaev_decomp::Gf2::new(&gen::irreducible(13));
    for n in ns {
        let np = n / 3;
        let f = (1usize << np) / 2 + 1;
        for rep in 0..3u32 {
            let mut rng = Sm64::keyed(&["b2lp-predict", &n.to_string(), &format!("{beta}"), &format!("{theta}"), &rep.to_string()]);
            let fs = fs_rule(f, beta, theta);
            let (cg, cl) = (CF::new(&gf), CF::new(&gf));
            let mut gr = LpGraph::new(ell, f, fs);
            let mut ech = Echelon::new(ell);
            let mut tg = 0u64;
            let mut yl: Vec<Comb> = vec![];
            let mut fed = 0;
            let mut done = false;
            let lam = 1.0 / 6.0;
            let el = (-lam as f64).exp();
            while !done && tg < 400 * f as u64 {
                tg += 1;
                // Poisson(λ) by inversion
                let mut k = 0;
                let mut p = el;
                let mut cum = p;
                let u = (rng.next() >> 11) as f64 / (1u64 << 53) as f64;
                while u > cum {
                    k += 1;
                    p *= lam / k as f64;
                    cum += p;
                }
                for _ in 0..k {
                    let mut cols: Vec<(usize, i64)> = vec![];
                    for _ in 0..3 {
                        let col = 1 + rng.below((f - 1) as u64) as usize;
                        let s = if rng.bits(1) == 1 { 1 } else { -1 };
                        if let Some(e) = cols.iter_mut().find(|e| e.0 == col) {
                            e.1 += s;
                        } else {
                            cols.push((col, s));
                        }
                    }
                    cols.retain(|e| e.1 != 0);
                    let row = Row { cols, a: rng.below(ell), b: rng.below(ell) };
                    if let Some(y) = gr.add(&cg, &row) {
                        yl.push(y);
                    }
                }
                while fed < yl.len() {
                    let dd = ech.add(&cl, &yl[fed].to_row());
                    fed += 1;
                    if dd.is_some() {
                        done = true;
                        break;
                    }
                }
            }
            let s = gr.st;
            let og = cg.ops();
            let ol = cl.ops();
            let v = json!({"n": n, "np": np, "F": f, "fs": fs, "beta": beta, "theta": theta, "rep": rep, "solved": done,
                "targets": tg, "targets_over_qp": tg as f64 / (1u64 << np) as f64, "yields": yl.len(), "fed": fed,
                "full": s.full, "one_lp": s.one_lp, "two_lp": s.two_lp, "yields_cycle": s.yields_cycle, "yields_join": s.yields_join, "groundings": s.groundings,
                "yield_weight_mean": s.yield_weight_sum as f64 / yl.len().max(1) as f64,
                "graph_units": og.h + og.z + og.za, "la_units": ol.z + ol.za});
            writeln!(fo, "{}", v).unwrap();
            eprintln!("{}", v);
        }
    }
}

/// Held-out theta pilot (PROTOCOL.md): `d2pilot-*` instances, 2 curves × 2 targets per rung.
fn pilot(out: &str, threads: usize, ns: Vec<u32>) {
    let grid: Vec<(f64, f64)> = [0.5, 1.0, 2.0, 4.0, 8.0, 16.0]
        .iter()
        .map(|&t| (2.0 / 3.0, t))
        .chain([1.0, 2.0, 4.0, 8.0, 16.0, 32.0].iter().map(|&t| (0.528, t)))
        .collect();
    let mut jobs = vec![];
    for &n in &ns {
        for ci in 0..2u32 {
            for ti in 0..2u32 {
                for &(b, t) in &grid {
                    jobs.push((n, ci, ti, b, t));
                }
            }
        }
    }
    jobs.reverse();
    par_jobs(out, threads, jobs, |(n, ci, ti, beta, theta)| {
        let np = n / 3;
        let inst = make_curve("d2pilot-curve", n, ci);
        let c = &inst.curve;
        let (q, dt) = make_target("d2pilot-target", &inst, n, ci, ti);
        let sub = Sub::new(&gen::subfield_basis(&c.gf, np));
        let mut v = run_d2(c, inst.g, q, &sub, np, &["d2pilot-stream", &n.to_string(), &ci.to_string(), &ti.to_string()], beta, theta, false);
        v["arm"] = json!("D2pilot");
        v["case"] = json!({"n": n, "np": np, "ci": ci, "ti": ti});
        v["correct"] = json!(v["d"].as_u64() == Some(dt));
        v
    });
}

/// Held-out rho-lane pilot for the new rungs: same rule as #5253 / #5261, wider grid.
fn rho_pilot(out: &str, threads: usize, ns: Vec<u32>) {
    let mut jobs = vec![];
    for &n in &ns {
        for ci in 0..2u32 {
            for w in [16usize, 32, 64, 128, 256] {
                for ti in 0..8u32 {
                    jobs.push((n, ci, ti, w));
                }
            }
        }
    }
    jobs.reverse();
    par_jobs(out, threads, jobs, |(n, ci, ti, w)| {
        let inst = make_curve("pilot-curve", n, ci);
        let (q, d) = make_target("pilot-target", &inst, n, ci, ti);
        let mut o = rho_record(&inst.curve, inst.g, q, w, &["pilot-rho", &n.to_string(), &ci.to_string(), &ti.to_string(), &w.to_string()]);
        o["n"] = json!(n);
        o["ci"] = json!(ci);
        o["ti"] = json!(ti);
        o["correct"] = json!(o["d"].as_u64() == Some(d));
        o
    });
}

fn par_jobs<J: Send + 'static>(out: &str, threads: usize, jobs: Vec<J>, f: impl Fn(J) -> serde_json::Value + Send + Sync + 'static) {
    let file = Arc::new(Mutex::new(std::fs::OpenOptions::new().create(true).append(true).open(out).unwrap()));
    let jobs = Arc::new(Mutex::new(jobs));
    let f = Arc::new(f);
    let mut hs = vec![];
    for _ in 0..threads {
        let (jobs, file, f) = (jobs.clone(), file.clone(), f.clone());
        hs.push(std::thread::spawn(move || loop {
            let job = jobs.lock().unwrap().pop();
            let Some(job) = job else { break };
            let v = f(job);
            let mut fl = file.lock().unwrap();
            writeln!(fl, "{}", v).unwrap();
            fl.flush().unwrap();
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
}

/// Evaluation run: arms from the protocol (`D2`, `D2e`, `D2all`, `RHO`), per rung.
fn run(protocol: &str, out: &str, threads: usize) {
    let p: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(protocol).unwrap()).unwrap();
    let rungs: Vec<serde_json::Value> = p["rungs"].as_array().unwrap().clone();
    let curves = p["curves_per_rung"].as_u64().unwrap() as u32;
    let targets = p["targets_per_curve"].as_u64().unwrap() as u32;
    let rho_seeds = p["rho_seeds_per_target"].as_u64().unwrap() as u32;
    let armdef = p["arms_d2"].clone();
    let mut jobs: Vec<(u32, u32, u32, u32, String, u32)> = vec![];
    for r in &rungs {
        let n = r["n"].as_u64().unwrap() as u32;
        let np = r["np"].as_u64().unwrap() as u32;
        let curves = r["curves"].as_u64().map(|x| x as u32).unwrap_or(curves);
        let targets = r["targets"].as_u64().map(|x| x as u32).unwrap_or(targets);
        let arms: Vec<String> = r["run_arms"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_string()).collect();
        for ci in 0..curves {
            for ti in 0..targets {
                for a in &arms {
                    if a == "RHO" {
                        for si in 0..rho_seeds {
                            jobs.push((n, np, ci, ti, a.clone(), si));
                        }
                    } else {
                        jobs.push((n, np, ci, ti, a.clone(), 0));
                    }
                }
            }
        }
    }
    let lanes: HashMap<u32, usize> = rungs.iter().filter_map(|r| r["rho_lanes"].as_u64().map(|w| (r["n"].as_u64().unwrap() as u32, w as usize))).collect();
    jobs.reverse();
    par_jobs(out, threads, jobs, move |(n, np, ci, ti, arm, si)| {
        let inst = make_curve("curve", n, ci);
        let c = &inst.curve;
        let (q, d_true) = make_target("target", &inst, n, ci, ti);
        let (ns, cs, ts) = (n.to_string(), ci.to_string(), ti.to_string());
        let mut v = if arm == "RHO" {
            let sis = si.to_string();
            let mut v = rho_record(c, inst.g, q, lanes[&n], &["rho", &ns, &cs, &ts, &sis]);
            v["seed_index"] = json!(si);
            v
        } else {
            let a = &armdef[&arm];
            let (beta, theta) = (a["beta"].as_f64().unwrap(), a["theta"].as_f64().unwrap());
            let sub = Sub::new(&gen::subfield_basis(&c.gf, np));
            let mut v = run_d2(c, inst.g, q, &sub, np, &["icstream-a1", &ns, &cs, &ts], beta, theta, true);
            v["arm"] = json!(arm);
            v
        };
        v["case"] = json!({"n": n, "np": np, "ci": ci, "ti": ti, "curve": curve_json(n, np, ci, &inst)});
        v["correct"] = json!(v["d"].as_u64() == Some(d_true));
        eprintln!("case n={n} ci={ci} ti={ti} {arm} si={si} correct={} meq={:.3e}", v["correct"], v["cost"]["meq_s1_u1"].as_f64().unwrap());
        v
    });
}

