//! Index-calculus / Gröbner cell on the prime-field Semaev object, using the
//! repo's own degree-bounded F4 (`cryptanalysis::f4_fp`).
//!
//! System (m = 3 decomposition of a fixed target Q):
//!     S4(x1, x2, x3, xQ) = 0
//!     h(x_i) = 0, i = 1..3            (h = membership polynomial of the base)
//!
//! Two bases with the same number of x-values:
//!   window   : k smallest x-values                    -> generic membership h
//!   pullback : {x : x^2 in T}, |T| = k/2               -> even h = prod (x^2 - t)
//!
//! Reports F4's `solving_degree` / `degree_reached` / matrix size for each,
//! plus a correctness check of S4 against the group law.
//!
//! Honest scope: toy primes; the base is small relative to p, so each cell is a
//! near-empty ideal (see the G30-004 status note). This is an instrument
//! exercise on the real object, not a degree-of-regularity result.

use crypto_lib::cryptanalysis::f4_fp::{f4, normalise, Ordering, Poly, F4Options};
use std::collections::HashMap;
use std::time::Duration;

type B = HashMap<Vec<u32>, u64>;

fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn bconst(c: u64) -> B {
    let mut m = HashMap::new();
    m.insert(vec![0, 0, 0], c);
    m
}

fn bvar(i: usize) -> B {
    let mut e = vec![0, 0, 0];
    e[i] = 1;
    let mut m = HashMap::new();
    m.insert(e, 1);
    m
}

fn badd(a: &B, b: &B, p: u64) -> B {
    let mut out = a.clone();
    for (e, c) in b {
        let v = out.entry(e.clone()).or_insert(0);
        *v = (*v + c) % p;
    }
    out.retain(|_, v| *v != 0);
    out
}

fn bmul(a: &B, b: &B, p: u64) -> B {
    let mut out: B = HashMap::new();
    for (ea, ca) in a {
        for (eb, cb) in b {
            let mut e = ea.clone();
            for (i, k) in eb.iter().enumerate() {
                e[i] += k;
            }
            let v = out.entry(e).or_insert(0);
            *v = (*v + mulmod(*ca, *cb, p)) % p;
        }
    }
    out.retain(|_, v| *v != 0);
    out
}

fn bscale(a: &B, c: u64, p: u64) -> B {
    let mut out = a.clone();
    for v in out.values_mut() {
        *v = mulmod(*v, c, p);
    }
    out.retain(|_, v| *v != 0);
    out
}

fn to_poly(b: &B, p: u64) -> Poly {
    let terms: Vec<(Vec<u32>, u64)> = b.iter().map(|(e, c)| (e.clone(), *c)).collect();
    normalise(&terms, p, Ordering::Grevlex)
}

/// S3 as a quadratic in its third argument, coefficients (A, B, C) as
/// polynomials in the two supplied variables (which may be constants).
fn s3_abc(p1: &B, p2: &B, a: u64, b: u64, p: u64) -> (B, B, B) {
    // A = (x1 - x2)^2
    let diff = badd(p1, &bscale(p2, p - 1, p), p);
    let aa = bmul(&diff, &diff, p);
    // B = -2[(x1 + x2)(x1 x2 + a) + 2b]
    let s = badd(p1, p2, p);
    let pr = bmul(p1, p2, p);
    let inner = badd(&pr, &bconst(a), p);
    let t = badd(&bmul(&s, &inner, p), &bconst(2 * b % p), p);
    let bb = bscale(&t, p - 2, p);
    // C = (x1 x2 - a)^2 - 4b(x1 + x2)
    let u = badd(&pr, &bconst(p - a), p);
    let cc = badd(&bmul(&u, &u, p), &bscale(&s, (p - 4 * b % p) % p, p), p);
    (aa, bb, cc)
}

/// S4(x1,x2,x3,x4) = Res_D(S3(x1,x2,D), S3(x3,x4,D)) for two quadratics in D.
fn s4(p: u64, a: u64, b: u64, x1: &B, x2: &B, x3: &B, x4: &B) -> B {
    let (a1, b1, c1) = s3_abc(x1, x2, a, b, p);
    let (a2, b2, c2) = s3_abc(x3, x4, a, b, p);
    let ac = badd(&bmul(&a1, &c2, p), &bscale(&bmul(&a2, &c1, p), p - 1, p), p);
    let ab = badd(&bmul(&a1, &b2, p), &bscale(&bmul(&a2, &b1, p), p - 1, p), p);
    let bc = badd(&bmul(&b1, &c2, p), &bscale(&bmul(&b2, &c1, p), p - 1, p), p);
    badd(&bmul(&ac, &ac, p), &bscale(&bmul(&ab, &bc, p), p - 1, p), p)
}

fn membership(xs: &[u64], p: u64, i: usize) -> B {
    let mut h = bconst(1);
    for w in xs {
        let f = badd(&bvar(i), &bconst((p - w % p) % p), p);
        h = bmul(&h, &f, p);
    }
    h
}

fn eadd(p: u64, a: u64, l: (u64, u64), r: (u64, u64)) -> Option<(u64, u64)> {
    let (x1, y1) = l;
    let (x2, y2) = r;
    let lam = if x1 == x2 {
        if (y1 + y2) % p == 0 {
            return None;
        }
        mulmod(mulmod(3, mulmod(x1, x1, p), p) + a, modinv(2 * y1 % p, p), p)
    } else {
        mulmod((y2 + p - y1) % p, modinv((x2 + p - x1) % p, p), p)
    };
    let x3 = (mulmod(lam, lam, p) + 2 * p - x1 - x2) % p;
    let y3 = (mulmod(lam, (x1 + p - x3) % p, p) + p - y1) % p;
    Some((x3, y3))
}

fn modinv(a: u64, p: u64) -> u64 {
    let mut t = 0i64;
    let mut newt = 1i64;
    let mut r = p as i64;
    let mut newr = (a % p) as i64;
    while newr != 0 {
        let q = r / newr;
        let tmp = t - q * newt;
        t = newt;
        newt = tmp;
        let tmp2 = r - q * newr;
        r = newr;
        newr = tmp2;
    }
    ((t % p as i64 + p as i64) % p as i64) as u64
}

fn curve_points(p: u64, a: u64, b: u64) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    for x in 0..p {
        let rhs = (mulmod(mulmod(x, x, p), x, p) + mulmod(a, x, p) + b) % p;
        // sqrt for p = 3 mod 4
        let y = powmod(rhs, (p + 1) / 4, p);
        if mulmod(y, y, p) == rhs {
            out.push((x, y));
            if y != 0 {
                out.push((x, (p - y) % p));
            }
        }
    }
    out
}

fn powmod(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, p);
        }
        a = mulmod(a, a, p);
        e >>= 1;
    }
    r
}

fn main() {
    let p: u64 = 4099;
    let (a, b) = (1u64, 1u64);
    let pts = curve_points(p, a, b);
    println!("p={p} a={a} b={b} #E={}", pts.len());

    // correctness check: S4 vanishes on genuine 4-sums to O (skip the
    // degenerate leading-coefficient cases x1 == x2 or x3 == x4, where the
    // two-quadratic resultant formula does not apply)
    let mut by_x: HashMap<u64, (u64, u64)> = HashMap::new();
    for q in &pts {
        by_x.entry(q.0).or_insert(*q);
    }
    let mut ux: Vec<(u64, u64)> = by_x.values().copied().collect();
    ux.sort_unstable();
    let mut checked = 0;
    'check: for i in 0..ux.len().min(30) {
        for j in (i + 1)..ux.len().min(30) {
            for k in (j + 1)..ux.len().min(30) {
                if ux[i].0 == ux[j].0 || ux[j].0 == ux[k].0 {
                    continue;
                }
                let Some(s12) = eadd(p, a, ux[i], ux[j]) else { continue };
                let Some(s123) = eadd(p, a, s12, ux[k]) else { continue };
                let p4 = (s123.0, (p - s123.1) % p);
                if p4.0 == ux[i].0 || p4.0 == ux[j].0 || p4.0 == ux[k].0 {
                    continue;
                }
                let c = |v: u64| {
                    let mut m = HashMap::new();
                    m.insert(vec![0, 0, 0], v);
                    m
                };
                let f = s4(p, a, b, &c(ux[i].0), &c(ux[j].0), &c(ux[k].0), &c(p4.0));
                let val = f.get(&vec![0, 0, 0]).copied().unwrap_or(0);
                assert_eq!(val, 0, "S4 failed on a genuine 4-sum");
                checked += 1;
                if checked >= 20 {
                    break 'check;
                }
            }
        }
    }
    println!("S4 correctness: {checked}/20 genuine 4-sums vanish");

    let xs_all: Vec<u64> = {
        let mut v: Vec<u64> = pts.iter().map(|q| q.0).collect();
        v.sort_unstable();
        v.dedup();
        v
    };

    for k in [6usize, 8] {
        let wx: Vec<u64> = xs_all.iter().copied().take(k).collect();
        // pullback: |T| = k/2, x-set = {x : x^2 in T}
        let mut t: Vec<u64> = Vec::new();
        for x in xs_all.iter().filter(|x| **x != 0) {
            let sq = mulmod(*x, *x, p);
            if !t.contains(&sq) {
                t.push(sq);
            }
            if t.len() == (k / 2).max(2) {
                break;
            }
        }
        let px: Vec<u64> = (0..p).filter(|x| t.contains(&mulmod(*x, *x, p))).collect();
        // decomposable target from the window base (real points only)
        let wpts: Vec<(u64, u64)> = wx.iter().filter_map(|x| by_x.get(x).copied()).collect();
        let mut target = None;
        'outer: for i in 0..wpts.len() {
            for j in 0..wpts.len() {
                for l in 0..wpts.len() {
                    if let Some(s12) = eadd(p, a, wpts[i], wpts[j]) {
                        if let Some(s123) = eadd(p, a, s12, wpts[l]) {
                            target = Some(s123.0);
                            break 'outer;
                        }
                    }
                }
            }
        }

        for (name, set) in [("window", &wx), ("pullback", &px)] {
            // per-family decomposable target (a target from one base need not decompose in the other)
            let fpts: Vec<(u64, u64)> = set.iter().filter_map(|x| by_x.get(x).copied()).collect();
            let mut ft = None;
            'fam: for i in 0..fpts.len() {
                for j in 0..fpts.len() {
                    for l in 0..fpts.len() {
                        if let Some(a12) = eadd(p, a, fpts[i], fpts[j]) {
                            if let Some(a123) = eadd(p, a, a12, fpts[l]) {
                                ft = Some(a123.0);
                                break 'fam;
                            }
                        }
                    }
                }
            }
            let xq = ft.unwrap_or(set[0]);
            let mut polys = vec![to_poly(&s4(p, a, b, &bvar(0), &bvar(1), &bvar(2), &bconst(xq)), p)];
            for i in 0..3 {
                polys.push(to_poly(&membership(set, p, i), p));
            }
            let opts = F4Options::new(Ordering::Grevlex, 40)
                .with_budget(Duration::from_secs(120));
            let rep = f4(&polys, 3, p, &opts);
            println!(
                "k={k:2} {name:9} |xs|={:3} deg_h={:2} inconsistent={} solving_degree={:2} reached={:2} rows={} cols={} ms={:.1} above_bound={} timed_out={}",
                set.len(),
                set.len(),
                rep.inconsistent,
                rep.solving_degree,
                rep.degree_reached,
                rep.max_rows,
                rep.max_cols,
                rep.ms,
                rep.pairs_above_bound,
                rep.timed_out
            );
        }
    }
}
