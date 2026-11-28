//! bdsl_harness — arm D-S: arm D (Gaudry–Diem k = 3 Weil descent, #5261)
//! with its echelon linear algebra replaced by structured Gaussian
//! elimination + Wiedemann mod ℓ.  Study `bdsl-20261127/v1`.
//!
//! The relation phase is #5261's, line for line: the #5253 field, curves,
//! factor base, A1 target stream, group-law sign resolution, and #5261's
//! `descent.rs`, all included unchanged by `#[path]`.  Only the linear
//! algebra and the stopping rule (PROTOCOL.md) differ.  An uncounted shadow
//! copy of #5253's echelon is fed the same rows in the same order, so every
//! record also states where arm D would have stopped and what its
//! relation-phase counters were there (compared bitwise with #5261's
//! records by `analyze`).
//!
//! ```text
//! bdsl_harness selftest <n> <n'>
//! bdsl_harness run <protocol.json> <out.jsonl> [threads]
//! bdsl_harness analyze <protocol.json> <merged.jsonl> <summary.json> <figure.svg>
//! bdsl_harness wcal <n>          # indicative timing: mod-ell product vs field multiplication
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
mod sparse_la;

use descent::Descent;
use field::{meq, Curve, Ops, Pt, CF};
use ic::*;
use seeds::Sm64;
use serde_json::json;
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
// ── end verbatim ──

/// Frozen stopping-rule and LA parameters (PROTOCOL.md; also in protocol.json).
pub const CHECK_START_FRAC: f64 = 0.70;
pub const CHECK_STEPS_PER_FB: f64 = 256.0;
pub const SGE_KMAX: usize = 64;
pub const WIED_ATTEMPTS: u32 = 4;
pub const WIED_U_WEIGHT: usize = 16;
pub const TRACK_MAX_N: u32 = 30;

/// One cold single-target solve with arm D-S (or D-S-B, the reduced
/// factor base of about F/λ classes, when `lam` is given).
pub fn run_ds(c: &Curve, g: Pt, q: Pt, sub: &Sub, np: u32, stream: &[&str], la_stream: &[&str], lam: Option<(f64, &str, &[&str])>, shadow: bool) -> serde_json::Value {
    let gf = &c.gf;
    let ell = c.ell;
    let t0 = std::time::Instant::now();
    let (cf_sh, cf_set, cf_red, cf_rf, cf_lift) = (CF::new(gf), CF::new(gf), CF::new(gf), CF::new(gf), CF::new(gf));
    let cf_la = CF::new(gf);
    let cf_shadow = CF::new(gf); // uncounted
    let fb = build_fb(c, &cf_sh, sub);
    // reduced base (arm D-S-B): class x kept iff keyed hash < 2^64/λ; T (x = 0) always kept
    let keep: Option<Vec<bool>> = lam.map(|(l, _lab, key)| {
        fb.xs
            .iter()
            .map(|&x| {
                cf_sh.hash();
                if x == 0 {
                    return true;
                }
                let xs = format!("{x:#x}");
                let mut parts: Vec<&str> = key.to_vec();
                parts.push(&xs);
                (seeds::derive(&parts) as f64) < (u64::MAX as f64) / l
            })
            .collect()
    });
    let o_fb = cf_sh.ops();
    let fcols = match &keep {
        None => fb.xs.iter().filter(|&&x| x != 0).count(),
        Some(k) => fb.xs.iter().zip(k).filter(|(&x, &kk)| x != 0 && kk).count(),
    };
    let mut dsc = Descent::new(c, &cf_set, sub, np);
    let mut rng = Sm64::keyed(stream);
    let mut la_rng = Sm64::keyed(la_stream);
    let mut tg = Targets::new_ap(c, &cf_sh, g, q, &mut rng);
    let mut rows: Vec<Row> = vec![];
    let (mut o_targets, mut o_resolve, mut o_filter) = (Ops::default(), Ops::default(), Ops::default());
    let mut set_d = vec![];
    let (mut unresolved, mut ntargets) = (0u64, 0u64);
    let mut w_chk = 0u128;
    let mut d = None;
    let cap = 60 * fb.xs.len() as u64 * if lam.is_some() { 16 } else { 1 } + 200;
    let step = ((fcols as f64 / CHECK_STEPS_PER_FB).ceil() as usize).max(1);
    let mut next_check = ((CHECK_START_FRAC * fcols as f64).ceil() as usize).max(1);
    let mut cnt = sparse_la::Cnt::default();
    let prm = sparse_la::Params { kmax: SGE_KMAX, attempts: WIED_ATTEMPTS, u_weight: WIED_U_WEIGHT };
    let (mut checks, mut la_calls) = (0u64, 0u64);
    let mut check_log = vec![];
    let mut last_rep = sparse_la::LaReport::default();
    let mut ech = Echelon::new(ell);
    let mut fed = 0usize;
    let mut shadow_rec: Option<serde_json::Value> = None;
    let track = c.n <= TRACK_MAX_N && lam.is_none();
    let mut track_ok: Option<bool> = None;
    let ncol_bound = fb.xs.len();
    let mut drop_sub = 0u64;
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
        if let Some(k) = &keep {
            let s = cf_sh.ops();
            set_d.retain(|t| {
                t.iter().all(|x| {
                    cf_sh.hash();
                    k[fb.idx[x]]
                }) || {
                    drop_sub += 1;
                    false
                }
            });
            o_filter = o_filter.add(&cf_sh.ops().sub(&s));
        }
        let s1 = cf_sh.ops();
        for t in &set_d {
            match resolve(c, &cf_sh, &fb, t, r, ra, rb) {
                Some(row) => rows.push(row),
                None => unresolved += 1,
            }
        }
        o_resolve = o_resolve.add(&cf_sh.ops().sub(&s1));
        // uncounted shadow: #5253's echelon fed exactly as arm D feeds it
        if shadow && shadow_rec.is_none() {
            let tc = std::time::Instant::now();
            let mut dd = None;
            while fed < rows.len() {
                dd = ech.add(&cf_shadow, &rows[fed]);
                fed += 1;
                if dd.is_some() {
                    break;
                }
            }
            if let Some(dd) = dd {
                let ph: Vec<(&str, Ops)> = vec![
                    ("fb", o_fb),
                    ("setup", cf_set.ops()),
                    ("targets", o_targets),
                    ("descent", cf_red.ops()),
                    ("rootfind", cf_rf.ops()),
                    ("lift", cf_lift.ops()),
                    ("resolve", o_resolve),
                ];
                shadow_rec = Some(json!({"d": dd, "targets": ntargets, "relations": rows.len(), "columns": ncols_of(&rows), "phases": phases_json(&ph),
                                         "echelon_rows_in": ech.rows_in, "echelon_useless_dependencies": ech.useless_dependencies}));
            }
            w_chk += tc.elapsed().as_nanos();
        }
        // stopping rule: 2-core excess >= 1 at a checkpoint, then the LA
        if rows.len() >= next_check {
            next_check = rows.len() + step;
            checks += 1;
            let s = cf_la.ops();
            let (ex, cr, cc) = sparse_la::core_excess(&rows, ncol_bound, &mut cnt);
            debug_assert_eq!(cf_la.ops(), s);
            if ex >= 1 {
                la_calls += 1;
                let mut rep = sparse_la::LaReport::default();
                let dd = sparse_la::solve(&cf_la, &mut cnt, &rows, ncol_bound, ell, &mut la_rng, &prm, track, &mut rep);
                check_log.push(json!({"relations": rows.len(), "targets": ntargets, "core_excess": ex, "core_rows": cr, "core_cols": cc, "la": dd.is_some(),
                    "rows_out": rep.rows_out, "cols_out": rep.cols_out, "att": rep.wiedemann_attempts, "den0": rep.wiedemann_den_zero, "fail": rep.wiedemann_fail, "useless": rep.sge_empty_useless}));
                if let (Some(dv), true) = (dd, track) {
                    let tc = std::time::Instant::now();
                    track_ok = Some(rep.orig_combination.as_ref().map(|cb| sparse_la::check_combination(&rows, cb, ell, dv)).unwrap_or(false));
                    w_chk += tc.elapsed().as_nanos();
                }
                last_rep = rep;
                if let Some(dv) = dd {
                    d = Some(dv);
                    break;
                }
            }
        }
    }
    // finish the shadow (uncounted) if D-S stopped first (not expected: see PROTOCOL.md)
    let s3 = cf_sh.ops();
    let verified = d.map(|dd| c.mul(&cf_sh, dd, g) == q).unwrap_or(false);
    let o_ver = cf_sh.ops().sub(&s3);
    let mut ph: Vec<(&str, Ops)> = vec![
        ("fb", o_fb),
        ("setup", cf_set.ops()),
        ("targets", o_targets),
        ("descent", cf_red.ops()),
        ("rootfind", cf_rf.ops()),
        ("lift", cf_lift.ops()),
        ("resolve", o_resolve),
    ];
    if keep.is_some() {
        ph.push(("fb_filter", o_filter));
    }
    ph.push(("la", cf_la.ops()));
    ph.push(("verify", o_ver));
    let tot = total(&ph);
    let st = dsc.st;
    let wall = t0.elapsed().as_nanos() - w_chk;
    let mut cost = cost_json(&tot, c.n);
    // sensitivity readings (PROTOCOL.md): ±1 products charged as z; SGE bookkeeping charged at u
    let mut t_pm1 = tot;
    t_pm1.z += cnt.pm1_skipped;
    cost["meq_s1_u1_pm1_charged"] = json!(meq(&t_pm1, c.n, 1.0, 1.0));
    cost["meq_s1_u1_pm1_and_structural_charged"] = json!(meq(&t_pm1, c.n, 1.0, 1.0) + cnt.structural as f64);
    let rep = &last_rep;
    let arm = match lam {
        None => "DS".to_string(),
        Some((_, lab, _)) => format!("DSB_{lab}"),
    };
    json!({
        "arm": arm, "d": d, "verified": verified, "targets": ntargets, "relations": rows.len(),
        "columns": ncols_of(&rows), "fb": fb.xs.len(), "fb_cols_used": fcols, "rows_unresolved": unresolved,
        "decompositions_dropped_by_reduced_base": drop_sub, "lambda": lam.map(|l| l.0),
        "la": "sge+wiedemann", "stop_rule": {"start_frac": CHECK_START_FRAC, "step": step, "checks": checks, "la_calls": la_calls, "log": check_log},
        "la_report": {
            "rows_in": rep.rows_in, "cols_in": rep.cols_in, "core_rows": rep.core_rows, "core_cols": rep.core_cols,
            "sge_pivots": rep.sge_pivots, "sge_singletons": rep.sge_singletons, "sge_empty_useless": rep.sge_empty_useless,
            "sge_empty_useful": rep.sge_empty_useful, "excess_pruned": rep.excess_pruned, "rows_out": rep.rows_out, "cols_out": rep.cols_out, "nnz_out": rep.nnz_out,
            "wiedemann_attempts": rep.wiedemann_attempts, "wiedemann_den_zero": rep.wiedemann_den_zero, "wiedemann_fail": rep.wiedemann_fail,
            "minpoly_deg": rep.minpoly_deg, "z_sge": rep.z_sge, "za_sge": rep.za_sge, "z_wied": rep.z_wied, "za_wied": rep.za_wied,
        },
        "la_structural_ops_uncharged": cnt.structural, "la_pm1_products_uncharged": cnt.pm1_skipped,
        "track_check": track_ok, "shadow_echelon": shadow_rec,
        "phases": phases_json(&ph), "total": tot.json(), "cost": cost,
        "wall_ns": wall as u64, "check_wall_ns": w_chk as u64,
        "dstats": {
            "targets": st.targets, "deg_u_mean": st.deg_u_sum as f64 / st.targets.max(1) as f64, "deg_u_max": st.deg_u_max,
            "decompositions": st.decompositions,
            "fallback_rank": st.fallback_rank, "fallback_uzero": st.fallback_uzero,
            "fallback_quartic_zero": st.fallback_quartic_zero,
        },
        "note": "arm D relation phase (#5261 descent.rs, A1 stream) + SGE (singletons, weight-bounded Markowitz pivots by cross-multiplication) + Wiedemann mod ell; stop at the first checkpoint with 2-core excess >= 1 whose LA returns d",
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("selftest") => selftest(args[2].parse().unwrap(), args[3].parse().unwrap()),
        Some("run") => run(&args[2], &args[3], args.get(4).map(|s| s.parse().unwrap()).unwrap_or(1)),
        Some("analyze") => analysis::analyze(&args[2], &args[3], &args[4], &args[5]),
        Some("wcal") => wcal(args[2].parse().unwrap()),
        Some("dev") => dev(args[2].parse().unwrap(), args[3].parse().unwrap(), args[4].parse().unwrap(), args.get(5).map(|s| s == "shadow").unwrap_or(false)),
        _ => eprintln!("usage: see source header"),
    }
}

/// Self-test on separate instances (never evaluation): random sparse
/// systems with a planted log against the echelon; then cold solves.
fn selftest(n: u32, np: u32) {
    let inst = make_curve("bdsl-selftest-curve", n, 0);
    let c = &inst.curve;
    println!("{}", curve_json(n, np, 0, &inst));
    let basis = gen::subfield_basis(&c.gf, np);
    let sub = Sub::new(&basis);
    // 1. synthetic systems: rows = random 3-sparse ±1 over F columns with consistent
    //    right-hand sides a + b d (logs of columns planted); SGE+Wiedemann must give d.
    let ell = c.ell;
    let mut rng = Sm64::keyed(&["bdsl-selftest", "synthetic", &n.to_string()]);
    let cf = CF::new(&c.gf);
    let mut ok = 0;
    for trial in 0..40 {
        let f = 50 + rng.below(400) as usize;
        let dtrue = 1 + rng.below(ell - 1);
        let logs: Vec<u64> = (0..f).map(|_| rng.below(ell)).collect();
        let mut rows = vec![];
        let mut ech = Echelon::new(ell);
        let mut dech = None;
        loop {
            let mut cols: Vec<(usize, i64)> = vec![];
            for _ in 0..3 {
                let k = rng.below(f as u64) as usize;
                let e = if rng.below(2) == 0 { 1 } else { -1 };
                if let Some(x) = cols.iter_mut().find(|x| x.0 == k) {
                    x.1 += e;
                } else {
                    cols.push((k, e));
                }
            }
            cols.retain(|x| x.1 != 0);
            // Σ e·log = a + b·d  →  pick b, set a
            let b = rng.below(ell);
            let s: u64 = cols.iter().fold(0u64, |acc, &(k, e)| (acc + ((e.rem_euclid(ell as i64) as u128 * logs[k] as u128) % ell as u128) as u64) % ell);
            let bd = ((b as u128 * dtrue as u128) % ell as u128) as u64;
            let a = (s + ell - bd) % ell;
            // a relation is Σ e P = a G + b Q, i.e. columns' logs sum to a + b d;
            // the solver's convention is Σ c_j (a_j + b_j d) = 0 over dependencies,
            // so supply the right-hand side as is.
            rows.push(Row { cols, a, b });
            if dech.is_none() {
                dech = ech.add(&cf, rows.last().unwrap());
            }
            if dech.is_some() && rows.len() >= f + 2 {
                break;
            }
        }
        let mut cnt = sparse_la::Cnt::default();
        let mut rep = sparse_la::LaReport::default();
        let prm = sparse_la::Params { kmax: SGE_KMAX, attempts: WIED_ATTEMPTS, u_weight: WIED_U_WEIGHT };
        let mut lr = Sm64::keyed(&["bdsl-selftest", "la", &trial.to_string()]);
        let dd = sparse_la::solve(&cf, &mut cnt, &rows, f, ell, &mut lr, &prm, true, &mut rep);
        let comb_ok = dd.map(|dv| rep.orig_combination.as_ref().map(|cb| sparse_la::check_combination(&rows, cb, ell, dv)).unwrap_or(false));
        assert_eq!(dd, Some(dtrue), "synthetic trial {trial}: SGE+Wiedemann d (rows {}, f {f}, rep {:?})", rows.len(), rep);
        assert_eq!(dech, Some(dtrue), "synthetic trial {trial}: echelon d");
        assert_eq!(comb_ok, Some(true), "synthetic trial {trial}: combination check");
        ok += 1;
        if trial < 3 {
            println!("synthetic f={f} rows={} core={}x{} pivots={} out={}x{} nnz={} attempts={} deg={}", rows.len(), rep.core_rows, rep.core_cols, rep.sge_pivots, rep.rows_out, rep.cols_out, rep.nnz_out, rep.wiedemann_attempts, rep.minpoly_deg);
        }
    }
    println!("{ok}/40 synthetic systems: SGE+Wiedemann d == echelon d == planted d, combination verified");
    // 2. cold solves on selftest instances, D-S against the shadow echelon
    for ti in 1..9u32 {
        let (q, dtrue) = make_target("bdsl-selftest-target", &inst, n, 0, ti);
        let ts = ti.to_string();
        let ns = n.to_string();
        let o = run_ds(c, inst.g, q, &sub, np, &["bdsl-selftest-stream", &ns, &ts], &["bdsl-selftest-la", &ns, &ts], None, true);
        println!(
            "DS ti={ti} d_ok={} verified={} targets={} rels={} shadow_targets={} shadow_rels={} la_calls={} core={}x{} out={}x{} meq={:.3e} la_meq={:.3e} track={}",
            o["d"].as_u64() == Some(dtrue), o["verified"], o["targets"], o["relations"], o["shadow_echelon"]["targets"], o["shadow_echelon"]["relations"],
            o["stop_rule"]["la_calls"], o["la_report"]["core_rows"], o["la_report"]["core_cols"], o["la_report"]["rows_out"], o["la_report"]["cols_out"],
            o["cost"]["meq_s1_u1"].as_f64().unwrap(), field::meq(&Ops::from_json(&o["phases"]["la"]), n, 1.0, 1.0), o["track_check"]
        );
        assert_eq!(o["d"].as_u64(), Some(dtrue));
        assert_eq!(o["shadow_echelon"]["d"].as_u64(), Some(dtrue));
        assert!(o["relations"].as_u64() >= o["shadow_echelon"]["relations"].as_u64());
        if n <= TRACK_MAX_N {
            assert_eq!(o["track_check"].as_bool(), Some(true));
        }
    }
    // 3. reduced base runs
    let (q, dtrue) = make_target("bdsl-selftest-target", &inst, n, 0, 0);
    for (l, lab) in [(2f64.sqrt(), "l1.41"), (2.0, "l2")] {
        let ns = n.to_string();
        let key = ["fbsub", "bdsl-selftest", ns.as_str(), lab];
        let o = run_ds(c, inst.g, q, &sub, np, &["bdsl-selftest-stream", &ns, "0"], &["bdsl-selftest-la", &ns, "0", lab], Some((l, lab, &key)), false);
        println!("DSB {lab} d_ok={} fb_cols={} targets={} rels={} meq={:.3e}", o["d"].as_u64() == Some(dtrue), o["fb_cols_used"], o["targets"], o["relations"], o["cost"]["meq_s1_u1"].as_f64().unwrap());
        assert_eq!(o["d"].as_u64(), Some(dtrue));
    }
    println!("selftest PASS");
}

/// Engineering run on a selftest instance (never evaluation data): one D-S record.
fn dev(n: u32, np: u32, ti: u32, shadow: bool) {
    let inst = make_curve("bdsl-selftest-curve", n, 0);
    let c = &inst.curve;
    let basis = gen::subfield_basis(&c.gf, np);
    let sub = Sub::new(&basis);
    let (q, dtrue) = make_target("bdsl-selftest-target", &inst, n, 0, ti);
    let (ns, ts) = (n.to_string(), ti.to_string());
    let mut o = run_ds(c, inst.g, q, &sub, np, &["bdsl-selftest-stream", &ns, &ts], &["bdsl-selftest-la", &ns, &ts], None, shadow);
    o["correct"] = json!(o["d"].as_u64() == Some(dtrue));
    println!("{}", o);
}

fn run(protocol: &str, out: &str, threads: usize) {
    let p: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(protocol).unwrap()).unwrap();
    let rungs: Vec<serde_json::Value> = p["rungs"].as_array().unwrap().clone();
    let curves = p["curves_per_rung"].as_u64().unwrap() as u32;
    let targets = p["targets_per_curve"].as_u64().unwrap() as u32;
    let file = Arc::new(Mutex::new(std::fs::OpenOptions::new().create(true).append(true).open(out).unwrap()));
    // job: (n, np, ci, ti, lambda label or "", shadow)
    let mut jobs: Vec<(u32, u32, u32, u32, String, bool)> = vec![];
    for r in &rungs {
        let n = r["n"].as_u64().unwrap() as u32;
        let np = r["np"].as_u64().unwrap() as u32;
        let curves = r["curves"].as_u64().map(|x| x as u32).unwrap_or(curves);
        let targets = r["targets"].as_u64().map(|x| x as u32).unwrap_or(targets);
        let shadow = r["shadow_echelon"].as_bool().unwrap_or(false);
        let arms: Vec<String> = r["arms"].as_array().map(|a| a.iter().map(|x| x.as_str().unwrap().to_string()).collect()).unwrap_or(vec!["DS".into()]);
        for arm in arms {
            let lab = if arm == "DS" { String::new() } else { arm.trim_start_matches("DSB_").to_string() };
            for ci in 0..curves {
                for ti in 0..targets {
                    jobs.push((n, np, ci, ti, lab.clone(), shadow && lab.is_empty()));
                }
            }
        }
    }
    jobs.sort_by_key(|j| std::cmp::Reverse((j.0, j.4.clone(), j.2, j.3)));
    let jobs = Arc::new(Mutex::new(jobs));
    let mut hs = vec![];
    for _ in 0..threads {
        let (jobs, file) = (jobs.clone(), file.clone());
        hs.push(std::thread::spawn(move || loop {
            let job = jobs.lock().unwrap().pop();
            let Some((n, np, ci, ti, lab, shadow)) = job else { break };
            let inst = make_curve("curve", n, ci);
            let c = &inst.curve;
            let (q, d_true) = make_target("target", &inst, n, ci, ti);
            let basis = gen::subfield_basis(&c.gf, np);
            let sub = Sub::new(&basis);
            let (ns, cs, ts) = (n.to_string(), ci.to_string(), ti.to_string());
            let stream = ["icstream-a1", ns.as_str(), cs.as_str(), ts.as_str()];
            let la_stream_ds = ["la-ds", ns.as_str(), cs.as_str(), ts.as_str()];
            let la_stream_b = ["la-dsb", ns.as_str(), cs.as_str(), ts.as_str(), lab.as_str()];
            let key = ["fbsub", ns.as_str(), cs.as_str(), lab.as_str()];
            let lam = match lab.as_str() {
                "" => None,
                "l1.41" => Some((2f64.sqrt(), "l1.41", &key[..])),
                "l2" => Some((2.0, "l2", &key[..])),
                other => panic!("unknown arm label {other}"),
            };
            let la_stream: &[&str] = if lam.is_none() { &la_stream_ds } else { &la_stream_b };
            let mut v = run_ds(c, inst.g, q, &sub, np, &stream, la_stream, lam, shadow);
            v["case"] = json!({"n": n, "np": np, "ci": ci, "ti": ti, "curve": curve_json(n, np, ci, &inst)});
            v["correct"] = json!(v["d"].as_u64() == Some(d_true));
            let mut f = file.lock().unwrap();
            writeln!(f, "{}", v).unwrap();
            f.flush().unwrap();
            eprintln!("case n={n} ci={ci} ti={ti} {} done correct={} meq={:.3e}", v["arm"], v["correct"], v["cost"]["meq_s1_u1"].as_f64().unwrap());
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
}

/// Indicative only (not an isolated host, no PMU): best-of-7 wall time of a
/// mod-ℓ product (u128 %) against one F_{2^n} multiplication of crypto's Gf2.
fn wcal(n: u32) {
    let inst = make_curve("bdsl-wcal-curve", n, 0);
    let c = &inst.curve;
    let ell = c.ell;
    let mut rng = Sm64::keyed(&["bdsl-wcal", &n.to_string()]);
    let xs: Vec<u64> = (0..4096).map(|_| rng.below(ell)).collect();
    let fs: Vec<u64> = (0..4096).map(|_| rng.bits(n).max(1)).collect();
    let reps = 2000;
    let (mut best_z, mut best_m) = (f64::MAX, f64::MAX);
    for _ in 0..7 {
        let t = std::time::Instant::now();
        let mut acc = 1u64;
        for _ in 0..reps {
            for &x in &xs {
                acc = ((acc as u128 * x as u128) % ell as u128) as u64 | 1;
            }
        }
        std::hint::black_box(acc);
        best_z = best_z.min(t.elapsed().as_nanos() as f64 / (reps * xs.len()) as f64);
        let t = std::time::Instant::now();
        let mut acc = 1u64;
        for _ in 0..reps {
            for &x in &fs {
                acc = c.gf.mul(acc, x) | 1;
            }
        }
        std::hint::black_box(acc);
        best_m = best_m.min(t.elapsed().as_nanos() as f64 / (reps * fs.len()) as f64);
    }
    println!("{}", json!({"n": n, "ell_bits": 64 - ell.leading_zeros(), "ns_per_mod_ell_product": best_z, "ns_per_field_mul": best_m, "ratio_z_over_m": best_z / best_m, "method": "best-of-7 dependent-chain wall time, shared 4-vCPU container, indicative only"}));
}
