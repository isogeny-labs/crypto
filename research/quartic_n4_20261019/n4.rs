// N4 plane-quartic index calculus: instance checks, solver, matched rho.
// Build: rustc --edition 2021 -O n4.rs
#![allow(dead_code, unused_imports, unused_variables, unused_mut, static_mut_refs)]

mod common;
mod ghs;
mod ic;
mod solver;
mod summ;
#[path = "../../src/cryptanalysis/plane_quartic_pic.rs"]
mod pic;

use common::*;
use ghs::*;

fn cubics_class(cv: &pic::PlaneQuartic, rows: Vec<Vec<u32>>) -> pic::PicClass {
    cv.reduce_deg6(&pic::Gen { k: 3, rows })
}

fn check(q: u64, seed: u64) {
    let t0 = std::time::Instant::now();
    let inst = Inst::search(q, seed, 200000).expect("no instance");
    println!(
        "instance q={} order={} ell={} tries={} search_s={:.2}",
        q,
        inst.order,
        inst.ell,
        inst.tries,
        t0.elapsed().as_secs_f64()
    );
    assert!(inst.ec.on(&inst.g));
    // N1 = q+1 by brute force (small q)
    if q <= 700 {
        let m4 = pic::mons_of_degree(4);
        let qq = q;
        let ev = |p: [u64; 3]| -> u64 {
            let mut s = 0u64;
            for (i, &(a, b, c)) in m4.iter().enumerate() {
                if inst.f[i] == 0 {
                    continue;
                }
                let t = powmod(p[0], a as u64, qq) * powmod(p[1], b as u64, qq) % qq * powmod(p[2], c as u64, qq) % qq;
                s = (s + inst.f[i] as u64 * t) % qq;
            }
            s
        };
        let mut n1 = 0;
        for y in 0..q {
            for z in 0..q {
                if ev([1, y, z]) == 0 {
                    n1 += 1;
                }
            }
        }
        for z in 0..q {
            if ev([0, 1, z]) == 0 {
                n1 += 1;
            }
        }
        if ev([0, 0, 1]) == 0 {
            n1 += 1;
        }
        println!("N1 = {} (q+1 = {})", n1, q + 1);
        assert_eq!(n1, q + 1);
    }
    let p0 = pic::first_rational_point_canonical(q as u32, &inst.f).expect("no point");
    let cv = pic::PlaneQuartic::new(q as u32, inst.f, p0).expect("pic new");
    // transfer divisors: points on the quartic, F_q-rational divisor
    let kf = &inst.kf;
    let m4 = pic::mons_of_degree(4);
    let mut rng = Rng::new(seed ^ 77);
    let evk = |w: &[K; 3]| -> K {
        let mut s = KZERO;
        for (i, &(a, b, c)) in m4.iter().enumerate() {
            if inst.f[i] == 0 {
                continue;
            }
            let mut t = kf.from_fq(inst.f[i] as u64);
            for _ in 0..a {
                t = kf.mul(&t, &w[0]);
            }
            for _ in 0..b {
                t = kf.mul(&t, &w[1]);
            }
            for _ in 0..c {
                t = kf.mul(&t, &w[2]);
            }
            s = kf.add(&s, &t);
        }
        s
    };
    // collect valid multiples
    let mut hs: Vec<(u64, pic::PicClass)> = Vec::new();
    let mut tried = 0;
    while hs.len() < 14 && tried < 400 {
        tried += 1;
        let n = rng.below(inst.ell - 1) + 1;
        let p = inst.ec.mul(&inst.g, n as u128);
        let Some(ws) = inst.transfer_w(&p) else { continue };
        for w in &ws {
            assert_eq!(evk(w), KZERO, "transfer point off the quartic");
        }
        let Some(rows) = inst.transfer_cubics(&p) else { continue };
        hs.push((n, cubics_class(&cv, rows)));
    }
    println!("valid transfers collected: {} of {} tried", hs.len(), tried);
    // hom check: need triples (a,b,a+b): build by choosing a,b then a+b
    let mut bshift: Option<pic::PicClass> = None;
    let mut trip = 0;
    let mut fails = 0;
    let mut tries2 = 0;
    while trip < 8 && tries2 < 600 {
        tries2 += 1;
        let a = rng.below(inst.ell - 1) + 1;
        let b = rng.below(inst.ell - 1) + 1;
        let c = (a + b) % inst.ell;
        if c == 0 {
            continue;
        }
        let mut cls = Vec::new();
        for n in [a, b, c] {
            let p = inst.ec.mul(&inst.g, n as u128);
            match inst.transfer_cubics(&p) {
                Some(rows) => cls.push(cubics_class(&cv, rows)),
                None => break,
            }
        }
        if cls.len() < 3 {
            continue;
        }
        let bsh = cv.sub(&cv.add(&cls[0], &cls[1]), &cls[2]);
        match &bshift {
            None => bshift = Some(bsh),
            Some(b0) => {
                if *b0 != bsh {
                    fails += 1;
                }
            }
        }
        trip += 1;
    }
    println!("hom triples {} failures {}", trip, fails);
    assert!(trip >= 3 && fails == 0);
    // Jacobian order: random classes killed by 4*ell, and (for most) not by 4
    let pts: Vec<pic::Pt> = if q <= 8191 {
        let p = cv.rational_points_bruteforce();
        println!("C(F_q) points by pic: {}", p.len());
        assert_eq!(p.len() as u64, q + 1);
        p
    } else {
        // above 8191 the O(q^2) enumeration is infeasible: use sampled points
        let curve = ic::Curve::new(q, &inst.f);
        let mut r2 = Rng::new(seed ^ 99);
        solver::sample_points(&curve, 200, &mut r2).iter().map(|p| [p[0] as u32, p[1] as u32, p[2] as u32]).collect()
    };
    let mut bad = 0;
    for _ in 0..10 {
        let mut x = cv.zero();
        for _ in 0..5 {
            let p = pts[rng.below(pts.len() as u64) as usize];
            x = cv.add(&x, &cv.point_class(p));
        }
        if cv.mul_scalar(&x, inst.order) != cv.zero() {
            bad += 1;
        }
    }
    println!("#Jac * x != 0 failures: {}", bad);
    assert_eq!(bad, 0);
    println!("CHECK OK");
}

fn cnt_json(c: &Cnt) -> String {
    format!(
        "{{\"mul\":{},\"inv\":{},\"add\":{},\"lmul\":{},\"ladd\":{},\"hash\":{},\"cmp\":{}}}",
        c.mul, c.inv, c.add, c.lmul, c.ladd, c.hash, c.cmp
    )
}

/// measured time ratios (min of 5 repetitions): inversion / multiplication in F_q,
/// mod-ell multiplication / F_q multiplication, and the F_q-multiplication-equivalent of one
/// E0(K) addition (counted, then priced with the inversion ratio).
fn ratios(inst: &Inst) -> (f64, f64, f64) {
    use std::hint::black_box;
    use std::time::Instant;
    let q = inst.q;
    let fp = Fp { q };
    let ell = inst.ell;
    let mut best = [f64::MAX; 3];
    for _ in 0..5 {
        let n = 2_000_000u64;
        let t = Instant::now();
        let mut x = 12345 % q;
        let y = 54321 % q;
        for _ in 0..n {
            x = black_box(x * y % q);
            x ^= 1;
        }
        black_box(x);
        best[0] = best[0].min(t.elapsed().as_secs_f64() / n as f64);
        let n2 = 200_000u64;
        let t = Instant::now();
        let mut z = 3u64;
        for _ in 0..n2 {
            z = black_box(inv_mod_u64(z % (q - 1) + 1, q));
        }
        black_box(z);
        best[1] = best[1].min(t.elapsed().as_secs_f64() / n2 as f64);
        let t = Instant::now();
        let mut x = 12345 % ell;
        let y = 54321 % ell;
        for _ in 0..n {
            x = black_box(((x as u128 * y as u128) % ell as u128) as u64);
            x ^= 1;
        }
        black_box(x);
        best[2] = best[2].min(t.elapsed().as_secs_f64() / n as f64);
    }
    let r_inv = best[1] / best[0];
    let r_l = best[2] / best[0];
    // one E0 addition
    let mut rng = Rng::new(5);
    let p = inst.ec.rand_point(&mut rng);
    let p2 = inst.ec.rand_point(&mut rng);
    let b = snap();
    let reps = 200;
    let mut acc = p;
    for _ in 0..reps {
        acc = inst.ec.add(&acc, &p2);
    }
    let d = snap().since(&b);
    let gae = (d.mul as f64 + r_inv * d.inv as f64) / reps as f64;
    (r_inv, r_l, gae)
}

fn solve_cmd(q: u64, seed: u64, nfb: usize, rows_mult: f64, verify: bool) {
    let inst = Inst::search(q, seed, 200000).expect("no instance");
    let (r_inv, r_l, gae) = ratios(&inst);
    println!("{{\"rec\":\"ratios\",\"q\":{},\"seed\":{},\"r_inv\":{:.3},\"r_lmul\":{:.3},\"gae_meq\":{:.2}}}", q, seed, r_inv, r_l, gae);
    let mut rng = Rng::new(seed ^ 0x1234);
    let k = rng.below(inst.ell - 1) + 1;
    let prm = solver::Params { nfb, rows_mult, max_desc: 100000 };
    let rep = solver::ic_solve(&inst, &prm, k, seed, verify);
    let mut ph = Vec::new();
    for (n, c, w) in &rep.phases {
        ph.push(format!("{{\"phase\":\"{}\",\"cnt\":{},\"wall\":{:.4}}}", n, cnt_json(c), w));
    }
    let st: Vec<String> = rep.stats.iter().map(|(k, v)| format!("\"{}\":{}", k, v)).collect();
    println!(
        "{{\"rec\":\"ic\",\"q\":{},\"seed\":{},\"ell\":{},\"nfb_param\":{},\"k\":{},\"k_found\":{},\"ok\":{},\"fail\":\"{}\",\"phases\":[{}],\"stats\":{{{}}}}}",
        q, seed, inst.ell, nfb, k,
        rep.k_found.map(|x| x.to_string()).unwrap_or("null".into()), rep.ok, rep.fail, ph.join(","), st.join(",")
    );
    let r = solver::rho(&inst, k, seed);
    println!(
        "{{\"rec\":\"rho\",\"q\":{},\"seed\":{},\"ell\":{},\"ok\":{},\"steps\":{},\"dps\":{},\"cnt\":{},\"wall\":{:.4}}}",
        q, seed, inst.ell, r.ok, r.steps, r.dps, cnt_json(&r.cnt), r.wall
    );
}

fn flat_cnt(prefix: &str, c: &Cnt, out: &mut Vec<String>) {
    out.push(format!("\"{p}.mul\":{},\"{p}.inv\":{},\"{p}.add\":{},\"{p}.lmul\":{},\"{p}.ladd\":{},\"{p}.hash\":{},\"{p}.cmp\":{}", c.mul, c.inv, c.add, c.lmul, c.ladd, c.hash, c.cmp, p = prefix));
}

/// One protocol unit: instance (q, seed); IC arm B0 with N = ceil(c q^0.6) (c grows by 0.4 on
/// failure, every attempt charged) or arm A0 (c0 <= 0: all rational points); `nrho` matched-rho
/// runs on the same instance with independent k. Output: flat JSON records.
fn unit_cmd(q: u64, seed: u64, c0: f64, nrho: usize, rows_mult: f64) {
    let arm = if c0 > 0.0 { "B0" } else if rows_mult < 0.0 { "A0r" } else { "A0" };
    let retry_rows = rows_mult < 0.0; // post-hoc arm A0r: rows_mult = -m, +0.3 per retry
    let rows_mult = rows_mult.abs();
    let inst = Inst::search(q, seed, 200000).expect("no instance");
    let (r_inv, r_l, gae) = ratios(&inst);
    // counted E0 addition, for the group-addition equivalent
    let mut rng = Rng::new(seed ^ 0x1234);
    let (gm, gi) = {
        let p = inst.ec.rand_point(&mut rng);
        let p2 = inst.ec.rand_point(&mut rng);
        let b = snap();
        let mut acc = p;
        for _ in 0..100 {
            acc = inst.ec.add(&acc, &p2);
        }
        let d = snap().since(&b);
        (d.mul as f64 / 100.0, d.inv as f64 / 100.0)
    };
    println!("{{\"rec\":\"ratios\",\"arm\":\"{}\",\"q\":{},\"seed\":{},\"ell\":{},\"order\":{},\"r_inv\":{:.4},\"r_lmul\":{:.4},\"gae_mul\":{:.3},\"gae_inv\":{:.3}}}", arm, q, seed, inst.ell, inst.order, r_inv, r_l, gm, gi);
    let k = rng.below(inst.ell - 1) + 1;
    let mut c = c0;
    let mut attempt = 0;
    loop {
        attempt += 1;
        let nfb = if c0 > 0.0 { ((c * (q as f64).powf(0.6)).ceil() as usize).min(q as usize + 1) } else { 0 };
        let rm = if retry_rows { rows_mult + 0.3 * (attempt as f64 - 1.0) } else { rows_mult };
        let prm = solver::Params { nfb, rows_mult: rm, max_desc: 100000 };
        let rep = solver::ic_solve(&inst, &prm, k, seed * 100 + attempt, true);
        let mut f: Vec<String> = Vec::new();
        for (n, cc, w) in &rep.phases {
            flat_cnt(n, cc, &mut f);
            f.push(format!("\"{}.wall\":{:.4}", n, w));
        }
        for (k2, v) in &rep.stats {
            f.push(format!("\"s.{}\":{}", k2, v));
        }
        println!(
            "{{\"rec\":\"ic\",\"arm\":\"{}\",\"q\":{},\"seed\":{},\"attempt\":{},\"c\":{:.2},\"nfb\":{},\"ell\":{},\"k\":{},\"ok\":{},\"fail\":\"{}\",{}}}",
            arm, q, seed, attempt, c, nfb, inst.ell, k, rep.ok, rep.fail, f.join(",")
        );
        if rep.ok || attempt >= 6 || (c0 <= 0.0 && !retry_rows) {
            break;
        }
        if c0 > 0.0 {
            c += 0.4;
        }
    }
    for i in 0..nrho {
        let kk = rng.below(inst.ell - 1) + 1;
        let r = solver::rho(&inst, kk, seed * 1000 + i as u64);
        let mut f = Vec::new();
        flat_cnt("c", &r.cnt, &mut f);
        println!(
            "{{\"rec\":\"rho\",\"q\":{},\"seed\":{},\"i\":{},\"ok\":{},\"steps\":{},\"dps\":{},\"wall\":{:.4},{}}}",
            q, seed, i, r.ok, r.steps, r.dps, r.wall, f.join(",")
        );
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(|s| s.as_str()) {
        Some("summarize") => summ::run(&a[2..]),
        Some("unit") => unit_cmd(a[2].parse().unwrap(), a[3].parse().unwrap(), a[4].parse().unwrap(), a[5].parse().unwrap(), a[6].parse().unwrap()),
        Some("solve") => solve_cmd(a[2].parse().unwrap(), a[3].parse().unwrap(), a[4].parse().unwrap(), a[5].parse().unwrap(), a.get(6).is_some()),
        Some("check") => check(a[2].parse().unwrap(), a[3].parse().unwrap()),
        _ => eprintln!("usage: n4 check <q> <seed>"),
    }
}
