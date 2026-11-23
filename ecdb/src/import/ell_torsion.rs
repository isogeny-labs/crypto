//! Frobenius on E[l] for every short-Weierstrass curve over F_p,
//! 5 <= p < 2^10 (the census-fp curves and every other stored curve over
//! those fields), for l in {2, 3, 5, 7, 11, 13}, l != p.
//!
//! # Mathematics
//!
//! Frobenius pi acts on E[l] ≅ F_l^2 with characteristic polynomial
//! X^2 - (t mod l) X + (p mod l). Its conjugacy class in GL_2(F_l) is fixed by
//! that polynomial except at a double root lambda = t/2, where pi is either
//! the scalar lambda or a Jordan block. A matrix M of that class determines
//! everything stored here:
//!
//! * `frobenius_action`: `split` (two distinct eigenvalues in F_l), `inert`
//!   (irreducible), `scalar` or `jordan`;
//! * `rational_isogenies`: the F_p-rational l-isogenies are the M-stable lines
//!   (2, 0, l + 1, 1 in the four cases);
//! * `torsion_point_degree`: least k with M^k v = v for some v != 0;
//! * `full_torsion_degree`: the order of M;
//! * `isogeny_field_degree`: least k with M^k L = L for some line L;
//! * `all_isogenies_degree`: least k with M^k a scalar;
//! * `division_poly_factor_degrees`: x(P) for P in E[l] \ {0} are the roots of
//!   psi_l (the cubic x^3 + A x + B for l = 2), and Frobenius permutes them as M
//!   permutes (F_l^2 \ {0}) / ±1, so each orbit of size k is one irreducible
//!   factor of degree k. Written as `1^2 2 4^2` (ascending degree^multiplicity,
//!   the traits.json format).
//!
//! # How each value is obtained and checked
//!
//! * psi_l is factored over F_p by distinct-degree factorization (deterministic,
//!   exact), after checking it is squarefree.
//! * When the double-root case arises, the scalar test of `end_ring.rs`
//!   (x^p ≡ x([lambda]P) mod psi_l) decides scalar against Jordan.
//! * The orbit structure of M predicted from (t mod l, p mod l, scalar) must
//!   equal the factor degrees found by the factorization. The scalar and the
//!   Jordan block give different patterns (a Jordan block has an orbit whose
//!   size is divisible by l), so the two methods confirm each other, and a
//!   wrong trace residue almost always contradicts the factorization.
//! * Identities: the recorded trace is recounted from the model; the point
//!   degree equals the least k with l | #E(F_{p^k}) (traces by the recurrence
//!   t_{k+1} = t t_k - p t_{k-1}); p^k ≡ 1 (mod l) at the full-torsion degree
//!   k (Weil pairing), so full rational l-torsion needs p ≡ 1 (mod l) and
//!   l^2 | #E(F_p); scalar implies l + 1 isogenies; the isogeny count matches
//!   the action.
//! * Against data ecdb holds: for l = 2, 3, 5 the count equals the out-degree
//!   of the complete census Vélu graph (census-fp curves, or any curve that is
//!   the source of a Vélu edge over these fields); with v_l(v) = 0 Frobenius is
//!   never a scalar on E[l] (Kohel), and with v_l(v) >= 1 it is a scalar
//!   exactly when the stored volcano level is below the depth; a stored
//!   `curve_ell_torsion` or `frobenius_ell` row must agree in every non-null
//!   column.
//!
//! A (curve, l) that fails a check is a reported problem and is not inserted.
//!
//! # Writes
//!
//! `curve_ell_torsion` (status `proved`) and per isogeny class `frobenius_ell`
//! (depth v_l(v), splitting of l in Q(pi) by the Kronecker symbol (D_K/l), the
//! isogeny count 1 + (D_K/l) at depth 0, and the characteristic polynomial
//! columns of migration 0013; depth, splitting and count stay NULL for
//! supersingular classes). Insert only (`ON CONFLICT DO NOTHING`): a stored row
//! is checked, never changed. A rerun inserts nothing.

use super::end_ring::{division_polys, is_fundamental, is_prime, monic, powmod, scalar_on, trim};
use super::{bulk, bulk1, Ctx, Report};
use anyhow::Result;
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

/// The primes l covered.
pub const ELLS: [u64; 6] = [2, 3, 5, 7, 11, 13];
/// Fields covered: 5 <= p < MAX_P (the census-fp fields).
pub const MAX_P: u64 = 1024;
/// The census Vélu graphs are complete for these l.
const GRAPH_ELLS: [u64; 3] = [2, 3, 5];

const METHOD: &str = "Frobenius on E[l]: class in GL_2(F_l) from X^2 - tX + p mod l and the scalar test \
                      x^p = x([t/2]P) mod psi_l; psi_l factored by distinct-degree factorization and checked \
                      against the predicted orbit structure; counts checked against the census Velu graph";

const CURVES_SQL: &str = "
SELECT c.id, coalesce(c.family, ''), ic.id, ic.p::int8, c.coefficients->>'a', c.coefficients->>'b',
       fg.trace::int8, fg.conductor_v::int8, fg.cm_discriminant::int8,
       coalesce(fg.is_supersingular, false) OR mod(fg.trace, ic.p) = 0
FROM ec.curve c
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
JOIN ec.frobenius_genus1 fg ON fg.isogeny_class_id = ic.id
WHERE c.genus = 1 AND c.model = 'short_weierstrass' AND ic.g = 1 AND ic.q = ic.p AND ic.p >= 5 AND ic.p < $1::int8
  AND fg.trace IS NOT NULL
ORDER BY ic.id, c.id";

const OUT_DEGREE_SQL: &str = "
SELECT i.source_curve_id, i.ell::int8, count(*)::int8
FROM ec.isogeny i
JOIN ec.abelian_variety av ON av.curve_id = i.source_curve_id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
WHERE ic.p < $1::int8 AND i.formula = 'velu' AND i.ell IN (2, 3, 5)
GROUP BY 1, 2";

const LEVELS_SQL: &str = "
SELECT el.curve_id, el.ell::int8, el.level
FROM ec.endomorphism_level el
JOIN ec.abelian_variety av ON av.curve_id = el.curve_id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
WHERE ic.p < $1::int8 AND el.level IS NOT NULL AND el.ell <= 13";

const EXISTING_TORSION_SQL: &str = "
SELECT t.curve_id, t.ell::int8, t.frobenius_action, t.rational_isogenies, t.torsion_point_degree,
       t.full_torsion_degree, t.isogeny_field_degree, t.all_isogenies_degree, t.division_poly_factor_degrees
FROM ec.curve_ell_torsion t
JOIN ec.abelian_variety av ON av.curve_id = t.curve_id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
WHERE ic.p < $1::int8 AND t.ell <= 13";

const EXISTING_FROB_ELL_SQL: &str = "
SELECT fe.isogeny_class_id, fe.ell::int8, fe.volcano_depth, fe.splitting, fe.rational_isogenies,
       fe.frobenius_trace_mod_ell, fe.frobenius_det_mod_ell
FROM ec.frobenius_ell fe
JOIN ec.isogeny_class_fq ic ON ic.id = fe.isogeny_class_id
WHERE ic.p < $1::int8 AND fe.ell <= 13";

const TORSION_SQL: &str = "
INSERT INTO ec.curve_ell_torsion (curve_id, ell, frobenius_action, rational_isogenies, torsion_point_degree,
       full_torsion_degree, isogeny_field_degree, all_isogenies_degree, division_poly_factor_degrees, status,
       provenance_id)
SELECT r.curve_id, r.ell, r.action, r.rational_isogenies, r.point_degree, r.full_degree, r.isogeny_degree,
       r.all_isogenies_degree, r.psi_degrees, 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, ell numeric, action text, rational_isogenies int,
     point_degree int, full_degree int, isogeny_degree int, all_isogenies_degree int, psi_degrees text)
ON CONFLICT (curve_id, ell) DO NOTHING";

/// Fill the NULL columns of a stored row that agreed with every computed column. The guard
/// repeats the agreement, so a row changed since it was read is left alone (and reported).
const TORSION_FILL_SQL: &str = "
UPDATE ec.curve_ell_torsion t SET
       frobenius_action = coalesce(t.frobenius_action, r.action),
       rational_isogenies = coalesce(t.rational_isogenies, r.rational_isogenies),
       torsion_point_degree = coalesce(t.torsion_point_degree, r.point_degree),
       full_torsion_degree = coalesce(t.full_torsion_degree, r.full_degree),
       isogeny_field_degree = coalesce(t.isogeny_field_degree, r.isogeny_degree),
       all_isogenies_degree = coalesce(t.all_isogenies_degree, r.all_isogenies_degree),
       division_poly_factor_degrees = coalesce(t.division_poly_factor_degrees, r.psi_degrees)
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, ell numeric, action text, rational_isogenies int,
     point_degree int, full_degree int, isogeny_degree int, all_isogenies_degree int, psi_degrees text)
WHERE t.curve_id = r.curve_id AND t.ell = r.ell
  AND (t.frobenius_action IS NULL OR t.frobenius_action = r.action
       OR (r.action = 'jordan' AND t.frobenius_action = 'ramified'))
  AND (t.rational_isogenies IS NULL OR t.rational_isogenies = r.rational_isogenies)
  AND (t.torsion_point_degree IS NULL OR t.torsion_point_degree = r.point_degree)
  AND (t.full_torsion_degree IS NULL OR t.full_torsion_degree = r.full_degree)
  AND (t.isogeny_field_degree IS NULL OR t.isogeny_field_degree = r.isogeny_degree)
  AND (t.all_isogenies_degree IS NULL OR t.all_isogenies_degree = r.all_isogenies_degree)
  AND (t.division_poly_factor_degrees IS NULL OR t.division_poly_factor_degrees = r.psi_degrees)
  AND (t.frobenius_action IS NULL OR t.rational_isogenies IS NULL OR t.torsion_point_degree IS NULL
       OR t.full_torsion_degree IS NULL OR t.isogeny_field_degree IS NULL OR t.all_isogenies_degree IS NULL
       OR t.division_poly_factor_degrees IS NULL)";

const FROB_ELL_SQL: &str = "
INSERT INTO ec.frobenius_ell (isogeny_class_id, ell, volcano_depth, splitting, rational_isogenies,
       frobenius_trace_mod_ell, frobenius_det_mod_ell, status, provenance_id)
SELECT r.class_id, r.ell, r.depth, r.splitting, r.rational_isogenies, r.trace_mod, r.det_mod, 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(class_id bigint, ell numeric, depth int, splitting text,
     rational_isogenies int, trace_mod int, det_mod int)
ON CONFLICT (isogeny_class_id, ell) DO NOTHING";

// ------------------------------------------------------------- polynomials over F_p, p < 2^16

type Poly = Vec<u64>; // ascending coefficients in [0, p)

fn psub(a: &[u64], b: &[u64], p: u64) -> Poly {
    let n = a.len().max(b.len());
    trim((0..n).map(|i| (a.get(i).copied().unwrap_or(0) + p - b.get(i).copied().unwrap_or(0)) % p).collect())
}

fn deg(a: &[u64]) -> usize {
    a.len().saturating_sub(1)
}

/// Lazy reduction modulo a monic g of degree n >= 1: p < 2^16, so every
/// entry stays below n p^2 + (len) p^2 < 2^41 before the final `% p`.
fn reduce(mut buf: Vec<u64>, g: &[u64], p: u64) -> Poly {
    let n = g.len() - 1;
    if buf.len() > n {
        for k in (n..buf.len()).rev() {
            let c = buf[k] % p;
            if c != 0 {
                let m = p - c;
                let base = k - n;
                for (i, &gi) in g[..n].iter().enumerate() {
                    buf[base + i] += m * gi;
                }
            }
        }
        buf.truncate(n);
    }
    buf.iter_mut().for_each(|x| *x %= p);
    trim(buf)
}

/// a b mod g (a, b reduced mod g, g monic).
fn mulmod(a: &[u64], b: &[u64], g: &[u64], p: u64) -> Poly {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut buf = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            buf[i + j] += x * y;
        }
    }
    reduce(buf, g, p)
}

/// x^e mod g.
fn xpow(mut e: u64, g: &[u64], p: u64) -> Poly {
    let mut r = reduce(vec![1], g, p);
    let mut b = reduce(vec![0, 1], g, p);
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(&r, &b, g, p);
        }
        b = mulmod(&b, &b, g, p);
        e >>= 1;
    }
    r
}

/// Monic gcd.
fn pgcd(a: &[u64], b: &[u64], p: u64) -> Poly {
    let (mut a, mut b) = (trim(a.to_vec()), trim(b.to_vec()));
    while !b.is_empty() {
        let bm = monic(&b, p);
        let r = if bm.len() == 1 { Vec::new() } else { reduce(a, &bm, p) };
        a = bm;
        b = r;
    }
    if a.is_empty() {
        a
    } else {
        monic(&a, p)
    }
}

/// a / g for a monic divisor g; None when g does not divide a.
fn pdiv(a: &[u64], g: &[u64], p: u64) -> Option<Poly> {
    let dg = deg(g);
    if a.len() < g.len() {
        return None;
    }
    let mut r = a.to_vec();
    let mut q = vec![0u64; r.len() - dg];
    for k in (dg..r.len()).rev() {
        let c = r[k] % p;
        q[k - dg] = c;
        if c != 0 {
            for (i, &gi) in g.iter().enumerate() {
                r[k - dg + i] = (r[k - dg + i] + (p - c) * gi) % p;
            }
        }
    }
    trim(r[..dg].to_vec()).is_empty().then(|| trim(q))
}

fn derivative(a: &[u64], p: u64) -> Poly {
    trim(a.iter().enumerate().skip(1).map(|(i, &c)| (i as u64 % p) * c % p).collect())
}

/// Degrees of the irreducible factors of a squarefree f over F_p (degree ->
/// multiplicity), by distinct-degree factorization with the Frobenius matrix
/// of F_p[x]/(f).
pub fn factor_degrees(f: &[u64], p: u64) -> Result<BTreeMap<u64, u64>, String> {
    let f = monic(&trim(f.to_vec()), p);
    let n = deg(&f);
    let mut out = BTreeMap::new();
    if n == 0 {
        return Ok(out);
    }
    if pgcd(&f, &derivative(&f, p), p) != vec![1] {
        return Err("the division polynomial is not squarefree".into());
    }
    let xp = xpow(p, &f, p);
    // q[j] = x^(p j) mod f
    let mut q: Vec<Poly> = Vec::with_capacity(n);
    let mut cur = reduce(vec![1], &f, p);
    for _ in 0..n {
        let next = mulmod(&cur, &xp, &f, p);
        q.push(cur);
        cur = next;
    }
    let x = reduce(vec![0, 1], &f, p);
    let mut h = x.clone();
    let mut rem = f.clone();
    let mut i = 0u64;
    while 2 * (i as usize + 1) <= deg(&rem) {
        i += 1;
        // h <- h^p = sum_j h_j x^(p j)
        let mut acc = vec![0u64; n];
        for (j, &hj) in h.iter().enumerate() {
            if hj != 0 {
                for (k, &c) in q[j].iter().enumerate() {
                    acc[k] += hj * c;
                }
            }
        }
        acc.iter_mut().for_each(|c| *c %= p);
        h = trim(acc);
        let g = pgcd(&rem, &psub(&h, &x, p), p);
        let d = deg(&g) as u64;
        if d > 0 {
            if !d.is_multiple_of(i) {
                return Err(format!("distinct-degree factor of degree {d} is not a multiple of {i}"));
            }
            *out.entry(i).or_default() += d / i;
            rem = pdiv(&rem, &g, p).ok_or("distinct-degree factor does not divide")?;
        }
    }
    if deg(&rem) > 0 {
        *out.entry(deg(&rem) as u64).or_default() += 1;
    }
    let total: u64 = out.iter().map(|(d, m)| d * m).sum();
    if total != n as u64 {
        return Err(format!("factor degrees sum to {total}, not {n}"));
    }
    Ok(out)
}

/// `1^2 2 4^2`: ascending degree, `^multiplicity` when above 1.
pub fn degrees_string(d: &BTreeMap<u64, u64>) -> String {
    d.iter().map(|(k, m)| if *m == 1 { k.to_string() } else { format!("{k}^{m}") }).collect::<Vec<_>>().join(" ")
}

// ------------------------------------------------------------- Frobenius as a matrix over F_l

type M2 = [u64; 4]; // [a b; c d], row-major

fn mmul(x: &M2, y: &M2, l: u64) -> M2 {
    [
        (x[0] * y[0] + x[1] * y[2]) % l,
        (x[0] * y[1] + x[1] * y[3]) % l,
        (x[2] * y[0] + x[3] * y[2]) % l,
        (x[2] * y[1] + x[3] * y[3]) % l,
    ]
}

fn mvec(m: &M2, v: (u64, u64), l: u64) -> (u64, u64) {
    ((m[0] * v.0 + m[1] * v.1) % l, (m[2] * v.0 + m[3] * v.1) % l)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    Split,
    Inert,
    Scalar,
    Jordan,
}

impl Action {
    pub fn as_str(self) -> &'static str {
        match self {
            Action::Split => "split",
            Action::Inert => "inert",
            Action::Scalar => "scalar",
            Action::Jordan => "jordan",
        }
    }
}

/// What the conjugacy class of Frobenius on E[l] determines.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prediction {
    pub action: Action,
    pub rational_isogenies: u64,
    pub point_degree: u64,
    pub full_degree: u64,
    pub isogeny_degree: u64,
    pub all_isogenies_degree: u64,
    /// Orbits of Frobenius on (E[l] \ {0}) / ±1: degree -> count.
    pub psi_degrees: BTreeMap<u64, u64>,
}

/// Whether X^2 - t X + d has a double root mod l.
pub fn double_root(l: u64, tm: u64, dm: u64) -> bool {
    if l == 2 {
        tm.is_multiple_of(2)
    } else {
        (tm * tm + 4 * (l - dm % l)).is_multiple_of(l)
    }
}

/// The class of Frobenius from t mod l, p mod l and, at a double root,
/// whether it is the scalar.
pub fn predict(l: u64, tm: u64, dm: u64, scalar: bool) -> Prediction {
    let dbl = double_root(l, tm, dm);
    assert!(!scalar || dbl, "scalar needs a double root");
    let action = if dbl {
        if scalar {
            Action::Scalar
        } else {
            Action::Jordan
        }
    } else if l == 2 {
        Action::Inert
    } else {
        let disc = (tm * tm + 4 * (l - dm)) % l;
        if powmod(disc, (l - 1) / 2, l) == 1 {
            Action::Split
        } else {
            Action::Inert
        }
    };
    // a representative: the scalar, or the companion matrix (every
    // non-scalar 2x2 matrix is cyclic, so it is conjugate to its companion)
    let m: M2 = if scalar {
        let lam = if l == 2 { 1 } else { tm * l.div_ceil(2) % l };
        [lam, 0, 0, lam]
    } else {
        [0, (l - dm % l) % l, 1, tm % l]
    };
    let mut pows: Vec<M2> = vec![[1, 0, 0, 1]];
    loop {
        let next = mmul(pows.last().unwrap(), &m, l);
        pows.push(next);
        if next == [1, 0, 0, 1] {
            break;
        }
        assert!(pows.len() <= (l * l) as usize, "Frobenius is not invertible mod l");
    }
    let full_degree = (pows.len() - 1) as u64;
    let all_isogenies_degree =
        (1..pows.len()).find(|&k| pows[k][1] == 0 && pows[k][2] == 0 && pows[k][0] == pows[k][3]).unwrap() as u64;
    let neg = |v: (u64, u64)| ((l - v.0) % l, (l - v.1) % l);
    let mut point_degree = u64::MAX;
    let mut xcount: BTreeMap<u64, u64> = BTreeMap::new();
    for a in 0..l {
        for b in 0..l {
            if (a, b) == (0, 0) {
                continue;
            }
            let v = (a, b);
            let kp = (1..pows.len()).find(|&k| mvec(&pows[k], v, l) == v).unwrap() as u64;
            let kx = (1..pows.len())
                .find(|&k| {
                    let w = mvec(&pows[k], v, l);
                    w == v || w == neg(v)
                })
                .unwrap() as u64;
            point_degree = point_degree.min(kp);
            *xcount.entry(kx).or_default() += 1;
        }
    }
    let per_class = if l == 2 { 1 } else { 2 };
    let psi_degrees = xcount
        .into_iter()
        .map(|(k, c)| {
            assert!(c % (per_class * k) == 0);
            (k, c / per_class / k)
        })
        .collect();
    let lines: Vec<(u64, u64)> = (0..l).map(|y| (1, y)).chain([(0, 1)]).collect();
    let mut rational_isogenies = 0;
    let mut isogeny_degree = u64::MAX;
    for v in lines {
        let k = (1..pows.len())
            .find(|&k| {
                let w = mvec(&pows[k], v, l);
                (v.0 * w.1 + l * l - v.1 * w.0).is_multiple_of(l)
            })
            .unwrap() as u64;
        rational_isogenies += (k == 1) as u64;
        isogeny_degree = isogeny_degree.min(k);
    }
    Prediction {
        action,
        rational_isogenies,
        point_degree,
        full_degree,
        isogeny_degree,
        all_isogenies_degree,
        psi_degrees,
    }
}

/// One computed (curve, l).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Torsion {
    pub ell: u64,
    pub trace_mod: u64,
    pub det_mod: u64,
    pub pred: Prediction,
}

/// The division polynomial in x whose roots are the x(P), P in E[l] \ {0}
/// (l = 2: x^3 + A x + B), monic.
fn psi_x(l: u64, a: u64, b: u64, p: u64) -> Poly {
    if l == 2 {
        trim(vec![b, a, 0, 1])
    } else {
        let fs = division_polys(l.max(4) as usize, a, b, p);
        monic(&fs[l as usize], p)
    }
}

/// Frobenius on E[l] for y^2 = x^3 + a x + b over F_p with trace t (inputs
/// assumed validated by [`curve_torsion`]; l != p). Every per-l check of the
/// module documentation is enforced; a failure is the reason.
pub fn ell_torsion(p: u64, a: u64, b: u64, t: i64, l: u64) -> Result<Torsion, String> {
    let tm = t.rem_euclid(l as i64) as u64;
    let dm = p % l;
    let scalar = if double_root(l, tm, dm) {
        let lam = if l == 2 { 1 } else { tm * l.div_ceil(2) % l };
        let fs = division_polys(l.max(4) as usize, a, b, p);
        let big_f = trim(vec![b, a, 0, 1]);
        scalar_on(l, lam, &mut BTreeMap::new(), &fs, &big_f, p)
    } else {
        false
    };
    let pred = predict(l, tm, dm, scalar);
    let found = factor_degrees(&psi_x(l, a, b, p), p)?;
    if found != pred.psi_degrees {
        return Err(format!(
            "l = {l}: psi_l factors as [{}], Frobenius {} with X^2 - {tm}X + {dm} predicts [{}]",
            degrees_string(&found),
            pred.action.as_str(),
            degrees_string(&pred.psi_degrees)
        ));
    }
    // identities
    let want_isog = match pred.action {
        Action::Split => 2,
        Action::Inert => 0,
        Action::Jordan => 1,
        Action::Scalar => l + 1,
    };
    if pred.rational_isogenies != want_isog {
        return Err(format!("l = {l}: {} stable lines for a {} action", pred.rational_isogenies, pred.action.as_str()));
    }
    // least k with l | #E(F_{p^k}) = p^k + 1 - t_k, all mod l
    let li = l as i64;
    let (pm, t1) = ((p % l) as i64, t.rem_euclid(li));
    let (mut t_prev, mut t_cur, mut pk) = (2 % li, t1, pm);
    let mut k_count = None;
    for k in 1..=(l * l) {
        if (pk + 1 - t_cur).rem_euclid(li) == 0 {
            k_count = Some(k);
            break;
        }
        let t_next = (t1 * t_cur - pm * t_prev).rem_euclid(li);
        t_prev = t_cur;
        t_cur = t_next;
        pk = pk * pm % li;
    }
    if k_count != Some(pred.point_degree) {
        return Err(format!(
            "l = {l}: torsion point degree {} but the point counts give {k_count:?}",
            pred.point_degree
        ));
    }
    if powmod(p % l, pred.full_degree, l) != 1 % l {
        return Err(format!("l = {l}: E[l] over F_p^{} but p^k != 1 mod l", pred.full_degree));
    }
    if pred.full_degree == 1 {
        let order = (p as i64 + 1 - t) as u64;
        if p % l != 1 || !order.is_multiple_of(l * l) {
            return Err(format!("l = {l}: full rational l-torsion but p mod l = {} and #E = {order}", p % l));
        }
    }
    Ok(Torsion { ell: l, trace_mod: tm, det_mod: dm, pred })
}

/// Every l in [`ELLS`] other than p, after validating the curve: p a prime in
/// [5, 2^16), a nonsingular model, the trace in the Hasse interval and equal
/// to the point count.
pub fn curve_torsion(p: u64, a: u64, b: u64, t: i64) -> Result<Vec<Torsion>, String> {
    if !(5..1 << 16).contains(&p) || !is_prime(p) {
        return Err(format!("p = {p} is not a prime in [5, 2^16)"));
    }
    let (a, b) = (a % p, b % p);
    if (4 * powmod(a, 3, p) + 27 * b % p * b).is_multiple_of(p) {
        return Err("singular model".into());
    }
    if t * t > 4 * p as i64 {
        return Err(format!("t = {t} is outside the Hasse interval"));
    }
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
    ELLS.iter().filter(|&&l| l != p).map(|&l| ell_torsion(p, a, b, t, l)).collect()
}

/// (v, D_K) of an ordinary class when the stored values satisfy
/// t^2 - 4p = v^2 D_K with v > 0 and D_K a negative fundamental discriminant;
/// otherwise None (the depth v_l(v) and the splitting are then unknown). Without
/// the fundamental check a stored (v / m, m^2 D_K) would satisfy the identity
/// and give a wrong depth and splitting.
pub fn verified_vd(supersingular: bool, t: i64, p: u64, v: Option<i64>, d_k: Option<i64>) -> Option<(u64, i64)> {
    if supersingular {
        return None;
    }
    let (v, d) = (v?, d_k?);
    (v > 0 && is_fundamental(d) && t * t - 4 * p as i64 == v * v * d).then_some((v as u64, d))
}

/// v_l(n), n > 0.
fn val(mut n: u64, l: u64) -> u32 {
    let mut k = 0;
    while n > 0 && n.is_multiple_of(l) {
        n /= l;
        k += 1;
    }
    k
}

/// The Kronecker symbol (d/l) for a discriminant d.
pub fn kronecker(d: i64, l: u64) -> i64 {
    if l == 2 {
        match d.rem_euclid(8) {
            1 | 7 => 1,
            3 | 5 => -1,
            _ => 0,
        }
    } else {
        match powmod(d.rem_euclid(l as i64) as u64, (l - 1) / 2, l) {
            0 => 0,
            1 => 1,
            _ => -1,
        }
    }
}

/// What ecdb already holds about one (curve, l), for the cross-checks.
#[derive(Clone, Debug, Default)]
pub struct Known {
    /// Out-degree in the census Vélu graph, when that graph is complete at this curve.
    pub out_degree: Option<u64>,
    /// v_l(v) for an ordinary class with a verified t^2 - 4p = v^2 D_K.
    pub depth: Option<u32>,
    pub level: Option<i32>,
    pub existing: Option<ExistingRow>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExistingRow {
    pub action: Option<String>,
    pub rational_isogenies: Option<i32>,
    pub point_degree: Option<i32>,
    pub full_degree: Option<i32>,
    pub isogeny_degree: Option<i32>,
    pub all_isogenies_degree: Option<i32>,
    pub psi_degrees: Option<String>,
}

impl ExistingRow {
    /// Whether a column this importer computes is still NULL in the stored row.
    pub fn has_null(&self) -> bool {
        self.action.is_none()
            || self.rational_isogenies.is_none()
            || self.point_degree.is_none()
            || self.full_degree.is_none()
            || self.isogeny_degree.is_none()
            || self.all_isogenies_degree.is_none()
            || self.psi_degrees.is_none()
    }
}

/// The cross-checks against stored data; Ok(true) when a stored row agreed.
pub fn cross_check(tor: &Torsion, known: &Known) -> Result<bool, String> {
    let l = tor.ell;
    let pr = &tor.pred;
    if let Some(d) = known.out_degree {
        if d != pr.rational_isogenies {
            return Err(format!(
                "l = {l}: {} rational isogenies but out-degree {d} in the census graph",
                pr.rational_isogenies
            ));
        }
    }
    match known.depth {
        Some(0) if pr.action == Action::Scalar => {
            return Err(format!("l = {l}: scalar on E[l] but l does not divide v (Kohel)"));
        }
        Some(d) if d >= 1 => {
            if !matches!(pr.action, Action::Scalar | Action::Jordan) {
                return Err(format!("l = {l}: l | v but Frobenius is {}", pr.action.as_str()));
            }
            if let Some(lv) = known.level {
                if (lv < d as i32) != (pr.action == Action::Scalar) {
                    return Err(format!(
                        "l = {l}: stored level {lv} of depth {d} but Frobenius is {}",
                        pr.action.as_str()
                    ));
                }
            }
        }
        _ => {}
    }
    let Some(e) = &known.existing else { return Ok(false) };
    let int = |x: Option<i32>, y: u64, what: &str| match x {
        Some(x) if x as i64 != y as i64 => Err(format!("l = {l}: stored {what} {x}, ecdb computes {y}")),
        _ => Ok(()),
    };
    if let Some(a) = &e.action {
        // ell_isogenies.rs labels a Jordan block with l not dividing v `ramified`; this importer
        // (like DiSSECT) calls every Jordan block `jordan`. Accept the finer label only when it is
        // consistent with the verified depth.
        let ramified_ok = a == "ramified" && pr.action == Action::Jordan && !known.depth.is_some_and(|d| d >= 1);
        if a != pr.action.as_str() && !ramified_ok {
            return Err(format!("l = {l}: stored action {a}, ecdb computes {}", pr.action.as_str()));
        }
    }
    int(e.rational_isogenies, pr.rational_isogenies, "rational_isogenies")?;
    int(e.point_degree, pr.point_degree, "torsion_point_degree")?;
    int(e.full_degree, pr.full_degree, "full_torsion_degree")?;
    int(e.isogeny_degree, pr.isogeny_degree, "isogeny_field_degree")?;
    int(e.all_isogenies_degree, pr.all_isogenies_degree, "all_isogenies_degree")?;
    if let Some(s) = &e.psi_degrees {
        let mine = degrees_string(&pr.psi_degrees);
        if *s != mine {
            return Err(format!("l = {l}: stored psi_l degrees {s}, ecdb computes {mine}"));
        }
    }
    Ok(true)
}

// ------------------------------------------------------------- import

struct Row {
    id: i64,
    census: bool,
    class_id: i64,
    p: u64,
    a: Option<u64>,
    b: Option<u64>,
    t: i64,
    v: Option<i64>,
    d_k: Option<i64>,
    supersingular: bool,
}

type FrobEllRow = (Option<i32>, Option<String>, Option<i32>, Option<i32>, Option<i32>);

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let mut report = Report {
        input: format!("short-Weierstrass curves over F_p, 5 <= p < {MAX_P} (stored ec.curve), l in {ELLS:?}"),
        ..Default::default()
    };
    let client = ctx.client;
    let src = ctx
        .source(
            "ecdb/src/import/ell_torsion.rs",
            &crate::math::sha256_hex(include_str!("ell_torsion.rs")),
            "Frobenius on E[l], l <= 13, of every census curve, computed by ecdb",
        )
        .await?;
    let prov = ctx.provenance(src, METHOD, None).await?;
    let max_p = MAX_P as i64;

    let rows: Vec<Row> = client
        .query(CURVES_SQL, &[&max_p])
        .await?
        .iter()
        .map(|r| {
            let coeff = |i: usize| r.get::<_, Option<String>>(i).and_then(|s| s.parse::<u64>().ok());
            Row {
                id: r.get(0),
                census: r.get::<_, String>(1) == "census-fp",
                class_id: r.get(2),
                p: r.get::<_, i64>(3) as u64,
                a: coeff(4),
                b: coeff(5),
                t: r.get(6),
                v: r.get(7),
                d_k: r.get(8),
                supersingular: r.get(9),
            }
        })
        .collect();
    let mut out_deg: BTreeMap<(i64, u64), u64> = BTreeMap::new();
    let mut has_velu: BTreeSet<i64> = BTreeSet::new();
    for r in client.query(OUT_DEGREE_SQL, &[&max_p]).await? {
        out_deg.insert((r.get(0), r.get::<_, i64>(1) as u64), r.get::<_, i64>(2) as u64);
        has_velu.insert(r.get(0));
    }
    let levels: BTreeMap<(i64, u64), i32> = client
        .query(LEVELS_SQL, &[&max_p])
        .await?
        .iter()
        .map(|r| ((r.get(0), r.get::<_, i64>(1) as u64), r.get(2)))
        .collect();
    let existing: BTreeMap<(i64, u64), ExistingRow> = client
        .query(EXISTING_TORSION_SQL, &[&max_p])
        .await?
        .iter()
        .map(|r| {
            (
                (r.get(0), r.get::<_, i64>(1) as u64),
                ExistingRow {
                    action: r.get(2),
                    rational_isogenies: r.get(3),
                    point_degree: r.get(4),
                    full_degree: r.get(5),
                    isogeny_degree: r.get(6),
                    all_isogenies_degree: r.get(7),
                    psi_degrees: r.get(8),
                },
            )
        })
        .collect();
    let existing_fe: BTreeMap<(i64, u64), FrobEllRow> = client
        .query(EXISTING_FROB_ELL_SQL, &[&max_p])
        .await?
        .iter()
        .map(|r| ((r.get(0), r.get::<_, i64>(1) as u64), (r.get(2), r.get(3), r.get(4), r.get(5), r.get(6))))
        .collect();

    // the computation, in parallel (pure)
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get()).clamp(1, 8);
    let chunk = rows.len().div_ceil(threads).max(1);
    let computed: Vec<Result<Vec<Torsion>, String>> = std::thread::scope(|s| {
        let handles: Vec<_> = rows
            .chunks(chunk)
            .map(|part| {
                s.spawn(move || {
                    part.iter()
                        .map(|r| match (r.a, r.b) {
                            (Some(a), Some(b)) => curve_torsion(r.p, a, b, r.t),
                            _ => Err("coefficients a, b are not field elements".into()),
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles.into_iter().flat_map(|h| h.join().expect("ell_torsion worker panicked")).collect()
    });

    // per class: verified (v, D_K) for ordinary classes
    let class_vd = |r: &Row| -> Option<(u64, i64)> { verified_vd(r.supersingular, r.t, r.p, r.v, r.d_k) };

    let (mut ins_census, mut ins_other) = (Vec::new(), Vec::new());
    let mut fills = Vec::new();
    // class -> l -> (all members fine, the per-curve isogeny counts)
    let mut class_ok: BTreeMap<(i64, u64), (bool, Vec<u64>)> = BTreeMap::new();
    let mut class_rep: BTreeMap<i64, &Row> = BTreeMap::new();
    let mut bad_vd_classes: BTreeSet<i64> = BTreeSet::new();
    for (r, res) in rows.iter().zip(&computed) {
        report.count(if r.census { "ell_torsion_curves_census_fp" } else { "ell_torsion_curves_other" }, 1);
        class_rep.entry(r.class_id).or_insert(r);
        let vd = class_vd(r);
        if vd.is_none() && !r.supersingular {
            report.count("ell_torsion_ordinary_curves_without_verified_conductor", 1);
            if (r.v.is_some() || r.d_k.is_some()) && bad_vd_classes.insert(r.class_id) {
                report.problem(format!(
                    "class {} (p = {}, t = {}): stored v = {:?}, D_K = {:?} fail t^2 - 4p = v^2 D_K with D_K a \
                     negative fundamental discriminant; depth, splitting and count left NULL",
                    r.class_id, r.p, r.t, r.v, r.d_k
                ));
            }
        }
        let tors = match res {
            Ok(t) => t,
            Err(why) => {
                report.count("ell_torsion_curves_refused", 1);
                report.problem(format!("curve {} (p = {}, t = {}): {why}", r.id, r.p, r.t));
                for l in ELLS {
                    class_ok.entry((r.class_id, l)).or_insert((true, vec![])).0 = false;
                }
                continue;
            }
        };
        if ELLS.contains(&r.p) {
            report.count("ell_torsion_ell_equals_p_skipped", 1);
        }
        let graph_complete = r.census || has_velu.contains(&r.id);
        for tor in tors {
            let l = tor.ell;
            let known = Known {
                out_degree: (graph_complete && GRAPH_ELLS.contains(&l))
                    .then(|| out_deg.get(&(r.id, l)).copied().unwrap_or(0)),
                depth: vd.map(|(v, _)| val(v, l)),
                level: levels.get(&(r.id, l)).copied(),
                existing: existing.get(&(r.id, l)).cloned(),
            };
            let entry = class_ok.entry((r.class_id, l)).or_insert((true, vec![]));
            match cross_check(tor, &known) {
                Ok(agreed) => {
                    report.count("ell_torsion_pairs_ok", 1);
                    if known.out_degree.is_some() {
                        report.count(&format!("ell_torsion_graph_agree_l{l}"), 1);
                    }
                    if known.level.is_some() && known.depth.is_some_and(|d| d >= 1) {
                        report.count("ell_torsion_levels_agree", 1);
                    }
                    if agreed {
                        report.count("ell_torsion_existing_agree", 1);
                    }
                    entry.1.push(tor.pred.rational_isogenies);
                    let pr = &tor.pred;
                    let row = json!({
                        "curve_id": r.id, "ell": l, "action": pr.action.as_str(),
                        "rational_isogenies": pr.rational_isogenies, "point_degree": pr.point_degree,
                        "full_degree": pr.full_degree, "isogeny_degree": pr.isogeny_degree,
                        "all_isogenies_degree": pr.all_isogenies_degree,
                        "psi_degrees": degrees_string(&pr.psi_degrees),
                    });
                    if known.existing.as_ref().is_some_and(ExistingRow::has_null) {
                        fills.push(row.clone());
                    }
                    if known.existing.is_none() {
                        if r.census {
                            ins_census.push(row);
                        } else {
                            ins_other.push(row);
                        }
                    }
                }
                Err(why) => {
                    entry.0 = false;
                    report.count("ell_torsion_pairs_refused", 1);
                    report.problem(format!("curve {} (p = {}, t = {}): {why}", r.id, r.p, r.t));
                }
            }
        }
    }

    // frobenius_ell, one row per class and l
    let mut fe_rows = Vec::new();
    for (&(class_id, l), (ok, counts)) in &class_ok {
        let r = class_rep[&class_id];
        if l == r.p {
            continue;
        }
        let vd = class_vd(r);
        let depth = vd.map(|(v, _)| val(v, l) as i64);
        let chi = vd.map(|(_, d)| kronecker(d, l));
        let splitting = chi.map(|c| match c {
            1 => "split",
            -1 => "inert",
            _ => "ramified",
        });
        let isog = (depth == Some(0)).then(|| (1 + chi.unwrap()) as u64);
        let refuse = |report: &mut Report, why: String| {
            report.count("frobenius_ell_classes_refused", 1);
            report.problem(format!("class {class_id} (p = {}, t = {}), l = {l}: {why}", r.p, r.t));
        };
        if !ok {
            refuse(&mut report, "a member curve failed a check".into());
            continue;
        }
        if let Some(n) = isog {
            if let Some(bad) = counts.iter().find(|&&c| c != n) {
                refuse(&mut report, format!("a member has {bad} rational isogenies, 1 + (D_K/l) = {n}"));
                continue;
            }
        }
        let (tm, dm) = (r.t.rem_euclid(l as i64), (r.p % l) as i64);
        if let Some((ed, es, ei, et, edet)) = existing_fe.get(&(class_id, l)) {
            let mismatch = ed.is_some_and(|x| Some(x as i64) != depth)
                || es.as_ref().is_some_and(|x| Some(x.as_str()) != splitting)
                || ei.is_some_and(|x| Some(x as u64) != isog)
                || et.is_some_and(|x| x as i64 != tm)
                || edet.is_some_and(|x| x as i64 != dm);
            if mismatch {
                refuse(&mut report, format!("stored row {:?} disagrees", (ed, es, ei, et, edet)));
            } else {
                report.count("frobenius_ell_existing_agree", 1);
            }
            continue;
        }
        fe_rows.push(json!({
            "class_id": class_id, "ell": l, "depth": depth, "splitting": splitting, "rational_isogenies": isog,
            "trace_mod": tm, "det_mod": dm,
        }));
    }
    report.count("curve_ell_torsion_inserted_census_fp", bulk(client, TORSION_SQL, &ins_census, prov).await?);
    report.count("curve_ell_torsion_inserted_other", bulk(client, TORSION_SQL, &ins_other, prov).await?);
    // Rows that agreed but have NULL columns (e.g. ell_isogenies.rs stores only the action and
    // the count at l = 7, 11, 13): fill those columns. A row the guard turns away changed after
    // it was read; it is reported, not overwritten.
    let filled = bulk1(client, TORSION_FILL_SQL, &fills).await?;
    report.count("curve_ell_torsion_filled_null_columns", filled);
    if (filled as usize) < fills.len() {
        report.problem(format!(
            "curve_ell_torsion: {} agreeing rows with NULL columns were not filled (the stored row changed)",
            fills.len() - filled as usize
        ));
    }
    report.count("frobenius_ell_inserted", bulk(client, FROB_ELL_SQL, &fe_rows, prov).await?);
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    type Pt = Option<(u64, u64)>;

    fn inv(a: u64, p: u64) -> u64 {
        powmod(a, p - 2, p)
    }

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

    fn trace(p: u64, a: u64, b: u64) -> i64 {
        p as i64 + 1 - points(p, a, b).len() as i64
    }

    /// Number of roots of f in F_p, by evaluation.
    fn roots_mod(f: &[u64], p: u64) -> u64 {
        (0..p).filter(|&x| f.iter().rev().fold(0, |acc, c| (acc * x + c) % p) == 0).count() as u64
    }

    /// Tiny fields, every model (not only isomorphism-class representatives),
    /// against brute-force point arithmetic: a rational point of order l
    /// exists iff the point degree is 1; E[l] is rational iff the full degree
    /// is 1; the linear factors of psi_l are its roots in F_p; for l = 2 the
    /// rational isogenies are the rational 2-torsion points.
    #[test]
    fn brute_force_agreement_on_tiny_fields() {
        let mut seen_actions = BTreeSet::new();
        for p in [5u64, 7, 11, 13, 17, 19, 23, 29] {
            for a in 0..p {
                for b in 0..p {
                    if (4 * a * a * a + 27 * b * b) % p == 0 {
                        continue;
                    }
                    let pts = points(p, a, b);
                    let t = p as i64 + 1 - pts.len() as i64;
                    let tors = curve_torsion(p, a, b, t).unwrap_or_else(|e| panic!("p {p} ({a},{b}): {e}"));
                    for tor in &tors {
                        let l = tor.ell;
                        seen_actions.insert((l, tor.pred.action));
                        let ltors = pts.iter().filter(|q| mul(p, a, l, **q).is_none()).count() as u64;
                        // E(F_p)[l]: 1, l or l^2 points
                        assert_eq!(ltors > 1, tor.pred.point_degree == 1, "p {p} ({a},{b}) l {l}");
                        assert_eq!(ltors == l * l, tor.pred.full_degree == 1, "p {p} ({a},{b}) l {l}");
                        // rational x-coordinates of nonzero l-torsion = linear factors of psi_l
                        let xs: BTreeSet<u64> = pts
                            .iter()
                            .filter(|q| q.is_some() && mul(p, a, l, **q).is_none())
                            .map(|q| q.unwrap().0)
                            .collect();
                        let psi = psi_x(l, a, b, p);
                        let lin = tor.pred.psi_degrees.get(&1).copied().unwrap_or(0);
                        assert_eq!(roots_mod(&psi, p), lin, "p {p} ({a},{b}) l {l}");
                        assert!(xs.len() as u64 <= lin);
                        if l == 2 {
                            assert_eq!(tor.pred.rational_isogenies, lin);
                        }
                    }
                }
            }
        }
        // every action occurs
        for act in [Action::Split, Action::Inert, Action::Scalar, Action::Jordan] {
            assert!(seen_actions.iter().any(|(l, a)| *l > 2 && *a == act), "{act:?} never exercised");
        }
    }

    /// The number of rational l-isogenies equals the number of degree-(l-1)/2
    /// kernel polynomials: monic h | psi_l, deg h = (l-1)/2, whose roots are
    /// x(kP) for a point P of order l over the closure. Over tiny p we check it
    /// with the l = 3 case, where a kernel is a single root x0 of psi_3 (the
    /// subgroup {O, ±P}), i.e. the number of linear factors of psi_3.
    #[test]
    fn three_isogenies_are_linear_factors_of_psi3() {
        for p in [5u64, 7, 11, 13, 17, 19, 23, 29, 31] {
            for a in 0..p {
                for b in 0..p {
                    if (4 * a * a * a + 27 * b * b) % p == 0 {
                        continue;
                    }
                    let t = trace(p, a, b);
                    let tor = ell_torsion(p, a, b, t, 3).unwrap();
                    assert_eq!(
                        tor.pred.rational_isogenies,
                        tor.pred.psi_degrees.get(&1).copied().unwrap_or(0),
                        "p {p} ({a},{b})"
                    );
                }
            }
        }
    }

    /// The prime-field curves of docs/curves/traits.json (computed by
    /// src/cryptanalysis/curve_traits, imported into curve_ell_torsion), with
    /// their (p, a, b) from docs/curves/registry.json. Columns: l, action,
    /// psi degrees (None where traits leaves them out), isogenies, point,
    /// full, isogeny, all-isogenies degrees.
    #[allow(clippy::type_complexity)]
    const TRAITS: &[(u64, u64, u64, i64, &[(u64, &str, Option<&str>, u64, u64, u64, u64, u64)])] = &[
        // icv1-fp6-tm7-fc20da94
        (
            59,
            1,
            13,
            -7,
            &[
                (2, "inert", Some("3"), 0, 3, 3, 3, 3),
                (3, "inert", Some("4"), 0, 8, 8, 4, 4),
                (5, "inert", Some("6^2"), 0, 12, 12, 3, 3),
                (7, "split", None, 2, 3, 6, 1, 2),
                (11, "jordan", None, 1, 10, 110, 1, 11),
                (13, "inert", None, 0, 168, 168, 14, 14),
            ],
        ),
        // icv1-fp7-t4-0c09e92f
        (
            67,
            46,
            47,
            4,
            &[
                (2, "scalar", Some("1^3"), 3, 1, 1, 1, 1),
                (3, "scalar", Some("1^4"), 4, 2, 2, 1, 1),
                (5, "inert", Some("12"), 0, 24, 24, 6, 6),
                (7, "jordan", None, 1, 3, 21, 1, 7),
                (11, "split", None, 2, 10, 10, 1, 5),
                (13, "inert", None, 0, 168, 168, 14, 14),
            ],
        ),
        // icv1-fp7-tm5-7bc8be1d
        (
            101,
            3,
            17,
            -5,
            &[
                (2, "inert", Some("3"), 0, 3, 3, 3, 3),
                (3, "inert", Some("4"), 0, 8, 8, 4, 4),
                (5, "split", Some("2^6"), 2, 4, 4, 1, 2),
                (7, "inert", None, 0, 48, 48, 8, 8),
                (11, "inert", None, 0, 120, 120, 12, 12),
                (13, "inert", None, 0, 84, 84, 7, 7),
            ],
        ),
        // icv1-fp7-t2-4910f2d0
        (
            113,
            19,
            2,
            2,
            &[
                (2, "scalar", Some("1^3"), 3, 1, 1, 1, 1),
                (3, "inert", Some("4"), 0, 8, 8, 4, 4),
                (5, "inert", Some("12"), 0, 24, 24, 6, 6),
                (7, "jordan", None, 1, 1, 7, 1, 7),
                (11, "split", None, 2, 5, 5, 1, 5),
                (13, "inert", None, 0, 42, 42, 7, 7),
            ],
        ),
        // icv1-fp8-tm1-1ed3feb7
        (
            223,
            208,
            21,
            -1,
            &[
                (2, "inert", Some("3"), 0, 3, 3, 3, 3),
                (3, "scalar", Some("1^4"), 4, 1, 1, 1, 1),
                (5, "split", Some("1^2 2 4^2"), 2, 1, 4, 1, 4),
                (7, "inert", None, 0, 16, 16, 8, 8),
                (11, "jordan", None, 1, 5, 55, 1, 11),
                (13, "inert", None, 0, 168, 168, 14, 14),
            ],
        ),
        // icv1-fp10-t5-192cb216
        (
            827,
            1,
            15,
            5,
            &[
                (2, "inert", Some("3"), 0, 3, 3, 3, 3),
                (3, "inert", Some("4"), 0, 8, 8, 4, 4),
                (5, "inert", Some("4^3"), 0, 8, 8, 2, 2),
                (7, "scalar", None, 8, 2, 2, 1, 1),
                (11, "inert", None, 0, 120, 120, 12, 12),
                (13, "inert", None, 0, 56, 56, 14, 14),
            ],
        ),
        // icv1-fp10-tm3-e1039cbb
        (
            1009,
            1,
            14,
            -3,
            &[
                (2, "inert", Some("3"), 0, 3, 3, 3, 3),
                (3, "inert", Some("2^2"), 0, 4, 4, 2, 2),
                (5, "inert", Some("6^2"), 0, 12, 12, 3, 3),
                (7, "inert", None, 0, 8, 8, 4, 4),
                (11, "inert", None, 0, 120, 120, 12, 12),
                (13, "split", None, 2, 3, 12, 1, 12),
            ],
        ),
        // icv1-fp12-tm53-6c9ff4bb
        (
            3529,
            2574,
            2980,
            -53,
            &[
                (2, "inert", Some("3"), 0, 3, 3, 3, 3),
                (3, "jordan", Some("1 3"), 1, 2, 6, 1, 3),
                (5, "inert", Some("6^2"), 0, 12, 12, 3, 3),
                (7, "inert", None, 0, 8, 8, 4, 4),
                (11, "split", None, 2, 10, 10, 1, 5),
                (13, "split", None, 2, 4, 12, 1, 12),
            ],
        ),
    ];

    #[test]
    fn pinned_against_traits_json() {
        for &(p, a, b, t, rows) in TRAITS {
            let tors = curve_torsion(p, a, b, t).unwrap_or_else(|e| panic!("p {p}: {e}"));
            for &(l, act, psi, isog, pt, full, iso, all) in rows {
                let tor = tors.iter().find(|x| x.ell == l).unwrap();
                let pr = &tor.pred;
                assert_eq!(pr.action.as_str(), act, "p {p} l {l}");
                assert_eq!(
                    (
                        pr.rational_isogenies,
                        pr.point_degree,
                        pr.full_degree,
                        pr.isogeny_degree,
                        pr.all_isogenies_degree
                    ),
                    (isog, pt, full, iso, all),
                    "p {p} l {l}"
                );
                if let Some(s) = psi {
                    assert_eq!(degrees_string(&pr.psi_degrees), s, "p {p} l {l}");
                }
                // and the stored-row cross-check accepts the traits row
                let known = Known {
                    existing: Some(ExistingRow {
                        action: Some(act.into()),
                        rational_isogenies: Some(isog as i32),
                        point_degree: Some(pt as i32),
                        full_degree: Some(full as i32),
                        isogeny_degree: Some(iso as i32),
                        all_isogenies_degree: Some(all as i32),
                        psi_degrees: psi.map(String::from),
                    }),
                    ..Default::default()
                };
                assert_eq!(cross_check(tor, &known), Ok(true));
            }
        }
    }

    /// ell_isogenies.rs stores `ramified` for a Jordan block with l not dividing v; this importer
    /// says `jordan`. The finer label is accepted only when the depth does not contradict it.
    #[test]
    fn ramified_label_from_ell_isogenies() {
        let (p, a, b, t) = (3529u64, 2574u64, 2980u64, -53i64);
        let tor = curve_torsion(p, a, b, t).unwrap().into_iter().find(|x| x.ell == 3).unwrap();
        assert_eq!(tor.pred.action, Action::Jordan);
        let row = |act: &str| {
            Some(ExistingRow { action: Some(act.into()), rational_isogenies: Some(1), ..Default::default() })
        };
        let known = |act: &str, depth: Option<u32>| Known { existing: row(act), depth, ..Default::default() };
        assert_eq!(cross_check(&tor, &known("ramified", Some(0))), Ok(true));
        assert_eq!(cross_check(&tor, &known("ramified", None)), Ok(true));
        assert!(cross_check(&tor, &known("ramified", Some(1))).unwrap_err().contains("stored action"));
        assert!(cross_check(&tor, &known("split", Some(0))).unwrap_err().contains("stored action"));
        // a row with NULL columns is a fill candidate; a complete one is not
        assert!(row("ramified").unwrap().has_null());
        let full = ExistingRow {
            action: Some("jordan".into()),
            rational_isogenies: Some(1),
            point_degree: Some(1),
            full_degree: Some(1),
            isogeny_degree: Some(1),
            all_isogenies_degree: Some(1),
            psi_degrees: Some("1".into()),
        };
        assert!(!full.has_null());
    }

    /// y^2 = x^3 + 2x + 3 over F_101 has Sage's rational 2- and 3-isogenies
    /// (generated.rs test: kernels x + 66 for l = 3, a root of the cubic for l = 2).
    #[test]
    fn pinned_sage_isogenies_over_f101() {
        let t = trace(101, 2, 3);
        let tors = curve_torsion(101, 2, 3, t).unwrap();
        assert!(tors[0].pred.rational_isogenies >= 1);
        assert!(tors[1].pred.rational_isogenies >= 1);
        // psi_3(-66) = psi_3(35) = 0
        let psi3 = psi_x(3, 2, 3, 101);
        assert_eq!(psi3.iter().rev().fold(0, |acc, c| (acc * 35 + c) % 101), 0);
    }

    #[test]
    fn factor_degrees_of_known_polynomials() {
        // (x - 1)(x - 2)(x^2 + 2) over F_5: x^2 + 2 has no root (squares mod 5: 0, 1, 4)
        let f = pmul_test(&pmul_test(&[4, 1], &[3, 1], 5), &[2, 0, 1], 5);
        assert_eq!(degrees_string(&factor_degrees(&f, 5).unwrap()), "1^2 2");
        // x^5 - x - 1 is irreducible over F_5 (Artin–Schreier)
        assert_eq!(degrees_string(&factor_degrees(&[4, 4, 0, 0, 0, 1], 5).unwrap()), "5");
        // a square is refused
        assert!(factor_degrees(&pmul_test(&[1, 1], &[1, 1], 7), 7).is_err());
    }

    fn pmul_test(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
        super::super::end_ring::pmul(a, b, p)
    }

    #[test]
    fn corrupted_inputs_are_refused() {
        let (p, a, b) = (101u64, 3u64, 17u64);
        let t = trace(p, a, b);
        assert_eq!(t, -5);
        // a wrong recorded trace is refused by the recount
        assert!(curve_torsion(p, a, b, t + 2).unwrap_err().contains("point count"));
        // bypassing the recount, a wrong trace residue contradicts psi_l's factorization
        let wrong = (1..=20).filter(|d| ell_torsion(p, a, b, t + d, 5).is_err()).count();
        assert!(wrong >= 15, "only {wrong} of 20 wrong traces refused at l = 5");
        assert!(ell_torsion(p, a, b, t + 1, 5).unwrap_err().contains("predicts"));
        // a singular model
        assert!(curve_torsion(5, 0, 0, 0).is_err());
        // cross-checks against stored data
        let tor = ell_torsion(p, a, b, t, 5).unwrap();
        assert_eq!(tor.pred.rational_isogenies, 2);
        let bad_graph = Known { out_degree: Some(1), ..Default::default() };
        assert!(cross_check(&tor, &bad_graph).unwrap_err().contains("out-degree"));
        assert_eq!(cross_check(&tor, &Known { out_degree: Some(2), ..Default::default() }), Ok(false));
        let bad_row = Known {
            existing: Some(ExistingRow { psi_degrees: Some("1^12".into()), ..Default::default() }),
            ..Default::default()
        };
        assert!(cross_check(&tor, &bad_row).is_err());
        let bad_depth = Known { depth: Some(1), ..Default::default() };
        assert!(cross_check(&tor, &bad_depth).unwrap_err().contains("l | v"));
        // a scalar at depth 0 contradicts Kohel; a level above the floor needs the scalar
        let scal = ell_torsion(67, 46, 47, 4, 3).unwrap();
        assert_eq!(scal.pred.action, Action::Scalar);
        assert!(cross_check(&scal, &Known { depth: Some(0), ..Default::default() }).is_err());
        assert!(cross_check(&scal, &Known { depth: Some(1), level: Some(1), ..Default::default() }).is_err());
        assert_eq!(cross_check(&scal, &Known { depth: Some(1), level: Some(0), ..Default::default() }), Ok(false));
    }

    #[test]
    fn predictions_are_consistent() {
        for l in ELLS {
            for tm in 0..l {
                for dm in 1..l {
                    let dbl = double_root(l, tm, dm);
                    for scalar in [false, true] {
                        if scalar && !dbl {
                            continue;
                        }
                        let pr = predict(l, tm, dm, scalar);
                        let n: u64 = pr.psi_degrees.iter().map(|(d, m)| d * m).sum();
                        assert_eq!(n, if l == 2 { 3 } else { (l * l - 1) / 2 });
                        assert!(pr.point_degree <= pr.full_degree && pr.full_degree.is_multiple_of(pr.point_degree));
                        assert!(pr.full_degree.is_multiple_of(pr.all_isogenies_degree));
                        assert!(pr.isogeny_degree <= pr.all_isogenies_degree);
                        if scalar {
                            assert_eq!(pr.rational_isogenies, l + 1);
                        }
                    }
                }
            }
        }
    }

    /// A stored (v, D_K) that satisfies t^2 - 4p = v^2 D_K but with D_K not
    /// fundamental must not give a depth: p = 101, t = 2 has t^2 - 4p = -400 =
    /// 10^2 (-4), and (5, -16) and (1, -400) satisfy the identity too.
    #[test]
    fn non_fundamental_discriminant_is_not_trusted() {
        assert_eq!(verified_vd(false, 2, 101, Some(10), Some(-4)), Some((10, -4)));
        assert_eq!(verified_vd(false, 2, 101, Some(5), Some(-16)), None);
        assert_eq!(verified_vd(false, 2, 101, Some(1), Some(-400)), None);
        assert_eq!(verified_vd(false, 2, 101, Some(10), Some(-3)), None);
        assert_eq!(verified_vd(true, 0, 101, Some(2), Some(-101)), None);
        assert_eq!(verified_vd(false, 2, 101, None, Some(-4)), None);
    }

    #[test]
    fn kronecker_symbols() {
        assert_eq!(kronecker(-7, 2), 1); // -7 = 1 mod 8
        assert_eq!(kronecker(-3, 2), -1);
        assert_eq!(kronecker(-4, 2), 0);
        assert_eq!(kronecker(-3, 7), 1);
        assert_eq!(kronecker(-3, 5), -1);
        assert_eq!(kronecker(-15, 5), 0);
    }

    /// The whole census range without a database, timed:
    /// `cargo test --release -- --ignored full_census_ell_torsion --nocapture`.
    #[test]
    #[ignore]
    fn full_census_ell_torsion() {
        let start = std::time::Instant::now();
        let mut n = 0u64;
        for p in (5..MAX_P).filter(|&p| is_prime(p)) {
            let mut seen = vec![false; (p * p) as usize];
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
                    curve_torsion(p, a, b, -s).unwrap_or_else(|e| panic!("p {p} ({a},{b}): {e}"));
                    n += 1;
                }
            }
        }
        eprintln!("{n} curves in {:?}", start.elapsed());
    }
}
