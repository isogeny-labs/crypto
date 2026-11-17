//! Odd-n trace accounting (PROTOCOL.md): exact relation counts N(R) on
//! E: y^2 + xy = x^3 + a x^2 + b over GF(2^N), N = d n, for the standard
//! base x in F_q and for lines u F_q, split by 2E membership of the target.
//!
//! Modes:
//!   selftest                     field, curve and counter checks (no study data)
//!   run <n> <d> <out_dir>        one cell: curves, arms, targets, exact counts
//!   score <run_dir>              registered verdicts P1-P8 and the interpretation rule
//!   validate <run_dir> <claimed.json> <file=sha256>...   pcc validator report
#![allow(clippy::too_many_arguments)]

mod count;
mod ec;
mod gf;
#[path = "../../c1_twisted_line_ladder_20261027/src/json.rs"]
mod json;
mod rng;
mod score;

use count::{brute, Counter};
use ec::{Curve, Pt, O};
use gf::Gf;
use rng::{Rng, SEED};
use std::fmt::Write as _;

pub const ARMS: [&str; 6] = ["std", "tz", "tw", "twh", "hz", "mix"];
pub const T_SAMPLE: usize = 2000;
pub const CENSUS_MAX_N: u32 = 15;
pub const BRUTE_MAX: f64 = 3.0e7;

fn threads() -> usize {
    std::env::var("THREADS").ok().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
        std::thread::available_parallelism().map(|x| x.get()).unwrap_or(4)
    })
}

/// u / b^(1/4) in F_q, i.e. the line u F_q is stable under P -> P + (0, sqrt b)
fn t_stable(f: &Gf, d: u32, u: u32, b4: u32) -> bool {
    let z = f.mul(u, f.inv(b4));
    f.frob(z, d) == z
}

fn in_fq(f: &Gf, d: u32, z: u32) -> bool {
    f.frob(z, d) == z
}

struct Lines {
    tw: u32,
    hz: u32,
    mix: u32,
}

fn find_lines(f: &Gf, d: u32, b: u32, tag: &str) -> Option<Lines> {
    let sb = f.sqrt(b);
    let b4 = f.sqrt(sb);
    let mut rng = Rng::new(SEED, tag);
    let (mut tw, mut hz, mut mix) = (None, None, None);
    for _ in 0..(1u64 << 22) {
        let u = (rng.next() as u32) & f.mask;
        if u == 0 || in_fq(f, d, u) || t_stable(f, d, u, b4) {
            continue;
        }
        let t1 = f.tr_rel(u, d);
        let t2 = f.tr_rel(f.mul(sb, f.inv(u)), d);
        if t1 == 0 && t2 == 0 && tw.is_none() {
            tw = Some(u);
        } else if t1 == 0 && t2 != 0 && hz.is_none() {
            hz = Some(u);
        } else if t1 != 0 && t2 != 0 && mix.is_none() {
            mix = Some(u);
        }
        if tw.is_some() && hz.is_some() && mix.is_some() {
            return Some(Lines { tw: tw.unwrap(), hz: hz.unwrap(), mix: mix.unwrap() });
        }
    }
    None
}

/// factor base of the line u F_q^* (both signs), optionally filtered on t
fn line_fb(cv: &Curve, fq: &[u32], u: u32, keep: impl Fn(u32, u32) -> bool) -> Vec<Pt> {
    let f = cv.f;
    let mut v = vec![];
    for &t in &fq[1..] {
        let x = f.mul(u, t);
        if !keep(t, x) {
            continue;
        }
        if let Some(p) = cv.lift(x) {
            v.push(p);
            v.push(cv.neg(&p));
        }
    }
    v
}

fn tr_d(f: &Gf, d: u32, t: u32) -> u32 {
    let (mut s, mut w) = (0u32, t);
    for _ in 0..d {
        s ^= w;
        w = f.sq(w);
    }
    assert!(s <= 1);
    s
}

fn all_points(cv: &Curve) -> Vec<Pt> {
    let f = cv.f;
    let mut v = vec![O, cv.lift(0).unwrap()];
    for x in 1..=f.mask {
        if let Some(p) = cv.lift(x) {
            v.push(p);
            v.push(cv.neg(&p));
        }
    }
    v
}

fn random_point(cv: &Curve, rng: &mut Rng) -> Pt {
    let f = cv.f;
    loop {
        let r = rng.next();
        let x = (r as u32) & f.mask;
        let s = (r >> 40) & 1 == 1;
        if x == 0 {
            if !s {
                return cv.lift(0).unwrap();
            }
            continue;
        }
        if let Some(p) = cv.lift(x) {
            return if s { cv.neg(&p) } else { p };
        }
    }
}

fn par_count(cv: &Curve, ctr: &Counter, pts: &[Pt]) -> Vec<u64> {
    let mut out = vec![0u64; pts.len()];
    if pts.is_empty() {
        return out;
    }
    let th = threads();
    let chunk = pts.len().div_ceil(th);
    std::thread::scope(|s| {
        for (tc, cc) in pts.chunks(chunk).zip(out.chunks_mut(chunk)) {
            s.spawn(move || {
                for (t, c) in tc.iter().zip(cc.iter_mut()) {
                    *c = ctr.count(cv, t);
                }
            });
        }
    });
    out
}

fn ipow(x: i128, n: u32) -> i128 {
    (0..n).fold(1i128, |a, _| a * x)
}

fn mode_run(n: u32, d: u32, out_dir: &str) {
    let big_n = n * d;
    let f = Gf::new(big_n);
    let fq = f.subfield(d);
    let q = 1u64 << d;
    let census = big_n <= CENSUS_MAX_N;
    let path = format!("{out_dir}/cell_n{n}_d{d}.jsonl");
    let mut out = String::new();
    for id in ["B1", "B2", "B3", "NEG"] {
        let t0 = std::time::Instant::now();
        // curve: b of full degree N; a = 0 (B1), Tr(a) = 0 (B2, B3), Tr(a) = 1 (NEG);
        // redraw until a valid tw line, an hz line and a mix line exist
        let mut attempt = 0;
        let (a, b, lines) = loop {
            let mut rng = Rng::new(SEED, &format!("curve-n{n}-d{d}-{id}-{attempt}"));
            let want_tr = if id == "NEG" { 1 } else { 0 };
            let a = if id == "B1" {
                0
            } else {
                loop {
                    let a = (rng.next() as u32) & f.mask;
                    if a != 0 && f.tr(a) == want_tr {
                        break a;
                    }
                }
            };
            let b = loop {
                let b = (rng.next() as u32) & f.mask;
                if b != 0 && f.deg(b) == big_n {
                    break b;
                }
            };
            if let Some(l) = find_lines(&f, d, b, &format!("lines-n{n}-d{d}-{id}-{attempt}")) {
                break (a, b, l);
            }
            attempt += 1;
        };
        let cv = Curve { f: &f, a, b };
        let tr_a = f.tr(a);
        let ne = cv.order();
        let v = ne.trailing_zeros();
        let h = ne >> v;
        {
            let mut rng = Rng::new(SEED, &format!("order-check-n{n}-d{d}-{id}"));
            for _ in 0..16 {
                let p = random_point(&cv, &mut rng);
                assert!(cv.on(&p));
                assert_eq!(cv.mul(ne, &p), O, "order check");
            }
        }
        // arms
        let fbs: Vec<(&str, u32, Vec<Pt>)> = vec![
            ("std", 1, line_fb(&cv, &fq, 1, |_, _| true)),
            ("tz", 1, line_fb(&cv, &fq, 1, |_, x| f.tr(x) == tr_a)),
            ("tw", lines.tw, line_fb(&cv, &fq, lines.tw, |_, _| true)),
            ("twh", lines.tw, line_fb(&cv, &fq, lines.tw, |t, _| tr_d(&f, d, t) == 0)),
            ("hz", lines.hz, line_fb(&cv, &fq, lines.hz, |_, _| true)),
            ("mix", lines.mix, line_fb(&cv, &fq, lines.mix, |_, _| true)),
        ];
        for (name, _, fb) in &fbs {
            assert_eq!(*name, ARMS[fbs.iter().position(|z| z.0 == *name).unwrap()]);
            for p in fb {
                assert!(cv.on(p));
            }
        }
        // targets
        let mut classes: Vec<(&str, Vec<Pt>)> = vec![];
        let mut crit_mismatch = 0u64;
        let mut odd_mismatch = 0u64;
        let half = ne / 2;
        if census {
            let pts = all_points(&cv);
            assert_eq!(pts.len() as u64, ne);
            let (mut e2, mut off, mut odd) = (vec![], vec![], vec![]);
            for p in pts {
                let crit = cv.in_2e(&p);
                let scal = cv.mul(half, &p) == O;
                if crit != scal {
                    crit_mismatch += 1;
                }
                if crit { e2.push(p) } else { off.push(p) }
                if cv.mul(h, &p) == O {
                    odd.push(p);
                }
            }
            assert_eq!(odd.len() as u64, h);
            classes.push(("2E", e2));
            classes.push(("off", off));
            classes.push(("odd", odd));
        } else {
            let mut rng = Rng::new(SEED, &format!("targets-coset-n{n}-d{d}-{id}"));
            let (mut e2, mut off) = (vec![], vec![]);
            while e2.len() < T_SAMPLE || off.len() < T_SAMPLE {
                let p = random_point(&cv, &mut rng);
                let crit = cv.in_2e(&p);
                if crit != (cv.mul(half, &p) == O) {
                    crit_mismatch += 1;
                }
                if crit {
                    if e2.len() < T_SAMPLE {
                        e2.push(p)
                    }
                } else if off.len() < T_SAMPLE {
                    off.push(p)
                }
            }
            let mut rng = Rng::new(SEED, &format!("targets-odd-n{n}-d{d}-{id}"));
            let mut odd = vec![];
            while odd.len() < T_SAMPLE {
                let x = random_point(&cv, &mut rng);
                let r = cv.mul(1u64 << v, &x);
                if !cv.in_2e(&r) || cv.mul(half, &r) != O {
                    crit_mismatch += 1;
                }
                if cv.mul(h, &r) != O {
                    odd_mismatch += 1;
                }
                odd.push(r);
            }
            classes.push(("2E", e2));
            classes.push(("off", off));
            classes.push(("odd", odd));
        }
        // counts
        let mut arm_json = vec![];
        let mut counts: Vec<Vec<Vec<u64>>> = vec![]; // [arm][class][target]
        for (name, u, fb) in &fbs {
            let m = fb.len() as u64;
            let m0 = fb.iter().filter(|p| cv.in_2e(p)).count() as u64;
            let mut xs: Vec<u32> = fb.iter().map(|p| p.x).collect();
            xs.sort();
            xs.dedup();
            let c = xs.len() as u64;
            let ctr = Counter::build(&cv, fb, n as usize);
            let per: Vec<Vec<u64>> = classes.iter().map(|(_, ts)| par_count(&cv, &ctr, ts)).collect();
            let do_brute = m > 0 && (m as f64).powi(n as i32) <= BRUTE_MAX;
            let mut brute_mismatch = 0u64;
            let mut brute_checked = 0u64;
            if do_brute {
                let bm = brute(&cv, fb, n as usize);
                for ((_, ts), cs) in classes.iter().zip(&per) {
                    for (t, &cnt) in ts.iter().zip(cs) {
                        brute_checked += 1;
                        if bm.get(&t.key()).copied().unwrap_or(0) != cnt {
                            brute_mismatch += 1;
                        }
                    }
                }
            }
            let (sp, sm) = ((m0 + (m - m0)) as i128, m0 as i128 - (m - m0) as i128);
            let exact = if census {
                let s2: u128 = per[0].iter().map(|&x| x as u128).sum();
                let so: u128 = per[1].iter().map(|&x| x as u128).sum();
                let pred2 = (ipow(sp, n) + ipow(sm, n)) / 2;
                let predo = (ipow(sp, n) - ipow(sm, n)) / 2;
                format!(
                    ",\"sum_2E\":{s2},\"sum_off\":{so},\"identity_2E\":{},\"identity_off\":{},\"identity_all\":{}",
                    s2 as i128 == pred2,
                    so as i128 == predo,
                    (s2 + so) as i128 == ipow(m as i128, n)
                )
            } else {
                String::new()
            };
            arm_json.push(format!(
                "{{\"arm\":\"{name}\",\"u\":{u},\"m\":{m},\"m0\":{m0},\"m1\":{},\"c\":{c},\"brute\":{do_brute},\"brute_checked\":{brute_checked},\"brute_mismatch\":{brute_mismatch}{exact}}}",
                m - m0
            ));
            counts.push(per);
        }
        let secs = t0.elapsed().as_secs_f64();
        writeln!(
            out,
            "{{\"type\":\"curve\",\"n\":{n},\"d\":{d},\"N\":{big_n},\"q\":{q},\"id\":\"{id}\",\"attempt\":{attempt},\"poly\":{},\"a\":{a},\"b\":{b},\"tr_a\":{tr_a},\"nE\":{ne},\"v\":{v},\"h\":{h},\"mode\":\"{}\",\"T\":[{}],\"crit_mismatch\":{crit_mismatch},\"odd_mismatch\":{odd_mismatch},\"seconds\":{secs:.2},\"arms\":[{}]}}",
            f.poly,
            if census { "census" } else { "sample" },
            classes.iter().map(|c| c.1.len().to_string()).collect::<Vec<_>>().join(","),
            arm_json.join(",")
        )
        .unwrap();
        for (ci, (cname, _)) in classes.iter().enumerate() {
            let parts: Vec<String> = fbs
                .iter()
                .enumerate()
                .map(|(ai, (name, _, _))| {
                    let v: Vec<String> = counts[ai][ci].iter().map(|x| x.to_string()).collect();
                    format!("\"{name}\":[{}]", v.join(","))
                })
                .collect();
            writeln!(out, "{{\"type\":\"targets\",\"n\":{n},\"d\":{d},\"id\":\"{id}\",\"class\":\"{cname}\",\"counts\":{{{}}}}}", parts.join(",")).unwrap();
        }
        eprintln!("n={n} d={d} {id}: nE={ne} v={v} {secs:.1}s");
    }
    std::fs::write(&path, out).unwrap();
}

fn mode_selftest() {
    let mut fails = 0;
    let mut check = |ok: bool, what: &str| {
        if !ok {
            fails += 1;
            eprintln!("SELFTEST FAIL: {what}");
        }
    };
    for big_n in [9u32, 12, 15, 16, 20, 25] {
        let f = Gf::new(big_n);
        let mut rng = Rng::new(SEED, &format!("selftest-field-{big_n}"));
        for _ in 0..2000 {
            let (a, b, c) = ((rng.next() as u32) & f.mask, (rng.next() as u32) & f.mask, (rng.next() as u32) & f.mask);
            check(f.mul(f.mul(a, b), c) == f.mul(a, f.mul(b, c)), "assoc");
            check(f.mul(a, b ^ c) == f.mul(a, b) ^ f.mul(a, c), "distrib");
            if a != 0 {
                check(f.mul(a, f.inv(a)) == 1, "inverse");
            }
            check(f.sq(f.sqrt(a)) == a, "sqrt");
            check(f.tr(a) == f.tr(f.sq(a)), "trace frobenius");
            match f.solve_as(a) {
                Some(z) => check(f.sq(z) ^ z == a && f.tr(a) == 0, "AS root"),
                None => check(f.tr(a) == 1, "AS solvable iff Tr = 0"),
            }
        }
        for d in [3u32, 4, 5] {
            if big_n % d != 0 || big_n / d < 3 || big_n / d > 5 {
                continue;
            }
            let fq = f.subfield(d);
            check(fq.len() == 1 << d, "subfield size");
            for &t in &fq {
                check(f.frob(t, d) == t, "subfield fixed by q-Frobenius");
            }
            let n = big_n / d;
            // Tr_N(t) = n Tr_d(t) for t in F_q
            for &t in &fq {
                check(f.tr(t) == (n & 1) * tr_d(&f, d, t), "Tr_N = n Tr_d on F_q");
            }
            // Tr_N(u t) = Tr_d(t Tr_{N/d}(u))
            for _ in 0..200 {
                let u = (rng.next() as u32) & f.mask;
                let t = fq[(rng.next() % fq.len() as u64) as usize];
                check(f.tr(f.mul(u, t)) == tr_d(&f, d, f.mul(t, f.tr_rel(u, d))), "transitivity of trace");
            }
        }
    }
    // group law and counters on small selftest curves
    for (n, d) in [(3u32, 3u32), (4, 3), (5, 3)] {
        let big_n = n * d;
        let f = Gf::new(big_n);
        let fq = f.subfield(d);
        let mut rng = Rng::new(SEED, &format!("selftest-curve-{n}-{d}"));
        let a = (rng.next() as u32) & f.mask;
        let b = loop {
            let b = (rng.next() as u32) & f.mask;
            if b != 0 {
                break b;
            }
        };
        let cv = Curve { f: &f, a, b };
        let ne = cv.order();
        let pts = all_points(&cv);
        check(pts.len() as u64 == ne, "point count");
        for _ in 0..300 {
            let p = pts[(rng.next() % ne) as usize];
            let q = pts[(rng.next() % ne) as usize];
            let r = pts[(rng.next() % ne) as usize];
            check(cv.on(&cv.add(&p, &q)), "closure");
            check(cv.add(&cv.add(&p, &q), &r) == cv.add(&p, &cv.add(&q, &r)), "associativity");
            check(cv.add(&p, &q) == cv.add(&q, &p), "commutativity");
            check(cv.mul(ne, &p) == O, "order kills");
            check(cv.in_2e(&p) == (cv.mul(ne / 2, &p) == O), "2E criterion");
        }
        let fb = line_fb(&cv, &fq, 1, |_, _| true);
        let ctr = Counter::build(&cv, &fb, n as usize);
        let bm = brute(&cv, &fb, n as usize);
        let mut tot = 0u64;
        for p in &pts {
            let c = ctr.count(&cv, p);
            tot += c;
            check(bm.get(&p.key()).copied().unwrap_or(0) == c, "MITM = brute force");
        }
        check(tot == (fb.len() as u64).pow(n), "sum N = m^n");
    }
    println!("selftest: {}", if fails == 0 { "PASS".to_string() } else { format!("FAIL ({fails})") });
    if fails > 0 {
        std::process::exit(1);
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(|s| s.as_str()) {
        Some("selftest") => mode_selftest(),
        Some("run") => mode_run(a[2].parse().unwrap(), a[3].parse().unwrap(), &a[4]),
        Some("score") => score::score(&a[2]),
        Some("validate") => score::validate(&a[2], &a[3], &a[4..]),
        _ => eprintln!("usage: odd_n_trace selftest | run <n> <d> <out_dir> | score <run_dir> | validate <run_dir> <claimed.json> <file=sha256>..."),
    }
}
