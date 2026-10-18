//! Point counting and curve search: `#E(F_q)` by baby-step giant-step on
//! random points, random prime-field or binary curves whose conductor `f`
//! is divisible by the requested primes (so their isogeny class has the
//! requested volcanoes and is large), and the Koblitz table.

use std::collections::{HashMap, HashSet};

use rand::rngs::SmallRng;
use rand::Rng;

use super::classno;
use super::field::{factor, is_prime, Curve, F2m, Fp, Fq, Pt};

/// `#E(F_q)` by BSGS on random points, intersected until one Hasse
/// candidate remains; `None` after ten points.
pub fn group_order<F: Fq>(f: &F, q: f64, e: &Curve, rng: &mut SmallRng) -> Option<u64> {
    let s = (4.0 * q.sqrt()).ceil() as u64 + 2;
    let lo = (q as u64 + 1).saturating_sub(s / 2);
    let m = (s as f64).sqrt().ceil() as u64 + 1;
    let mut cands: Option<HashSet<u64>> = None;
    for _ in 0..10 {
        let p = Some(f.ec_random(e, rng));
        let mut baby: HashMap<Pt, u64> = HashMap::with_capacity(m as usize);
        let mut jp: Pt = None;
        for j in 0..m {
            baby.entry(f.ec_neg(e, jp)).or_insert(j);
            jp = f.ec_add(e, jp, p);
        }
        let step = f.ec_mul(e, p, m);
        let mut g = f.ec_mul(e, p, lo);
        let mut found = HashSet::new();
        for i in 0..=m {
            if let Some(&j) = baby.get(&g) {
                found.insert(lo + i * m + j);
            }
            g = f.ec_add(e, g, step);
        }
        cands = Some(match cands {
            None => found,
            Some(c) => c.intersection(&found).copied().collect(),
        });
        let c = cands.as_ref().unwrap();
        let hasse: Vec<u64> = c
            .iter()
            .copied()
            .filter(|&v| ((v as f64) - (q + 1.0)).powi(2) <= 4.0 * q)
            .collect();
        if hasse.len() == 1 {
            return Some(hasse[0]);
        }
    }
    None
}

/// Check a claimed order: eight random points vanish under it.
pub fn order_holds<F: Fq>(f: &F, e: &Curve, order: u64, rng: &mut SmallRng) -> bool {
    (0..8).all(|_| f.ec_mul(e, Some(f.ec_random(e, rng)), order).is_none())
}

pub fn largest_prime_factor(n: u64) -> u64 {
    factor(n).last().map(|&(q, _)| q).unwrap_or(1)
}

/// A found curve with its arithmetic.
#[derive(Clone, Debug)]
pub struct Found {
    pub curve: Curve,
    pub order: u64,
    pub n: u64,
    pub cofactor: u64,
    pub t: i64,
    pub d_abs: u64,
    pub dk: i64,
    pub f: u64,
    pub tried: u64,
}

pub fn describe<F: Fq>(fq: &F, q: f64, e: Curve, order: u64, tried: u64) -> Found {
    let t = (q as i128 + 1 - order as i128) as i64;
    let d_abs = {
        // exact: 4q − t² in i128 when q < 2^62
        let q_int = q as u128;
        (4 * q_int as i128 - (t as i128) * (t as i128)) as u64
    };
    let n = largest_prime_factor(order);
    let (dk, f) = classno::fundamental(d_abs);
    let _ = fq;
    Found {
        curve: e,
        order,
        n,
        cofactor: order / n,
        t,
        d_abs,
        dk,
        f,
        tried,
    }
}

/// Whether `f` is divisible by every `ℓ^e` in `want`, with `n` prime to every `ℓ`.
fn wanted(found: &Found, want: &[(u64, u32)], max_cofactor: u64, min_n_bits: u32) -> bool {
    if found.cofactor > max_cofactor || found.n < (1u64 << min_n_bits) {
        return false;
    }
    want.iter()
        .all(|&(l, e)| found.f.is_multiple_of(l.pow(e)) && !found.n.is_multiple_of(l))
}

/// Random prime-field curves over random `bits`-bit primes until the
/// conductor has the wanted prime powers.  `j0` restricts to `y² = x³ + b`.
pub fn search_prime(
    bits: u32,
    want: &[(u64, u32)],
    max_cofactor: u64,
    j0: bool,
    rng: &mut SmallRng,
) -> (Fp, Found) {
    let mut tried = 0u64;
    let want2 = want.iter().any(|&(l, _)| l == 2);
    loop {
        let p = loop {
            let c = rng.gen_range((1u64 << (bits - 1))..(1u64 << bits));
            let c = if want2 { (c & !7) | 1 } else { c | 1 };
            let c = if j0 { c - c % 3 + 1 } else { c };
            if c >> (bits - 1) == 1 && is_prime(c) {
                break c;
            }
        };
        let f = Fp { p };
        for _ in 0..300 {
            tried += 1;
            let e = Curve {
                a: if j0 { 0 } else { rng.gen_range(1..p) },
                b: rng.gen_range(1..p),
            };
            match f.ec_j(&e) {
                Some(0) if j0 => {}
                Some(j) if !j0 && j != 0 && j != 1728 % p => {}
                _ => continue,
            }
            let Some(order) = group_order(&f, p as f64, &e, rng) else {
                continue;
            };
            let found = describe(&f, p as f64, e, order, tried);
            if wanted(&found, want, max_cofactor, 20) {
                return (f, found);
            }
        }
    }
}

/// Random binary curves `y² + xy = x³ + ax² + b` over `F_{2^m}` (with the
/// first irreducible polynomial of degree `m`) until the conductor has the
/// wanted odd prime powers.
pub fn search_binary(
    m: u32,
    want: &[(u64, u32)],
    max_cofactor: u64,
    rng: &mut SmallRng,
) -> (F2m, Found) {
    let f = F2m::new(m, F2m::irreducible_poly(m, 0));
    let q = 2f64.powi(m as i32);
    let mut tried = 0u64;
    loop {
        tried += 1;
        let e = Curve {
            a: f.frandom(rng),
            b: f.frandom(rng),
        };
        if e.b == 0 {
            continue;
        }
        let Some(order) = group_order(&f, q, &e, rng) else {
            continue;
        };
        let found = describe(&f, q, e, order, tried);
        if wanted(&found, want, max_cofactor, 20) {
            return (f, found);
        }
    }
}

/// The Koblitz curves `E_a/F_{2^m}` for `m` in a range: order, largest
/// prime factor, conductor and its factorisation.
pub fn koblitz_table(m_lo: u32, m_hi: u32) -> Vec<serde_json::Value> {
    let mut out = Vec::new();
    for m in m_lo..=m_hi {
        for a in [0u64, 1] {
            let t = classno::koblitz_trace(a, m);
            let order = ((1i128 << m) + 1 - t) as u64;
            let n = largest_prime_factor(order);
            let d_abs = ((1i128 << (m + 2)) - t * t) as u64;
            let (dk, f) = classno::fundamental(d_abs);
            out.push(serde_json::json!({
                "m": m, "a": a, "trace": t, "order": order, "n": n, "cofactor": order / n,
                "n_bits": 64 - n.leading_zeros(), "fundamental_disc": dk, "conductor": f,
                "conductor_factored": factor(f),
            }));
        }
    }
    out
}
