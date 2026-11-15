//! End(E) of every supersingular short-Weierstrass curve over F_p,
//! 5 <= p < 2^10 (the census-fp curves and every other stored curve over those
//! fields). Most enrichment skips these curves; `end_ring.rs` refuses them
//! (p | t).
//!
//! # Mathematics
//!
//! For p >= 5 a supersingular E/F_p has t = 0, so Frobenius pi satisfies
//! pi^2 = -p and the F_p-rational endomorphism ring End_{F_p}(E) (the
//! centraliser of pi) is an order of Q(sqrt(-p)) containing Z[pi] = Z[sqrt(-p)].
//! Delfs and Galbraith (Des. Codes Cryptogr. 78 (2016), Thm 2.7 and §2.2):
//!
//! * p = 1 mod 4: Z[sqrt(-p)] is the maximal order (D_K = -4p), and every
//!   curve has End_{F_p}(E) = Z[sqrt(-p)]; there are h(-4p) F_p-isomorphism
//!   classes.
//! * p = 3 mod 4: D_K = -p and End_{F_p}(E) is either Z[(1+sqrt(-p))/2]
//!   (the *surface*, conductor 1, disc -p) or Z[sqrt(-p)] (the *floor*,
//!   conductor 2, disc -4p). (1+pi)/2 is an endomorphism iff 1 + pi kills
//!   E[2], i.e. pi fixes E[2] pointwise, i.e. x^3 + A x + B has three roots in
//!   F_p. With #E(F_p) = p + 1 = 0 mod 4 the cubic has one or three roots:
//!   three on the surface, one on the floor. There are h(-p) surface classes
//!   and h(-4p) floor classes, and h(-4p) = h(-p) for p = 7 mod 8, 3 h(-p)
//!   for p = 3 mod 8.
//!
//! The decision uses the rational 2-torsion, computed twice: by enumerating
//! the roots of the cubic, and by Kohel's test x^p = x mod x^3 + A x + B
//! (`end_ring::scalar_on` on E[2]); the two must agree.
//!
//! **Over the algebraic closure** (Deuring), End(E) is a maximal order of
//! the quaternion algebra B_{p,inf} ramified exactly at p and infinity, so every
//! row stores `quaternion_ramification = {p}`. The maximal order itself is
//! identified exactly where ecdb can certify it, for the two curves with
//! extra automorphisms:
//!
//! * j = 1728 (B = 0, p = 3 mod 4): i = [sqrt(-1)]: (x, y) -> (-x, sqrt(-1) y),
//!   j = pi, i^2 = -1, j^2 = -p, ij = -ji. On the floor (-A a non-square)
//!   O = <1, i, (i+j)/2, (1+k)/2>; on the surface (-A a square)
//!   O = <1, i, (1+j)/2, (i+k)/2>.
//! * j = 0 (A = 0, p = 2 mod 3): i = 2 omega + 1 = sqrt(-3) with
//!   omega: (x, y) -> (zeta_3 x, y), j = pi, i^2 = -3, j^2 = -p, ij = -ji.
//!   O = <1, (1+i)/2, (j+k)/2, (i+k)/3> when B is a square in F_p,
//!   <1, (1+i)/2, (j+k)/2, (i-k)/3> when it is not.
//!
//! These are the bases of McMurdy and of Eisenträger, Hallgren, Lauter,
//! Morrison and Petit (Eurocrypt 2018, §5) for y^2 = x^3 + x and
//! y^2 = x^3 + 1. They are certified per curve, not taken on trust:
//!
//! 1. each generator is an endomorphism: alpha/n is one iff alpha kills
//!    ker[n]; for j = 1728, (i + j)/2 and (1 + k)/2 need pi(r) = -r on the
//!    roots r of x^3 + A x (iff -A is a non-square), (1 + j)/2 and (i + k)/2 need
//!    pi(r) = r (iff -A is a square); for j = 0, (1+i)/2 = 1 + omega and
//!    (j+k)/2 = (1 + omega) pi are products of endomorphisms, and
//!    (i ± k)/3 = ∓(1 ∓ j) i^{-1} is one iff (1 ∓ pi) kills ker(i) =
//!    {O, (0, ±sqrt B)}, i.e. iff B is (resp. is not) a square;
//! 2. the lattice is an order (contains 1, closed under multiplication, by
//!    exact rational arithmetic in (a, b)_Q) with det(trd(e_r conj(e_s))) = p^2, so
//!    its reduced discriminant is p. A definite quaternion algebra with an
//!    order of reduced discriminant p is B_{p,inf}, and that order is maximal;
//!    End(E) is an order containing it, hence equal to it.
//!
//! The norm form is stored as the Gram matrix G = (trd(e_r conj(e_s))):
//! nrd(sum c_r e_r) = c^T G c / 2.
//!
//! Every other F_p-rational supersingular j is a documented limit: its
//! maximal order type would need endomorphisms beyond pi (isogeny loops over
//! F_{p^2}), so `quaternion_order_basis` stays NULL.
//!
//! # Checks
//!
//! Before anything is written:
//! - per curve: p prime, nonsingular, the stored trace is 0, the point count
//!   (-sum_x chi(x^3 + A x + B)) is 0, the root count is 1 or 3 and agrees with
//!   Kohel's test, p = 1 mod 4 forces one root (#E = p + 1 = 2 mod 4), and the
//!   quaternion certificate above where it applies;
//! - per isogeny class (the t = 0 class of one F_p): F_p-isomorphic curves
//!   ((A, B) ~ (u^4 A, u^6 B)) share their order; the number of classes with
//!   conductor f is h(f^2 D_K) (Deuring/Waterhouse, `end_ring::check_class`)
//!   and the surface/floor counts equal the Delfs-Galbraith formula above.
//!   The census holds every curve over F_p, so a short or over class is
//!   refused whole;
//! - where ecdb already holds an `endomorphism_ring` row ([`reconcile`]), every
//!   stored value must agree with the computed order or be implied by it:
//!   over the base field, kind `imaginary_quadratic_order` or `unknown`, equal
//!   discriminant, conductor and `is_maximal`, `conductor_divides` a multiple
//!   of f, `contains` a subset of the computed list, `cm_field_id` a quadratic
//!   field of discriminant D_K, `quaternion_ramification` = {p}, a
//!   `quaternion_order_basis` only where ecdb certifies the same one, and a
//!   status other than `refuted` / `not_applicable`. Otherwise the class is
//!   refused, so no unverified stored value is ever kept under status `proved`.
//!
//! # Writes
//!
//! `endomorphism_ring` (over_closure = false, kind `imaginary_quadratic_order`
//! for End_{F_p}(E), status `proved`, `conductor_divides` = v = [O_K : Z[pi]],
//! `contains`, `quaternion_ramification` = {p}, `quaternion_order_basis` for
//! j = 0, 1728) for curves without a row; a row that agrees has only its NULL
//! columns filled, its kind set from `unknown` and its status raised to
//! `proved` (with ecdb's method and provenance when the status is raised).
//! The UPDATE repeats every check as a guard; rows it or the insert's
//! `ON CONFLICT DO NOTHING` skip are reported as a problem
//! (`endomorphism_rings_write_refused_conflict`). Insert-or-update only.
//!
//! # Report
//!
//! `ss_curves`, `ss_classes_ok`, `ss_classes_refused` (each also a problem),
//! `ss_curves_refused_input`, `ss_surface`, `ss_floor`, `ss_maximal_p1mod4`,
//! `ss_quaternion_order_certified`, `ss_existing_agree`,
//! `endomorphism_rings_inserted_supersingular`,
//! `endomorphism_rings_updated_supersingular`,
//! `endomorphism_rings_write_refused_conflict`.

use super::end_ring::{check_class, class_number, division_polys, is_prime, iso_key, powmod, scalar_on, trim};
use super::{bulk, Ctx, Report};
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// Fields covered: 5 <= p < MAX_P (the census-fp fields).
pub const MAX_P: u64 = 1024;

const METHOD: &str = "Delfs-Galbraith: End_{F_p}(E) = Z[(1+sqrt(-p))/2] iff E[2] is F_p-rational, else Z[sqrt(-p)]; \
                      class checked against h(-p), h(-4p); Deuring: End over F_p-bar maximal in B_{p,inf}, basis \
                      certified (endomorphisms + reduced discriminant p) for j = 0, 1728";

const CURVES_SQL: &str = "
SELECT c.id, coalesce(c.family, ''), ic.id, ic.p::int8, c.coefficients->>'a', c.coefficients->>'b',
       ic.trace::int8, er.curve_id IS NOT NULL, er.kind, er.over_closure, er.discriminant::int8, er.conductor_f::int8,
       er.conductor_divides::int8, er.is_maximal, er.contains, er.cm_field_id,
       (SELECT n.fundamental_discriminant::int8 FROM ec.number_field n WHERE n.id = er.cm_field_id AND n.degree = 2),
       er.quaternion_ramification::text, er.quaternion_order_basis, er.status::text, er.method IS NULL,
       er.provenance_id IS NULL,
       (SELECT n.id FROM ec.number_field n WHERE n.degree = 2
          AND n.fundamental_discriminant = CASE WHEN ic.p % 4 = 1 THEN -4 * ic.p ELSE -ic.p END ORDER BY n.id LIMIT 1)
FROM ec.curve c
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
LEFT JOIN ec.endomorphism_ring er ON er.curve_id = c.id
WHERE c.genus = 1 AND c.model = 'short_weierstrass' AND ic.g = 1 AND ic.q = ic.p AND ic.p >= 5 AND ic.p < $1::int8
  AND (coalesce(ic.is_supersingular, false) OR ic.trace = 0)
ORDER BY ic.id, c.id";

const END_SQL: &str = "
INSERT INTO ec.endomorphism_ring (curve_id, over_closure, kind, discriminant, conductor_f, conductor_divides,
       is_maximal, contains, cm_field_id, quaternion_ramification, quaternion_order_basis, status, method,
       provenance_id)
SELECT r.curve_id, false, 'imaginary_quadratic_order', r.disc, r.f, r.v, r.f = 1, r.contains, nf.id, ARRAY[r.p],
       r.quat, 'proved', r.method, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, disc numeric, f numeric, v numeric, d_k numeric, p numeric,
     contains text[], quat jsonb, method text)
LEFT JOIN LATERAL (SELECT n.id FROM ec.number_field n WHERE n.fundamental_discriminant = r.d_k AND n.degree = 2
                   ORDER BY n.id LIMIT 1) nf ON true
ON CONFLICT (curve_id) DO NOTHING";

/// Complete an existing row that [`reconcile`] found consistent with the computed End_{F_p}(E): its NULL columns
/// are filled and its status becomes `proved` (every stored value is then one ecdb verified). A stored value is
/// never overwritten: the guards repeat every [`reconcile`] check, so a row changed in between is left alone, and
/// the caller reports rows sent but not updated. Method and provenance become ecdb's when the status is raised
/// (the proof is then ecdb's), and are only filled when the row was already `proved`.
const END_UPDATE_SQL: &str = "
UPDATE ec.endomorphism_ring er SET kind = 'imaginary_quadratic_order', discriminant = r.disc, conductor_f = r.f,
       is_maximal = (r.f = 1), conductor_divides = coalesce(er.conductor_divides, r.v),
       contains = coalesce(er.contains, r.contains), cm_field_id = coalesce(er.cm_field_id, nf.id),
       quaternion_ramification = ARRAY[r.p], quaternion_order_basis = coalesce(er.quaternion_order_basis, r.quat),
       method = CASE WHEN er.status = 'proved' THEN coalesce(er.method, r.method) ELSE r.method END,
       provenance_id = CASE WHEN er.status = 'proved' THEN coalesce(er.provenance_id, $2::bigint) ELSE $2::bigint END,
       status = 'proved'
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, disc numeric, f numeric, v numeric, d_k numeric, p numeric,
     contains text[], quat jsonb, method text)
LEFT JOIN LATERAL (SELECT n.id FROM ec.number_field n WHERE n.fundamental_discriminant = r.d_k AND n.degree = 2
                   ORDER BY n.id LIMIT 1) nf ON true
WHERE er.curve_id = r.curve_id AND NOT er.over_closure AND er.kind IN ('imaginary_quadratic_order', 'unknown')
  AND (er.discriminant IS NULL OR er.discriminant = r.disc) AND (er.conductor_f IS NULL OR er.conductor_f = r.f)
  AND (er.is_maximal IS NULL OR er.is_maximal = (r.f = 1))
  AND (er.conductor_divides IS NULL OR mod(er.conductor_divides, r.f) = 0)
  AND (er.contains IS NULL OR er.contains <@ r.contains)
  AND (er.quaternion_ramification IS NULL OR er.quaternion_ramification::numeric[] = ARRAY[r.p])
  AND (er.quaternion_order_basis IS NULL OR er.quaternion_order_basis = r.quat)
  AND (er.cm_field_id IS NULL OR EXISTS (SELECT 1 FROM ec.number_field n WHERE n.id = er.cm_field_id
                                         AND n.degree = 2 AND n.fundamental_discriminant = r.d_k))
  AND er.status NOT IN ('refuted', 'not_applicable')";

// ------------------------------------------------------------- quaternion orders

/// Scale of the stored quaternion coordinates: every basis element has denominators dividing 6.
const SCALE: i128 = 6;

/// Product in (a, b)_Q on coordinates over 1, i, j, k (i^2 = a, j^2 = b, k = ij).
fn qmul(a: i128, b: i128, x: &[i128; 4], y: &[i128; 4]) -> [i128; 4] {
    [
        x[0] * y[0] + a * x[1] * y[1] + b * x[2] * y[2] - a * b * x[3] * y[3],
        x[0] * y[1] + x[1] * y[0] - b * x[2] * y[3] + b * x[3] * y[2],
        x[0] * y[2] + x[2] * y[0] + a * x[1] * y[3] - a * x[3] * y[1],
        x[0] * y[3] + x[3] * y[0] + x[1] * y[2] - x[2] * y[1],
    ]
}

/// trd(x conj(y)) = 2 (x0 y0 - a x1 y1 - b x2 y2 + a b x3 y3).
fn trd_xbar(a: i128, b: i128, x: &[i128; 4], y: &[i128; 4]) -> i128 {
    2 * (x[0] * y[0] - a * x[1] * y[1] - b * x[2] * y[2] + a * b * x[3] * y[3])
}

fn det4(m: &[[i128; 4]; 4]) -> i128 {
    let minor = |r: usize, c: usize| -> i128 {
        let rows: Vec<usize> = (0..4).filter(|&x| x != r).collect();
        let cols: Vec<usize> = (0..4).filter(|&x| x != c).collect();
        let e = |i: usize, j: usize| m[rows[i]][cols[j]];
        e(0, 0) * (e(1, 1) * e(2, 2) - e(1, 2) * e(2, 1)) - e(0, 1) * (e(1, 0) * e(2, 2) - e(1, 2) * e(2, 0))
            + e(0, 2) * (e(1, 0) * e(2, 1) - e(1, 1) * e(2, 0))
    };
    (0..4).map(|c| if c % 2 == 0 { m[0][c] * minor(0, c) } else { -m[0][c] * minor(0, c) }).sum()
}

/// adj(m), so that m adj(m) = det(m) I.
fn adj4(m: &[[i128; 4]; 4]) -> [[i128; 4]; 4] {
    let mut out = [[0i128; 4]; 4];
    for (r, row) in out.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            // cofactor C[c][r] (transpose)
            let rows: Vec<usize> = (0..4).filter(|&x| x != c).collect();
            let cols: Vec<usize> = (0..4).filter(|&x| x != r).collect();
            let e = |i: usize, j: usize| m[rows[i]][cols[j]];
            let d = e(0, 0) * (e(1, 1) * e(2, 2) - e(1, 2) * e(2, 1))
                - e(0, 1) * (e(1, 0) * e(2, 2) - e(1, 2) * e(2, 0))
                + e(0, 2) * (e(1, 0) * e(2, 1) - e(1, 1) * e(2, 0));
            *cell = if (r + c) % 2 == 0 { d } else { -d };
        }
    }
    out
}

/// A maximal order of B_{p,inf} = (a, b)_Q, certified.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaxOrder {
    pub a: i64,
    pub b: i64,
    /// Basis rows over 1, i, j, k, scaled by [`SCALE`].
    pub basis6: [[i128; 4]; 4],
    /// G = (trd(e_r conj(e_s))), integral, det G = p^2.
    pub gram: [[i128; 4]; 4],
}

/// Certify that `basis6 / 6` spans an order of (a, b)_Q (a, b < 0) with reduced discriminant p; it is then a
/// maximal order of B_{p,inf}. Returns the Gram matrix of the trace form.
pub fn certify_order(a: i64, b: i64, p: u64, basis6: &[[i128; 4]; 4]) -> Result<[[i128; 4]; 4], String> {
    if a >= 0 || b >= 0 {
        return Err(format!("({a}, {b})_Q is not definite"));
    }
    let (qa, qb) = (a as i128, b as i128);
    let det = det4(basis6);
    if det == 0 {
        return Err("the basis is linearly dependent".into());
    }
    let adj = adj4(basis6);
    // coordinates of a vector w (scaled by s) in the basis: w/s * (basis6/6)^{-1} = w * adj * 6 / (s det)
    let coords = |w: &[i128; 4], s: i128| -> Option<[i128; 4]> {
        let mut out = [0i128; 4];
        for (c, o) in out.iter_mut().enumerate() {
            let num: i128 = (0..4).map(|r| w[r] * adj[r][c]).sum::<i128>() * SCALE;
            let den = s * det;
            if num % den != 0 {
                return None;
            }
            *o = num / den;
        }
        Some(out)
    };
    if coords(&[SCALE, 0, 0, 0], SCALE).is_none() {
        return Err("1 is not in the lattice".into());
    }
    for x in basis6 {
        for y in basis6 {
            if coords(&qmul(qa, qb, x, y), SCALE * SCALE).is_none() {
                return Err(format!("the lattice is not closed under multiplication: {x:?} * {y:?}"));
            }
        }
    }
    let mut gram = [[0i128; 4]; 4];
    for (r, row) in gram.iter_mut().enumerate() {
        for (s, cell) in row.iter_mut().enumerate() {
            let v = trd_xbar(qa, qb, &basis6[r], &basis6[s]);
            if v % (SCALE * SCALE) != 0 {
                return Err("a reduced trace is not integral".into());
            }
            *cell = v / (SCALE * SCALE);
        }
    }
    let d = det4(&gram);
    let p2 = (p as i128) * (p as i128);
    if d != p2 {
        return Err(format!("det(trd(e_r conj(e_s))) = {d}, not p^2 = {p2}: reduced discriminant is not p"));
    }
    Ok(gram)
}

fn legendre(x: u64, p: u64) -> i32 {
    match powmod(x % p, (p - 1) / 2, p) {
        0 => 0,
        1 => 1,
        _ => -1,
    }
}

/// Bases of the known maximal orders for j = 1728 and j = 0 (module documentation), scaled by 6.
pub fn known_basis(p: u64, a: u64, b: u64) -> Option<(i64, i64, [[i128; 4]; 4], &'static str)> {
    let pi = p as i64;
    if b == 0 && p % 4 == 3 {
        // -A a square: pi fixes E[2] (surface), else pi(r) = -r on E[2] (floor)
        if legendre((p - a) % p, p) == 1 {
            Some((-1, -pi, [[6, 0, 0, 0], [0, 6, 0, 0], [3, 0, 3, 0], [0, 3, 0, 3]], "<1, i, (1+j)/2, (i+k)/2>"))
        } else {
            Some((-1, -pi, [[6, 0, 0, 0], [0, 6, 0, 0], [0, 3, 3, 0], [3, 0, 0, 3]], "<1, i, (i+j)/2, (1+k)/2>"))
        }
    } else if a == 0 && p % 3 == 2 {
        if legendre(b, p) == 1 {
            Some((-3, -pi, [[6, 0, 0, 0], [3, 3, 0, 0], [0, 0, 3, 3], [0, 2, 0, 2]], "<1, (1+i)/2, (j+k)/2, (i+k)/3>"))
        } else {
            Some((-3, -pi, [[6, 0, 0, 0], [3, 3, 0, 0], [0, 0, 3, 3], [0, 2, 0, -2]], "<1, (1+i)/2, (j+k)/2, (i-k)/3>"))
        }
    } else {
        None
    }
}

fn frac(n: i128) -> String {
    let g = {
        let (mut x, mut y) = (n.abs(), SCALE);
        while y != 0 {
            (x, y) = (y, x % y);
        }
        x.max(1)
    };
    let (num, den) = (n / g, SCALE / g);
    if den == 1 {
        num.to_string()
    } else {
        format!("{num}/{den}")
    }
}

impl MaxOrder {
    pub fn to_json(&self, p: u64, label: &str, j: u64) -> Value {
        let (i_desc, i_sq) = if self.a == -1 {
            ("[sqrt(-1)]: (x, y) -> (-x, sqrt(-1) y)", -1)
        } else {
            ("2 omega + 1 = sqrt(-3), omega: (x, y) -> (zeta_3 x, y)", -3)
        };
        json!({
            "over": "algebraic closure",
            "algebra": { "i^2": i_sq, "j^2": -(p as i64), "k": "ij = -ji", "ramified": [p.to_string(), "inf"] },
            "j_invariant": j.to_string(),
            "i": i_desc,
            "j": "Frobenius pi: (x, y) -> (x^p, y^p)",
            "basis_label": label,
            "basis": self.basis6.iter().map(|r| r.iter().map(|&c| frac(c)).collect::<Vec<_>>()).collect::<Vec<_>>(),
            "basis_coordinates": "1, i, j, k",
            "reduced_discriminant": p.to_string(),
            "maximal": true,
            "norm_form_gram": self.gram.iter().map(|r| r.iter().map(|&c| c as i64).collect::<Vec<_>>()).collect::<Vec<_>>(),
            "norm_form": "nrd(sum c_r e_r) = c^T G c / 2, G = (trd(e_r conj(e_s)))",
            "certificate": "generators are endomorphisms (alpha/n with alpha killing ker[n], checked on E[2] or ker(i)); \
                            lattice is an order with det G = p^2, so it is maximal in B_{p,inf} and equals End(E)",
        })
    }
}

// ------------------------------------------------------------- End_{F_p}(E)

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SsEnd {
    /// 1 or 3: roots of x^3 + A x + B in F_p.
    pub roots: u32,
    /// Conductor of End_{F_p}(E) in O_K, K = Q(sqrt(-p)).
    pub f: u64,
    /// disc End_{F_p}(E) = f^2 D_K.
    pub disc: i64,
    /// v = [O_K : Z[pi]] and D_K with -4p = v^2 D_K.
    pub v: u64,
    pub d_k: i64,
    /// End over F_p-bar, where certified (j = 0, 1728).
    pub geometric: Option<(MaxOrder, &'static str)>,
}

impl SsEnd {
    pub fn surface(&self) -> bool {
        self.v == 2 && self.f == 1
    }

    pub fn contains(&self) -> Vec<&'static str> {
        if self.surface() {
            vec!["Z[pi]", "Z[(1+pi)/2]"]
        } else {
            vec!["Z[pi]"]
        }
    }
}

/// -4p = v^2 D_K.
pub fn split(p: u64) -> (u64, i64) {
    if p % 4 == 1 {
        (1, -4 * p as i64)
    } else {
        (2, -(p as i64))
    }
}

/// End_{F_p}(E) for the supersingular curve y^2 = x^3 + A x + B over F_p with stored trace t; every per-curve
/// check of the module documentation is enforced, a failure is the reason.
pub fn ss_end(p: u64, a: u64, b: u64, t: i64) -> Result<SsEnd, String> {
    if !(5..1 << 16).contains(&p) || !is_prime(p) {
        return Err(format!("p = {p} is not a prime in [5, 2^16)"));
    }
    let (a, b) = (a % p, b % p);
    if (4 * powmod(a, 3, p) + 27 * b % p * b).is_multiple_of(p) {
        return Err("singular model".into());
    }
    if t != 0 {
        return Err(format!("the stored trace is {t}; a supersingular curve over F_p, p >= 5, has t = 0"));
    }
    let mut is_sq = vec![false; p as usize];
    for y in 1..p {
        is_sq[(y * y % p) as usize] = true;
    }
    let mut roots = 0u32;
    let mut chi_sum = 0i64;
    for x in 0..p {
        match (x * x % p * x + a * x + b) % p {
            0 => roots += 1,
            r if is_sq[r as usize] => chi_sum += 1,
            _ => chi_sum -= 1,
        }
    }
    if chi_sum != 0 {
        return Err(format!("the point count gives t = {}, not 0: not supersingular", -chi_sum));
    }
    // Kohel's test on E[2]: pi = 1 on E[2] iff x^p = x mod x^3 + A x + B
    let fs = division_polys(4, a, b, p);
    let big_f = trim(vec![b, a, 0, 1]);
    let full = scalar_on(2, 1, &mut BTreeMap::new(), &fs, &big_f, p);
    if full != (roots == 3) {
        return Err(format!("{roots} roots of the cubic, but Kohel's test says E[2] rational = {full}"));
    }
    let (v, d_k) = split(p);
    let f = match (p % 4, roots) {
        (1, 1) => 1,
        (3, 3) => 1,
        (3, 1) => 2,
        _ => return Err(format!("{roots} roots of the cubic contradict #E = p + 1 with p = {} mod 4", p % 4)),
    };
    let disc = (f * f) as i64 * d_k;
    let geometric = match known_basis(p, a, b) {
        None => None,
        Some((qa, qb, basis6, label)) => {
            // the generator conditions are exactly the root/residue test known_basis branched on; the j = 1728
            // branch must agree with the surface/floor decision
            if b == 0 && (legendre((p - a) % p, p) == 1) != (roots == 3) {
                return Err("j = 1728: -A square disagrees with the rational 2-torsion".into());
            }
            let gram = certify_order(qa, qb, p, &basis6)?;
            Some((MaxOrder { a: qa, b: qb, basis6, gram }, label))
        }
    };
    if (b == 0 && p % 4 != 3) || (a == 0 && p % 3 != 2) {
        return Err("j = 0 or 1728 with p in the ordinary residue class".into());
    }
    Ok(SsEnd { roots, f, disc, v, d_k, geometric })
}

/// Delfs-Galbraith Thm 2.7: (surface, floor) F_p-isomorphism class counts; p = 1 mod 4 has one level, reported
/// as floor (End = Z[sqrt(-p)], which is maximal there).
pub fn dg_counts(p: u64) -> (u64, u64) {
    let pi = p as i64;
    match p % 8 {
        1 | 5 => (0, class_number(-4 * pi)),
        7 => (class_number(-pi), class_number(-pi)),
        _ => (class_number(-pi), 3 * class_number(-pi)),
    }
}

/// The class checks: Deuring/Waterhouse counts by conductor, and the Delfs-Galbraith surface/floor formula.
pub fn check_ss_class(p: u64, members: &[((u64, u64), &SsEnd)]) -> Result<(), String> {
    let (v, d_k) = split(p);
    let flat: Vec<((u64, u64), u64)> = members.iter().map(|(k, e)| (*k, e.f)).collect();
    check_class(v, d_k, &flat)?;
    let mut by_key: BTreeMap<(u64, u64), bool> = BTreeMap::new();
    for (k, e) in members {
        by_key.insert(*k, e.surface());
    }
    let surface = by_key.values().filter(|&&s| s).count() as u64;
    let floor = by_key.len() as u64 - surface;
    let want = dg_counts(p);
    if (surface, floor) != want {
        return Err(format!("surface/floor = {surface}/{floor}, Delfs-Galbraith gives {}/{}", want.0, want.1));
    }
    Ok(())
}

/// j(E) = 1728 * 4A^3 / (4A^3 + 27B^2).
fn j_invariant(p: u64, a: u64, b: u64) -> u64 {
    let n = 4 * powmod(a, 3, p) % p;
    let d = (n + 27 * b % p * b) % p;
    1728 % p * n % p * super::end_ring::inv(d, p) % p
}

// ------------------------------------------------------------- import

/// A stored `endomorphism_ring` row, as far as the reconciliation needs it.
#[derive(Clone, Debug, Default)]
pub struct Stored {
    pub kind: String,
    pub over_closure: bool,
    pub disc: Option<i64>,
    pub f: Option<i64>,
    pub divides: Option<i64>,
    pub is_maximal: Option<bool>,
    pub contains: Option<Vec<String>>,
    pub cm_field_id: Option<i64>,
    /// Fundamental discriminant of the stored `cm_field_id` (None if absent or not a quadratic field).
    pub cm_d_k: Option<i64>,
    /// `quaternion_ramification` as Postgres array text, e.g. `{7}`.
    pub ramification: Option<String>,
    pub basis: Option<Value>,
    pub status: String,
    pub method_null: bool,
    pub provenance_null: bool,
}

/// Compare a stored row with the computed End_{F_p}(E) (and its certified quaternion JSON, if any). Every stored
/// non-NULL value must agree exactly (or, for `conductor_divides` and `contains`, be implied by the computed
/// order); otherwise the reason is returned and the class is refused. `Ok(true)` when the row has something to
/// fill (a NULL column, kind `unknown`, or a status below `proved`), `Ok(false)` when it already holds everything.
pub fn reconcile(s: &Stored, e: &SsEnd, p: u64, quat: Option<&Value>, want_nf: Option<i64>) -> Result<bool, String> {
    let mut bad = Vec::new();
    if s.over_closure {
        bad.push("over_closure".to_string());
    }
    if s.kind != "imaginary_quadratic_order" && s.kind != "unknown" {
        bad.push(format!("kind {}", s.kind));
    }
    if s.disc.is_some_and(|d| d != e.disc) {
        bad.push(format!("discriminant {:?}", s.disc));
    }
    if s.f.is_some_and(|f| f != e.f as i64) {
        bad.push(format!("conductor {:?}", s.f));
    }
    if s.is_maximal.is_some_and(|m| m != (e.f == 1)) {
        bad.push(format!("is_maximal {:?}", s.is_maximal));
    }
    if s.divides.is_some_and(|d| d <= 0 || d % e.f as i64 != 0) {
        bad.push(format!("conductor_divides {:?}", s.divides));
    }
    let ours = e.contains();
    if let Some(c) = &s.contains {
        if !c.iter().all(|x| ours.contains(&x.as_str())) {
            bad.push(format!("contains {c:?} (End_F_p contains only {ours:?})"));
        }
    }
    if s.cm_field_id.is_some() && s.cm_d_k != Some(e.d_k) {
        bad.push(format!("cm_field_id {:?} (fundamental discriminant {:?})", s.cm_field_id, s.cm_d_k));
    }
    if s.ramification.as_ref().is_some_and(|r| *r != format!("{{{p}}}")) {
        bad.push(format!("quaternion_ramification {:?}", s.ramification));
    }
    if let Some(b) = &s.basis {
        // a stored basis ecdb cannot certify (no j = 0, 1728 certificate) or that differs from the certified one
        if quat != Some(b) {
            bad.push("a quaternion_order_basis ecdb does not certify for this curve".to_string());
        }
    }
    if s.status == "refuted" || s.status == "not_applicable" {
        bad.push(format!("status {}", s.status));
    }
    if !bad.is_empty() {
        return Err(bad.join(", "));
    }
    Ok(s.kind == "unknown"
        || s.disc.is_none()
        || s.f.is_none()
        || s.is_maximal.is_none()
        || s.divides.is_none()
        || s.contains.is_none()
        || (s.cm_field_id.is_none() && want_nf.is_some())
        || s.ramification.is_none()
        || (s.basis.is_none() && quat.is_some())
        || s.status != "proved"
        || s.method_null
        || s.provenance_null)
}

struct Row {
    id: i64,
    census: bool,
    p: u64,
    a: u64,
    b: u64,
    t: i64,
    stored: Option<Stored>,
    want_nf: Option<i64>,
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let mut report = Report {
        input: format!("supersingular short-Weierstrass curves over F_p, 5 <= p < {MAX_P} (stored ec.curve)"),
        ..Default::default()
    };
    let client = ctx.client;
    let src = ctx
        .source(
            "ecdb/src/import/supersingular.rs",
            &crate::math::sha256_hex(include_str!("supersingular.rs")),
            "End(E) of every supersingular census curve, computed by ecdb (Delfs-Galbraith, Deuring)",
        )
        .await?;
    let prov = ctx.provenance(src, METHOD, None).await?;

    let max_p = MAX_P as i64;
    let mut classes: BTreeMap<i64, Vec<Row>> = BTreeMap::new();
    for r in client.query(CURVES_SQL, &[&max_p]).await? {
        let coeff = |i: usize| r.get::<_, Option<String>>(i).and_then(|s| s.parse::<u64>().ok());
        let (Some(a), Some(b), Some(t)) = (coeff(4), coeff(5), r.get::<_, Option<i64>>(6)) else {
            report.count("ss_curves_refused_input", 1);
            report.problem(format!("curve {}: coefficients or trace missing", r.get::<_, i64>(0)));
            continue;
        };
        classes.entry(r.get(2)).or_default().push(Row {
            id: r.get(0),
            census: r.get::<_, String>(1) == "census-fp",
            p: r.get::<_, i64>(3) as u64,
            a,
            b,
            t,
            stored: r.get::<_, bool>(7).then(|| Stored {
                kind: r.get(8),
                over_closure: r.get(9),
                disc: r.get(10),
                f: r.get(11),
                divides: r.get(12),
                is_maximal: r.get(13),
                contains: r.get(14),
                cm_field_id: r.get(15),
                cm_d_k: r.get(16),
                ramification: r.get(17),
                basis: r.get(18),
                status: r.get(19),
                method_null: r.get(20),
                provenance_null: r.get(21),
            }),
            want_nf: r.get(22),
        });
    }

    let (mut inserts, mut updates) = (Vec::new(), Vec::new());
    'class: for (class_id, rows) in &classes {
        report.count("ss_curves", rows.len() as u64);
        let mut computed = Vec::new();
        for r in rows {
            match ss_end(r.p, r.a, r.b, r.t) {
                Ok(e) => computed.push(e),
                Err(why) => {
                    report.count("ss_classes_refused", 1);
                    report.problem(format!("class {class_id}: curve {}: {why}", r.id));
                    continue 'class;
                }
            }
        }
        let members: Vec<((u64, u64), &SsEnd)> =
            rows.iter().zip(&computed).map(|(r, e)| (iso_key(r.p, r.a, r.b), e)).collect();
        if let Err(why) = check_ss_class(rows[0].p, &members) {
            report.count("ss_classes_refused", 1);
            report.problem(format!("class {class_id} (p = {}): {why}", rows[0].p));
            continue;
        }
        let quats: Vec<Option<Value>> = rows
            .iter()
            .zip(&computed)
            .map(|(r, e)| e.geometric.as_ref().map(|(o, label)| o.to_json(r.p, label, j_invariant(r.p, r.a, r.b))))
            .collect();
        let mut agree = 0u64;
        let mut fill = vec![false; rows.len()];
        for (i, (r, e)) in rows.iter().zip(&computed).enumerate() {
            let Some(s) = &r.stored else { continue };
            match reconcile(s, e, r.p, quats[i].as_ref(), r.want_nf) {
                Ok(needs) => fill[i] = needs,
                Err(why) => {
                    report.count("ss_classes_refused", 1);
                    report.problem(format!(
                        "class {class_id}: curve {} has a stored End row that disagrees with End_F_p = disc {} \
                         (f = {}): {why}",
                        r.id, e.disc, e.f
                    ));
                    continue 'class;
                }
            }
            agree += 1;
        }
        report.count("ss_classes_ok", 1);
        report.count("ss_existing_agree", agree);
        for (i, (r, e)) in rows.iter().zip(&computed).enumerate() {
            match (r.p % 4, e.surface()) {
                (1, _) => report.count("ss_maximal_p1mod4", 1),
                (_, true) => report.count("ss_surface", 1),
                _ => report.count("ss_floor", 1),
            }
            if quats[i].is_some() {
                report.count("ss_quaternion_order_certified", 1);
            }
            let row = json!({ "curve_id": r.id, "disc": e.disc, "f": e.f, "v": e.v, "d_k": e.d_k, "p": r.p,
                "contains": e.contains(), "quat": quats[i], "method": METHOD, "census": r.census });
            if r.stored.is_none() {
                inserts.push(row);
            } else if fill[i] {
                updates.push(row);
            }
        }
    }
    let inserted = bulk(client, END_SQL, &inserts, prov).await?;
    report.count("endomorphism_rings_inserted_supersingular", inserted);
    let updated = bulk(client, END_UPDATE_SQL, &updates, prov).await?;
    report.count("endomorphism_rings_updated_supersingular", updated);
    let (lost_ins, lost_upd) =
        ((inserts.len() as u64).saturating_sub(inserted), (updates.len() as u64).saturating_sub(updated));
    if lost_ins + lost_upd > 0 {
        // rows written or changed by someone else between the read and the write: nothing was overwritten
        report.count("endomorphism_rings_write_refused_conflict", lost_ins + lost_upd);
        report.problem(format!(
            "{lost_ins} insert(s) found an existing endomorphism_ring row and {lost_upd} update(s) were refused by \
             the guards (the stored row changed or disagrees); those rows were left unchanged"
        ));
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    type Pt = Option<(u64, u64)>;

    fn add(p: u64, a: u64, s: Pt, t: Pt) -> Pt {
        let inv = super::super::end_ring::inv;
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

    /// Every supersingular F_p-isomorphism class over F_p, by brute-force point counting.
    fn ss_census(p: u64) -> Vec<(u64, u64)> {
        let mut seen = BTreeSet::new();
        let mut out = Vec::new();
        for a in 0..p {
            for b in 0..p {
                if (4 * a * a * a + 27 * b * b) % p == 0 || !seen.insert(iso_key(p, a, b)) {
                    continue;
                }
                if points(p, a, b).len() as u64 == p + 1 {
                    out.push((a, b));
                }
            }
        }
        out
    }

    /// Brute force: the surface/floor decision agrees with the group law (number of P with 2P = O is 4 iff
    /// (1+pi)/2 is an endomorphism), and every class satisfies Deuring and Delfs-Galbraith.
    #[test]
    fn brute_force_agreement_on_small_fields() {
        let (mut surf, mut floor, mut quat) = (0, 0, 0);
        for p in [5u64, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 101, 103, 107] {
            let curves = ss_census(p);
            let mut computed = Vec::new();
            for &(a, b) in &curves {
                let e = ss_end(p, a, b, 0).unwrap_or_else(|w| panic!("p {p} ({a},{b}): {w}"));
                let pts = points(p, a, b);
                let two_torsion = pts.iter().filter(|&&q| add(p, a, q, q).is_none()).count();
                assert_eq!(two_torsion == 4, e.surface(), "p {p} ({a},{b})");
                assert_eq!(two_torsion as u32, e.roots + 1);
                if p % 4 == 3 {
                    if e.surface() {
                        surf += 1
                    } else {
                        floor += 1
                    }
                }
                quat += e.geometric.is_some() as u32;
                computed.push((iso_key(p, a, b), e));
            }
            let members: Vec<_> = computed.iter().map(|(k, e)| (*k, e)).collect();
            check_ss_class(p, &members).unwrap_or_else(|w| panic!("p {p}: {w}"));
        }
        assert!(surf > 10 && floor > 10 && quat > 10, "{surf} {floor} {quat}");
    }

    /// h(-p), h(-4p) (LMFDB / Cox): h(-7) = h(-28) = 1, h(-11) = 1, h(-44) = 3, h(-23) = h(-92) = 3,
    /// h(-52) = 2, h(-31) = 3, h(-43) = 1, h(-172) = 3.
    #[test]
    fn pinned_class_numbers_and_counts() {
        for (d, h) in
            [(-7, 1), (-28, 1), (-11, 1), (-44, 3), (-23, 3), (-92, 3), (-52, 2), (-31, 3), (-43, 1), (-172, 3)]
        {
            assert_eq!(class_number(d), h, "h({d})");
        }
        assert_eq!(dg_counts(23), (3, 3));
        assert_eq!(dg_counts(11), (1, 3));
        assert_eq!(dg_counts(13), (0, 2));
        // over F_7 the only supersingular j is 1728: y^2 = x^3 + x on the floor (x^2 + 1 irreducible),
        // y^2 = x^3 + 3x on the surface (x^2 + 3 = (x - 2)(x + 2))
        let e1 = ss_end(7, 1, 0, 0).unwrap();
        let e3 = ss_end(7, 3, 0, 0).unwrap();
        assert_eq!((e1.f, e1.disc, e1.surface()), (2, -28, false));
        assert_eq!((e3.f, e3.disc, e3.surface()), (1, -7, true));
    }

    /// The literature bases (McMurdy; Eisenträger-Hallgren-Lauter-Morrison-Petit §5): y^2 = x^3 + x for
    /// p = 3 mod 4 gives <1, i, (i+j)/2, (1+k)/2>, y^2 = x^3 + 1 for p = 2 mod 3 gives
    /// <1, (1+i)/2, (j+k)/2, (i+k)/3>.
    #[test]
    fn pinned_literature_maximal_orders() {
        for p in [7u64, 11, 19, 23, 31, 43, 103, 431, 1019] {
            let e = ss_end(p, 1, 0, 0).unwrap();
            let (o, label) = e.geometric.unwrap();
            assert_eq!(label, "<1, i, (i+j)/2, (1+k)/2>");
            assert_eq!((o.a, o.b), (-1, -(p as i64)));
            assert_eq!(det4(&o.gram), (p * p) as i128);
            let js = o.to_json(p, label, j_invariant(p, 1, 0));
            assert_eq!(js["basis"][2], json!(["0", "1/2", "1/2", "0"]));
            assert_eq!(js["j_invariant"], json!((1728 % p).to_string()));
        }
        for p in [5u64, 11, 17, 23, 29, 41, 47, 1013] {
            let e = ss_end(p, 0, 1, 0).unwrap();
            let (o, label) = e.geometric.unwrap();
            assert_eq!(label, "<1, (1+i)/2, (j+k)/2, (i+k)/3>");
            assert_eq!(det4(&o.gram), (p * p) as i128);
            let js = o.to_json(p, label, 0);
            assert_eq!(js["basis"][3], json!(["0", "1/3", "0", "1/3"]));
        }
        // Gram of <1, i, (i+j)/2, (1+k)/2> over p = 7: nrd form with det 49
        let o = ss_end(7, 1, 0, 0).unwrap().geometric.unwrap().0;
        assert_eq!(o.gram, [[2, 0, 0, 1], [0, 2, 1, 0], [0, 1, 4, 0], [1, 0, 0, 4]]);
    }

    #[test]
    fn corrupted_inputs_are_refused() {
        // a nonzero stored trace, an ordinary curve claimed supersingular, a singular model
        assert!(ss_end(7, 1, 0, 2).is_err());
        assert!(ss_end(7, 1, 1, 0).unwrap_err().contains("point count"));
        assert!(ss_end(7, 0, 0, 0).is_err());
        // a wrong basis: the surface basis offered for the floor curve y^2 = x^3 + x over F_7 is still an
        // order, but a non-maximal lattice, a non-order and a non-definite algebra are refused
        assert!(certify_order(-1, -7, 7, &[[6, 0, 0, 0], [0, 6, 0, 0], [0, 0, 6, 0], [0, 0, 0, 6]])
            .unwrap_err()
            .contains("reduced discriminant"));
        assert!(certify_order(-1, -7, 7, &[[6, 0, 0, 0], [0, 6, 0, 0], [0, 3, 3, 0], [2, 0, 0, 2]]).is_err());
        assert!(certify_order(1, -7, 7, &[[6, 0, 0, 0], [0, 6, 0, 0], [0, 3, 3, 0], [3, 0, 0, 3]]).is_err());
        // the j = 1728 basis over a p = 1 mod 4 field (ordinary j) would not be offered, and (-1, -13) has no
        // order of reduced discriminant 13 with this shape
        assert!(certify_order(-1, -13, 13, &[[6, 0, 0, 0], [0, 6, 0, 0], [0, 3, 3, 0], [3, 0, 0, 3]]).is_err());
        // class checks: a dropped curve, a mislabelled curve, a duplicate with another order
        let p = 23;
        let curves = ss_census(p);
        let computed: Vec<_> = curves.iter().map(|&(a, b)| (iso_key(p, a, b), ss_end(p, a, b, 0).unwrap())).collect();
        let members: Vec<_> = computed.iter().map(|(k, e)| (*k, e)).collect();
        check_ss_class(p, &members).unwrap();
        assert!(check_ss_class(p, &members[1..]).is_err());
        let mut flipped = computed.clone();
        let fe = &mut flipped[0].1;
        fe.f = 3 - fe.f;
        fe.disc = (fe.f * fe.f) as i64 * fe.d_k;
        let bad: Vec<_> = flipped.iter().map(|(k, e)| (*k, e)).collect();
        assert!(check_ss_class(p, &bad).is_err());
        let mut dup = members.clone();
        let other = computed.iter().find(|(_, e)| e.f != computed[0].1.f).unwrap();
        dup.push((computed[0].0, &other.1));
        assert!(check_ss_class(p, &dup).is_err());
    }

    /// Existing rows: an agreeing partial row is filled, a complete one (what the insert writes) needs nothing, and
    /// every stored value that contradicts End_{F_p}(E) or that ecdb cannot certify refuses the class instead of
    /// being kept under status `proved`.
    #[test]
    fn existing_rows_are_reconciled_or_refused() {
        // the floor curve y^2 = x^3 + x over F_11, with its certified j = 1728 order
        let p = 11;
        let e = ss_end(p, 1, 0, 0).unwrap();
        let quat = e.geometric.as_ref().map(|(o, l)| o.to_json(p, l, j_invariant(p, 1, 0)));
        // every F_11 class has j = 0 or 1728; over F_23 two classes have another j
        let generic = ss_census(23)
            .into_iter()
            .map(|(a, b)| ss_end(23, a, b, 0).unwrap())
            .find(|g| g.geometric.is_none())
            .unwrap();
        assert_eq!((e.f, e.disc, e.d_k), (2, -44, -11));
        let full = Stored {
            kind: "imaginary_quadratic_order".into(),
            disc: Some(-44),
            f: Some(2),
            divides: Some(2),
            is_maximal: Some(false),
            contains: Some(vec!["Z[pi]".into()]),
            cm_field_id: Some(9),
            cm_d_k: Some(-11),
            ramification: Some("{11}".into()),
            basis: quat.clone(),
            status: "proved".into(),
            ..Default::default()
        };
        assert_eq!(reconcile(&full, &e, p, quat.as_ref(), Some(9)), Ok(false));
        let partial =
            Stored { kind: "unknown".into(), status: "unknown".into(), method_null: true, ..Default::default() };
        assert_eq!(reconcile(&partial, &e, p, quat.as_ref(), None), Ok(true));
        // a stored bound that the computed conductor satisfies is kept
        assert_eq!(reconcile(&Stored { divides: Some(4), ..full.clone() }, &e, p, quat.as_ref(), Some(9)), Ok(false));
        let conflicts = [
            Stored { over_closure: true, ..full.clone() },
            Stored { kind: "quaternion_maximal_order".into(), ..full.clone() },
            Stored { disc: Some(-11), ..full.clone() },
            Stored { f: Some(1), ..full.clone() },
            Stored { is_maximal: Some(true), ..full.clone() },
            Stored { divides: Some(3), ..full.clone() },
            Stored { contains: Some(vec!["Z[pi]".into(), "Z[(1+pi)/2]".into()]), ..full.clone() },
            Stored { cm_d_k: Some(-44), ..full.clone() },
            Stored { cm_d_k: None, ..full.clone() },
            Stored { ramification: Some("{2}".into()), ..full.clone() },
            Stored { basis: Some(json!({"basis": "something else"})), ..full.clone() },
            Stored { status: "refuted".into(), ..full.clone() },
            Stored { status: "not_applicable".into(), ..full.clone() },
        ];
        for s in &conflicts {
            assert!(reconcile(s, &e, p, quat.as_ref(), Some(9)).is_err(), "{s:?}");
        }
        // a quaternion basis stored for a curve whose order ecdb cannot certify is refused, not upgraded to proved
        let uncertified =
            Stored { kind: "unknown".into(), status: "heuristic".into(), basis: quat, ..Default::default() };
        assert!(reconcile(&uncertified, &generic, 23, None, None).unwrap_err().contains("does not certify"));
    }

    /// Every F_p, 5 <= p < 2^10: the supersingular classes satisfy Deuring, Delfs-Galbraith, and the j = 0,
    /// 1728 certificates. Supersingular curves are found by j (their j is a root of the supersingular
    /// polynomial in F_p), here by brute force over (A, B) with a fast character sum.
    #[test]
    #[ignore = "full census sweep (about ten seconds); run with --ignored"]
    fn full_census_sweep() {
        let (mut primes, mut classes, mut surface, mut floor, mut quat) = (0, 0, 0, 0, 0);
        for p in (5..MAX_P).filter(|&p| is_prime(p)) {
            let mut is_sq = vec![false; p as usize];
            for y in 1..p {
                is_sq[(y * y % p) as usize] = true;
            }
            let mut seen = vec![false; (p * p) as usize];
            let mut computed = Vec::new();
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
                    if s == 0 {
                        computed.push((iso_key(p, a, b), ss_end(p, a, b, 0).unwrap()));
                    }
                }
            }
            let members: Vec<_> = computed.iter().map(|(k, e)| (*k, e)).collect();
            check_ss_class(p, &members).unwrap_or_else(|w| panic!("p {p}: {w}"));
            primes += 1;
            classes += computed.len();
            surface += computed.iter().filter(|(_, e)| e.surface()).count();
            floor += computed.iter().filter(|(_, e)| p % 4 == 3 && !e.surface()).count();
            quat += computed.iter().filter(|(_, e)| e.geometric.is_some()).count();
        }
        eprintln!(
            "{primes} fields, {classes} supersingular classes: {surface} surface, {floor} floor, {} with p = 1 mod 4; \
             {quat} quaternion orders certified",
            classes - surface - floor
        );
    }
}
