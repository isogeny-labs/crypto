//! L2b generator: writes an msolve input file for the constrained prime-field
//! Semaev system  S_4(x1,x2,x3; z) = 0,  prod_{v in V}(x_i - v) = 0  (i=1..3).
//! Reuses `Curve::summation_poly` from src/cryptanalysis/semaev_prime_poly.rs
//! (included by path, not rewritten).
//!
//! arm real: the true S_4 and the interval base V (first N x-coordinates of
//!   curve points scanning x = 0,1,2,...).
//! arm ctrl: a polynomial with the same support (same dense-box monomials that
//!   are nonzero in S_4) and uniformly random coefficients, plus membership
//!   polynomials of the same degree N on N distinct uniformly random elements of F_p.
//!
//! usage: l2b-gen <p> <curve_seed> <target_seed> <arm> <out> [N]
//! prints one JSON line of the instance parameters on stdout.
#![allow(dead_code)]
#[path = "../../../../../src/cryptanalysis/semaev_prime_poly.rs"]
mod spp;
use rand::{rngs::StdRng, Rng, SeedableRng};
use spp::{Curve, Fp};
use std::io::Write;

fn is_qr(f: Fp, p: u64, v: u64) -> bool {
    v != 0 && f.pow(v, (p - 1) / 2) == 1
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let p: u64 = a[1].parse().unwrap();
    let cs: u64 = a[2].parse().unwrap();
    let ts: u64 = a[3].parse().unwrap();
    let arm = a[4].as_str();
    let out = &a[5];
    let n: usize = if a.len() > 6 {
        a[6].parse().unwrap()
    } else {
        (p as f64).cbrt().round() as usize
    };
    let f = Fp::new(p);
    let mut rc = StdRng::seed_from_u64(p ^ (cs << 20) ^ 0xC0FFEE);
    // curve: random (a,b), nonsingular
    let (ca, cb) = loop {
        let ca = rc.gen_range(0..p);
        let cb = rc.gen_range(0..p);
        let disc = f.add(f.mul(4, f.pow(ca, 3)), f.mul(27, f.mul(cb, cb)));
        if disc != 0 {
            break (ca, cb);
        }
    };
    let curve = Curve::new(p, ca, cb);
    let rhs = |x: u64| f.add(f.add(f.mul(x, f.mul(x, x)), f.mul(ca, x)), cb);
    // target x(R): random valid x
    let mut rt = StdRng::seed_from_u64(p ^ (cs << 20) ^ (ts << 40) ^ 0xBEEF);
    let z = loop {
        let z = rt.gen_range(0..p);
        if is_qr(f, p, rhs(z)) {
            break z;
        }
    };
    // interval base V
    let mut vset = Vec::new();
    let mut x = 0u64;
    while vset.len() < n {
        if is_qr(f, p, rhs(x)) {
            vset.push(x);
        }
        x += 1;
    }
    let interval_top = x;
    let sp = curve.summation_poly(3, z);
    let side = sp.deg + 1;
    let mut rr = StdRng::seed_from_u64(p ^ (cs << 20) ^ (ts << 40) ^ 0xABCD);
    let (coeffs, mset): (Vec<u64>, Vec<u64>) = if arm == "real" {
        (sp.coeffs.clone(), vset.clone())
    } else {
        let c: Vec<u64> = sp
            .coeffs
            .iter()
            .map(|&c| if c == 0 { 0 } else { rr.gen_range(1..p) })
            .collect();
        let mut s = std::collections::BTreeSet::new();
        while s.len() < n {
            s.insert(rr.gen_range(0..p));
        }
        (c, s.into_iter().collect())
    };
    let mut terms: Vec<String> = Vec::new();
    let mut maxdeg = 0;
    let mut nterms = 0;
    for (idx, &c) in coeffs.iter().enumerate() {
        if c == 0 {
            continue;
        }
        let e = [idx % side, (idx / side) % side, idx / (side * side)];
        maxdeg = maxdeg.max(e[0] + e[1] + e[2]);
        nterms += 1;
        let mut t = format!("{}", c);
        for (i, &ei) in e.iter().enumerate() {
            if ei > 0 {
                t.push_str(&format!("*x{}^{}", i + 1, ei));
            }
        }
        terms.push(t);
    }
    // membership: prod (X - v) coefficients
    let mut poly = vec![1u64];
    for &v in &mset {
        let mut np = vec![0u64; poly.len() + 1];
        for (i, &c) in poly.iter().enumerate() {
            np[i + 1] = f.add(np[i + 1], c);
            np[i] = f.sub(np[i], f.mul(c, v));
        }
        poly = np;
    }
    let mut polys = vec![terms.join("+")];
    for i in 1..=3 {
        let mut t = Vec::new();
        for (d, &c) in poly.iter().enumerate() {
            if c != 0 {
                if d == 0 {
                    t.push(format!("{}", c));
                } else {
                    t.push(format!("{}*x{}^{}", c, i, d));
                }
            }
        }
        polys.push(t.join("+"));
    }
    let mut fh = std::fs::File::create(out).unwrap();
    writeln!(fh, "x1,x2,x3\n{}\n{}", p, polys.join(",\n")).unwrap();
    println!(
        "{{\"p\":{},\"curve_seed\":{},\"target_seed\":{},\"arm\":\"{}\",\"a\":{},\"b\":{},\"z\":{},\"N\":{},\"interval_top\":{},\"f_terms\":{},\"f_total_degree\":{},\"memb_degree\":{}}}",
        p, cs, ts, arm, ca, cb, z, n, interval_top, nterms, maxdeg, n
    );
}
