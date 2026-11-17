//! The Prym complement E' of every genus-2 cover source, as an ecdb curve with
//! its relations.
//!
//! A `prime_quadratic_pullback_v1` source H: v^2 = G(u^2), with
//! G(x) = x^3 + g2 x^2 + g1 x + g0 and g0 != 0, has the involutions
//! (u, v) -> (-u, v) and (u, v) -> (-u, -v). Their quotients are
//!
//! * E: v^2 = G(x), by (u, v) -> (x, v) = (u^2, v): the recorded target, and
//! * E': w^2 = x G(x), by (u, v) -> (x, w) = (u^2, u v),
//!
//! and Jac(H) is isogenous over F_q to E x E' (E' is the Prym variety of
//! H -> E). [`super::cover_lpoly`] stores L(T) = (1 - tT + qT^2)(1 - t'T +
//! qT^2) of Jac(H). This pass materialises E'.
//!
//! # The model of E'
//!
//! The change of variables Z = g0 / x, V = g0 w / x^2 takes w^2 = x G(x) to
//! V^2 = Z^3 + g1 Z^2 + g0 g2 Z + g0^2, and X = Z + g1/3, Y = V (p > 3) to the
//! short Weierstrass model
//!
//! ```text
//! E': Y^2 = X^3 + A X + B,   A = g0 g2 - g1^2/3,   B = g0^2 - g0 g1 g2/3 + 2 g1^3/27,
//! ```
//!
//! so (u, v) -> (X, Y) = (g0/u^2 + g1/3, g0 v/u^3) is the explicit map H -> E'.
//! E' is named under the ICV1 rule (`docs/curves/ICV1.md`): the model JSON of
//! (A, B) over the field of H, its hash and the trace t'.
//!
//! # Cross-checks (a failure is a problem and nothing is stored for H)
//!
//! * t' = -a1 - t from the stored L-polynomial and the target's stored trace
//!   t, and a2 = 2q + t t' (so t is a root and t' the other), |t'| <= 2 sqrt q.
//! * #E' = q + 1 - t' equals the group order of the short model found
//!   independently, by baby-step giant-step on (A, B) (a different equation
//!   from the one [`super::cover_lpoly`] counted), or exhaustively for
//!   q < 2^20 when the search is inconclusive. An inconclusive order above
//!   that is skipped, never guessed.
//! * j(E') two ways: 1728 * 4A^3 / (4A^3 + 27B^2) from the short model, and
//!   6912 I^3 / (4I^3 - J^2) from the invariants I = g1^2 - 3 g0 g2 and
//!   J = 9 g0 g1 g2 - 27 g0^2 - 2 g1^3 of the quartic x^4 + g2 x^3 + g1 x^2 +
//!   g0 x itself.
//! * The field's ICV1 name recomputed from its modulus equals the stored one;
//!   an existing curve with the same model must carry the same slug, and an
//!   existing class of that slug the same trace.
//!
//! # Rows
//!
//! For E' (insert or nothing, as for the dataset curves): `curve` (family
//! `prym-complement`, unless it already exists), `genus1_invariants`, the
//! `isogeny_class_fq` row of t', `abelian_variety.isogeny_class_id`,
//! `curve_zeta` and the `icv1_slug` alias. Then `curve_relation` E -> E' of
//! kind `prym_complement` (migration 0013), one per target of H, with the maps
//! and g0, g1, g2 in `map`, and `av_component` rows Jac(H) ~ Jac(E) x Jac(E')
//! (relation `isogeny_factor`, multiplicity 2 when E' is E). The status is
//! that of the L-polynomial (`proved`, or `probable` when the field is not
//! proved), given the independent order. A source whose Jacobian already has
//! `isogeny_factor` rows is left alone. Nothing is deleted.
//!
//! # Report
//!
//! `prym_computed`, `prym_already_complete`, `prym_skipped` (with
//! `prym_skipped_*`), `prym_crosscheck_failures` (each also a problem),
//! `prym_order_exhaustive`, `prym_curves_seen`, `prym_curves_existing`,
//! `prym_curves_new`, `prym_curve_conflicts`, `prym_invariants`,
//! `prym_isogeny_classes`, `prym_jacobians_linked`, `prym_zeta`,
//! `prym_aliases`, `prym_relations`, `prym_relations_already_present`,
//! `prym_self_complements`, `prym_av_components`,
//! `prym_av_components_already_present`.

use super::cover_lpoly::{gpow, group_order, Bsgs, ExtField, Fp64, Gf, PRIME_CONSTRUCTION};
use super::covers::fq_from_parts;
use super::registry::{
    class_row, extension_field, invariant_row, prime_field, FieldSpec, ALIAS_SQL, CLASS_SQL, CURVE_SQL, INVARIANT_SQL,
};
use super::siblings::prime_icv1;
use super::{bulk, bulk1, Ctx, Report};
use crate::field::{invariants_from_a, Elt, Encoding, Fq};
use crate::math::{hasse_ok, icv1_extension_field, sha256_hex};
use anyhow::{bail, Result};
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};

/// Largest prime (bits) whose short model is counted by baby-step giant-step on words.
const MAX_FP_BITS: u64 = 62;
/// Exhaustive count of the short model when the search is inconclusive, below this q.
const EXHAUSTIVE_MAX: u128 = 1 << 20;

pub const FAMILY: &str = "prym-complement";

/// The short Weierstrass model of E' and what was checked on it.
#[derive(Debug, Clone, PartialEq)]
pub struct Prym {
    /// g0, g1, g2 of G.
    pub g: [Elt; 3],
    pub a: Elt,
    pub b: Elt,
    /// j(E'), equal from the short model and from the quartic.
    pub j: Elt,
    /// t' = -a1 - t.
    pub trace: BigInt,
    /// #E'(F_q) = q + 1 - t'.
    pub order: BigInt,
    /// The independent order came from an exhaustive count (else baby-step giant-step).
    pub exhaustive: bool,
}

#[derive(Debug)]
pub enum Outcome {
    Done(Box<Prym>),
    /// Not derived, with the reason (never a guess).
    Skip(&'static str),
    /// A cross-check failed: a problem, never inserted.
    Fail(String),
}

/// (A, B) of E': Y^2 = X^3 + A X + B from g0, g1, g2 (p > 3).
pub fn short_model(k: &Fq, g0: &Elt, g1: &Elt, g2: &Elt) -> Result<(Elt, Elt)> {
    let third = k.inv(&k.from_int(3))?;
    let g1sq = k.mul(g1, g1);
    let a = k.sub(&k.mul(g0, g2), &k.mul(&g1sq, &third));
    let b0 = k.mul(g0, g0);
    let b1 = k.mul(&k.mul(&k.mul(g0, g1), g2), &third);
    let b2 = k.mul(&k.scale(&k.mul(&g1sq, g1), 2), &k.inv(&k.from_int(27))?);
    Ok((a, k.add(&k.sub(&b0, &b1), &b2)))
}

/// j of Y^2 = X^3 + A X + B from its a-invariants (c4^3 / Delta).
pub fn short_j(k: &Fq, a: &Elt, b: &Elt) -> Result<Elt> {
    let z = k.zero();
    Ok(invariants_from_a(k, [z.clone(), z.clone(), z, a.clone(), b.clone()])?.j)
}

/// j of the Jacobian of w^2 = x^4 + g2 x^3 + g1 x^2 + g0 x from the quartic's
/// invariants: I = 12ae - 3bd + c^2, J = 72ace + 9bcd - 27ad^2 - 27eb^2 - 2c^3
/// with (a, b, c, d, e) = (1, g2, g1, g0, 0), and j = 6912 I^3 / (4I^3 - J^2).
pub fn quartic_j(k: &Fq, g0: &Elt, g1: &Elt, g2: &Elt) -> Result<Elt> {
    let i = k.sub(&k.mul(g1, g1), &k.scale(&k.mul(g0, g2), 3));
    let g1c = k.mul(&k.mul(g1, g1), g1);
    let j = k.sub(&k.sub(&k.scale(&k.mul(&k.mul(g0, g1), g2), 9), &k.scale(&k.mul(g0, g0), 27)), &k.scale(&g1c, 2));
    let i3 = k.mul(&k.mul(&i, &i), &i);
    let den = k.sub(&k.scale(&i3, 4), &k.mul(&j, &j));
    if k.is_zero(&den) {
        bail!("4I^3 - J^2 = 0: the quartic is singular");
    }
    k.div(&k.scale(&i3, 6912), &den)
}

/// t' = -a1 - t from L = 1 + a1 T + a2 T^2 + q a1 T^3 + q^2 T^4 and the trace t
/// of one factor; requires the shape, a2 = 2q + t t' and the Hasse bound for t'.
pub fn complement_trace(q: &BigInt, l: &[BigInt], t: &BigInt) -> Result<BigInt, String> {
    let shape = l.len() == 5 && l[0].is_one() && l[4] == q * q && l[3] == q * &l[1];
    if !shape {
        return Err(format!("stored L(T) {l:?} is not a genus-2 Weil polynomial shape over F_{q}"));
    }
    let tp = -&l[1] - t;
    if l[2] != BigInt::from(2) * q + t * &tp {
        return Err(format!(
            "target trace {t} is not a factor of L = {l:?}: a2 = {} but 2q + t t' = {} for t' = {tp}",
            l[2],
            BigInt::from(2) * q + t * &tp
        ));
    }
    if !hasse_ok(&tp, q) {
        return Err(format!("t' = {tp} violates the Hasse bound over F_{q}"));
    }
    Ok(tp)
}

/// #E(F_q) of y^2 = x^3 + a x + b by listing x (q small).
fn count_short<F: Gf>(f: &F, a: &F::E, b: &F::E) -> u128 {
    let q = f.order();
    let one = f.one();
    let mut n = 1u128;
    for i in 0..q {
        let x = f.elt_of_index(i);
        let r = f.add(&f.add(&f.mul(&f.mul(&x, &x), &x), &f.mul(a, &x)), b);
        if f.is_zero(&r) {
            n += 1;
        } else if gpow(f, &r, (q - 1) / 2) == one {
            n += 2;
        }
    }
    n
}

/// The order of the short model: Ok((order, exhaustive)), or a skip/fail.
fn order_on<F: Gf>(f: &F, a: F::E, b: F::E) -> Result<(u128, bool), Outcome> {
    match group_order(f, f.zero(), a.clone(), b.clone()) {
        Ok(n) => Ok((n, false)),
        Err(Bsgs::Ambiguous(_)) if f.order() < EXHAUSTIVE_MAX => Ok((count_short(f, &a, &b), true)),
        Err(Bsgs::Ambiguous(_)) => Err(Outcome::Skip("order_inconclusive")),
        Err(Bsgs::Inconsistent(m)) => Err(Outcome::Fail(format!("group order search on E' inconsistent: {m}"))),
    }
}

/// #E'(F_q) of the short model, independently of the stored L-polynomial.
pub fn short_order(k: &Fq, a: &Elt, b: &Elt) -> Result<(u128, bool), Outcome> {
    if k.degree() == 1 {
        if k.p.bits() > MAX_FP_BITS {
            return Err(Outcome::Skip("field_too_large"));
        }
        let (Some(p), Some(a0), Some(b0)) = (k.p.to_u64(), a[0].to_u64(), b[0].to_u64()) else {
            return Err(Outcome::Skip("field_too_large"));
        };
        order_on(&Fp64 { p }, a0, b0)
    } else {
        let Some(ext) = ExtField::new(k) else { return Err(Outcome::Skip("field_too_large")) };
        order_on(&ext, a.clone(), b.clone())
    }
}

/// E' of the cover source with even monic sextic `f` (ascending, length 7),
/// cross-checked against the stored L-polynomial `l` and the target trace `t`.
pub fn derive(k: &Fq, f: &[Elt], l: &[BigInt], t: &BigInt) -> Outcome {
    if k.encoding == Encoding::HexBitset || k.p <= BigInt::from(3) {
        return Outcome::Skip("unsupported_field");
    }
    let one = k.from_int(1);
    let even = f.len() == 7 && [1, 3, 5].iter().all(|&i| k.is_zero(&f[i])) && f[6] == one && !k.is_zero(&f[0]);
    if !even {
        return Outcome::Skip("unsupported_shape");
    }
    let (g0, g1, g2) = (&f[0], &f[2], &f[4]);
    let (a, b) = match short_model(k, g0, g1, g2) {
        Ok(ab) => ab,
        Err(e) => return Outcome::Fail(format!("short model of E': {e}")),
    };
    let j = match short_j(k, &a, &b) {
        Ok(j) => j,
        Err(e) => return Outcome::Fail(format!("E' short model: {e}")),
    };
    match quartic_j(k, g0, g1, g2) {
        Ok(jq) if jq == j => {}
        Ok(jq) => {
            return Outcome::Fail(format!(
                "j(E') = {} from the short model but {} from the quartic x G(x)",
                k.format(&j),
                k.format(&jq)
            ))
        }
        Err(e) => return Outcome::Fail(format!("quartic x G(x): {e}")),
    }
    let q = num_traits::pow(k.p.clone(), k.degree());
    let tp = match complement_trace(&q, l, t) {
        Ok(tp) => tp,
        Err(e) => return Outcome::Fail(e),
    };
    let order = &q + BigInt::one() - &tp;
    let (n, exhaustive) = match short_order(k, &a, &b) {
        Ok(r) => r,
        Err(o) => return o,
    };
    if BigInt::from(n) != order {
        return Outcome::Fail(format!(
            "#E' = {n} from the short model, but q + 1 - t' = {order} from the stored L-polynomial (t' = {tp})"
        ));
    }
    Outcome::Done(Box::new(Prym { g: [g0.clone(), g1.clone(), g2.clone()], a, b, j, trace: tp, order, exhaustive }))
}

/// ICV1 for y^2 = x^3 + a x + b over GF(p^k) = GF(p)[t]/(f) with #E = order:
/// (slug, model JSON, its SHA-256, trace), `docs/curves/ICV1.md` (extension).
/// The slug is icv1-fp<bits of p>k<k>-t<trace>-<first 8 hex of the model hash>.
pub fn extension_icv1(k: &Fq, a: &Elt, b: &Elt, order: &BigInt) -> Result<(String, String, String, BigInt)> {
    let deg = k.degree();
    if deg < 2 || k.encoding != Encoding::DecimalVector {
        bail!("not an extension field of degree >= 2");
    }
    let disc = k.add(&k.scale(&k.mul(&k.mul(a, a), a), 4), &k.scale(&k.mul(b, b), 27));
    if k.is_zero(&disc) {
        bail!("singular: 4a^3 + 27b^2 = 0");
    }
    let q = num_traits::pow(k.p.clone(), deg);
    let t = &q + BigInt::one() - order;
    if !hasse_ok(&t, &q) {
        bail!("order {order} violates the Hasse bound for q = {q}");
    }
    let low = &k.f[..deg];
    let list = |e: &[BigInt]| e.iter().map(|c| format!("\"{c}\"")).collect::<Vec<_>>().join(",");
    let field = icv1_extension_field(&k.p, low);
    let model_json = format!(
        "{{\"a\":[{}],\"b\":[{}],\"field\":\"{field}\",\"form\":\"y^2=x^3+a*x+b\",\"k\":\"{deg}\",\"modulus\":[{}],\"p\":\"{}\",\"v\":\"1\"}}",
        list(a),
        list(b),
        list(low),
        k.p
    );
    let sha = sha256_hex(&model_json);
    let tr = if t.is_negative() { format!("m{}", t.abs()) } else { t.to_string() };
    let slug = format!("icv1-fp{}k{deg}-t{tr}-{}", k.p.bits(), &sha[..8]);
    Ok((slug, model_json, sha, t))
}

/// The ICV1 field name of `k` recomputed from its modulus, and its field spec.
fn field_spec(k: &Fq) -> Result<FieldSpec> {
    if k.degree() == 1 {
        prime_field(&format!("fp-{}", k.p), &k.p)
    } else {
        let low = &k.f[..k.degree()];
        extension_field(&icv1_extension_field(&k.p, low), &k.p, low)
    }
}

/// (slug, model JSON, hash, coefficient map) of E' under the ICV1 rule.
pub fn icv1_of(k: &Fq, p: &Prym) -> Result<(String, String, String, Map<String, Value>)> {
    let (slug, model_json, sha, t) = if k.degree() == 1 {
        prime_icv1(&k.p, &p.a[0], &p.b[0], &p.order)?
    } else {
        extension_icv1(k, &p.a, &p.b, &p.order)?
    };
    if t != p.trace {
        bail!("ICV1 trace {t} differs from t' = {}", p.trace);
    }
    let mut coeff = Map::new();
    coeff.insert("a".into(), json!(k.format(&p.a)));
    coeff.insert("b".into(), json!(k.format(&p.b)));
    Ok((slug, model_json, sha, coeff))
}

// --------------------------------------------------------------- import

struct Target {
    curve_id: i64,
    slug: Option<String>,
    model: String,
    trace: Option<BigInt>,
}

struct Job {
    h_id: i64,
    h_av: i64,
    fq: Fq,
    icv1_field: Option<String>,
    f: Vec<String>,
    l: Vec<BigInt>,
    status: String,
    targets: Vec<Target>,
}

const JOBS_SQL: &str = "
SELECT h.id, hav.id, hm.f, fl.kind::text, fl.characteristic::text, fl.defining_polynomial::text[], fl.modulus_hex,
       fl.icv1_field, ic.l_polynomial::text[], ic.status::text,
       (SELECT jsonb_agg(jsonb_build_object('curve_id', e.id, 'slug', e.icv1_slug,
                                            'model', e.model::text, 'trace', eic.trace::text) ORDER BY e.id)
          FROM (SELECT DISTINCT m.target_curve_id FROM ec.curve_morphism m
                 WHERE m.source_curve_id = h.id AND m.kind = 'hyperelliptic_pullback'
                   AND m.construction = 'prime_quadratic_pullback_v1') mt
          JOIN ec.curve e ON e.id = mt.target_curve_id
          JOIN ec.abelian_variety eav ON eav.curve_id = e.id
          LEFT JOIN ec.isogeny_class_fq eic ON eic.id = eav.isogeny_class_id AND eic.g = 1)
FROM ec.curve h
JOIN ec.hyperelliptic_model hm ON hm.curve_id = h.id
JOIN ec.field fl ON fl.id = h.field_id
JOIN ec.abelian_variety hav ON hav.curve_id = h.id
JOIN ec.isogeny_class_fq ic ON ic.id = hav.isogeny_class_id AND ic.g = 2
WHERE h.genus = 2 AND h.family = 'cover'
  AND EXISTS (SELECT 1 FROM ec.curve_morphism m
               WHERE m.source_curve_id = h.id AND m.kind = 'hyperelliptic_pullback'
                 AND m.construction = 'prime_quadratic_pullback_v1')
  AND NOT EXISTS (SELECT 1 FROM ec.av_component c WHERE c.av_id = hav.id AND c.relation = 'isogeny_factor')
ORDER BY h.id";

const DONE_SQL: &str = "
SELECT count(DISTINCT c.av_id) FROM ec.av_component c
JOIN ec.abelian_variety hav ON hav.id = c.av_id
JOIN ec.curve h ON h.id = hav.curve_id
WHERE c.relation = 'isogeny_factor' AND h.genus = 2 AND h.family = 'cover'";

/// Existing curves that share a model (field, coefficients) or a slug with an E',
/// with the class their Jacobian is linked to (any g) and their stored #E(F_q).
const EXISTING_SQL: &str = "
SELECT r.slug, c.icv1_slug, eic.trace::text, eic.g, z.point_counts[1]::text
FROM jsonb_to_recordset($1::jsonb) AS r(slug text, field text, coefficients jsonb)
JOIN ec.field f ON f.icv1_field = r.field
JOIN ec.curve c ON (c.field_id = f.id AND c.model = 'short_weierstrass' AND c.coefficients = r.coefficients)
                OR c.icv1_slug = r.slug
LEFT JOIN ec.abelian_variety av ON av.curve_id = c.id
LEFT JOIN ec.isogeny_class_fq eic ON eic.id = av.isogeny_class_id
LEFT JOIN ec.curve_zeta z ON z.curve_id = c.id";

/// Link E''s Jacobian to the class of t' only where no class is linked yet
/// (registry's LINK_CLASS_SQL replaces a different link; this never does).
const LINK_SQL: &str = "
UPDATE ec.abelian_variety av SET isogeny_class_id = ic.id
FROM jsonb_to_recordset($1::jsonb) AS r(key text, label text)
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.isogeny_class_fq ic ON ic.label = r.label
WHERE av.curve_id = c.id AND av.isogeny_class_id IS NULL";

/// Whether an existing curve found by model or slug agrees with E' (`slug`, t', #E'):
/// the same slug, a linked class (if any) of genus 1 and trace t', a stored
/// #E(F_q) (if any) equal to #E'. Err names the disagreement.
pub fn existing_agrees(
    slug: &str,
    have: Option<&str>,
    class_g: Option<i32>,
    class_trace: Option<&str>,
    zeta_order: Option<&str>,
    tp: &BigInt,
    order: &BigInt,
) -> Result<(), String> {
    if have != Some(slug) {
        return Err(format!("an existing curve with this model or slug is named {have:?}"));
    }
    if let Some(g) = class_g {
        if g != 1 {
            return Err(format!("the existing curve's Jacobian is linked to a class of genus {g}"));
        }
    }
    if let Some(t) = class_trace {
        if t.parse::<BigInt>().ok().as_ref() != Some(tp) {
            return Err(format!("the existing class has trace {t}, t' = {tp}"));
        }
    }
    if let Some(n) = zeta_order {
        if n.parse::<BigInt>().ok().as_ref() != Some(order) {
            return Err(format!("the existing curve_zeta has #E(F_q) = {n}, #E' = {order}"));
        }
    }
    Ok(())
}

const ZETA_SQL: &str = "
INSERT INTO ec.curve_zeta (curve_id, isogeny_class_id, point_counts, order_check, order_method, provenance_id)
SELECT c.id, av.isogeny_class_id, r.point_counts, r.status::ec.fact_status, r.method, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, point_counts numeric[], status text, method text)
JOIN ec.curve c ON c.icv1_slug = r.key
JOIN ec.abelian_variety av ON av.curve_id = c.id
ON CONFLICT (curve_id) DO NOTHING";

const RELATION_SQL: &str = "
INSERT INTO ec.curve_relation (source_curve_id, target_curve_id, kind, field_of_definition_id, map, status, provenance_id)
SELECT r.source_id, c.id, 'prym_complement'::ec.relation_kind, c.field_id, r.map, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(source_id bigint, slug text, map jsonb, status text)
JOIN ec.curve c ON c.icv1_slug = r.slug
WHERE c.id <> r.source_id
ON CONFLICT DO NOTHING";

const COMPONENT_SQL: &str = "
INSERT INTO ec.av_component (av_id, component_av_id, multiplicity, relation, map, status, provenance_id)
SELECT r.av_id, av.id, r.multiplicity, 'isogeny_factor', r.map, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(av_id bigint, curve_id bigint, slug text, multiplicity int, map jsonb, status text)
JOIN ec.curve c ON c.id = r.curve_id OR (r.curve_id IS NULL AND c.icv1_slug = r.slug)
JOIN ec.abelian_variety av ON av.curve_id = c.id
ON CONFLICT DO NOTHING";

const METHOD_BSGS: &str = "Prym complement E' of a prime_quadratic_pullback_v1 cover source H: v^2 = G(u^2): \
    w^2 = x G(x) taken to Y^2 = X^3 + A X + B by X = g0/x + g1/3, Y = g0 w/x^2; t' = -a1 - t from the stored \
    L-polynomial of Jac(H) and the target trace t, with a2 = 2q + t t'; #E' = q + 1 - t' equal to the group order of \
    (A, B) found by baby-step giant-step in the Hasse interval (unique candidate); j(E') equal from (A, B) and from \
    the invariants I, J of the quartic x G(x)";
/// The provenance of every row this pass writes: which order search ran for a
/// given E' is in its `curve_zeta.order_method` (METHOD_BSGS or METHOD_COUNT).
const METHOD_PROV: &str = "Prym complement E' of a prime_quadratic_pullback_v1 cover source H: v^2 = G(u^2): \
    w^2 = x G(x) taken to Y^2 = X^3 + A X + B by X = g0/x + g1/3, Y = g0 w/x^2; t' = -a1 - t from the stored \
    L-polynomial of Jac(H) and the target trace t, with a2 = 2q + t t'; #E' = q + 1 - t' equal to the group order of \
    (A, B) found by baby-step giant-step in the Hasse interval (unique candidate), or by an exhaustive count when \
    q < 2^20 and the search is inconclusive (curve_zeta.order_method says which); j(E') equal from (A, B) and from \
    the invariants I, J of the quartic x G(x)";
const METHOD_COUNT: &str = "Prym complement E' of a prime_quadratic_pullback_v1 cover source H: v^2 = G(u^2): \
    w^2 = x G(x) taken to Y^2 = X^3 + A X + B by X = g0/x + g1/3, Y = g0 w/x^2; t' = -a1 - t from the stored \
    L-polynomial of Jac(H) and the target trace t, with a2 = 2q + t t'; #E' = q + 1 - t' equal to the exhaustive \
    count of (A, B); j(E') equal from (A, B) and from the invariants I, J of the quartic x G(x)";

/// Derive every E' on the available cores.
fn run_jobs(jobs: &[Job]) -> Vec<Outcome> {
    let next = std::sync::atomic::AtomicUsize::new(0);
    let threads = std::thread::available_parallelism().map_or(2, |n| n.get()).min(16);
    let results = std::sync::Mutex::new(Vec::with_capacity(jobs.len()));
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some(job) = jobs.get(i) else { break };
                let out = run_job(job);
                if let Ok(mut r) = results.lock() {
                    r.push((i, out));
                }
            });
        }
    });
    let mut r = results.into_inner().unwrap_or_default();
    r.sort_by_key(|(i, _)| *i);
    r.into_iter().map(|(_, o)| o).collect()
}

fn run_job(job: &Job) -> Outcome {
    let k = &job.fq;
    let Some(f) = job.f.iter().map(|s| k.parse(s).ok()).collect::<Option<Vec<Elt>>>() else {
        return Outcome::Skip("unparsed_model");
    };
    let traces: BTreeSet<&BigInt> = job.targets.iter().filter_map(|t| t.trace.as_ref()).collect();
    let t = match traces.len() {
        0 => return Outcome::Skip("no_target_trace"),
        1 => traces.into_iter().next().cloned().unwrap_or_default(),
        _ => return Outcome::Fail(format!("the targets of H carry different traces {traces:?}")),
    };
    derive(k, &f, &job.l, &t)
}

/// One E' ready to insert, with the rows that hang on it.
struct Ready<'a> {
    job: &'a Job,
    prym: Box<Prym>,
    slug: String,
    status: &'static str,
}

/// Compute and store E' for every genus-2 cover source with an L-polynomial
/// and no decomposition yet.
pub async fn import(ctx: &Ctx<'_>, report: &mut Report) -> Result<()> {
    let client = ctx.client;
    let done: i64 = client.query_one(DONE_SQL, &[]).await?.get(0);
    report.count("prym_already_complete", done as u64);
    let rows = client.query(JOBS_SQL, &[]).await?;
    let mut jobs = Vec::with_capacity(rows.len());
    for r in rows {
        let kind: String = r.get(3);
        let Some(fq) = fq_from_parts(&kind, &r.get::<_, String>(4), r.get(5), r.get(6))? else {
            report.count("prym_skipped", 1);
            report.count("prym_skipped_unsupported_field", 1);
            continue;
        };
        let jf = |v: Value| -> Vec<String> {
            v.as_array().into_iter().flatten().filter_map(|x| x.as_str().map(str::to_string)).collect()
        };
        let l: Vec<String> = r.get(8);
        let targets: Option<Value> = r.get(10);
        let targets = targets
            .as_ref()
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|t| {
                Some(Target {
                    curve_id: t["curve_id"].as_i64()?,
                    slug: t["slug"].as_str().map(str::to_string),
                    model: t["model"].as_str().unwrap_or_default().to_string(),
                    trace: t["trace"].as_str().and_then(|x| x.parse().ok()),
                })
            })
            .collect();
        jobs.push(Job {
            h_id: r.get(0),
            h_av: r.get(1),
            fq,
            icv1_field: r.get(7),
            f: jf(r.get(2)),
            l: l.iter().filter_map(|x| x.parse().ok()).collect(),
            status: r.get(9),
            targets,
        });
    }
    if jobs.is_empty() {
        return Ok(());
    }
    let jobs = std::sync::Arc::new(jobs);
    let outcomes = {
        let jobs = jobs.clone();
        tokio::task::spawn_blocking(move || run_jobs(&jobs)).await?
    };

    let mut ready: Vec<Ready<'_>> = vec![];
    let mut curves: BTreeMap<String, (Value, Value, Value, Value, Value)> = BTreeMap::new();
    for (job, out) in jobs.iter().zip(outcomes) {
        let prym = match out {
            Outcome::Done(p) => p,
            Outcome::Skip(reason) => {
                report.count("prym_skipped", 1);
                report.count(&format!("prym_skipped_{reason}"), 1);
                continue;
            }
            Outcome::Fail(msg) => {
                report.count("prym_crosscheck_failures", 1);
                report.problem(format!("cover source curve id {}: Prym complement not stored: {msg}", job.h_id));
                continue;
            }
        };
        let res: Result<(String, &'static str)> = (|| {
            let spec = field_spec(&job.fq)?;
            let fid = spec.row["icv1_field"].as_str().unwrap_or_default().to_string();
            if job.icv1_field.as_deref() != Some(fid.as_str()) {
                bail!("the field's stored ICV1 name {:?} is not {fid}, recomputed from its modulus", job.icv1_field);
            }
            let (slug, model_json, sha, coeff) = icv1_of(&job.fq, &prym)?;
            let status = match job.status.as_str() {
                "proved" => "proved",
                "probable" => "probable",
                other => bail!("the stored L-polynomial has status {other}, neither proved nor probable"),
            };
            if !curves.contains_key(&slug) {
                let jtext = job.fq.format(&prym.j);
                let mut inv = invariant_row(&slug, &job.fq, "short_weierstrass", &coeff, Some(&jtext))?;
                inv["status"] = json!("proved");
                let (label, mut class) = class_row(&spec, &prym.trace)?;
                class["status"] = json!(status);
                let method = if prym.exhaustive { METHOD_COUNT } else { METHOD_BSGS };
                let curve = json!({
                    "field": fid, "model": "short_weierstrass", "form": "y^2=x^3+a*x+b",
                    "equation": format!("y^2 = x^3 + ({})*x + ({})", job.fq.format(&prym.a), job.fq.format(&prym.b)),
                    "coefficients": coeff, "family": FAMILY, "model_json": model_json, "model_sha256": sha,
                    "icv1": null, "slug": slug, "is_smooth": true, "smoothness_status": "proved",
                });
                let zeta = json!({
                    "key": slug, "point_counts": [prym.order.to_string()], "status": status, "method": method,
                });
                curves.insert(slug.clone(), (curve, inv, class, json!({ "key": slug, "label": label }), zeta));
            }
            Ok((slug, status))
        })();
        match res {
            Ok((slug, status)) => {
                report.count("prym_computed", 1);
                if prym.exhaustive {
                    report.count("prym_order_exhaustive", 1);
                }
                ready.push(Ready { job, prym, slug, status });
            }
            Err(e) => {
                report.count("prym_crosscheck_failures", 1);
                report.problem(format!("cover source curve id {}: Prym complement not stored: {e}", job.h_id));
            }
        }
    }

    // Existing curves: the same model must carry the same slug, the same slug the same trace.
    let probe: Vec<Value> = curves
        .iter()
        .map(|(slug, (c, ..))| json!({ "slug": slug, "field": c["field"], "coefficients": c["coefficients"] }))
        .collect();
    let mut existing: BTreeSet<String> = BTreeSet::new();
    let mut conflicts: BTreeSet<String> = BTreeSet::new();
    for chunk in probe.chunks(2000) {
        for r in client.query(EXISTING_SQL, &[&Value::Array(chunk.to_vec())]).await? {
            let slug: String = r.get(0);
            let have: Option<String> = r.get(1);
            let trace: Option<String> = r.get(2);
            let g: Option<i32> = r.get(3);
            let zeta: Option<String> = r.get(4);
            let want = &curves[&slug].0;
            let Some(x) = ready.iter().find(|x| x.slug == slug) else { continue };
            let verdict = existing_agrees(
                &slug,
                have.as_deref(),
                g,
                trace.as_deref(),
                zeta.as_deref(),
                &x.prym.trace,
                &x.prym.order,
            );
            match verdict {
                Ok(()) => {
                    existing.insert(slug);
                }
                Err(why) => {
                    report.problem(format!(
                        "E' {slug} (model {}): {why}; not stored",
                        want["equation"].as_str().unwrap_or_default()
                    ));
                    conflicts.insert(slug);
                }
            }
        }
    }
    existing.retain(|s| !conflicts.contains(s));
    report.count("prym_curve_conflicts", conflicts.len() as u64);
    if !conflicts.is_empty() {
        let before = ready.len();
        ready.retain(|x| !conflicts.contains(&x.slug));
        report.count("prym_crosscheck_failures", (before - ready.len()) as u64);
        curves.retain(|s, _| !conflicts.contains(s));
    }
    report.count("prym_curves_seen", curves.len() as u64);
    report.count("prym_curves_existing", existing.len() as u64);

    let src = client
        .query_one(
            // The uri keeps this source distinct under UNIQUE (kind, repository, path, sha256, uri),
            // as enrich's 'ecdb:enrich'.
            "WITH ins AS (INSERT INTO ec.source (kind, title, uri, provenance_class)
                          VALUES ('computation', 'ecdb Prym complements of cover sources', 'ecdb:cover-prym', 'internal')
                          ON CONFLICT DO NOTHING RETURNING id)
             SELECT id FROM ins UNION ALL
             SELECT id FROM ec.source WHERE kind = 'computation' AND uri = 'ecdb:cover-prym'
                AND repository IS NULL LIMIT 1",
            &[],
        )
        .await?
        .get::<_, i64>(0);
    let prov = ctx.provenance(src, METHOD_PROV, None).await?;
    let new: Vec<Value> = curves.iter().filter(|(s, _)| !existing.contains(*s)).map(|(_, v)| v.0.clone()).collect();
    let (mut invs, mut classes, mut links, mut zetas, mut aliases) = (vec![], vec![], vec![], vec![], vec![]);
    let mut seen_class = BTreeSet::new();
    for (slug, (_, inv, class, link, zeta)) in &curves {
        invs.push(inv.clone());
        if seen_class.insert(class["label"].as_str().unwrap_or_default().to_string()) {
            classes.push(class.clone());
        }
        links.push(link.clone());
        zetas.push(zeta.clone());
        aliases.push(json!({ "key": slug, "namespace": "icv1_slug", "alias": slug, "retired": false }));
    }
    report.count("prym_curves_new", bulk(client, CURVE_SQL, &new, prov).await?);
    report.count("prym_invariants", bulk(client, INVARIANT_SQL, &invs, prov).await?);
    report.count("prym_aliases", bulk1(client, ALIAS_SQL, &aliases).await?);
    report.count("prym_isogeny_classes", bulk(client, CLASS_SQL, &classes, prov).await?);
    report.count("prym_jacobians_linked", bulk1(client, LINK_SQL, &links).await?);
    report.count("prym_zeta", bulk(client, ZETA_SQL, &zetas, prov).await?);

    let (mut relations, mut components) = (vec![], vec![]);
    for x in &ready {
        let k = &x.job.fq;
        let g: Vec<String> = x.prym.g.iter().map(|e| k.format(e)).collect();
        let map = json!({
            "explicit": true,
            "via_cover_source_curve_id": x.job.h_id,
            "construction": PRIME_CONSTRUCTION,
            "g": { "g0": g[0], "g1": g[1], "g2": g[2] },
            "cover_source": "H: v^2 = G(u^2), G(x) = x^3 + g2 x^2 + g1 x + g0",
            "h_to_e": "(u, v) -> (x, y) = (u^2, v) onto v^2 = G(x), the recorded cover (curve_morphism)",
            "h_to_e_prime": "(u, v) -> (X, Y) = (g0/u^2 + g1/3, g0 v/u^3) onto E'",
            "quartic_to_short": "w^2 = x G(x) -> Y^2 = X^3 + A X + B by X = g0/x + g1/3, Y = g0 w/x^2",
            "theorem": "Jac(H) is isogenous over F_q to E x E', the quotients of H by (u, v) -> (-u, v) and (-u, -v); \
                        E' is the Prym variety of H -> E",
        });
        let self_complement = x.job.targets.iter().any(|t| t.slug.as_deref() == Some(x.slug.as_str()));
        if self_complement {
            report.count("prym_self_complements", 1);
        }
        for t in &x.job.targets {
            if t.slug.as_deref() != Some(x.slug.as_str()) {
                relations.push(json!({ "source_id": t.curve_id, "slug": x.slug, "map": map, "status": x.status }));
            }
        }
        // One factor E: a short Weierstrass target when there is one (two targets are two models of one curve).
        let e = x.job.targets.iter().find(|t| t.model == "short_weierstrass").or_else(|| x.job.targets.first());
        let Some(e) = e else { continue };
        let comp = |what: &str| json!({ "factor": what, "map": map });
        if self_complement {
            components.push(json!({
                "av_id": x.job.h_av, "curve_id": e.curve_id, "slug": null, "multiplicity": 2,
                "map": comp("E = E'"), "status": x.status,
            }));
        } else {
            components.push(json!({
                "av_id": x.job.h_av, "curve_id": e.curve_id, "slug": null, "multiplicity": 1,
                "map": comp("E (the target)"), "status": x.status,
            }));
            components.push(json!({
                "av_id": x.job.h_av, "curve_id": null, "slug": x.slug, "multiplicity": 1,
                "map": comp("E' (the Prym complement)"), "status": x.status,
            }));
        }
    }
    // A row already present (the same E -> E' from another source H) is counted, not replaced.
    let n_rel = bulk(client, RELATION_SQL, &relations, prov).await?;
    report.count("prym_relations", n_rel);
    report.count("prym_relations_already_present", (relations.len() as u64).saturating_sub(n_rel));
    let n_comp = bulk(client, COMPONENT_SQL, &components, prov).await?;
    report.count("prym_av_components", n_comp);
    report.count("prym_av_components_already_present", (components.len() as u64).saturating_sub(n_comp));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import::cover_lpoly::{genus2_l, quotient_traces};
    use num_traits::Zero;

    fn fp(p: u64) -> Fq {
        Fq::prime(BigInt::from(p))
    }

    fn e(k: &Fq, n: i64) -> Elt {
        k.from_int(n)
    }

    /// The even sextic G(u^2) of `prime_quadratic_pullback_v1` for F = x^3 + a x + b:
    /// G(x) = F(x + c) for the first c with F(c) != 0 (as the cover_lpoly tests).
    fn cover_sextic(p: u64, a: u64, b: u64) -> Vec<u64> {
        let c = (0..4u64).find(|&c| !(c * c * c + a * c + b).is_multiple_of(p)).unwrap();
        let g0 = (c * c * c + a * c + b) % p;
        vec![g0, 0, (3 * c * c + a) % p, 0, 3 * c % p, 0, 1]
    }

    fn elts(k: &Fq, f: &[u64]) -> Vec<Elt> {
        f.iter().map(|&c| e(k, c as i64)).collect()
    }

    fn brute_short(p: u64, a: u64, b: u64) -> u64 {
        let mut n = 1;
        for x in 0..p {
            for y in 0..p {
                if (y * y) % p == (x * x % p * x + a * x + b) % p {
                    n += 1;
                }
            }
        }
        n
    }

    /// Points of the smooth model of w^2 = x^4 + g2 x^3 + g1 x^2 + g0 x: the
    /// affine pairs plus the two points at infinity (leading coefficient 1).
    fn brute_quartic(p: u64, g0: u64, g1: u64, g2: u64) -> u64 {
        let mut n = 2;
        for x in 0..p {
            let r = (x * x % p * x % p * x + g2 * (x * x % p * x % p) + g1 * (x * x % p) + g0 * x) % p;
            n += (0..p).filter(|&w| w * w % p == r).count() as u64;
        }
        n
    }

    fn brute_j(p: u64, a: u64, b: u64) -> u64 {
        let k = fp(p);
        let num = 1728 * 4 * a % p * a % p * a % p;
        let den = (4 * a % p * a % p * a + 27 * b % p * b) % p;
        let r = k.div(&e(&k, num as i64), &e(&k, den as i64)).unwrap();
        r[0].to_u64().unwrap()
    }

    fn done(o: Outcome) -> Prym {
        match o {
            Outcome::Done(p) => *p,
            other => panic!("expected a result, got {other:?}"),
        }
    }

    /// Brute force on tiny fields: for every short Weierstrass target over
    /// F_p, p in {7, 11, 13, 17, 19, 23}, the short model of E' has as many
    /// points as the quartic w^2 = x G(x) and as q + 1 - t' from the
    /// L-polynomial, and both j-invariants agree with a direct evaluation.
    #[test]
    fn tiny_fields_agree_with_brute_force() {
        let mut checked = 0;
        for p in [7u64, 11, 13, 17, 19, 23] {
            let k = fp(p);
            for a in 0..p {
                for b in 0..p {
                    if (4 * a * a * a + 27 * b * b).is_multiple_of(p) {
                        continue;
                    }
                    let f = cover_sextic(p, a, b);
                    let (g0, g1, g2) = (f[0], f[2], f[4]);
                    // H must be smooth: x G(x) squarefree (G(x) has no repeated root as E is smooth, g0 != 0).
                    let fe = elts(&k, &f);
                    let t = p as i128 + 1 - brute_short(p, a, b) as i128;
                    // t' by listing the quartic w^2 = x G(x) itself; the BSGS of cover_lpoly agrees when conclusive.
                    let tp = p as i128 + 1 - brute_quartic(p, g0, g1, g2) as i128;
                    if let Ok((t_bsgs, tp_bsgs)) = quotient_traces(&Fp64 { p }, &f) {
                        assert_eq!((t_bsgs, tp_bsgs), (t, tp));
                    }
                    let l = genus2_l(&BigInt::from(p), &BigInt::from(t), &BigInt::from(tp));
                    let out = derive(&k, &fe, &l, &BigInt::from(t));
                    let prym = done(out);
                    let (sa, sb) = (prym.a[0].to_u64().unwrap(), prym.b[0].to_u64().unwrap());
                    let n_short = brute_short(p, sa, sb);
                    assert_eq!(n_short, brute_quartic(p, g0, g1, g2), "p={p} a={a} b={b}");
                    assert_eq!(BigInt::from(n_short), prym.order);
                    assert_eq!(prym.trace, BigInt::from(tp));
                    assert_eq!(prym.j[0].to_u64().unwrap(), brute_j(p, sa, sb));
                    checked += 1;
                }
            }
        }
        // Every smooth target: sum of p^2 minus the singular (a, b) for these six primes.
        assert_eq!(checked, 1428);
    }

    /// Pinned against the L-polynomials ecdb holds for live cover sources
    /// (cover_lpoly's `live_cover_rows_have_the_independent_l_polynomial`):
    /// icv1-fp10-t15-87f382ed over GF(571), L = [1, 22, 587, ...], so t' = -37,
    /// #E' = 609; and icv1-fp24-t553-4d349218 over GF(8416129),
    /// L = [1, -2156, 17718717, ...], so t' = 1603. (A, B), j and the slug
    /// match an independent Python reference (not committed).
    #[test]
    fn live_sources_pin_the_complement() {
        let k = fp(571);
        let f = elts(&k, &[137, 0, 508, 0, 0, 0, 1]);
        let q = BigInt::from(571);
        let l: Vec<BigInt> = [1, 22, 587, 571 * 22, 571 * 571].iter().map(|&x| BigInt::from(x)).collect();
        let prym = done(derive(&k, &f, &l, &BigInt::from(15)));
        assert_eq!((prym.a.clone(), prym.b.clone()), (e(&k, 390), e(&k, 247)));
        assert_eq!(prym.trace, BigInt::from(-37));
        assert_eq!(prym.order, BigInt::from(609));
        assert_eq!(prym.j, e(&k, 237));
        assert_eq!(BigInt::from(brute_short(571, 390, 247)), prym.order);
        let (slug, mj, _, coeff) = icv1_of(&k, &prym).unwrap();
        assert_eq!(slug, "icv1-fp10-tm37-338863bc");
        assert_eq!(mj, r#"{"a":"390","b":"247","field":"fp-571","form":"y^2=x^3+a*x+b","p":"571","v":"1"}"#);
        assert_eq!(coeff["a"], "390");
        assert_eq!(complement_trace(&q, &l, &BigInt::from(15)), Ok(BigInt::from(-37)));

        let p = 8416129u64;
        let k = fp(p);
        let f = elts(&k, &[1713668, 0, 1040051, 0, 0, 0, 1]);
        let qi = p as i128;
        let l: Vec<BigInt> = [1, -2156, 17718717, qi * -2156, qi * qi].iter().map(|&x| BigInt::from(x)).collect();
        let prym = done(derive(&k, &f, &l, &BigInt::from(553)));
        assert!(!prym.exhaustive);
        assert_eq!(prym.trace, BigInt::from(1603));
        assert_eq!(prym.order, BigInt::from(qi + 1 - 1603));
        assert_eq!((prym.a.clone(), prym.b.clone()), (e(&k, 6325923), e(&k, 8070383)));
        assert_eq!(prym.j, e(&k, 4133712));
    }

    /// docs/curves/ICV1.md's extension example: y^2 = x^3 + x + 1 over
    /// GF(5)[t]/(t^2 + 2), #E = 27, is icv1-fp3k2-tm1-dd44529a.
    #[test]
    fn extension_icv1_reproduces_the_spec_example() {
        let k = Fq::extension(BigInt::from(5), &[BigInt::from(2), BigInt::from(0)]);
        let one = k.from_int(1);
        let (slug, mj, sha, t) = extension_icv1(&k, &one, &one, &BigInt::from(27)).unwrap();
        assert_eq!(slug, "icv1-fp3k2-tm1-dd44529a");
        assert_eq!(
            mj,
            r#"{"a":["1","0"],"b":["1","0"],"field":"fpk-5-2-90ac2fda","form":"y^2=x^3+a*x+b","k":"2","modulus":["2","0"],"p":"5","v":"1"}"#
        );
        assert!(sha.starts_with("dd44529a"));
        assert_eq!(t, BigInt::from(-1));
        assert!(extension_icv1(&k, &one, &one, &BigInt::from(100)).is_err());
    }

    /// Over GF(13^2) the complement's trace follows the subfield:
    /// t'_{q^2} = t'^2 - 2 * 13, and the slug is an extension slug.
    #[test]
    fn extension_field_complement_follows_the_subfield() {
        let p = 13u64;
        let f = cover_sextic(p, 2, 3);
        let (t, tp) = quotient_traces(&Fp64 { p }, &f).unwrap();
        let k = Fq::extension(BigInt::from(p), &[BigInt::from(11), BigInt::from(0)]);
        let lift: Vec<Elt> = f.iter().map(|&c| vec![BigInt::from(c), BigInt::zero()]).collect();
        let q = BigInt::from(169);
        let (te, tpe) = (t * t - 26, tp * tp - 26);
        let l = genus2_l(&q, &BigInt::from(te), &BigInt::from(tpe));
        let prym = done(derive(&k, &lift, &l, &BigInt::from(te)));
        assert_eq!(prym.trace, BigInt::from(tpe));
        // The base-field complement's model, lifted, is the same model.
        let kp = fp(p);
        let base = done(derive(
            &kp,
            &elts(&kp, &f),
            &genus2_l(&BigInt::from(p), &BigInt::from(t), &BigInt::from(tp)),
            &BigInt::from(t),
        ));
        assert_eq!(prym.a, vec![base.a[0].clone(), BigInt::zero()]);
        let (slug, ..) = icv1_of(&k, &prym).unwrap();
        assert!(slug.starts_with("icv1-fp4k2-t"), "{slug}");
    }

    /// Corrupted inputs are failures or skips, never results.
    #[test]
    fn corrupted_inputs_are_rejected() {
        let k = fp(571);
        let f = elts(&k, &[137, 0, 508, 0, 0, 0, 1]);
        let good: Vec<BigInt> = [1, 22, 587, 571 * 22, 571 * 571].iter().map(|&x| BigInt::from(x)).collect();
        // a2 off by one: t is no longer a factor.
        let mut bad = good.clone();
        bad[2] += 1;
        assert!(matches!(derive(&k, &f, &bad, &BigInt::from(15)), Outcome::Fail(_)));
        // A wrong target trace.
        assert!(matches!(derive(&k, &f, &good, &BigInt::from(17)), Outcome::Fail(_)));
        // A consistent but false L = L_E L_fake (t' + 2): the shape and a2 pass, the
        // independent order of the short model does not.
        let fake = genus2_l(&BigInt::from(571), &BigInt::from(15), &BigInt::from(-35));
        match derive(&k, &f, &fake, &BigInt::from(15)) {
            Outcome::Fail(m) => assert!(m.contains("from the short model"), "{m}"),
            other => panic!("expected a failure, got {other:?}"),
        }
        // A broken shape (q a1 T^3 term) and a non-even sextic.
        let mut shape = good.clone();
        shape[3] += 1;
        assert!(matches!(derive(&k, &f, &shape, &BigInt::from(15)), Outcome::Fail(_)));
        let mut odd = f.clone();
        odd[1] = e(&k, 1);
        assert!(matches!(derive(&k, &odd, &good, &BigInt::from(15)), Outcome::Skip("unsupported_shape")));
        // A field of characteristic 3 is outside the construction.
        assert!(matches!(
            derive(&fp(3), &elts(&fp(3), &[1, 0, 0, 0, 0, 0, 1]), &good, &BigInt::from(0)),
            Outcome::Skip(_)
        ));
    }

    /// An existing curve is reused only when it agrees with E' in every stored
    /// fact; a different slug, a non-genus-1 or wrong-trace class link, or a
    /// wrong stored #E(F_q) is a conflict (never overwritten).
    #[test]
    fn existing_curves_must_agree() {
        let (tp, n) = (BigInt::from(-37), BigInt::from(609));
        let s = "icv1-fp10-tm37-338863bc";
        assert_eq!(existing_agrees(s, Some(s), None, None, None, &tp, &n), Ok(()));
        assert_eq!(existing_agrees(s, Some(s), Some(1), Some("-37"), Some("609"), &tp, &n), Ok(()));
        assert!(existing_agrees(s, None, None, None, None, &tp, &n).is_err());
        assert!(existing_agrees(s, Some("icv1-fp10-tm37-00000000"), None, None, None, &tp, &n).is_err());
        assert!(existing_agrees(s, Some(s), Some(2), Some("-37"), None, &tp, &n).is_err());
        assert!(existing_agrees(s, Some(s), Some(1), Some("37"), None, &tp, &n).is_err());
        assert!(existing_agrees(s, Some(s), None, None, Some("610"), &tp, &n).is_err());
        assert!(existing_agrees(s, Some(s), Some(1), Some("-37"), Some("x"), &tp, &n).is_err());
        // The class link never replaces a stored one.
        assert!(LINK_SQL.contains("isogeny_class_id IS NULL"));
        assert!(!LINK_SQL.contains("IS DISTINCT FROM"));
    }

    /// The two j formulas agree on random models over a 61-bit prime, and the
    /// quartic formula alone differs from the short one on a corrupted g1.
    #[test]
    fn j_two_ways() {
        let p = (1u64 << 61) - 1;
        let k = fp(p);
        let mut s = 0x9e3779b97f4a7c15u64;
        for _ in 0..50 {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let g: Vec<Elt> = (0..3).map(|i| e(&k, ((s >> (i * 20)) % 1_000_003) as i64 + 1)).collect();
            let (a, b) = short_model(&k, &g[0], &g[1], &g[2]).unwrap();
            let Ok(j) = short_j(&k, &a, &b) else { continue };
            assert_eq!(quartic_j(&k, &g[0], &g[1], &g[2]).unwrap(), j);
            let g1x = k.add(&g[1], &k.from_int(1));
            if let Ok(jx) = quartic_j(&k, &g[0], &g1x, &g[2]) {
                assert_ne!(jx, j);
            }
        }
    }
}
