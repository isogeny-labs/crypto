//! bcr_harness — binary composite index calculus with the tuned S₄ root
//! finder against matched negation-map rho.  Study `bcr-20261126/v1`.
//!
//! ```text
//! bcr_harness selftest <n> <n'>
//! bcr_harness pilot <out.jsonl> <n>...              # rho lane tuning, pilot curves
//! bcr_harness run <protocol.json> <out.jsonl> [threads]
//! bcr_harness analyze <protocol.json> <runs.jsonl> <summary.json> <figure.svg>
//! ```
#![allow(dead_code, unused_imports, clippy::all)]
mod analysis;
mod binary_ecc;
mod cryptanalysis;
mod field;
mod gen;
mod ic;
mod rho;
mod seeds;

use field::{meq, Curve, Ops, Pt, CF};
use ic::*;
use seeds::Sm64;
use serde_json::json;
use std::io::Write;
use std::sync::{Arc, Mutex};

pub struct Case {
    pub n: u32,
    pub np: u32,
    pub ci: u32,
    pub inst: gen::Instance,
    pub sub: Sub,
    pub q: Pt,
    pub d_true: u64,
    pub ti: u32,
}

pub fn make_curve(domain: &str, n: u32, ci: u32) -> gen::Instance {
    let mut rng = Sm64::keyed(&[domain, &n.to_string(), &ci.to_string()]);
    gen::random_instance(n, &mut rng)
}

/// The public target `Q = [d]G`; `d` stays with the evaluator.
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

// ───────────────────────── pairs-and-solve (arms F, F0, G) ──────────────────

pub fn run_ps(c: &Curve, g: Pt, q: Pt, sub: &Sub, stream: &[&str], g_max: u64, la_stream: &[&str], ap: bool, la_ech: bool) -> Vec<serde_json::Value> {
    let gf = &c.gf;
    let t0 = std::time::Instant::now();
    let (cf_sh, cf_pt, cf_fac, cf_gcd, cf_lv) = (CF::new(gf), CF::new(gf), CF::new(gf), CF::new(gf), CF::new(gf));
    let fb = build_fb(c, &cf_sh, sub);
    let o_fb = cf_sh.ops();
    let t_pt = std::time::Instant::now();
    let pt = pair_table(c, &cf_pt, &fb);
    let w_pt = t_pt.elapsed().as_nanos();
    let lv = subspace_poly(&cf_lv, &sub.basis);
    let mut rng = Sm64::keyed(stream);
    let mut tg = if ap { Targets::new_ap(c, &cf_sh, g, q, &mut rng) } else { Targets::new(c, &cf_sh, g, q, &mut rng) };
    let mut la_rng = Sm64::keyed(la_stream);
    let mut rows: Vec<Row> = vec![];
    let (mut o_targets, mut o_resolve, mut o_la) = (Ops::default(), Ops::default(), Ops::default());
    let mut la = LaStats::default();
    let mut ech = if la_ech { Some(Echelon::new(c.ell)) } else { None };
    let mut fed = 0usize;
    let (mut set_f, mut set_g) = (vec![], vec![]);
    let (mut w_fac, mut w_gcd) = (0u128, 0u128);
    let (mut g_checked, mut mismatches, mut unresolved, mut ntargets) = (0u64, 0u64, 0u64, 0u64);
    let mut next_try = 0usize;
    let mut d = None;
    let cap = 60 * fb.xs.len() as u64 + 200;
    while ntargets < cap {
        let s0 = cf_sh.ops();
        let (r, ra, rb) = tg.next(c, &cf_sh);
        o_targets = o_targets.add(&cf_sh.ops().sub(&s0));
        ntargets += 1;
        let Pt::A(xr, _) = r else { continue };
        if xr == 0 {
            continue;
        }
        let tt = std::time::Instant::now();
        solve_factored(c, &cf_fac, sub, &fb, &pt, xr, &mut set_f);
        w_fac += tt.elapsed().as_nanos();
        if g_checked < g_max {
            let tt = std::time::Instant::now();
            solve_gcd(c, &cf_gcd, &fb, &lv, xr, &mut set_g);
            w_gcd += tt.elapsed().as_nanos();
            g_checked += 1;
            if set_g != set_f {
                mismatches += 1;
            }
        }
        let s1 = cf_sh.ops();
        for t in &set_f {
            match resolve(c, &cf_sh, &fb, t, r, ra, rb) {
                Some(row) => rows.push(row),
                None => unresolved += 1,
            }
        }
        o_resolve = o_resolve.add(&cf_sh.ops().sub(&s1));
        if let Some(ech) = ech.as_mut() {
            let s2 = cf_sh.ops();
            let mut dd = None;
            while fed < rows.len() {
                dd = ech.add(&cf_sh, &rows[fed]);
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
            continue;
        }
        let nc = ncols_of(&rows);
        if rows.len() >= nc + 4 && rows.len() >= next_try {
            let s2 = cf_sh.ops();
            let dd = solve_log(&cf_sh, &rows, c.ell, &mut la_rng, &mut la);
            o_la = o_la.add(&cf_sh.ops().sub(&s2));
            if let Some(dd) = dd {
                d = Some(dd);
                break;
            }
            next_try = rows.len() + 4;
        }
    }
    let s3 = cf_sh.ops();
    let verified = d.map(|dd| c.mul(&cf_sh, dd, g) == q).unwrap_or(false);
    let o_ver = cf_sh.ops().sub(&s3);
    let wall_rest = t0.elapsed().as_nanos() - w_fac - w_gcd - w_pt;
    let shared: Vec<(&str, Ops)> = vec![("fb", o_fb), ("targets", o_targets), ("resolve", o_resolve), ("la", o_la), ("verify", o_ver)];
    let a_tab = cf_pt.ops();
    let fac = cf_fac.ops();
    let gcd_measured = cf_gcd.ops();
    // gcd on all targets: exact when g_checked covers every target with x_R ≠ 0,
    // else the per-target mean times the target count (flagged).
    let tgt_used = ntargets; // includes the (rare) skipped x_R = 0 / O draws
    let gcd_exact = g_checked >= tgt_used || g_max == u64::MAX;
    let scale = if g_checked == 0 { 0.0 } else { tgt_used as f64 / g_checked as f64 };
    let gcd_total = if gcd_exact {
        gcd_measured
    } else {
        let s = gcd_measured.scale(scale);
        Ops { m: s[0] as u64, s: s[1] as u64, i: s[2] as u64, l: s[3] as u64, h: s[4] as u64, z: s[5] as u64, za: s[6] as u64 }
    };
    let f0_stage_a = {
        let s = a_tab.scale(tgt_used as f64);
        Ops { m: s[0] as u64, s: s[1] as u64, i: s[2] as u64, l: s[3] as u64, h: s[4] as u64, z: s[5] as u64, za: s[6] as u64 }
    };
    let common = json!({
        "d": d, "verified": verified, "targets": tgt_used, "relations": rows.len(),
        "columns": ncols_of(&rows), "fb": fb.xs.len(), "rows_unresolved": unresolved,
        "la_calls": la.calls, "la_attempts": la.attempts, "la": if la_ech { "echelon" } else { "wiedemann" }, "echelon_rows_in": ech.as_ref().map(|e| e.rows_in), "echelon_useless_dependencies": ech.as_ref().map(|e| e.useless_dependencies), "pair_u": pt.nu,
        "gcd_targets_checked": g_checked, "gcd_set_mismatches": mismatches,
    });
    let mut outs = vec![];
    let mk = |arm: &str, extra: Vec<(&'static str, Ops)>, wall: u128, note: &str| {
        let mut ph = shared.clone();
        ph.extend(extra);
        let tot = total(&ph);
        let mut v = common.clone();
        v["arm"] = json!(arm);
        v["phases"] = phases_json(&ph);
        v["total"] = tot.json();
        v["cost"] = cost_json(&tot, c.n);
        v["wall_ns"] = json!(wall as u64);
        v["note"] = json!(note);
        v
    };
    outs.push(mk("F", vec![("pairtable", a_tab), ("rootfind", fac)], wall_rest + w_pt + w_fac, "factored S3*S3 route, pair table, batched inversions"));
    outs.push(mk(
        "F0",
        vec![("rootfind_stage_a", f0_stage_a), ("rootfind", fac)],
        wall_rest + w_pt * tgt_used as u128 + w_fac,
        "factored route without pair table: stage A per target (= pair-table ops x targets, exact)",
    ));
    let gwall = if g_checked == 0 { 0 } else { (w_gcd as f64 * scale) as u128 };
    outs.push(mk(
        "G",
        vec![("lv", cf_lv.ops()), ("rootfind", gcd_total)],
        wall_rest + gwall,
        if gcd_exact { "subspace gcd on every pair of every target (exact)" } else { "subspace gcd counted on the first gcd_targets_checked targets, scaled to all targets" },
    ));
    for o in outs.iter_mut() {
        o["gcd_exact"] = json!(gcd_exact);
    }
    outs
}

// ───────────────────────── meet in the middle (M3, M2) ──────────────────────

pub fn run_mitm(c: &Curve, g: Pt, q: Pt, sub: &Sub, m: u32, stream: &[&str], la_stream: &[&str], ap: bool, la_ech: bool) -> serde_json::Value {
    let gf = &c.gf;
    let t0 = std::time::Instant::now();
    let (cf_sh, cf_t, cf_p) = (CF::new(gf), CF::new(gf), CF::new(gf));
    let fb = build_fb(c, &cf_sh, sub);
    let o_fb = cf_sh.ops();
    let pt = pair_table(c, &cf_t, &fb);
    let uh = uhash(&cf_t, &pt);
    drop(pt);
    let mut rng = Sm64::keyed(stream);
    let mut tg = if ap { Targets::new_ap(c, &cf_sh, g, q, &mut rng) } else { Targets::new(c, &cf_sh, g, q, &mut rng) };
    let mut la_rng = Sm64::keyed(la_stream);
    let mut rows: Vec<Row> = vec![];
    let (mut o_targets, mut o_resolve, mut o_la) = (Ops::default(), Ops::default(), Ops::default());
    let mut la = LaStats::default();
    let mut ech = if la_ech { Some(Echelon::new(c.ell)) } else { None };
    let mut fed = 0usize;
    let mut s3 = vec![];
    let mut s2 = vec![];
    let (mut unresolved, mut ntargets) = (0u64, 0u64);
    let mut next_try = 0usize;
    let mut d = None;
    let f = fb.xs.len() as u64;
    let cap = if m == 3 { 60 * f + 200 } else { 60 * f * (1u64 << sub.basis.len()) + 2000 };
    while ntargets < cap {
        let a0 = cf_sh.ops();
        let (r, ra, rb) = tg.next(c, &cf_sh);
        o_targets = o_targets.add(&cf_sh.ops().sub(&a0));
        ntargets += 1;
        let Pt::A(xr, _) = r else { continue };
        if xr == 0 {
            continue;
        }
        let a1 = cf_sh.ops();
        if m == 3 {
            solve_m3(c, &cf_p, &fb, &uh, xr, &mut s3);
            for t in &s3 {
                match resolve(c, &cf_sh, &fb, t, r, ra, rb) {
                    Some(row) => rows.push(row),
                    None => unresolved += 1,
                }
            }
        } else {
            solve_m2(&cf_p, &fb, &uh, xr, &mut s2);
            for t in &s2 {
                match resolve(c, &cf_sh, &fb, t, r, ra, rb) {
                    Some(row) => rows.push(row),
                    None => unresolved += 1,
                }
            }
        }
        o_resolve = o_resolve.add(&cf_sh.ops().sub(&a1));
        if let Some(ech) = ech.as_mut() {
            let a2 = cf_sh.ops();
            let mut dd = None;
            while fed < rows.len() {
                dd = ech.add(&cf_sh, &rows[fed]);
                fed += 1;
                if dd.is_some() {
                    break;
                }
            }
            o_la = o_la.add(&cf_sh.ops().sub(&a2));
            if let Some(dd) = dd {
                d = Some(dd);
                break;
            }
            continue;
        }
        let nc = ncols_of_fast(&rows, f as usize);
        if rows.len() >= nc + 4 && rows.len() >= next_try {
            let a2 = cf_sh.ops();
            let dd = solve_log(&cf_sh, &rows, c.ell, &mut la_rng, &mut la);
            o_la = o_la.add(&cf_sh.ops().sub(&a2));
            if let Some(dd) = dd {
                d = Some(dd);
                break;
            }
            next_try = rows.len() + 4;
        }
    }
    let a3 = cf_sh.ops();
    let verified = d.map(|dd| c.mul(&cf_sh, dd, g) == q).unwrap_or(false);
    let o_ver = cf_sh.ops().sub(&a3);
    let ph: Vec<(&str, Ops)> = vec![
        ("fb", o_fb),
        ("table", cf_t.ops()),
        ("targets", o_targets),
        ("probe", cf_p.ops()),
        ("resolve", o_resolve),
        ("la", o_la),
        ("verify", o_ver),
    ];
    let tot = total(&ph);
    json!({
        "arm": if m == 3 { "M3" } else { "M2" },
        "d": d, "verified": verified, "targets": ntargets, "relations": rows.len(),
        "columns": ncols_of(&rows), "fb": fb.xs.len(), "rows_unresolved": unresolved,
        "la_calls": la.calls, "la_attempts": la.attempts, "la": if la_ech { "echelon" } else { "wiedemann" }, "echelon_rows_in": ech.as_ref().map(|e| e.rows_in), "echelon_useless_dependencies": ech.as_ref().map(|e| e.useless_dependencies), "table_keys": uh.map.len(),
        "phases": phases_json(&ph), "total": tot.json(), "cost": cost_json(&tot, c.n),
        "wall_ns": t0.elapsed().as_nanos() as u64,
        "note": if m == 3 { "m=3 MITM: u-table of pairs, probe x(R±P3) by one AS step" } else { "m=2 MITM: u-table of pairs, probe x(R)" },
    })
}

/// Distinct columns, by a bitmap (the stop rule runs after every target).
fn ncols_of_fast(rows: &[Row], f: usize) -> usize {
    thread_local! { static SEEN: std::cell::RefCell<(usize, Vec<bool>, usize, usize)> = Default::default(); }
    SEEN.with(|s| {
        let mut s = s.borrow_mut();
        let ptr = rows.as_ptr() as usize;
        if s.1.len() != f || rows.len() < s.2 || s.3 != ptr {
            *s = (0, vec![false; f], 0, ptr);
        }
        let mut cnt = s.0;
        for r in &rows[s.2..] {
            for c in &r.cols {
                if !s.1[c.0] {
                    s.1[c.0] = true;
                    cnt += 1;
                }
            }
        }
        s.0 = cnt;
        s.2 = rows.len();
        s.3 = rows.as_ptr() as usize;
        cnt
    })
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

fn curve_json(n: u32, np: u32, ci: u32, inst: &gen::Instance) -> serde_json::Value {
    let c = &inst.curve;
    json!({"n": n, "np": np, "ci": ci, "a": c.a, "b": format!("{:#x}", c.b), "irr_low_terms": c.irr_low,
           "order": c.order, "h": c.h, "ell": c.ell, "g": format!("{:?}", inst.g), "gen_attempts": inst.attempts})
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("selftest") => selftest(args[2].parse().unwrap(), args[3].parse().unwrap()),
        Some("pilot") => pilot(&args[2], args[3..].iter().map(|s| s.parse().unwrap()).collect()),
        Some("run") => run(&args[2], &args[3], args.get(4).map(|s| s.parse().unwrap()).unwrap_or(1)),
        Some("analyze") => analysis::analyze(&args[2], &args[3], &args[4], &args[5]),
        Some("dupcheck") => dupcheck(&args[2], &args[3]),
        Some("curves") => {
            for n in args[2..].iter().map(|s| s.parse::<u32>().unwrap()) {
                for ci in 0..4 {
                    let inst = make_curve("curve", n, ci);
                    println!("{}", curve_json(n, n / 3, ci, &inst));
                }
            }
        }
        _ => eprintln!("usage: see source header"),
    }
}

fn selftest(n: u32, np: u32) {
    use cryptanalysis::binary_radical_root::FactoredSubspaceOracle;
    use cryptanalysis::semaev_decomp::{quartic_in_x3_general, SubspaceOracle};
    let inst = make_curve("selftest-curve", n, 0);
    let c = &inst.curve;
    let cf = CF::new(&c.gf);
    println!("{}", curve_json(n, np, 0, &inst));
    assert!(c.on_curve(inst.g));
    assert_eq!(c.mul(&cf, c.ell, inst.g), Pt::Inf);
    // group law: associativity on random combos
    let mut rng = Sm64::keyed(&["selftest", &n.to_string()]);
    for _ in 0..50 {
        let (a, b, e) = (rng.below(c.ell), rng.below(c.ell), rng.below(c.ell));
        let (p, q, r) = (c.mul(&cf, a, inst.g), c.mul(&cf, b, inst.g), c.mul(&cf, e, inst.g));
        assert_eq!(c.add(&cf, c.add(&cf, p, q), r), c.add(&cf, p, c.add(&cf, q, r)));
        assert_eq!(c.add(&cf, p, q), c.mul(&cf, (a + b) % c.ell, inst.g));
        assert_eq!(c.joint(&cf, a, inst.g, b, q), c.mul(&cf, (a + (b as u128 * b as u128 % c.ell as u128) as u64) % c.ell, inst.g));
    }
    // quartic vs crypto
    for _ in 0..2000 {
        let (x1, x2, xr) = (rng.bits(n), rng.bits(n), rng.bits(n).max(1));
        let mine = quartic(&cf, c.b, x1, x2, xr, c.gf.sqr(xr), c.gf.sqr(c.gf.sqr(xr)));
        let theirs = quartic_in_x3_general(x1, x2, xr, c.b, &c.gf);
        let dm = (0..5).rev().find(|&k| mine[k] != 0);
        let dt = (0..7).rev().find(|&k| theirs.c[k] != 0);
        assert_eq!(dm, dt, "degree");
        if let Some(dd) = dm {
            for k in 0..5 {
                assert_eq!(c.gf.mul(mine[k], theirs.c[dd]), c.gf.mul(theirs.c[k], mine[dd]), "proportional");
            }
        }
    }
    println!("quartic: proportional to crypto quartic_in_x3_general on 2000 random inputs");
    let basis = gen::subfield_basis(&c.gf, np);
    let sub = Sub::new(&basis);
    for &v in &sub.span {
        assert_eq!(c.gf.sqr_k(v, np), v);
    }
    let fb = build_fb(c, &cf, &sub);
    for p in &fb.pts {
        assert!(c.on_curve(*p));
    }
    println!("subfield F_2^{np}: {} elements, factor base {} points", sub.span.len(), fb.xs.len());
    let pt = pair_table(c, &cf, &fb);
    let uh = uhash(&cf, &pt);
    let lv = subspace_poly(&cf, &basis);
    let cfac = FactoredSubspaceOracle::new(&basis, c.b, &c.gf).unwrap();
    let cgcd = SubspaceOracle::new(&basis, c.b, &c.gf);
    let (mut sf, mut sg, mut s3) = (vec![], vec![], vec![]);
    let (mut hits, mut tot) = (0, 0);
    let (q, _) = make_target("selftest-target", &inst, n, 0, 0);
    let mut tg = Targets::new(c, &cf, inst.g, q, &mut rng);
    for _ in 0..300 {
        let (r, ra, rb) = tg.next(c, &cf);
        assert_eq!(r, c.joint(&cf, ra, inst.g, rb, q));
        let Pt::A(xr, _) = r else { continue };
        solve_factored(c, &cf, &sub, &fb, &pt, xr, &mut sf);
        solve_gcd(c, &cf, &fb, &lv, xr, &mut sg);
        solve_m3(c, &cf, &fb, &uh, xr, &mut s3);
        assert_eq!(sf, sg, "factored vs gcd");
        assert_eq!(sf, s3, "factored vs m3");
        let (w1, _) = cfac.decompose(xr, &c.gf);
        let (w2, _) = cgcd.decompose(xr, &c.gf);
        assert_eq!(w1.is_some(), w2.is_some(), "crypto oracles agree");
        if !sf.is_empty() {
            assert!(w1.is_some(), "crypto factored oracle must find a witness");
        }
        if let Some(w) = w1 {
            if w.iter().all(|x| fb.idx.contains_key(x)) {
                let mut t = w;
                t.sort_unstable();
                assert!(sf.contains(&t), "crypto witness in our set");
            }
        }
        for t in &sf {
            assert!(resolve(c, &cf, &fb, t, r, ra, rb).is_some(), "resolve");
        }
        hits += sf.len();
        tot += 1;
    }
    println!("{tot} targets: factored == gcd == m3 sets, consistent with crypto oracles; {hits} relations");
    let (q, d) = make_target("selftest-target", &inst, n, 0, 1);
    let outs = run_ps(c, inst.g, q, &sub, &["selftest-ps", &n.to_string()], u64::MAX, &["selftest-la"], false, false);
    for o in &outs {
        println!("{} d={} verified={} targets={} rels={} meq={:.3e}", o["arm"], o["d"], o["verified"], o["targets"], o["relations"], o["cost"]["meq_s1_u1"].as_f64().unwrap());
    }
    assert_eq!(outs[0]["d"].as_u64(), Some(d));
    for m in [3, 2] {
        let o = run_mitm(c, inst.g, q, &sub, m, &["selftest-ps", &n.to_string()], &["selftest-la"], false, false);
        println!("{} d={} verified={} targets={} rels={} meq={:.3e}", o["arm"], o["d"], o["verified"], o["targets"], o["relations"], o["cost"]["meq_s1_u1"].as_f64().unwrap());
        assert_eq!(o["d"].as_u64(), Some(d));
    }
    // Amendments A1 + A2 on the same selftest instance, 6 targets.
    for ti in 1..7u32 {
        let (q, d) = make_target("selftest-target", &inst, n, 0, ti);
        let st = ti.to_string();
        let outs = run_ps(c, inst.g, q, &sub, &["selftest-ps-a1", &n.to_string(), &st], u64::MAX, &["selftest-la"], true, true);
        let mut line = format!("A1+A2 ti={ti}:");
        for o in &outs {
            assert_eq!(o["d"].as_u64(), Some(d), "A1+A2 {}", o["arm"]);
            line += &format!(" {}(t={},rels={},meq={:.2e})", o["arm"].as_str().unwrap(), o["targets"], o["relations"], o["cost"]["meq_s1_u1"].as_f64().unwrap());
        }
        for m in [3, 2] {
            let o = run_mitm(c, inst.g, q, &sub, m, &["selftest-ps-a1", &n.to_string(), &st], &["selftest-la"], true, true);
            assert_eq!(o["d"].as_u64(), Some(d), "A1+A2 M{m}");
            line += &format!(" {}(t={},rels={},meq={:.2e})", o["arm"].as_str().unwrap(), o["targets"], o["relations"], o["cost"]["meq_s1_u1"].as_f64().unwrap());
        }
        println!("{line}");
    }
    for w in [1usize, 4, 16] {
        let o = rho_record(c, inst.g, q, w, &["selftest-rho", &w.to_string()]);
        println!("RHO W={w} d={} verified={} steps={} meq={:.3e}", o["d"], o["verified"], o["steps"], o["cost"]["meq_s1_u1"].as_f64().unwrap());
        assert_eq!(o["d"].as_u64(), Some(d));
    }
    println!("selftest PASS");
}

fn pilot(out: &str, ns: Vec<u32>) {
    let mut f = std::fs::File::create(out).unwrap();
    for n in ns {
        for ci in 0..2 {
            let inst = make_curve("pilot-curve", n, ci);
            let c = &inst.curve;
            for w in [1usize, 2, 4, 8, 16, 32, 64] {
                for ti in 0..16 {
                    let (q, d) = make_target("pilot-target", &inst, n, ci, ti);
                    let mut o = rho_record(c, inst.g, q, w, &["pilot-rho", &n.to_string(), &ci.to_string(), &ti.to_string(), &w.to_string()]);
                    o["n"] = json!(n);
                    o["ci"] = json!(ci);
                    o["ti"] = json!(ti);
                    o["correct"] = json!(o["d"].as_u64() == Some(d));
                    writeln!(f, "{}", o).unwrap();
                }
            }
            eprintln!("pilot n={n} ci={ci} done");
        }
    }
}

fn run(protocol: &str, out: &str, threads: usize) {
    let p: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(protocol).unwrap()).unwrap();
    let rungs: Vec<serde_json::Value> = p["rungs"].as_array().unwrap().clone();
    let curves = p["curves_per_rung"].as_u64().unwrap() as u32;
    let targets = p["targets_per_curve"].as_u64().unwrap() as u32;
    let rho_seeds = p["rho_seeds_per_target"].as_u64().unwrap() as u32;
    let ap = p["target_walk"].as_str() == Some("ap");
    let ech = p["linear_algebra"].as_str() == Some("echelon");
    let arms: Vec<String> = p["ic_arms"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_string()).collect();
    let file = Arc::new(Mutex::new(std::fs::OpenOptions::new().create(true).append(true).open(out).unwrap()));
    let mut jobs: Vec<(u32, u32, usize, u32, u32, u64)> = vec![];
    for r in &rungs {
        let n = r["n"].as_u64().unwrap() as u32;
        let np = r["np"].as_u64().unwrap() as u32;
        let w = r["rho_lanes"].as_u64().unwrap() as usize;
        let gmax = r["gcd_targets"].as_u64().unwrap_or(u64::MAX);
        let curves = r["curves"].as_u64().map(|x| x as u32).unwrap_or(curves);
        let targets = r["targets"].as_u64().map(|x| x as u32).unwrap_or(targets);
        for ci in 0..curves {
            for ti in 0..targets {
                jobs.push((n, np, w, ci, ti, gmax));
            }
        }
    }
    // largest first, for balance
    jobs.sort_by_key(|j| std::cmp::Reverse(j.0));
    let jobs = Arc::new(Mutex::new(jobs));
    let mut hs = vec![];
    let arms = Arc::new(arms);
    for _ in 0..threads {
        let (jobs, file, arms) = (jobs.clone(), file.clone(), arms.clone());
        let ap = ap;
        let ech = ech;
        hs.push(std::thread::spawn(move || loop {
            let job = jobs.lock().unwrap().pop();
            let Some((n, np, w, ci, ti, gmax)) = job else { break };
            let inst = make_curve("curve", n, ci);
            let c = &inst.curve;
            let (q, d_true) = make_target("target", &inst, n, ci, ti);
            let basis = gen::subfield_basis(&c.gf, np);
            let sub = Sub::new(&basis);
            let (ns, cs, ts) = (n.to_string(), ci.to_string(), ti.to_string());
            let stream = [if ap { "icstream-a1" } else { "icstream" }, ns.as_str(), cs.as_str(), ts.as_str()];
            let case = json!({"n": n, "np": np, "ci": ci, "ti": ti, "curve": curve_json(n, np, ci, &inst)});
            let emit = |mut v: serde_json::Value| {
                v["case"] = case.clone();
                v["correct"] = json!(v["d"].as_u64() == Some(d_true));
                let mut f = file.lock().unwrap();
                writeln!(f, "{}", v).unwrap();
                f.flush().unwrap();
            };
            // counterbalanced order over (case index)
            let order: Vec<&str> = {
                let mut o = vec!["RHO", "PS", "M3", "M2"];
                let k = ((ci * 7 + ti) % 4) as usize;
                o.rotate_left(k);
                o
            };
            for a in order {
                match a {
                    "RHO" => {
                        for si in 0..rho_seeds {
                            let sis = si.to_string();
                            let mut v = rho_record(c, inst.g, q, w, &["rho", &ns, &cs, &ts, &sis]);
                            v["seed_index"] = json!(si);
                            emit(v);
                        }
                    }
                    "PS" if arms.iter().any(|x| x == "F") => {
                        for v in run_ps(c, inst.g, q, &sub, &stream, gmax, &["la", &ns, &cs, &ts, "PS"], ap, ech) {
                            emit(v);
                        }
                    }
                    "M3" if arms.iter().any(|x| x == "M3") => emit(run_mitm(c, inst.g, q, &sub, 3, &stream, &["la", &ns, &cs, &ts, "M3"], ap, ech)),
                    "M2" if arms.iter().any(|x| x == "M2") => emit(run_mitm(c, inst.g, q, &sub, 2, &stream, &["la", &ns, &cs, &ts, "M2"], ap, ech)),
                    _ => {}
                }
            }
            eprintln!("case n={n} ci={ci} ti={ti} done");
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
}

/// For every IC record of the frozen run, regenerate the frozen relation-target
/// stream up to the number of targets the arm consumed and count repeated
/// targets (the r-adding target walk can enter a cycle).
fn dupcheck(runs: &str, out: &str) {
    let text = std::fs::read_to_string(runs).unwrap();
    let mut f = std::fs::File::create(out).unwrap();
    let mut seen_case: std::collections::HashSet<(u64, u64, u64)> = Default::default();
    let mut maxt: std::collections::BTreeMap<(u64, u64, u64), Vec<(String, u64)>> = Default::default();
    for line in text.lines() {
        let v: serde_json::Value = serde_json::from_str(line).unwrap();
        let arm = v["arm"].as_str().unwrap();
        if arm == "RHO" || arm == "F0" || arm == "G" {
            continue;
        }
        let c = &v["case"];
        let key = (c["n"].as_u64().unwrap(), c["ci"].as_u64().unwrap(), c["ti"].as_u64().unwrap());
        seen_case.insert(key);
        maxt.entry(key).or_default().push((arm.to_string(), v["targets"].as_u64().unwrap()));
    }
    for ((n, ci, ti), arms) in maxt {
        let (n, ci, ti) = (n as u32, ci as u32, ti as u32);
        let inst = make_curve("curve", n, ci);
        let c = &inst.curve;
        let (q, _) = make_target("target", &inst, n, ci, ti);
        let cf = CF::new(&c.gf);
        let mut rng = Sm64::keyed(&["icstream", &n.to_string(), &ci.to_string(), &ti.to_string()]);
        let mut tg = Targets::new(c, &cf, inst.g, q, &mut rng);
        let tmax = arms.iter().map(|a| a.1).max().unwrap();
        let mut seen: std::collections::HashMap<u64, u64> = Default::default();
        let mut first_rep: Option<u64> = None;
        let mut reps_at: Vec<(u64, u64)> = arms.iter().map(|a| (a.1, 0)).collect();
        let mut reps = 0u64;
        for k in 0..tmax {
            let (r, _, _) = tg.next(c, &cf);
            let x = r.x();
            if seen.insert(x, k).is_some() {
                reps += 1;
                if first_rep.is_none() {
                    first_rep = Some(k);
                }
            }
            for e in reps_at.iter_mut() {
                if k + 1 == e.0 {
                    e.1 = reps;
                }
            }
        }
        for (i, (arm, t)) in arms.iter().enumerate() {
            writeln!(f, "{}", json!({"n": n, "ci": ci, "ti": ti, "arm": arm, "targets": t, "repeated_targets": reps_at[i].1, "first_repeat_index": first_rep})).unwrap();
        }
    }
}
