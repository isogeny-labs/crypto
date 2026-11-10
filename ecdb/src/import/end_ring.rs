//! End(E) for every ordinary short-Weierstrass curve over F_p, 5 <= p < 2^10
//! (the census-fp curves and every other curve over those fields), by
//! Kohel's criterion on the l^k-torsion, prime by prime of the Frobenius
//! conductor. The ℓ-volcano importer (`volcano.rs`) only pins End(E) when
//! the conductor v of Z[pi] is {2, 3, 5}-smooth; this one covers every v.
//!
//! # Mathematics
//!
//! Let t^2 - 4p = v^2 D_K with D_K fundamental, so Z[pi] is the order of
//! conductor v in O_K, and let f be the conductor of End(E) (Z[pi] ⊆ End(E),
//! so f | v). For a prime l with d = v_l(v) >= 1:
//!
//! * (pi - a)/l^k is an endomorphism iff pi - a kills E[l^k], i.e. pi acts on
//!   E[l^k] as the scalar a.
//! * Write pi = a0 + v w (w = (D_K + sqrt D_K)/2). Locally at l,
//!   (pi - a0)/l^k lies in the order Z + f O_K iff k <= d - v_l(f).
//!
//! Hence v_l(f) = d - k_max, where k_max is the largest k <= d with pi a
//! scalar on E[l^k] (Kohel 1996, Ch. 4; Bisson-Sutherland 2011, §2). This is
//! the ℓ-volcano level counted from the crater.
//!
//! **The test.** A scalar a has trace 2a = t mod l^k and is a unit. For N =
//! l^k let g_N be the monic polynomial whose roots are the x-coordinates of
//! the nonzero points of E[N] (f_N for N odd, F f_N for N even, F = x^3 +
//! A x + B, f_n the x-only division polynomials). pi is the scalar ±a on
//! E[N] iff x^p = x([a]P) on every P in E[N] \ {0}, i.e.
//!
//! >  den_a x^p = den_a x - num_a   (mod g_N)
//!
//! with x([a]P) = x - num_a/den_a: num_a = 4F f_{a-1} f_{a+1}, den_a =
//! f_a^2 for a odd, num_a = f_{a-1} f_{a+1}, den_a = 4F f_a^2 for a even;
//! den_a is a unit mod g_N since gcd(a, N) = 1. (If pi P = ±aP for every P,
//! every vector of E[N] is an eigenvector of pi a^{-1} with eigenvalue ±1,
//! so pi = ±a.) For l odd, a = t/2 mod N is the only candidate; for l = 2
//! both odd a with 2a = t mod 2^k are tried.
//!
//! # Checks
//!
//! Every value stored passes, before anything is written:
//! - the input: p prime, nonzero discriminant, |t| <= 2 sqrt p, p ∤ t, t
//!   recounted as -sum_x chi(x^3 + A x + B),
//!   t^2 - 4p = v^2 D_K with D_K a negative fundamental discriminant;
//! - per curve: scalar on E[l^k] implies scalar on E[l^(k-1)]; a scalar a
//!   has a^2 = p mod l^k (det pi), and l^2k divides #E(F_p) = p + 1 - t when
//!   a = 1 (resp. #E'(F_p) = p + 1 + t of the twist when a = -1);
//! - f | v and disc End(E) = f^2 D_K divides t^2 - 4p with square quotient;
//! - per isogeny class, Deuring/Waterhouse: the F_p-isomorphism classes with
//!   End(E) = O_f form a torsor under Pic(O_f), so for every f | v exactly
//!   h(f^2 D_K) distinct curves (by F_p-isomorphism, (A, B) ~ (u^4 A, u^6 B))
//!   must have conductor f. The census holds every curve over F_p, so a class
//!   that is short (incomplete) or over (inconsistent) is refused whole;
//! - where ecdb already holds End(E) (`endomorphism_ring.conductor_f`) or a
//!   level (`endomorphism_level`), it must agree exactly, or the class is
//!   refused.
//!
//! # Writes
//!
//! `endomorphism_ring` (status `proved`, `conductor_divides` = v,
//! `contains` = {Z[pi]}) for curves without a row, or a row whose conductor is
//! NULL (updated in place); `endomorphism_level` for every l | v (depth v_l(v),
//! the level, status `proved`) where no row exists. Insert-or-update only.
//!
//! # Report
//!
//! `end_ring_curves`, `end_ring_classes_ok`, `end_ring_classes_refused`
//! (each also a problem), `end_ring_curves_refused_input`,
//! `end_ring_existing_agree`, `end_ring_levels_existing_agree`,
//! `endomorphism_rings_inserted_census_fp` (the previously-null census-fp
//! curves filled), `endomorphism_rings_inserted_other`,
//! `endomorphism_rings_updated_null_conductor`, `endomorphism_levels_inserted`.

use super::{bulk, Ctx, Report};
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

/// Fields covered: 5 <= p < MAX_P (the census-fp fields).
pub const MAX_P: u64 = 1024;

const METHOD: &str = "Kohel: v_l(f) = v_l(v) - max{k : pi is a scalar on E[l^k]}, tested by x^p = x([a]P) mod the \
                      l^k-division polynomial; class checked against h(f^2 D_K)";

const CURVES_SQL: &str = "
SELECT c.id, coalesce(c.family, ''), ic.id, ic.p::int8, c.coefficients->>'a', c.coefficients->>'b',
       fg.trace::int8, fg.conductor_v::int8, fg.cm_discriminant::int8,
       er.curve_id IS NOT NULL, er.conductor_f::int8, er.discriminant::int8,
       (SELECT coalesce(jsonb_agg(jsonb_build_array(el.ell::int8, el.level, el.depth)), '[]'::jsonb)
          FROM ec.endomorphism_level el WHERE el.curve_id = c.id AND el.level IS NOT NULL)
FROM ec.curve c
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
JOIN ec.frobenius_genus1 fg ON fg.isogeny_class_id = ic.id
LEFT JOIN ec.endomorphism_ring er ON er.curve_id = c.id
WHERE c.genus = 1 AND c.model = 'short_weierstrass' AND ic.g = 1 AND ic.q = ic.p AND ic.p >= 5 AND ic.p < $1::int8
  AND fg.conductor_v IS NOT NULL AND fg.cm_discriminant IS NOT NULL AND NOT coalesce(fg.is_supersingular, false)
ORDER BY ic.id, c.id";

const END_SQL: &str = "
INSERT INTO ec.endomorphism_ring (curve_id, over_closure, kind, discriminant, conductor_f, conductor_divides,
       is_maximal, contains, cm_field_id, status, method, provenance_id)
SELECT r.curve_id, false, 'imaginary_quadratic_order', r.disc, r.f, r.v, r.f = 1, ARRAY['Z[pi]'], nf.id, 'proved',
       r.method, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, disc numeric, f numeric, v numeric, d_k numeric, method text)
LEFT JOIN LATERAL (SELECT n.id FROM ec.number_field n WHERE n.fundamental_discriminant = r.d_k AND n.degree = 2
                   ORDER BY n.id LIMIT 1) nf ON true
ON CONFLICT (curve_id) DO NOTHING";

/// Fill a row that exists with an unknown conductor; never touches a known one, and never overwrites a
/// stored discriminant, `is_maximal`, `conductor_divides` or kind that disagrees with the proved result.
/// Rows the guards skip are counted by the caller and reported as problems (updated < sent).
const END_UPDATE_SQL: &str = "
UPDATE ec.endomorphism_ring er SET discriminant = r.disc, conductor_f = r.f, is_maximal = (r.f = 1),
       conductor_divides = coalesce(er.conductor_divides, r.v), status = 'proved', method = r.method,
       provenance_id = $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, disc numeric, f numeric, v numeric, method text)
WHERE er.curve_id = r.curve_id AND er.conductor_f IS NULL AND er.kind = 'imaginary_quadratic_order'
  AND (er.conductor_divides IS NULL OR mod(er.conductor_divides, r.f) = 0)
  AND (er.discriminant IS NULL OR er.discriminant = r.disc)
  AND (er.is_maximal IS NULL OR er.is_maximal = (r.f = 1))";

const LEVEL_SQL: &str = "
INSERT INTO ec.endomorphism_level (curve_id, ell, depth, level, method, status, provenance_id)
SELECT r.curve_id, r.ell, r.depth, r.level, r.method, 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, ell numeric, depth int, level int, method text)
ON CONFLICT (curve_id, ell) DO NOTHING";

// ------------------------------------------------------------- polynomials over F_p

type Poly = Vec<u64>; // ascending coefficients in [0, p)

fn trim(mut a: Poly) -> Poly {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}

fn pmul(a: &[u64], b: &[u64], p: u64) -> Poly {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    // p < 2^16: each product < 2^32, and at most 2^31 of them fit in a u64 sum.
    let mut out = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            out[i + j] += x * y;
        }
        if i % 1024 == 1023 {
            out.iter_mut().for_each(|c| *c %= p);
        }
    }
    trim(out.into_iter().map(|c| c % p).collect())
}

fn psub(a: &[u64], b: &[u64], p: u64) -> Poly {
    let n = a.len().max(b.len());
    trim((0..n).map(|i| (a.get(i).copied().unwrap_or(0) + p - b.get(i).copied().unwrap_or(0)) % p).collect())
}

fn pscale(a: &[u64], c: u64, p: u64) -> Poly {
    trim(a.iter().map(|x| x * (c % p) % p).collect())
}

fn powmod(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % p;
        }
        b = b * b % p;
        e >>= 1;
    }
    r
}

fn inv(a: u64, p: u64) -> u64 {
    powmod(a, p - 2, p)
}

/// a mod g, g monic.
fn prem(a: &[u64], g: &[u64], p: u64) -> Poly {
    let dg = g.len() - 1;
    let mut r: Poly = a.to_vec();
    while r.len() > dg {
        let lead = r.pop().unwrap();
        if lead == 0 {
            continue;
        }
        let shift = r.len() - dg;
        for (i, &c) in g[..dg].iter().enumerate() {
            r[shift + i] = (r[shift + i] + (p - lead) * c) % p;
        }
    }
    trim(r)
}

fn monic(g: &[u64], p: u64) -> Poly {
    let li = inv(*g.last().unwrap(), p);
    pscale(g, li, p)
}

/// x^e mod g.
fn xpow(e: u64, g: &[u64], p: u64) -> Poly {
    let mut r: Poly = prem(&[1], g, p);
    let mut b: Poly = prem(&[0, 1], g, p);
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            r = prem(&pmul(&r, &b, p), g, p);
        }
        b = prem(&pmul(&b, &b, p), g, p);
        e >>= 1;
    }
    r
}

/// The x-only division polynomials f_0..=f_n of y^2 = x^3 + A x + B:
/// psi_n = f_n for n odd and psi_n = 2y f_n for n even.
fn division_polys(n: usize, a: u64, b: u64, p: u64) -> Vec<Poly> {
    let m = |v: i64| v.rem_euclid(p as i64) as u64;
    let (ai, bi) = (a as i64, b as i64);
    let big_f = trim(vec![b % p, a % p, 0, 1]);
    let g2 = pmul(&pscale(&big_f, 4, p), &pscale(&big_f, 4, p), p); // (4F)^2 = (2y)^4
    let mut f: Vec<Poly> = vec![
        Vec::new(),
        vec![1],
        vec![1],
        trim(vec![m(-ai * ai), m(12 * bi), m(6 * ai), 0, 3]),
        trim(vec![
            m(2 * (-8 * bi * bi - ai * ai * ai)),
            m(-8 * ai * bi),
            m(-10 * ai * ai),
            m(40 * bi),
            m(10 * ai),
            0,
            2,
        ]),
    ];
    for k in 5..=n {
        let mm = k / 2;
        let next = if k % 2 == 1 {
            let t1 = pmul(&f[mm + 2], &pmul(&f[mm], &pmul(&f[mm], &f[mm], p), p), p);
            let t2 = pmul(&f[mm - 1], &pmul(&f[mm + 1], &pmul(&f[mm + 1], &f[mm + 1], p), p), p);
            if mm % 2 == 0 {
                psub(&pmul(&g2, &t1, p), &t2, p)
            } else {
                psub(&t1, &pmul(&g2, &t2, p), p)
            }
        } else {
            let t1 = pmul(&f[mm + 2], &pmul(&f[mm - 1], &f[mm - 1], p), p);
            let t2 = pmul(&f[mm - 2], &pmul(&f[mm + 1], &f[mm + 1], p), p);
            pmul(&f[mm], &psub(&t1, &t2, p), p)
        };
        f.push(next);
    }
    f.truncate(n + 1);
    f
}

/// Whether pi acts on E[N] as the scalar ±a (a a unit mod N, 1 <= a < N).
fn scalar_on(n: u64, a: u64, xp_cache: &mut BTreeMap<u64, Poly>, fs: &[Poly], big_f: &[u64], p: u64) -> bool {
    let n_us = n as usize;
    let g = if n.is_multiple_of(2) { monic(&pmul(big_f, &fs[n_us], p), p) } else { monic(&fs[n_us], p) };
    let xp = xp_cache.entry(n).or_insert_with(|| xpow(p, &g, p)).clone();
    let a_us = a as usize;
    let four_f = pscale(big_f, 4, p);
    let (num, den) = if a % 2 == 1 {
        (pmul(&four_f, &pmul(&fs[a_us - 1], &fs[a_us + 1], p), p), pmul(&fs[a_us], &fs[a_us], p))
    } else {
        (pmul(&fs[a_us - 1], &fs[a_us + 1], p), pmul(&four_f, &pmul(&fs[a_us], &fs[a_us], p), p))
    };
    let (num, den) = (prem(&num, &g, p), prem(&den, &g, p));
    let lhs = prem(&pmul(&den, &xp, p), &g, p);
    let rhs = psub(&prem(&pmul(&den, &[0, 1], p), &g, p), &num, p);
    lhs == rhs
}

// ------------------------------------------------------------- integers

fn is_prime(n: u64) -> bool {
    n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| !n.is_multiple_of(d))
}

fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut d = 2;
    while d * d <= n {
        let mut e = 0;
        while n.is_multiple_of(d) {
            n /= d;
            e += 1;
        }
        if e > 0 {
            out.push((d, e));
        }
        d += 1;
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

fn is_fundamental(d: i64) -> bool {
    if d >= 0 {
        return false;
    }
    let squarefree = |m: u64| factor(m).iter().all(|&(_, e)| e == 1);
    let n = d.unsigned_abs();
    match d.rem_euclid(4) {
        1 => squarefree(n),
        0 => {
            let m = d / 4;
            matches!(m.rem_euclid(4), 2 | 3) && squarefree(m.unsigned_abs())
        }
        _ => false,
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

/// h(D) for a negative discriminant D: primitive reduced forms (a, b, c).
pub fn class_number(d: i64) -> u64 {
    assert!(d < 0 && d.rem_euclid(4) <= 1);
    let mut h = 0;
    let mut a = 1i64;
    while 3 * a * a <= -d {
        for b in -a + 1..=a {
            let num = b * b - d;
            if num % (4 * a) != 0 {
                continue;
            }
            let c = num / (4 * a);
            if c < a || (b < 0 && a == c) {
                continue;
            }
            if gcd(gcd(a, b), c) == 1 {
                h += 1;
            }
        }
        a += 1;
    }
    h
}

/// The F_p-isomorphism class of y^2 = x^3 + A x + B: min over u of (u^4 A, u^6 B).
pub fn iso_key(p: u64, a: u64, b: u64) -> (u64, u64) {
    (1..p).map(|u| (a * powmod(u, 4, p) % p, b * powmod(u, 6, p) % p)).min().unwrap()
}

// ------------------------------------------------------------- the conductor

/// v_l(f) at one prime l | v.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Level {
    pub ell: u64,
    pub depth: u32,
    pub level: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EndRing {
    /// Conductor of End(E) in O_K.
    pub f: u64,
    /// disc End(E) = f^2 D_K.
    pub disc: i64,
    pub levels: Vec<Level>,
}

/// The conductor of End(E) for y^2 = x^3 + A x + B over F_p with trace t and
/// t^2 - 4p = v^2 D_K. Every input identity and per-curve check of the module
/// documentation is enforced; a failure is the reason.
pub fn end_ring(p: u64, a: u64, b: u64, t: i64, v: u64, d_k: i64) -> Result<EndRing, String> {
    if !(5..1 << 16).contains(&p) || !is_prime(p) {
        return Err(format!("p = {p} is not a prime in [5, 2^16)"));
    }
    let (a, b) = (a % p, b % p);
    if (4 * powmod(a, 3, p) + 27 * b % p * b).is_multiple_of(p) {
        return Err("singular model".into());
    }
    let pi = p as i64;
    if t * t > 4 * pi {
        return Err(format!("t = {t} is outside the Hasse interval"));
    }
    if t.rem_euclid(pi) == 0 {
        return Err("supersingular (p | t)".into());
    }
    // the trace itself, recounted: #E = p + 1 + sum_x chi(x^3 + A x + B)
    let mut is_sq = vec![false; p as usize];
    for y in 1..p {
        is_sq[(y * y % p) as usize] = true;
    }
    let chi_sum: i64 = (0..p)
        .map(|x| match (x * x % p * x + a * x + b) % p {
            0 => 0,
            r if is_sq[r as usize] => 1,
            _ => -1,
        })
        .sum();
    if -chi_sum != t {
        return Err(format!("the recorded trace is {t}, the point count gives {}", -chi_sum));
    }
    if v == 0 || t * t - 4 * pi != (v * v) as i64 * d_k {
        return Err(format!("t^2 - 4p = {} != v^2 D_K = {v}^2 * {d_k}", t * t - 4 * pi));
    }
    if !is_fundamental(d_k) {
        return Err(format!("D_K = {d_k} is not a negative fundamental discriminant"));
    }
    let primes = factor(v);
    let n_max = primes.iter().map(|&(l, d)| l.pow(d)).max().unwrap_or(1) as usize;
    let fs = if n_max >= 2 { division_polys(n_max.max(4), a, b, p) } else { Vec::new() };
    let big_f = trim(vec![b, a, 0, 1]);
    let order = |s: i64| (pi + 1 - s * t) as u64; // #E (s = 1) or #E' (s = -1)
    let mut f = 1u64;
    let mut levels = Vec::new();
    for &(l, d) in &primes {
        let mut xp_cache = BTreeMap::new();
        let mut scalar = Vec::new(); // scalar[k-1]: pi is a scalar on E[l^k]
        for k in 1..=d {
            let n = l.pow(k);
            let cands: Vec<u64> = if l == 2 {
                if k == 1 {
                    vec![1]
                } else {
                    let half = n / 2;
                    let a0 = (t.rem_euclid(n as i64) as u64 / 2) % half;
                    [a0, a0 + half].into_iter().filter(|x| x % 2 == 1).collect()
                }
            } else {
                // (n + 1)/2 = 1/2 mod n (n odd)
                vec![(t.rem_euclid(n as i64) as u64 * n.div_ceil(2)) % n]
            };
            let mut hit = None;
            for &c in &cands {
                if scalar_on(n, c, &mut xp_cache, &fs, &big_f, p) {
                    hit = Some(c);
                    break;
                }
            }
            if let Some(c) = hit {
                if (c * c) % n != p % n {
                    return Err(format!("pi = {c} on E[{n}] but {c}^2 != p mod {n}"));
                }
                let n2 = n * n;
                let ok_plus = c == 1 && order(1) % n2 == 0;
                let ok_minus = c == n - 1 && order(-1) % n2 == 0;
                let needs = c == 1 || c == n - 1;
                let ok = if l == 2 {
                    !needs || order(1) % n2 == 0 || order(-1) % n2 == 0
                } else {
                    !needs || ok_plus || ok_minus
                };
                if !ok {
                    return Err(format!("pi = ±1 on E[{n}] but {n}^2 divides neither p + 1 - t nor p + 1 + t"));
                }
            }
            scalar.push(hit.is_some());
        }
        let k_max = scalar.iter().take_while(|&&s| s).count() as u32;
        if scalar.iter().skip(k_max as usize).any(|&s| s) {
            return Err(format!("pi is a scalar on E[{l}^k] for some k > {k_max} but not on E[{l}^{}]", k_max + 1));
        }
        let level = d - k_max;
        f *= l.pow(level);
        levels.push(Level { ell: l, depth: d, level });
    }
    if !v.is_multiple_of(f) {
        return Err(format!("f = {f} does not divide v = {v}"));
    }
    let disc = (f * f) as i64 * d_k;
    let q = v / f;
    if (q * q) as i64 * disc != t * t - 4 * pi {
        return Err("t^2 - 4p != (v/f)^2 f^2 D_K".into());
    }
    Ok(EndRing { f, disc, levels })
}

/// Deuring/Waterhouse on one isogeny class: for every f | v exactly h(f^2 D_K)
/// distinct F_p-isomorphism classes have conductor f. `members` is (iso key,
/// conductor) per curve; isomorphic curves must share the conductor.
pub fn check_class(v: u64, d_k: i64, members: &[((u64, u64), u64)]) -> Result<(), String> {
    let mut by_key: BTreeMap<(u64, u64), u64> = BTreeMap::new();
    for (key, f) in members {
        if let Some(old) = by_key.insert(*key, *f) {
            if old != *f {
                return Err(format!("isomorphic curves {key:?} with conductors {old} and {f}"));
            }
        }
    }
    let mut count: BTreeMap<u64, u64> = BTreeMap::new();
    for f in by_key.values() {
        *count.entry(*f).or_default() += 1;
    }
    for f in (1..=v).filter(|f| v.is_multiple_of(*f)) {
        let want = class_number((f * f) as i64 * d_k);
        let have = count.remove(&f).unwrap_or(0);
        if have != want {
            let what = if have < want { "short (class incomplete)" } else { "over" };
            return Err(format!("{have} curves with conductor {f}, h({}) = {want}: {what}", (f * f) as i64 * d_k));
        }
    }
    if let Some((f, n)) = count.into_iter().next() {
        return Err(format!("{n} curves with conductor {f}, which does not divide v = {v}"));
    }
    Ok(())
}

// ------------------------------------------------------------- import

struct Row {
    id: i64,
    census: bool,
    p: u64,
    a: u64,
    b: u64,
    t: i64,
    v: u64,
    d_k: i64,
    has_row: bool,
    old_f: Option<i64>,
    old_disc: Option<i64>,
    old_levels: Vec<(u64, i32, Option<i32>)>,
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let mut report = Report {
        input: format!("ordinary short-Weierstrass curves over F_p, 5 <= p < {MAX_P} (stored ec.curve)"),
        ..Default::default()
    };
    let client = ctx.client;
    let src = ctx
        .source(
            "ecdb/src/import/end_ring.rs",
            &crate::math::sha256_hex(include_str!("end_ring.rs")),
            "End(E) of every ordinary census curve, computed by ecdb (Kohel's l^k-torsion criterion)",
        )
        .await?;
    let prov = ctx.provenance(src, METHOD, None).await?;

    let max_p = MAX_P as i64;
    let mut classes: BTreeMap<i64, Vec<Row>> = BTreeMap::new();
    for r in client.query(CURVES_SQL, &[&max_p]).await? {
        let p = r.get::<_, i64>(3) as u64;
        let coeff = |i: usize| r.get::<_, Option<String>>(i).and_then(|s| s.parse::<u64>().ok());
        let (Some(a), Some(b)) = (coeff(4), coeff(5)) else {
            report.count("end_ring_curves_refused_input", 1);
            report.problem(format!("curve {}: coefficients a, b are not field elements", r.get::<_, i64>(0)));
            continue;
        };
        let levels: Value = r.get(12);
        let old_levels = levels
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|x| Some((x[0].as_u64()?, x[1].as_i64()? as i32, x[2].as_i64().map(|d| d as i32))))
            .collect();
        classes.entry(r.get(2)).or_default().push(Row {
            id: r.get(0),
            census: r.get::<_, String>(1) == "census-fp",
            p,
            a,
            b,
            t: r.get(6),
            v: r.get::<_, i64>(7) as u64,
            d_k: r.get(8),
            has_row: r.get(9),
            old_f: r.get(10),
            old_disc: r.get(11),
            old_levels,
        });
    }

    let (mut ends_census, mut ends_other, mut updates, mut levels) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    'class: for (class_id, rows) in &classes {
        report.count("end_ring_curves", rows.len() as u64);
        let mut computed = Vec::new();
        for r in rows {
            match end_ring(r.p, r.a, r.b, r.t, r.v, r.d_k) {
                Ok(e) => computed.push(e),
                Err(why) => {
                    report.count("end_ring_classes_refused", 1);
                    report.problem(format!("class {class_id}: curve {}: {why}", r.id));
                    continue 'class;
                }
            }
        }
        let members: Vec<((u64, u64), u64)> =
            rows.iter().zip(&computed).map(|(r, e)| (iso_key(r.p, r.a, r.b), e.f)).collect();
        if let Err(why) = check_class(rows[0].v, rows[0].d_k, &members) {
            report.count("end_ring_classes_refused", 1);
            report.problem(format!("class {class_id} (p = {}, t = {}): {why}", rows[0].p, rows[0].t));
            continue;
        }
        // agreement with what ecdb already holds
        let (mut agree, mut lagree) = (0u64, 0u64);
        for (r, e) in rows.iter().zip(&computed) {
            if let Some(of) = r.old_f {
                if of as u64 != e.f || r.old_disc.is_some_and(|d| d != e.disc) {
                    report.count("end_ring_classes_refused", 1);
                    report.problem(format!(
                        "class {class_id}: curve {} has stored conductor {of} (disc {:?}), ecdb computes {} (disc {})",
                        r.id, r.old_disc, e.f, e.disc
                    ));
                    continue 'class;
                }
                agree += 1;
            } else if let Some(d) = r.old_disc.filter(|&d| d != e.disc) {
                // conductor unknown but a discriminant is stored: it must still agree, or the class is refused
                report.count("end_ring_classes_refused", 1);
                report.problem(format!(
                    "class {class_id}: curve {} has stored discriminant {d} with no conductor, ecdb computes {} \
                     (f = {})",
                    r.id, e.disc, e.f
                ));
                continue 'class;
            }
            for &(ell, lv, depth) in &r.old_levels {
                let (want_l, want_d) =
                    e.levels.iter().find(|x| x.ell == ell).map_or((0, 0), |x| (x.level as i32, x.depth as i32));
                if lv != want_l || depth.is_some_and(|d| d != want_d) {
                    report.count("end_ring_classes_refused", 1);
                    report.problem(format!(
                        "class {class_id}: curve {} has stored level {lv} (depth {depth:?}) at l = {ell}, ecdb computes \
                         {want_l} (depth {want_d})",
                        r.id
                    ));
                    continue 'class;
                }
                lagree += 1;
            }
        }
        report.count("end_ring_classes_ok", 1);
        report.count("end_ring_existing_agree", agree);
        report.count("end_ring_levels_existing_agree", lagree);
        for (r, e) in rows.iter().zip(&computed) {
            let row = json!({ "curve_id": r.id, "disc": e.disc, "f": e.f, "v": r.v, "d_k": r.d_k, "method": METHOD });
            if !r.has_row {
                if r.census {
                    ends_census.push(row);
                } else {
                    ends_other.push(row);
                }
            } else if r.old_f.is_none() {
                updates.push(row);
            }
            let known: BTreeSet<u64> = r.old_levels.iter().map(|x| x.0).collect();
            for lv in e.levels.iter().filter(|lv| !known.contains(&lv.ell)) {
                levels.push(json!({ "curve_id": r.id, "ell": lv.ell, "depth": lv.depth, "level": lv.level,
                    "method": METHOD }));
            }
        }
    }
    report.count("endomorphism_rings_inserted_census_fp", bulk(client, END_SQL, &ends_census, prov).await?);
    report.count("endomorphism_rings_inserted_other", bulk(client, END_SQL, &ends_other, prov).await?);
    let updated = bulk(client, END_UPDATE_SQL, &updates, prov).await?;
    report.count("endomorphism_rings_updated_null_conductor", updated);
    let skipped = (updates.len() as u64).saturating_sub(updated);
    if skipped > 0 {
        // the guards in END_UPDATE_SQL refused rows whose stored kind, discriminant, is_maximal or
        // conductor_divides contradict the proved conductor: nothing was overwritten, but say so
        report.count("endomorphism_rings_update_refused_conflict", skipped);
        report.problem(format!(
            "{skipped} stored endomorphism_ring row(s) with a NULL conductor were left unchanged because their \
             kind, discriminant, is_maximal or conductor_divides contradict the proved conductor"
        ));
    }
    report.count("endomorphism_levels_inserted", bulk(client, LEVEL_SQL, &levels, prov).await?);
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// (iso key, conductor) of one curve of a class.
    type Member = ((u64, u64), u64);

    /// Affine points of y^2 = x^3 + A x + B and the group law, by brute force.
    type Pt = Option<(u64, u64)>;

    fn add(p: u64, a: u64, s: Pt, t: Pt) -> Pt {
        let (Some((x1, y1)), Some((x2, y2))) = (s, t) else { return s.or(t) };
        let lam = if x1 == x2 {
            if (y1 + y2) % p == 0 {
                return None;
            }
            (3 * x1 * x1 + a) % p * inv(2 * y1 % p, p) % p
        } else {
            (y2 + p - y1) % p * inv((x2 + p - x1) % p, p) % p
        };
        let x3 = (lam * lam % p + 2 * p - x1 - x2) % p;
        let y3 = (lam * ((x1 + p - x3) % p) % p + p - y1) % p;
        Some((x3, y3))
    }

    fn mul(p: u64, a: u64, n: u64, s: Pt) -> Pt {
        (0..n).fold(None, |acc, _| add(p, a, acc, s))
    }

    fn points(p: u64, a: u64, b: u64) -> Vec<Pt> {
        let mut out = vec![None];
        for x in 0..p {
            let r = (x * x % p * x + a * x + b) % p;
            for y in 0..p {
                if y * y % p == r {
                    out.push(Some((x, y)));
                }
            }
        }
        out
    }

    /// t^2 - 4p = v^2 D_K with D_K fundamental.
    fn split_disc(t: i64, p: u64) -> (u64, i64) {
        let delta = t * t - 4 * p as i64;
        let mut v = 1u64;
        for (l, e) in factor(delta.unsigned_abs()) {
            v *= l.pow(e / 2);
        }
        let mut dk = delta / (v * v) as i64;
        if dk.rem_euclid(4) != 1 {
            dk *= 4;
            v /= 2;
        }
        assert!(is_fundamental(dk), "{delta} -> {v} {dk}");
        (v, dk)
    }

    fn nonresidue(p: u64) -> u64 {
        (2..p).find(|&d| powmod(d, (p - 1) / 2, p) == p - 1).unwrap()
    }

    fn val(n: u64, l: u64) -> u32 {
        if n == 0 {
            return 64;
        }
        let (mut n, mut k) = (n, 0);
        while n % l == 0 {
            n /= l;
            k += 1;
        }
        k
    }

    #[test]
    fn class_numbers_match_known_values() {
        // Cox, Primes of the form x^2 + ny^2, and the class number formula.
        assert_eq!(class_number(-3), 1);
        assert_eq!(class_number(-4), 1);
        assert_eq!(class_number(-23), 3);
        assert_eq!(class_number(-56), 4);
        assert_eq!(class_number(-147), 2); // 1 * 7 (1 - (-3/7)/7) / 3
        assert_eq!(class_number(-12), 1);
        assert_eq!(class_number(-16), 1);
        assert_eq!(class_number(-196), 4); // -4 * 7^2: 7 (1 + 1/7) / 2
    }

    #[test]
    fn division_polys_vanish_on_torsion() {
        let (p, a, b) = (101u64, 3u64, 7u64);
        let fs = division_polys(9, a, b, p);
        let pts = points(p, a, b);
        for n in 3..=9u64 {
            for pt in &pts {
                let Some((x, y)) = pt else { continue };
                if *y == 0 {
                    continue; // 2-torsion: the factor 2y of psi_n for n even
                }
                let ev = fs[n as usize].iter().rev().fold(0, |acc, c| (acc * x + c) % p);
                assert_eq!(ev == 0, mul(p, a, n, *pt).is_none(), "n = {n}, P = {pt:?}");
            }
        }
    }

    /// Every curve over small F_p: the conductor agrees with the full rational
    /// l^k-torsion of E and its twist, and every class satisfies Deuring.
    #[test]
    fn brute_force_agreement_on_small_fields() {
        let mut nontrivial = 0;
        for p in [5u64, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 53, 61, 67, 73, 97, 101] {
            let nr = nonresidue(p);
            let mut classes: BTreeMap<i64, Vec<Member>> = BTreeMap::new();
            let mut seen = BTreeSet::new();
            for a in 0..p {
                for b in 0..p {
                    if (4 * a * a * a + 27 * b * b) % p == 0 || !seen.insert(iso_key(p, a, b)) {
                        continue;
                    }
                    let pts = points(p, a, b);
                    let t = p as i64 + 1 - pts.len() as i64;
                    if t.rem_euclid(p as i64) == 0 {
                        continue;
                    }
                    let (v, dk) = split_disc(t, p);
                    let e = end_ring(p, a, b, t, v, dk).unwrap();
                    classes.entry(t).or_default().push((iso_key(p, a, b), e.f));
                    // full rational l^k-torsion: (pi - 1)/l^k in Z + f O_K
                    let tw = points(p, a * nr % p * nr % p, b * nr % p * nr % p * nr % p);
                    for lv in &e.levels {
                        nontrivial += (lv.level > 0) as u32;
                        for (s, pts) in [(1i64, &pts), (-1, &tw)] {
                            // s pi = c0 + s v w with c0 = s (t - v D_K)/2
                            let c0 = s * (t - v as i64 * dk) / 2 - 1;
                            let k_pred = val(c0.unsigned_abs(), lv.ell).min(lv.depth - lv.level);
                            let mut k_bf = 0;
                            while {
                                let n = lv.ell.pow(k_bf + 1);
                                pts.iter()
                                    .filter(|q| mul(p, a * if s < 0 { nr * nr % p } else { 1 } % p, n, **q).is_none())
                                    .count() as u64
                                    == n * n
                            } {
                                k_bf += 1;
                            }
                            assert!(k_bf >= k_pred.min(lv.depth), "p {p} ({a},{b}) l {} s {s}", lv.ell);
                            assert_eq!(
                                k_bf.min(lv.depth - lv.level),
                                k_pred,
                                "p {p} ({a},{b}) t {t} l {} s {s}",
                                lv.ell
                            );
                        }
                    }
                }
            }
            for (t, members) in &classes {
                let (v, dk) = split_disc(*t, p);
                check_class(v, dk, members).unwrap_or_else(|e| panic!("p = {p}, t = {t}: {e}"));
            }
        }
        assert!(nontrivial > 10, "only {nontrivial} non-crater levels exercised");
    }

    /// j = 0 over F_37 with t = ±1: t^2 - 148 = -147 = 7^2 (-3). The curve has
    /// the automorphism of order 3, so End = Z[zeta_3] (f = 1) although 7 | v;
    /// the other two curves of the class sit on the floor, f = 7, and
    /// h(-147) = 2 (class number formula).
    #[test]
    fn pinned_j0_crater_over_f37() {
        let p = 37;
        let mut j0 = None;
        for b in 1..p {
            let t = p as i64 + 1 - points(p, 0, b).len() as i64;
            if t.abs() == 1 {
                j0 = Some((b, t));
                break;
            }
        }
        let (b, t) = j0.unwrap();
        let e = end_ring(p, 0, b, t, 7, -3).unwrap();
        assert_eq!(e, EndRing { f: 1, disc: -3, levels: vec![Level { ell: 7, depth: 1, level: 0 }] });
        let mut floor = BTreeSet::new();
        for a in 1..p {
            for bb in 0..p {
                if (4 * a * a * a + 27 * bb * bb) % p != 0
                    && p as i64 + 1 - points(p, a, bb).len() as i64 == t
                    && end_ring(p, a, bb, t, 7, -3).unwrap().f == 7
                {
                    floor.insert(iso_key(p, a, bb));
                }
            }
        }
        assert_eq!(floor.len(), 2);
    }

    /// y^2 = x^3 - x over F_13 with t = ±6: t^2 - 52 = -16 = 2^2 (-4), End = Z[i].
    #[test]
    fn pinned_j1728_over_f13() {
        let p = 13;
        let t = p as i64 + 1 - points(p, p - 1, 0).len() as i64;
        assert_eq!(t.abs(), 6);
        assert_eq!(end_ring(p, p - 1, 0, t, 2, -4).unwrap().f, 1);
    }

    #[test]
    fn corrupted_inputs_are_refused() {
        // a wrong discriminant split, a wrong trace, a singular model
        assert!(end_ring(37, 0, 1, 1, 1, -147).is_err());
        assert!(end_ring(37, 0, 1, 3, 7, -3).is_err());
        assert!(end_ring(37, 0, 0, 1, 7, -3).is_err());
        // a class with one curve replaced by a curve of another class
        let p = 37;
        let t = 1;
        let mut members = Vec::new();
        for a in 0..p {
            for b in 0..p {
                if (4 * a * a * a + 27 * b * b) % p != 0 && p as i64 + 1 - points(p, a, b).len() as i64 == t {
                    members.push(((a, b), iso_key(p, a, b)));
                }
            }
        }
        let mut keys = BTreeSet::new();
        members.retain(|(_, k)| keys.insert(*k));
        assert_eq!(members.len(), 3); // h(-3) + h(-147)
        let good: Vec<_> = members.iter().map(|((a, b), k)| (*k, end_ring(p, *a, *b, t, 7, -3).unwrap().f)).collect();
        check_class(7, -3, &good).unwrap();
        // corrupt: swap one floor curve for a curve whose trace is not 1
        let (oa, ob) = (1..p)
            .flat_map(|a| (1..p).map(move |b| (a, b)))
            .find(|&(a, b)| (4 * a * a * a + 27 * b * b) % p != 0 && p as i64 + 1 - points(p, a, b).len() as i64 != t)
            .unwrap();
        let mut bad = good.clone();
        let i = bad.iter().position(|x| x.1 == 7).unwrap();
        // the stranger is refused per curve: its point count contradicts the class's trace
        assert!(end_ring(p, oa, ob, t, 7, -3).unwrap_err().contains("point count"));
        // and with the class check alone: the stranger listed as a crater curve makes the class over
        bad[i] = (iso_key(p, oa, ob), 1);
        assert!(check_class(7, -3, &bad).is_err());
        // a dropped curve makes the class short, a duplicated key with another conductor is refused
        assert!(check_class(7, -3, &good[1..]).is_err());
        let mut dup = good.clone();
        dup.push((good[0].0, if good[0].1 == 1 { 7 } else { 1 }));
        assert!(check_class(7, -3, &dup).is_err());
    }

    /// Every F_p-isomorphism class of ordinary curves over F_p, with its trace.
    fn census(p: u64) -> Vec<(u64, u64, i64)> {
        let mut seen = vec![false; (p * p) as usize];
        let mut out = Vec::new();
        let mut is_sq = vec![false; p as usize];
        for y in 1..p {
            is_sq[(y * y % p) as usize] = true;
        }
        for a in 0..p {
            for b in 0..p {
                if seen[(a * p + b) as usize] || (4 * a * a * a + 27 * b * b) % p == 0 {
                    continue;
                }
                for u in 1..p {
                    seen[(a * powmod(u, 4, p) % p * p + b * powmod(u, 6, p) % p) as usize] = true;
                }
                let s: i64 = (0..p)
                    .map(|x| match (x * x % p * x + a * x + b) % p {
                        0 => 0,
                        r if is_sq[r as usize] => 1,
                        _ => -1,
                    })
                    .sum();
                if s.rem_euclid(p as i64) != 0 {
                    out.push((a, b, -s));
                }
            }
        }
        out
    }

    fn census_sweep(primes: impl Iterator<Item = u64>) -> BTreeMap<&'static str, u64> {
        let mut stats = BTreeMap::new();
        for p in primes.filter(|&p| is_prime(p)) {
            let mut classes: BTreeMap<i64, Vec<Member>> = BTreeMap::new();
            for (a, b, t) in census(p) {
                let (v, dk) = split_disc(t, p);
                let e = end_ring(p, a, b, t, v, dk).unwrap_or_else(|w| panic!("p {p} ({a},{b}): {w}"));
                *stats.entry("curves").or_default() += 1;
                let mut r = v;
                for l in [2, 3, 5] {
                    while r % l == 0 {
                        r /= l;
                    }
                }
                if r > 1 {
                    *stats.entry("curves_v_not_235_smooth").or_default() += 1;
                    *stats
                        .entry(if e.levels.iter().any(|x| x.ell > 5 && x.level > 0) {
                            "of_which_floor_at_some_l_ge_7"
                        } else {
                            "of_which_crater_at_every_l_ge_7"
                        })
                        .or_default() += 1;
                }
                classes.entry(t).or_default().push(((a, b), e.f));
            }
            for (t, m) in &classes {
                let (v, dk) = split_disc(*t, p);
                check_class(v, dk, m).unwrap_or_else(|e| panic!("p = {p}, t = {t}: {e}"));
                *stats.entry("classes").or_default() += 1;
            }
        }
        stats
    }

    /// Deuring holds for every class over two fields with 7, 11, 13 | v cases.
    #[test]
    fn census_classes_over_mid_fields() {
        let s = census_sweep([197u64, 251].into_iter());
        assert!(s["curves_v_not_235_smooth"] > 0, "{s:?}");
    }

    /// The whole census-fp range (5 <= p < 2^10) without a database:
    /// `cargo test --release -- --ignored full_census_sweep --nocapture`.
    #[test]
    #[ignore]
    fn full_census_sweep() {
        let s = census_sweep(5..MAX_P);
        eprintln!("{s:?}");
    }
}
