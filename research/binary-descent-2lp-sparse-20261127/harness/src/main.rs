//! b2lsp_harness — arm D2S: #5291's double-large-prime (2LP) variant of the
//! #5261 Gaudry–Diem k = 3 Weil-descent oracle, with a shortest-cycle (BFS)
//! large-prime forest (`bfs.rs`) and #5290's sparse linear algebra
//! (SGE + scalar Wiedemann mod ℓ, `sparse_la.rs`).  Study `b2lsp-20261127/v1`.
//!
//! Reused unchanged by `#[path]`: #5253's field, curves, factor base, A1
//! relation-target stream, group-law resolution, echelon (cross-check only)
//! and rho; #5261's `descent.rs`; #5291's `graph.rs` (`Comb`, `inv_mod`, and
//! `LpGraph` as an uncounted shadow for the yield-count identity); #5290's
//! `sparse_la.rs`.
//!
//! ```text
//! b2lsp_harness selftest <n> <n'>
//! b2lsp_harness pilot <out.jsonl> <threads> <n>...           (held-out alpha pilot; grid in source)
//! b2lsp_harness pilot-summary <pilot.jsonl>                  (part A rule -> beta, theta)
//! b2lsp_harness predict <out.jsonl> <beta> <theta> <n>...     (abstract-graph simulation)
//! b2lsp_harness rho-pilot <out.jsonl> <threads> <n> <W>...    (held-out rho-lane pilot)
//! b2lsp_harness run <protocol.json> <out.jsonl> [threads]
//! b2lsp_harness analyze <protocol.json> <runs.jsonl> <summary.json> <figure.svg>
//! ```
#![allow(dead_code, unused_imports, clippy::all)]
#![recursion_limit = "256"]
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
#[path = "../../../binary-descent-2lp-20261127/harness/src/graph.rs"]
mod graph;
#[path = "../../../binary-descent-sparse-la-20261127/harness/src/sparse_la.rs"]
mod sparse_la;

mod analysis;
mod bfs;

use bfs::BfsLp;
use descent::Descent;
use field::{meq, Curve, Ops, Pt, CF};
use graph::{Comb, LpGraph};
use ic::*;
use seeds::Sm64;
use serde_json::json;
use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex};

// ── verbatim from the #5253 / #5261 / #5291 main.rs (same curves and targets) ──
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
/// #5291's small-base rule: `fs = min(F − 1, ⌈θ · F^β⌉)`; θ < 0 means every class small.
pub fn fs_rule(f: usize, beta: f64, theta: f64) -> usize {
    if theta < 0.0 {
        return f - 1;
    }
    ((theta * (f as f64).powf(beta)).ceil() as usize).clamp(1, f - 1)
}
/// #5291's group-law check of one relation over the small base (uncounted).
fn check_comb(c: &Curve, cf: &CF, fb: &Fb, hp: &mut HashMap<usize, Pt>, hg: Pt, hq: Pt, y: &Comb) -> bool {
    let mut lhs = Pt::Inf;
    for &(col, e) in &y.cols {
        let p = *hp.entry(col).or_insert_with(|| c.mul(cf, c.h, fb.pts[col]));
        lhs = c.add(cf, lhs, c.mul(cf, e, p));
    }
    let rhs = c.joint(cf, y.a, hg, y.b, hq);
    lhs == rhs
}
// ── end verbatim ──

/// #5290's frozen stopping-rule and LA parameters, unchanged (its PROTOCOL.md).
pub const CHECK_START_FRAC: f64 = 0.70;
pub const CHECK_STEPS_PER_FB: f64 = 256.0;
pub const SGE_KMAX: usize = 64;
pub const WIED_ATTEMPTS: u32 = 4;
pub const WIED_U_WEIGHT: usize = 16;
pub const TRACK_MAX_N: u32 = 30;

pub struct Opts {
    /// group-law check of every row fed to the LA (uncounted)
    pub check_rows: bool,
    /// shadow #5291 LpGraph on the same relations: yield-count identity (uncounted)
    pub shadow_lp: bool,
    /// echelon cross-check of the sparse kernel when n <= TRACK_MAX_N (uncounted)
    pub xcheck: bool,
}

/// One cold single-target solve with arm D2S: A1 stream, #5261 oracle,
/// small base `1..=fs`, BFS forest, #5290 stopping rule and SGE + Wiedemann.
pub fn run_d2s(c: &Curve, g: Pt, q: Pt, sub: &Sub, np: u32, stream: &[&str], la_stream: &[&str], fs_req: Option<usize>, beta: f64, theta: f64, o: &Opts) -> serde_json::Value {
    let gf = &c.gf;
    let ell = c.ell;
    let t0 = std::time::Instant::now();
    let (cf_sh, cf_set, cf_red, cf_rf, cf_lift, cf_gr, cf_la) = (CF::new(gf), CF::new(gf), CF::new(gf), CF::new(gf), CF::new(gf), CF::new(gf), CF::new(gf));
    let (cf_chk, cf_shadow) = (CF::new(gf), CF::new(gf)); // uncounted
    let fb = build_fb(c, &cf_sh, sub);
    let o_fb = cf_sh.ops();
    let f = fb.xs.len();
    assert_eq!(fb.xs[0], 0, "T is column 0");
    let fs = fs_req.map(|x| x.clamp(1, f - 1)).unwrap_or_else(|| fs_rule(f, beta, theta));
    let mut dsc = Descent::new(c, &cf_set, sub, np);
    let mut gr = BfsLp::new(ell, f, fs);
    let mut lp = if o.shadow_lp { Some(LpGraph::new(ell, f, fs)) } else { None };
    let mut rng = Sm64::keyed(stream);
    let mut la_rng = Sm64::keyed(la_stream);
    let mut tg = Targets::new_ap(c, &cf_sh, g, q, &mut rng);
    let (mut o_targets, mut o_resolve) = (Ops::default(), Ops::default());
    let mut set_d = vec![];
    let (mut unresolved, mut ntargets, mut nrel) = (0u64, 0u64, 0u64);
    let mut w_chk = 0u128;
    let mut d = None;
    let cap = 400 * f as u64 + 2000;
    let step = ((fs as f64 / CHECK_STEPS_PER_FB).ceil() as u64).max(1);
    let mut next_check = ((CHECK_START_FRAC * fs as f64).ceil() as u64).max(1);
    let mut cnt = sparse_la::Cnt::default();
    let prm = sparse_la::Params { kmax: SGE_KMAX, attempts: WIED_ATTEMPTS, u_weight: WIED_U_WEIGHT };
    let (mut checks, mut la_calls) = (0u64, 0u64);
    let mut check_log = vec![];
    let mut last_rep = sparse_la::LaReport::default();
    let mut last_build = bfs::Build::default();
    let track = c.n <= TRACK_MAX_N;
    let mut track_ok: Option<bool> = None;
    let mut xcheck_ok: Option<bool> = None;
    let (mut row_checks, mut row_bad) = (0u64, 0u64);
    let (mut lp_cmp, mut lp_bad) = (0u64, 0u64);
    let mut hp: HashMap<usize, Pt> = HashMap::new();
    let (hg, hq) = (c.mul(&cf_chk, c.h, g), c.mul(&cf_chk, c.h, q));
    let mut rows_t: Vec<Row> = vec![];
    let mut la_rows_last = 0usize;
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
            gr.add(&cf_gr, row);
            if let Some(l) = lp.as_mut() {
                let tc = std::time::Instant::now();
                l.add(&cf_shadow, row);
                w_chk += tc.elapsed().as_nanos();
            }
        }
        // stopping rule (#5290's, on the yield upper bound): checkpoint, BFS build, 2-core excess, LA
        if gr.y_ub() >= next_check {
            next_check = gr.y_ub() + step;
            checks += 1;
            let (ys, bstat) = gr.build(&cf_gr);
            if let Some(l) = lp.as_ref() {
                let tc = std::time::Instant::now();
                lp_cmp += 1;
                let s = l.st;
                if s.full + s.yields_cycle + s.yields_join != bstat.yields_full + bstat.yields_cycle + bstat.empty_dropped {
                    lp_bad += 1;
                }
                w_chk += tc.elapsed().as_nanos();
            }
            let rows: Vec<Row> = ys.iter().map(|y| y.to_row()).collect();
            let (ex, cr, cc) = sparse_la::core_excess(&rows, fs + 1, &mut cnt);
            last_build = bstat;
            if ex >= 1 {
                la_calls += 1;
                if o.check_rows {
                    let tc = std::time::Instant::now();
                    for y in &ys {
                        row_checks += 1;
                        assert!(y.cols.iter().all(|e| gr.is_small(e.0)), "row free of large primes");
                        if !check_comb(c, &cf_chk, &fb, &mut hp, hg, hq, y) {
                            row_bad += 1;
                        }
                    }
                    w_chk += tc.elapsed().as_nanos();
                }
                let mut rep = sparse_la::LaReport::default();
                let dd = sparse_la::solve(&cf_la, &mut cnt, &rows, fs + 1, ell, &mut la_rng, &prm, track, &mut rep);
                check_log.push(json!({"yields": rows.len(), "yields_b0": ys.iter().filter(|y| y.b == 0).count(), "yields_empty": ys.iter().filter(|y| y.cols.is_empty()).count(), "y_ub": gr.y_ub(), "targets": ntargets, "core_excess": ex, "core_rows": cr, "core_cols": cc, "la": dd.is_some(),
                    "rows_out": rep.rows_out, "cols_out": rep.cols_out, "att": rep.wiedemann_attempts, "den0": rep.wiedemann_den_zero, "fail": rep.wiedemann_fail, "useless": rep.sge_empty_useless}));
                if let (Some(dv), true) = (dd, track) {
                    let tc = std::time::Instant::now();
                    track_ok = Some(rep.orig_combination.as_ref().map(|cb| sparse_la::check_combination(&rows, cb, ell, dv)).unwrap_or(false));
                    if o.xcheck {
                        let mut ech = Echelon::new(ell);
                        let mut de = None;
                        for rr in &rows {
                            de = ech.add(&cf_shadow, rr);
                            if de.is_some() {
                                break;
                            }
                        }
                        xcheck_ok = Some(de == Some(dv));
                    }
                    w_chk += tc.elapsed().as_nanos();
                }
                la_rows_last = rows.len();
                last_rep = rep;
                if let Some(dv) = dd {
                    d = Some(dv);
                    break;
                }
            }
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
        ("la", cf_la.ops()),
        ("verify", o_ver),
    ];
    let tot = total(&ph);
    let st = dsc.st;
    let gs = gr.st;
    let wall = t0.elapsed().as_nanos() - w_chk;
    let mut cost = cost_json(&tot, c.n);
    let mut t_pm1 = tot;
    t_pm1.z += cnt.pm1_skipped + gs.pm1_skipped;
    cost["meq_s1_u1_pm1_charged"] = json!(meq(&t_pm1, c.n, 1.0, 1.0));
    cost["meq_s1_u1_pm1_and_structural_charged"] = json!(meq(&t_pm1, c.n, 1.0, 1.0) + cnt.structural as f64);
    let b = &last_build;
    let rep = &last_rep;
    json!({
        "d": d, "verified": verified, "targets": ntargets, "relations": nrel, "fb": f, "fs": fs, "beta": beta, "theta": theta,
        "rows_unresolved": unresolved, "capped": d.is_none() && ntargets >= cap, "target_cap": cap,
        "la": "sge+wiedemann (#5290 sparse_la.rs)", "forest": "bfs",
        "stop_rule": {"start_frac": CHECK_START_FRAC, "step": step, "checks": checks, "la_calls": la_calls, "log": check_log},
        "la_rows": la_rows_last,
        "la_report": {
            "rows_in": rep.rows_in, "cols_in": rep.cols_in, "core_rows": rep.core_rows, "core_cols": rep.core_cols,
            "sge_pivots": rep.sge_pivots, "sge_singletons": rep.sge_singletons, "sge_empty_useless": rep.sge_empty_useless,
            "sge_empty_useful": rep.sge_empty_useful, "excess_pruned": rep.excess_pruned, "rows_out": rep.rows_out, "cols_out": rep.cols_out, "nnz_out": rep.nnz_out,
            "wiedemann_attempts": rep.wiedemann_attempts, "wiedemann_den_zero": rep.wiedemann_den_zero, "wiedemann_fail": rep.wiedemann_fail,
            "minpoly_deg": rep.minpoly_deg, "z_sge": rep.z_sge, "za_sge": rep.za_sge, "z_wied": rep.z_wied, "za_wied": rep.za_wied,
        },
        "la_structural_ops_uncharged": cnt.structural, "la_pm1_products_uncharged": cnt.pm1_skipped, "graph_pm1_products_uncharged": gs.pm1_skipped,
        "track_check": track_ok, "echelon_xcheck": xcheck_ok,
        "row_checks": row_checks, "row_check_failures": row_bad,
        "lp_shadow_compares": lp_cmp, "lp_shadow_mismatches": lp_bad,
        "graph": {
            "rows": gs.rows, "full": gs.full, "one_lp": gs.one_lp, "two_lp": gs.two_lp, "discarded_3lp": gs.discarded_3lp,
            "cycle_edges_uf": gs.cycle_edges_uf, "builds": gs.builds,
            "yields_full": b.yields_full, "yields_cycle": b.yields_cycle, "groundings": b.groundings, "tree_edges": b.tree_edges,
            "comps_bfs": b.comps_bfs, "max_depth": b.max_depth, "mean_depth_used": b.mean_depth_used,
            "reduce_steps": b.reduce_steps, "max_reduce_steps": b.max_reduce_steps,
            "yield_weight_mean": b.weight_sum as f64 / (b.yields_full + b.yields_cycle).max(1) as f64, "yield_weight_max": b.weight_max,
            "cycle_len_mean": b.cycle_len_sum as f64 / b.yields_cycle.max(1) as f64, "cycle_len_max": b.cycle_len_max,
            "empty_dropped": b.empty_dropped,
        },
        "phases": phases_json(&ph), "total": tot.json(), "cost": cost,
        "wall_ns": wall as u64, "check_wall_ns": w_chk as u64,
        "dstats": {
            "targets": st.targets, "deg_u_mean": st.deg_u_sum as f64 / st.targets.max(1) as f64, "deg_u_max": st.deg_u_max,
            "decompositions": st.decompositions,
            "fallback_rank": st.fallback_rank, "fallback_uzero": st.fallback_uzero,
            "fallback_quartic_zero": st.fallback_quartic_zero,
        },
        "note": "#5261 k=3 descent oracle and A1 stream unchanged; relations with <=2 large primes stored; BFS spanning forest rebuilt at each #5290 checkpoint; deepest-first cycle reduction; #5290 SGE + Wiedemann on the small base",
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("selftest") => selftest(args[2].parse().unwrap(), args[3].parse().unwrap()),
        Some("pilot") => pilot(&args[2], args[3].parse().unwrap(), args[4..].iter().map(|s| s.parse().unwrap()).collect()),
        Some("pilot-summary") => analysis::pilot_summary(&args[2]),
        Some("predict") => predict(&args[2], args[3].parse().unwrap(), args[4].parse().unwrap(), args[5..].iter().map(|s| s.parse().unwrap()).collect()),
        Some("predict-summary") => analysis::predict_summary(&args[2], &args[3]),
        Some("rho-pilot") => rho_pilot(&args[2], args[3].parse().unwrap(), args[4].parse().unwrap(), args[5..].iter().map(|s| s.parse().unwrap()).collect()),
        Some("dev") => dev(args[2].parse().unwrap(), args[3].parse().unwrap(), args[4].parse().unwrap(), args[5].parse().unwrap()),
        Some("dev-ds") => dev_ds(args[2].parse().unwrap(), args[3].parse().unwrap(), args[4].parse().unwrap()),
        Some("run") => run(&args[2], &args[3], args.get(4).map(|s| s.parse().unwrap()).unwrap_or(1)),
        Some("analyze") => analysis::analyze(&args[2], &args[3], &args[4], &args[5]),
        _ => eprintln!("usage: see source header"),
    }
}

/// Planted-log graph self-test: every yield satisfies the planted logs; the
/// yield count equals #5291 LpGraph's on the same stream (rank identity).
fn graph_selftest() {
    let ell: u64 = (1u64 << 61) - 1;
    let gf = cryptanalysis::semaev_decomp::Gf2::new(&gen::irreducible(13));
    let cf = CF::new(&gf);
    let mm = |a: u64, b: u64| ((a as u128 * b as u128) % ell as u128) as u64;
    for (f, fs, seed) in [(200usize, 20usize, "a"), (2000, 60, "b"), (500, 499, "c"), (1000, 5, "d"), (4000, 300, "e"), (300, 1, "g")] {
        let mut rng = Sm64::keyed(&["b2lsp-selftest-graph", seed]);
        let logs: Vec<u64> = (0..f).map(|_| rng.below(ell)).collect();
        let dl = rng.below(ell);
        let mut gr = BfsLp::new(ell, f, fs);
        let mut lp = LpGraph::new(ell, f, fs);
        let mut lpy = 0u64;
        let mut builds = 0;
        for k in 0..(6 * f) {
            // seed "g": only 2-large-prime rows (no SV edges), so components ground on frustrated cycles
            let kk = if seed == "g" { 2 } else { 1 + (rng.below(3) as usize) };
            let mut cols: Vec<(usize, i64)> = vec![];
            for _ in 0..kk {
                let col = if seed == "g" { 2 + rng.below((f - 2) as u64) as usize } else { 1 + rng.below((f - 1) as u64) as usize };
                let s = if rng.bits(1) == 1 { 1 } else { -1 };
                if let Some(e) = cols.iter_mut().find(|e| e.0 == col) {
                    e.1 += s;
                } else {
                    cols.push((col, s));
                }
            }
            cols.retain(|e| e.1 != 0);
            if seed == "g" {
                // two distinct large primes, signs ±1
                let c1 = 2 + rng.below((f - 2) as u64) as usize;
                let c2 = 2 + (c1 - 2 + 1 + rng.below((f - 3) as u64) as usize) % (f - 2);
                cols = vec![(c1, if rng.bits(1) == 1 { 1 } else { -1 }), (c2, if rng.bits(1) == 1 { 1 } else { -1 })];
            }
            let lhs = cols.iter().fold(0u64, |acc, &(c, e)| (acc + mm(((e % ell as i64 + ell as i64) as u64) % ell, logs[c])) % ell);
            let b = rng.below(ell);
            let a = (lhs + ell - mm(b, dl)) % ell;
            let row = Row { cols, a, b };
            gr.add(&cf, &row);
            if lp.add(&cf, &row).is_some() {
                lpy += 1;
            }
            if k % (f / 2) == f / 2 - 1 {
                builds += 1;
                let (ys, bs) = gr.build(&cf);
                assert_eq!(ys.len() as u64 + bs.empty_dropped, lpy, "BFS yields = LpGraph yields (rank identity)");
                assert!(ys.len() as u64 <= gr.y_ub());
                for y in &ys {
                    assert!(y.cols.iter().all(|e| gr.is_small(e.0)));
                    let l = y.cols.iter().fold(0u64, |acc, &(c, e)| (acc + mm(e, logs[c])) % ell);
                    assert_eq!(l, (y.a + mm(y.b, dl)) % ell, "yield satisfies the planted logs");
                }
                if builds == 12 {
                    println!("graph selftest F={f} fs={fs}: rows={} yields={} (LpGraph {lpy}) y_ub={} cycle_len mean={:.2} max={} weight mean={:.2} max={} groundings={} | LpGraph max_reduce={} ",
                        gr.st.rows, ys.len(), gr.y_ub(), bs.cycle_len_sum as f64 / bs.yields_cycle.max(1) as f64, bs.cycle_len_max,
                        bs.weight_sum as f64 / ys.len().max(1) as f64, bs.weight_max, bs.groundings, lp.st.max_reduce_steps);
                }
            }
        }
    }
    println!("graph selftest PASS");
}

fn selftest(n: u32, np: u32) {
    graph_selftest();
    let inst = make_curve("b2lsp-selftest-curve", n, 0);
    let c = &inst.curve;
    println!("{}", curve_json(n, np, 0, &inst));
    let basis = gen::subfield_basis(&c.gf, np);
    let sub = Sub::new(&basis);
    let o = Opts { check_rows: true, shadow_lp: true, xcheck: true };
    for ti in 0..6u32 {
        let (q, dtrue) = make_target("b2lsp-selftest-target", &inst, n, 0, ti);
        let (ns, ts) = (n.to_string(), ti.to_string());
        let st = ["b2lsp-selftest-stream", ns.as_str(), ts.as_str()];
        for (beta, theta) in [(2.0 / 3.0, -1.0), (2.0 / 3.0, 1.0), (2.0 / 3.0, 3.0), (0.75, 2.0)] {
            let th = format!("{theta}");
            let v = run_d2s(c, inst.g, q, &sub, np, &st, &["b2lsp-selftest-la", &ns, &ts, &th], None, beta, theta, &o);
            println!(
                "D2S ti={ti} beta={beta:.3} theta={theta} fs={}/{} d_ok={} verified={} targets={} rels={} la_calls={} la_rows={} rowchk={}/{} lp={}/{} track={} xcheck={} meq={:.3e} w={:.2} clen={:.2}/{}",
                v["fs"], v["fb"], v["d"].as_u64() == Some(dtrue), v["verified"], v["targets"], v["relations"], v["stop_rule"]["la_calls"], v["la_rows"],
                v["row_checks"], v["row_check_failures"], v["lp_shadow_compares"], v["lp_shadow_mismatches"], v["track_check"], v["echelon_xcheck"],
                v["cost"]["meq_s1_u1"].as_f64().unwrap(), v["graph"]["yield_weight_mean"].as_f64().unwrap(), v["graph"]["cycle_len_mean"].as_f64().unwrap(), v["graph"]["cycle_len_max"]
            );
            assert_eq!(v["d"].as_u64(), Some(dtrue));
            assert_eq!(v["row_check_failures"].as_u64(), Some(0));
            assert_eq!(v["lp_shadow_mismatches"].as_u64(), Some(0));
            if n <= TRACK_MAX_N {
                assert_eq!(v["track_check"].as_bool(), Some(true));
                assert_eq!(v["echelon_xcheck"].as_bool(), Some(true));
            }
        }
    }
    println!("selftest PASS");
}

/// Engineering run on a selftest instance (never evaluation data): fs = ⌈α(F − 1)⌉.
fn dev(n: u32, np: u32, ti: u32, alpha: f64) {
    let inst = make_curve("b2lsp-selftest-curve", n, 0);
    let c = &inst.curve;
    let sub = Sub::new(&gen::subfield_basis(&c.gf, np));
    let (q, dtrue) = make_target("b2lsp-selftest-target", &inst, n, 0, ti);
    let (ns, ts) = (n.to_string(), ti.to_string());
    let fbl = { let cf = CF::new(&c.gf); build_fb(c, &cf, &sub).xs.len() };
    let o = Opts { check_rows: true, shadow_lp: true, xcheck: true };
    let mut v = run_d2s(c, inst.g, q, &sub, np, &["b2lsp-selftest-stream", &ns, &ts], &["b2lsp-selftest-la", &ns, &ts], Some(((alpha * (fbl - 1) as f64).ceil() as usize).max(1)), f64::NAN, f64::NAN, &o);
    v["correct"] = json!(v["d"].as_u64() == Some(dtrue));
    v["stop_rule"]["log"] = json!(null);
    println!("{}", v);
}

/// Engineering identity check on #5290's selftest instance (never evaluation data):
/// D2S with every class small on #5290's streams must equal `bdsl_harness dev n n' ti`.
fn dev_ds(n: u32, np: u32, ti: u32) {
    let inst = make_curve("bdsl-selftest-curve", n, 0);
    let c = &inst.curve;
    let sub = Sub::new(&gen::subfield_basis(&c.gf, np));
    let (q, dtrue) = make_target("bdsl-selftest-target", &inst, n, 0, ti);
    let (ns, ts) = (n.to_string(), ti.to_string());
    let o = Opts { check_rows: true, shadow_lp: true, xcheck: true };
    let mut v = run_d2s(c, inst.g, q, &sub, np, &["bdsl-selftest-stream", &ns, &ts], &["bdsl-selftest-la", &ns, &ts], None, 2.0 / 3.0, -1.0, &o);
    v["correct"] = json!(v["d"].as_u64() == Some(dtrue));
    println!("{}", v);
}

/// Abstract simulation (prediction only): F classes, Poisson(1/6) decompositions
/// per target of 3 uniform classes with uniform signs (#5291's model), the same
/// BfsLp, stopping rule and sparse LA; random right-hand sides.
fn predict(out: &str, beta: f64, theta: f64, ns: Vec<u32>) {
    let mut fo = std::fs::OpenOptions::new().create(true).append(true).open(out).unwrap();
    let ell: u64 = (1u64 << 61) - 1;
    let gf = cryptanalysis::semaev_decomp::Gf2::new(&gen::irreducible(13));
    for n in ns {
        let np = n / 3;
        let f = (1usize << np) / 2 + 1;
        let reps = if n >= 51 { 1 } else { 2 };
        for rep in 0..reps {
            let mut rng = Sm64::keyed(&["b2lsp-predict", &n.to_string(), &format!("{beta}"), &format!("{theta}"), &rep.to_string()]);
            let mut lr = Sm64::keyed(&["b2lsp-predict-la", &n.to_string(), &rep.to_string()]);
            let fs = fs_rule(f, beta, theta);
            let (cg, cl) = (CF::new(&gf), CF::new(&gf));
            let mut gr = BfsLp::new(ell, f, fs);
            let mut cnt = sparse_la::Cnt::default();
            let prm = sparse_la::Params { kmax: SGE_KMAX, attempts: WIED_ATTEMPTS, u_weight: WIED_U_WEIGHT };
            let step = ((fs as f64 / CHECK_STEPS_PER_FB).ceil() as u64).max(1);
            let mut next_check = ((CHECK_START_FRAC * fs as f64).ceil() as u64).max(1);
            let mut tg = 0u64;
            let mut done = false;
            let lam = 1.0 / 6.0;
            let el = (-lam as f64).exp();
            let mut bl = bfs::Build::default();
            let mut nrows = 0;
            let mut calls = 0;
            while !done && tg < 400 * f as u64 {
                tg += 1;
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
                    gr.add(&cg, &Row { cols, a: rng.below(ell), b: rng.below(ell) });
                }
                if gr.y_ub() >= next_check {
                    next_check = gr.y_ub() + step;
                    let (ys, b) = gr.build(&cg);
                    let rows: Vec<Row> = ys.iter().map(|y| y.to_row()).collect();
                    let (ex, _, _) = sparse_la::core_excess(&rows, fs + 1, &mut cnt);
                    bl = b;
                    if ex >= 1 {
                        calls += 1;
                        let mut rp = sparse_la::LaReport::default();
                        nrows = rows.len();
                        if sparse_la::solve(&cl, &mut cnt, &rows, fs + 1, ell, &mut lr, &prm, false, &mut rp).is_some() {
                            done = true;
                        }
                    }
                }
            }
            let (og, ol) = (cg.ops(), cl.ops());
            let v = json!({"n": n, "np": np, "F": f, "fs": fs, "beta": beta, "theta": theta, "rep": rep, "solved": done,
                "targets": tg, "targets_over_qp": tg as f64 / (1u64 << np) as f64, "yields": nrows, "la_calls": calls,
                "full": gr.st.full, "one_lp": gr.st.one_lp, "two_lp": gr.st.two_lp, "groundings": bl.groundings,
                "yield_weight_mean": bl.weight_sum as f64 / (bl.yields_full + bl.yields_cycle).max(1) as f64,
                "cycle_len_mean": bl.cycle_len_sum as f64 / bl.yields_cycle.max(1) as f64, "cycle_len_max": bl.cycle_len_max, "max_depth": bl.max_depth,
                "graph_units": og.h + og.z + og.za, "la_units": ol.z + ol.za});
            writeln!(fo, "{}", v).unwrap();
            eprintln!("{}", v);
        }
    }
}

/// Small-base grid of the held-out pilot (PROTOCOL.md part A): fs = ⌈α·(F − 1)⌉.
pub const PILOT_ALPHA: [f64; 9] = [0.045, 0.067, 0.1, 0.15, 0.22, 0.33, 0.5, 0.75, 1.0];

/// Held-out alpha pilot: `d2spilot-*` instances, 2 curves × 2 targets per rung.
fn pilot(out: &str, threads: usize, ns: Vec<u32>) {
    let mut jobs = vec![];
    for &n in &ns {
        for ci in 0..2u32 {
            for ti in 0..2u32 {
                for &a in &PILOT_ALPHA {
                    jobs.push((n, ci, ti, a));
                }
            }
        }
    }
    jobs.reverse();
    par_jobs(out, threads, jobs, |(n, ci, ti, alpha)| {
        let np = n / 3;
        let inst = make_curve("d2spilot-curve", n, ci);
        let c = &inst.curve;
        let (q, dt) = make_target("d2spilot-target", &inst, n, ci, ti);
        let sub = Sub::new(&gen::subfield_basis(&c.gf, np));
        let f = (1usize << np) / 2 + 1; // provisional; fs recomputed from the real F below
        let _ = f;
        let (ns_, cs, ts, as_) = (n.to_string(), ci.to_string(), ti.to_string(), format!("{alpha}"));
        // F is known only after the factor base is built; run_d2s clamps fs to F − 1.
        let fb_len = { let cf = CF::new(&c.gf); build_fb(c, &cf, &sub).xs.len() };
        let fs = ((alpha * (fb_len - 1) as f64).ceil() as usize).max(1);
        let o = Opts { check_rows: false, shadow_lp: false, xcheck: false };
        let mut v = run_d2s(c, inst.g, q, &sub, np, &["d2spilot-stream", &ns_, &cs, &ts], &["d2spilot-la", &ns_, &cs, &ts, &as_], Some(fs), f64::NAN, f64::NAN, &o);
        v["arm"] = json!("D2Spilot");
        v["alpha"] = json!(alpha);
        v["case"] = json!({"n": n, "np": np, "ci": ci, "ti": ti});
        v["correct"] = json!(v["d"].as_u64() == Some(dt));
        v
    });
}

/// Held-out rho-lane pilot (same rule as #5253 / #5291).
fn rho_pilot(out: &str, threads: usize, n: u32, ws: Vec<usize>) {
    let mut jobs = vec![];
    for ci in 0..2u32 {
        for &w in &ws {
            for ti in 0..8u32 {
                jobs.push((n, ci, ti, w));
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

/// Evaluation run: arms from the protocol (`D2S`, `D2S23`, `D2Sall`, `RHO`), per rung.
/// A rung may restrict cases with `cases: [[ci, ti], ...]` (used for rho top-ups).
fn run(protocol: &str, out: &str, threads: usize) {
    let p: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(protocol).unwrap()).unwrap();
    let rungs: Vec<serde_json::Value> = p["rungs"].as_array().unwrap().clone();
    let curves = p["curves_per_rung"].as_u64().unwrap() as u32;
    let targets = p["targets_per_curve"].as_u64().unwrap() as u32;
    let rho_seeds = p["rho_seeds_per_target"].as_u64().unwrap() as u32;
    let armdef = p["arms_d2s"].clone();
    let mut jobs: Vec<(u32, u32, u32, u32, String, u32)> = vec![];
    for r in &rungs {
        let n = r["n"].as_u64().unwrap() as u32;
        let np = r["np"].as_u64().unwrap() as u32;
        let curves = r["curves"].as_u64().map(|x| x as u32).unwrap_or(curves);
        let targets = r["targets"].as_u64().map(|x| x as u32).unwrap_or(targets);
        let arms: Vec<String> = r["run_arms"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_string()).collect();
        let cases: Vec<(u32, u32)> = match r["cases"].as_array() {
            Some(cs) => cs.iter().map(|x| (x[0].as_u64().unwrap() as u32, x[1].as_u64().unwrap() as u32)).collect(),
            None => (0..curves).flat_map(|ci| (0..targets).map(move |ti| (ci, ti))).collect(),
        };
        for &(ci, ti) in &cases {
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
            // D2Sall uses #5290's LA stream so that it must reproduce D-S bitwise
            let la: Vec<String> = if arm == "D2Sall" { vec!["la-ds".into(), ns.clone(), cs.clone(), ts.clone()] } else { vec!["la-d2s".into(), ns.clone(), cs.clone(), ts.clone(), arm.clone()] };
            let la_ref: Vec<&str> = la.iter().map(|s| s.as_str()).collect();
            let o = Opts { check_rows: true, shadow_lp: true, xcheck: true };
            let mut v = run_d2s(c, inst.g, q, &sub, np, &["icstream-a1", &ns, &cs, &ts], &la_ref, None, beta, theta, &o);
            v["arm"] = json!(arm);
            v
        };
        v["case"] = json!({"n": n, "np": np, "ci": ci, "ti": ti, "curve": curve_json(n, np, ci, &inst)});
        v["correct"] = json!(v["d"].as_u64() == Some(d_true));
        eprintln!("case n={n} ci={ci} ti={ti} {arm} si={si} correct={} meq={:.3e}", v["correct"], v["cost"]["meq_s1_u1"].as_f64().unwrap());
        v
    });
}
