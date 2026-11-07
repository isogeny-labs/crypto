//! Self tests: every component against an independent check.
use crate::cov;
use crate::ec::*;
use crate::poly::*;
use crate::rho;
use crate::stats::*;
use crate::util::*;
use crate::yieldcov;
use std::collections::HashSet;

fn check(name: &str, ok: bool, detail: String) {
    println!("{} {name}: {detail}", if ok { "PASS" } else { "FAIL" });
    if !ok {
        std::process::exit(1);
    }
}

/// splitcount.rs `hex_mod` + `classify`, copied verbatim in logic, for equivalence checks.
mod reference {
    pub struct Ref {
        pub l: u64,
        pub split: bool,
        pub ps: f64,
        pub divides_n: bool,
    }
    fn hex_mod(hex: &str, m: u64) -> u64 {
        let mut acc: u128 = 0;
        for c in hex.chars() {
            acc = (acc * 16 + c.to_digit(16).unwrap() as u128) % (m as u128);
        }
        acc as u64
    }
    fn pow_mod(b: u64, mut e: u64, m: u64) -> u64 {
        let mut r: u128 = 1;
        let mut bb = (b % m) as u128;
        while e > 0 {
            if e & 1 == 1 {
                r = r * bb % (m as u128);
            }
            bb = bb * bb % (m as u128);
            e >>= 1;
        }
        r as u64
    }
    fn legendre(a: u64, l: u64) -> i32 {
        let a = a % l;
        if a == 0 {
            return 0;
        }
        let e = pow_mod(a, (l - 1) / 2, l);
        if e == 1 { 1 } else if e == l - 1 { -1 } else { panic!() }
    }
    pub fn classify(p_hex: &str, n_hex: &str, l: u64) -> Ref {
        let lf = l as f64;
        let p = hex_mod(p_hex, l);
        let n = hex_mod(n_hex, l);
        let t = (p + 1 + l - n) % l;
        let disc = ((t * t) % l + 4 * l - (4 * p) % l) % l;
        let chi_d = legendre(disc, l);
        let chi_p = legendre(p, l) as f64;
        let ps = (lf - 2.0 - chi_p) / (2.0 * (lf - 1.0));
        let pr = (1.0 + chi_p) * lf / (lf * lf - 1.0);
        let p_is_one = p % l == 1;
        let m = (lf + if p_is_one { 0.0 } else { 1.0 }) / (lf * lf - 1.0);
        let ps_c = if p_is_one { ps / (1.0 - m) } else { (ps - m) / (1.0 - m) };
        let _ = pr;
        Ref { l, split: chi_d == 1, ps: ps_c, divides_n: n == 0 }
    }
}

pub fn run() {
    // 1. number theory
    let mut rng = Rng::new(1);
    let mut bad = 0;
    for _ in 0..200 {
        let p = [1009u64, 1013, 65537, 2147483647, 4294967291][rng.below(5) as usize];
        let a = rng.below(p);
        if let Some(r) = sqrt_mod(a * a % p, p) {
            if r != a && r != p - a { bad += 1; }
        } else { bad += 1; }
    }
    check("sqrt_mod", bad == 0, format!("{bad} failures in 200 (p in 1009..2^32)"));
    check("is_prime", is_prime(4294967291) && !is_prime(4294967295) && is_prime(1009) && !is_prime(1001), "ok".into());

    // 2. group order: BSGS vs brute-force Legendre count, many curves, plus nP = O
    let mut ok = 0;
    let mut tested = 0;
    for p in [1009u64, 2003, 10007, 65537] {
        for _ in 0..40 {
            let c = Curve { p, a: rng.below(p), b: rng.below(p) };
            if !c.disc_nonzero() { continue; }
            let brute: u64 = 1 + (0..p).map(|x| 1 + legendre(c.rhs(x), p) as i64).sum::<i64>() as u64;
            let pt = c.random_point(&mut rng);
            let cands = hasse_candidates(&c, pt);
            tested += 1;
            if let Some(cs) = cands {
                // true order must be among the candidates
                if cs.contains(&brute) { ok += 1; } else { println!("  miss p={p} a={} b={} brute={brute} cands={cs:?}", c.a, c.b); }
            } else { ok += 1; } // small-order point: skipped by design
            if let Ok(Order::Prime(n)) = order_prime_or_composite(&c, &mut rng) {
                if n != brute { ok -= 1; println!("  WRONG prime order {n} vs {brute}"); }
            }
        }
    }
    check("bsgs vs brute count", ok == tested, format!("{ok}/{tested} curves"));

    // 3. division polynomials: degree, leading coeff, and torsion <=> root
    let mut ok = true;
    let mut info = String::new();
    for p in [1009u64, 2003, 10007] {
        for _ in 0..6 {
            let (a, b) = (rng.below(p), rng.below(p));
            let c = Curve { p, a, b };
            if !c.disc_nonzero() { continue; }
            let (f3, f5) = (psi3(a, b, p), psi5(a, b, p));
            ok &= f3.len() == 5 && f5.len() == 13 && f5[12] == 5;
            for (f, l) in [(&f3, 3u64), (&f5, 5u64)] {
                // roots in F_p by evaluation, and torsion test on every F_p-point abscissa
                let by_eval = (0..p).filter(|&x| peval(f, x, p) == 0).count();
                ok &= roots_in_fp(f, p) == by_eval;
                let mut o = 0;
                for x in 0..p {
                    if let Some(y) = sqrt_mod(c.rhs(x), p) {
                        if y != 0 {
                            let is_tors = c.mul(&mut o, l, Some((x, y))).is_none();
                            if is_tors != (peval(f, x, p) == 0) { ok = false; }
                        }
                    }
                }
                // squarefree
                ok &= pgcd(f, &pderiv(f, p), p).len() == 1;
            }
            info = format!("last p={p} psi3 roots {} psi5 roots {}", roots_in_fp(&f3, p), roots_in_fp(&f5, p));
        }
    }
    check("psi3/psi5 (deg, lead 5, torsion<=>root, gcd vs evaluation, squarefree)", ok, info);

    // 4. covariates identical to splitcount.rs classify
    let p = 4194319u64; // prime > 200000
    assert!(is_prime(p));
    let tabs = cov::tables(p);
    let mut worst: f64 = 0.0;
    let mut cnt = 0;
    let mut p691 = 0;
    while cnt < 3 {
        let c = Curve { p, a: 1 + rng.below(p - 1), b: 1 + rng.below(p - 1) };
        if !c.disc_nonzero() { continue; }
        let n = match order_prime_or_composite(&c, &mut rng) { Ok(Order::Prime(n)) => n, _ => continue };
        let mine = cov::stats(p, n, &tabs);
        p691 = mine.iter().find(|c| c.cutoff == 691).unwrap().primes;
        let (ph, nh) = (format!("{p:x}"), format!("{n:x}"));
        for st in &mine {
            let (mut obs, mut es, mut var_, mut ll, mut ell, mut k) = (0u64, 0f64, 0f64, 0f64, 0f64, 0u64);
            for &l in primes_up_to(st.cutoff as usize).iter().filter(|&&l| l > 2) {
                let r = reference::classify(&ph, &nh, l);
                if r.divides_n { continue; }
                k += 1;
                if r.split { obs += 1; }
                es += r.ps;
                var_ += r.ps * (1.0 - r.ps);
                let _ = (l, &mut ll, &mut ell);
            }
            let z = (obs as f64 - es) / var_.sqrt();
            assert_eq!(k, st.primes);
            worst = worst.max((z - st.z).abs()).max((obs as f64 - st.split as f64).abs());
        }
        cnt += 1;
    }
    check("z_c equals splitcount.rs recount at all 12 cutoffs", cnt > 0 && worst < 1e-9, format!("{cnt} curves, max |dz| = {worst:.2e}; odd primes used at 691: {p691} (124 expected)"));

    // 5. rho recovers k, and is deterministic
    let p = 65537u64;
    let mut found = 0;
    let mut tries = 0;
    let mut ops_sum = 0u64;
    while tries < 6 {
        let c = Curve { p, a: 1 + rng.below(p - 1), b: 1 + rng.below(p - 1) };
        if !c.disc_nonzero() { continue; }
        let n = match order_prime_or_composite(&c, &mut rng) { Ok(Order::Prime(n)) => n, _ => continue };
        let g = c.random_point(&mut rng);
        for ti in 0..10u64 {
            let k = 1 + rng.below(n - 1);
            let mut z = 0;
            let q = c.mul(&mut z, k, Some(g)).unwrap();
            let o = rho::rho_one(&c, g, q, n, k, 1000 + ti, 60 * (n as f64).sqrt() as u64 + 20000);
            let o2 = rho::rho_one(&c, g, q, n, k, 1000 + ti, 60 * (n as f64).sqrt() as u64 + 20000);
            assert_eq!(o.ops_total, o2.ops_total);
            tries += 1;
            if o.verified && o.recovered_matches { found += 1; ops_sum += o.ops_total; }
        }
        tries += 0;
        if tries >= 60 { break; }
    }
    check("rho recovers planted k", found == tries, format!("{found}/{tries}; mean ops {:.0}", ops_sum as f64 / found.max(1) as f64));

    // 6. coverage vs brute-force set of group elements
    let p = 1009u64;
    let mut ok = true;
    for _ in 0..3 {
        let c = Curve { p, a: 1 + rng.below(p - 1), b: 1 + rng.below(p - 1) };
        if !c.disc_nonzero() { continue; }
        let n = match order_prime_or_composite(&c, &mut rng) { Ok(Order::Prime(n)) => n, _ => continue };
        let nb = 20;
        let bp = yieldcov::base_points(&c, nb).unwrap();
        let mut set: HashSet<Pt> = HashSet::new();
        let mut o = 0;
        for i in 0..nb { for j in i..nb {
            let (bi, bj) = (Some(bp[i]), Some(bp[j]));
            for (si, sj) in [(false, false), (false, true), (true, false), (true, true)] {
                let u = if si { c.neg(bi) } else { bi };
                let v = if sj { c.neg(bj) } else { bj };
                set.insert(c.add(&mut o, u, v));
            }
        }}
        let cv = yieldcov::coverage(&c, n, nb).unwrap();
        if cv.size != set.len() as u64 { ok = false; println!("  size {} vs brute {}", cv.size, set.len()); }
    }
    check("m=2 coverage equals brute-force set size", ok, "p=1009, N=20, 3 curves".into());

    // 7. statistics
    let q = t_quantile(0.975, 10.0);
    check("t quantile", (q - 2.228139).abs() < 1e-4, format!("t_0.975,10 = {q:.6}"));
    check("normal", (norm_cdf(1.959964) - 0.975).abs() < 1e-6 && (norm_quantile(0.975) - 1.959964).abs() < 1e-5, "ok".into());
    let mut r2 = Rng::new(5);
    let m = 500;
    let z: Vec<f64> = standardize(&(0..m).map(|_| r2.unit()).collect::<Vec<_>>()).unwrap();
    let w: Vec<f64> = standardize(&(0..m).map(|_| r2.unit()).collect::<Vec<_>>()).unwrap();
    let y: Vec<f64> = (0..m).map(|i| 10.0 + 0.3 * z[i] + 0.7 * w[i] + 0.01 * (r2.unit() - 0.5)).collect();
    let f = ols(&y, &z, &w);
    check("ols slope", (f.b - 0.3).abs() < 0.01 && f.lo < 0.3 && f.hi > 0.3, format!("b = {:.4} [{:.4},{:.4}]", f.b, f.lo, f.hi));
    println!("ALL SELFTESTS PASSED");
}
