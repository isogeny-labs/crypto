//! bwd_harness — arm D, the Gaudry–Diem k = 3 Weil-descent decomposition
//! oracle, on the #5253 binary composite IC harness.  Study `bwd-20261126/v1`.
//!
//! The field, curve, factor base, relation-target stream (A1), sign
//! resolution, echelon linear algebra (A2), rho and seed modules are the
//! #5253 files, included unchanged by `#[path]`, so curves, targets `Q`,
//! target streams and the counted unit are bitwise those of #5253.
//!
//! ```text
//! bwd_harness selftest <n> <n'>
//! bwd_harness run <protocol.json> <out.jsonl> [threads]
//! bwd_harness analyze <protocol.json> <runs.jsonl> <summary.json> <figure.svg>
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

mod analysis;
mod descent;

use descent::Descent;
use field::{meq, Curve, Ops, Pt, CF};
use ic::*;
use seeds::Sm64;
use serde_json::json;
use std::io::Write;
use std::sync::{Arc, Mutex};

// ── verbatim from the #5253 main.rs (same curves and targets) ──
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

/// Exhaustive group-law decomposition set (verification only, uncounted):
/// every multiset `{P_i, P_j, P_k}` of base classes with
/// `±P_i ± P_j ± P_k = ±R`, as sorted abscissae.
pub fn brute(c: &Curve, cf: &CF, fb: &Fb, r: Pt, out: &mut Vec<[u64; 3]>) {
    out.clear();
    let nr = c.neg(r);
    let f = fb.pts.len();
    for i in 0..f {
        for j in i..f {
            let s = [c.add(cf, fb.pts[i], fb.pts[j]), c.add(cf, fb.pts[i], c.neg(fb.pts[j]))];
            for k in j..f {
                let pk = fb.pts[k];
                let npk = c.neg(pk);
                let mut hit = false;
                for &u in &s {
                    for v in [pk, npk] {
                        let w = c.add(cf, u, v);
                        if w == r || w == nr {
                            hit = true;
                        }
                    }
                }
                if hit {
                    let mut t = [fb.xs[i], fb.xs[j], fb.xs[k]];
                    t.sort_unstable();
                    out.push(t);
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
}

/// One cold single-target solve with arm D on the A1 stream and the A2
/// echelon LA (the same stream and LA as #5253's F/M2 records).
/// `fcheck` targets are cross-checked against #5253's factored oracle F,
/// `bcheck` against the exhaustive group-law set (both uncounted).
pub fn run_d(c: &Curve, g: Pt, q: Pt, sub: &Sub, np: u32, stream: &[&str], fcheck: u64, bcheck: u64) -> serde_json::Value {
    let gf = &c.gf;
    let t0 = std::time::Instant::now();
    let (cf_sh, cf_set, cf_red, cf_rf, cf_lift) = (CF::new(gf), CF::new(gf), CF::new(gf), CF::new(gf), CF::new(gf));
    let (cf_chk, cf_bf) = (CF::new(gf), CF::new(gf));
    let fb = build_fb(c, &cf_sh, sub);
    let o_fb = cf_sh.ops();
    let mut dsc = Descent::new(c, &cf_set, sub, np);
    let pt = if fcheck > 0 { Some(pair_table(c, &cf_chk, &fb)) } else { None };
    let mut rng = Sm64::keyed(stream);
    let mut tg = Targets::new_ap(c, &cf_sh, g, q, &mut rng);
    let mut rows: Vec<Row> = vec![];
    let (mut o_targets, mut o_resolve, mut o_la) = (Ops::default(), Ops::default(), Ops::default());
    let mut ech = Echelon::new(c.ell);
    let mut fed = 0usize;
    let (mut set_d, mut set_f, mut set_b) = (vec![], vec![], vec![]);
    let (mut f_checked, mut f_mis, mut b_checked, mut b_mis) = (0u64, 0u64, 0u64, 0u64);
    let (mut unresolved, mut ntargets) = (0u64, 0u64);
    let mut w_chk = 0u128;
    let mut d = None;
    let cap = 60 * fb.xs.len() as u64 + 200;
    let mut mism_examples = vec![];
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
        let tc = std::time::Instant::now();
        if f_checked < fcheck {
            solve_factored(c, &cf_chk, sub, &fb, pt.as_ref().unwrap(), xr, &mut set_f);
            f_checked += 1;
            if set_f != set_d {
                f_mis += 1;
                if mism_examples.len() < 4 {
                    mism_examples.push(json!({"kind": "F", "target": ntargets, "xr": xr, "D": set_d, "F": set_f}));
                }
            }
        }
        if b_checked < bcheck {
            brute(c, &cf_bf, &fb, r, &mut set_b);
            b_checked += 1;
            if set_b != set_d {
                b_mis += 1;
                if mism_examples.len() < 8 {
                    mism_examples.push(json!({"kind": "brute", "target": ntargets, "xr": xr, "D": set_d, "B": set_b}));
                }
            }
        }
        w_chk += tc.elapsed().as_nanos();
        let s1 = cf_sh.ops();
        for t in &set_d {
            match resolve(c, &cf_sh, &fb, t, r, ra, rb) {
                Some(row) => rows.push(row),
                None => unresolved += 1,
            }
        }
        o_resolve = o_resolve.add(&cf_sh.ops().sub(&s1));
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
        ("la", o_la),
        ("verify", o_ver),
    ];
    let tot = total(&ph);
    let st = dsc.st;
    let wall = t0.elapsed().as_nanos() - w_chk;
    json!({
        "arm": "D", "d": d, "verified": verified, "targets": ntargets, "relations": rows.len(),
        "columns": ncols_of(&rows), "fb": fb.xs.len(), "rows_unresolved": unresolved,
        "la": "echelon", "echelon_rows_in": ech.rows_in, "echelon_useless_dependencies": ech.useless_dependencies,
        "phases": phases_json(&ph), "total": tot.json(), "cost": cost_json(&tot, c.n),
        "wall_ns": wall as u64, "check_wall_ns": w_chk as u64,
        "dstats": {
            "targets": st.targets, "deg_u_mean": st.deg_u_sum as f64 / st.targets.max(1) as f64, "deg_u_max": st.deg_u_max,
            "kroots_u": st.kroots_u, "solves": st.solves, "ksolutions": st.ksolutions, "nonsplit": st.nonsplit,
            "split_not_fb": st.split_not_fb, "decompositions": st.decompositions,
            "fallback_rank": st.fallback_rank, "fallback_uzero": st.fallback_uzero,
            "fallback_quartic_zero": st.fallback_quartic_zero, "degenerate_r2": st.degenerate_r2, "tau0_mult": st.tau0_mult,
        },
        "fcheck_targets": f_checked, "fcheck_mismatches": f_mis,
        "brute_targets": b_checked, "brute_mismatches": b_mis, "mismatch_examples": mism_examples,
        "note": "Gaudry-Diem k=3 Weil descent of symmetric S4 to F_{2^n'}, elimination to U(t) (deg<=20), subfield root finding by t^{q'} mod U and gcd, exact lift; A1 stream, A2 echelon",
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("selftest") => selftest(args[2].parse().unwrap(), args[3].parse().unwrap()),
        Some("run") => run(&args[2], &args[3], args.get(4).map(|s| s.parse().unwrap()).unwrap_or(1)),
        Some("analyze") => analysis::analyze(&args[2], &args[3], &args[4], &args[5]),
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
    let inst = make_curve("bwd-selftest-curve", n, 0);
    let c = &inst.curve;
    let cf = CF::new(&c.gf);
    println!("{}", curve_json(n, np, 0, &inst));
    let mut rng = Sm64::keyed(&["bwd-selftest", &n.to_string()]);
    let basis = gen::subfield_basis(&c.gf, np);
    let sub = Sub::new(&basis);
    let fb = build_fb(c, &cf, &sub);
    let dsc0 = Descent::new(c, &cf, &sub, np);
    // 1. symmetric S4 equals the #5253 resultant quartic (crypto-checked).
    for _ in 0..2000 {
        let (x1, x2, x3, xr) = (rng.bits(n), rng.bits(n), rng.bits(n), rng.bits(n).max(1));
        let q = quartic(&cf, c.b, x1, x2, xr, c.gf.sqr(xr), c.gf.sqr(c.gf.sqr(xr)));
        let mut v = 0;
        for &k in q.iter().rev() {
            v = c.gf.mul(v, x3) ^ k;
        }
        let e1 = x1 ^ x2 ^ x3;
        let e2 = c.gf.mul(x1, x2) ^ c.gf.mul(x1 ^ x2, x3);
        let e3 = c.gf.mul(c.gf.mul(x1, x2), x3);
        let s = dsc0.f_at(&cf, 0, xr, c.gf.sqr(e1), c.gf.sqr(e2), e3);
        assert_eq!(s, v, "symmetric S4 == resultant quartic evaluated");
    }
    println!("symmetric S4(E1,E2,t) == #5253 quartic (== crypto quartic_in_x3_general up to scale) on 2000 random inputs");
    // 2. reduction: U vanishes at every genuine solution; t | U; degree.
    let pt = pair_table(c, &cf, &fb);
    let (q, _) = make_target("bwd-selftest-target", &inst, n, 0, 0);
    let mut tg = Targets::new_ap(c, &cf, inst.g, q, &mut rng);
    let mut dsc = Descent::new(c, &cf, &sub, np);
    let (mut sd, mut sf, mut sb) = (vec![], vec![], vec![]);
    let (mut hits, mut tot, mut udeg) = (0usize, 0usize, std::collections::BTreeMap::<usize, usize>::new());
    let mut tval = 0usize;
    let nb = if n <= 21 { 400 } else if n <= 24 { 60 } else { 0 };
    for k in 0..400 {
        let (r, ra, rb) = tg.next(c, &cf);
        let Pt::A(xr, _) = r else { continue };
        let red = dsc.reduce(&cf, xr);
        let mut u = red.u.clone();
        while u.last() == Some(&0) {
            u.pop();
        }
        *udeg.entry(u.len().saturating_sub(1)).or_default() += 1;
        if !u.is_empty() && u[0] == 0 {
            tval += 1;
        }
        dsc.decompose(&cf, &cf, &cf, &fb, xr, &mut sd);
        dsc.frob_gcd = true;
        let mut sd2 = vec![];
        dsc.decompose(&cf, &cf, &cf, &fb, xr, &mut sd2);
        dsc.frob_gcd = false;
        assert_eq!(sd, sd2, "frobenius-gcd route == direct gcd(U, t^q' + t) route");
        solve_factored(c, &cf, &sub, &fb, &pt, xr, &mut sf);
        assert_eq!(sd, sf, "D == F set at target {k} (xr={xr:#x})");
        for t in &sd {
            let e1 = t[0] ^ t[1] ^ t[2];
            let e2 = c.gf.mul(t[0], t[1]) ^ c.gf.mul(t[0] ^ t[1], t[2]);
            let e3 = c.gf.mul(c.gf.mul(t[0], t[1]), t[2]);
            assert_eq!(descent::peval(&cf, &u, e3), 0, "U(e3) = 0");
            let _ = e1;
            let _ = e2;
            assert!(resolve(c, &cf, &fb, t, r, ra, rb).is_some(), "group-law replay");
        }
        if k < nb {
            brute(c, &cf, &fb, r, &mut sb);
            assert_eq!(sd, sb, "D == brute force at target {k}");
        }
        hits += sd.len();
        tot += 1;
    }
    println!("{tot} targets: D == F sets (and == exhaustive group-law sets on the first {nb}); {hits} decompositions; deg U histogram {:?}; U(0)=0 on {tval}", udeg);
    println!("dstats (both routes, so counts doubled) {:?}", dsc.st);
    // 3. full cold solves on selftest instances.
    for ti in 1..7u32 {
        let (q, dtrue) = make_target("bwd-selftest-target", &inst, n, 0, ti);
        let o = run_d(c, inst.g, q, &sub, np, &["bwd-selftest-stream", &n.to_string(), &ti.to_string()], 64, if n <= 21 { 64 } else { 0 });
        println!(
            "D ti={ti} d_ok={} verified={} targets={} rels={} meq={:.3e} fmis={} bmis={}",
            o["d"].as_u64() == Some(dtrue),
            o["verified"],
            o["targets"],
            o["relations"],
            o["cost"]["meq_s1_u1"].as_f64().unwrap(),
            o["fcheck_mismatches"],
            o["brute_mismatches"]
        );
        assert_eq!(o["d"].as_u64(), Some(dtrue));
        assert_eq!(o["fcheck_mismatches"].as_u64(), Some(0));
        assert_eq!(o["brute_mismatches"].as_u64(), Some(0));
    }
    println!("selftest PASS");
}

fn run(protocol: &str, out: &str, threads: usize) {
    let p: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(protocol).unwrap()).unwrap();
    let rungs: Vec<serde_json::Value> = p["rungs"].as_array().unwrap().clone();
    let curves = p["curves_per_rung"].as_u64().unwrap() as u32;
    let targets = p["targets_per_curve"].as_u64().unwrap() as u32;
    let file = Arc::new(Mutex::new(std::fs::OpenOptions::new().create(true).append(true).open(out).unwrap()));
    let mut jobs: Vec<(u32, u32, u32, u32, u64, u64)> = vec![];
    for r in &rungs {
        let n = r["n"].as_u64().unwrap() as u32;
        let np = r["np"].as_u64().unwrap() as u32;
        let fc = r["fcheck_targets"].as_u64().unwrap_or(0);
        let bc = r["brute_targets"].as_u64().unwrap_or(0);
        let curves = r["curves"].as_u64().map(|x| x as u32).unwrap_or(curves);
        let targets = r["targets"].as_u64().map(|x| x as u32).unwrap_or(targets);
        for ci in 0..curves {
            for ti in 0..targets {
                jobs.push((n, np, ci, ti, fc, bc));
            }
        }
    }
    jobs.sort_by_key(|j| std::cmp::Reverse((j.0, j.2, j.3)));
    let jobs = Arc::new(Mutex::new(jobs));
    let mut hs = vec![];
    for _ in 0..threads {
        let (jobs, file) = (jobs.clone(), file.clone());
        hs.push(std::thread::spawn(move || loop {
            let job = jobs.lock().unwrap().pop();
            let Some((n, np, ci, ti, fc, bc)) = job else { break };
            let inst = make_curve("curve", n, ci);
            let c = &inst.curve;
            let (q, d_true) = make_target("target", &inst, n, ci, ti);
            let basis = gen::subfield_basis(&c.gf, np);
            let sub = Sub::new(&basis);
            let (ns, cs, ts) = (n.to_string(), ci.to_string(), ti.to_string());
            let stream = ["icstream-a1", ns.as_str(), cs.as_str(), ts.as_str()];
            let mut v = run_d(c, inst.g, q, &sub, np, &stream, fc, bc);
            v["case"] = json!({"n": n, "np": np, "ci": ci, "ti": ti, "curve": curve_json(n, np, ci, &inst)});
            v["correct"] = json!(v["d"].as_u64() == Some(d_true));
            let mut f = file.lock().unwrap();
            writeln!(f, "{}", v).unwrap();
            f.flush().unwrap();
            eprintln!("case n={n} ci={ci} ti={ti} D done correct={} meq={:.3e}", v["correct"], v["cost"]["meq_s1_u1"].as_f64().unwrap());
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
}
