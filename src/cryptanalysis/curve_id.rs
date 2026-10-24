//! **ICV1 curve identities** — the one way this repository names a curve.
//!
//! The specification is `docs/curves/ICV1.md` and the reference
//! implementation is `scripts/curve_id.py`; this module reproduces it byte
//! for byte, and `tests/curve_id.rs` pins vectors that file produced.  An
//! identity is
//!
//! ```text
//! ICV1:<field>:<trace>:<order>:<j>:<end>:<level>:<path>:<model12>
//! ```
//!
//! with a slug `icv1-<field tag>-t<trace>-<model8>` (a negative trace is
//! written `tm<|t|>`), which is the name a curve carries in text, tables and
//! reports.  `<model12>` is the SHA-256 of the curve's canonical model JSON,
//! so two models of one curve — another modulus, another Weierstrass form —
//! are two identities.
//!
//! Reports written before ICV1 name curves by the spellings they used then
//! (`K_0 / GF(2^41)`, `bench-24bit`, `random-binary-n27-b845462`, …).
//! Those files are frozen.  The registry (`docs/curves/registry.json`) maps
//! every such spelling to its identity; [`resolve`] and [`same_curve`] read
//! the alias map generated from it (`curve_aliases.json`, beside this file),
//! so code that replays a frozen report, or looks a curve up in a table keyed
//! the old way, accepts either name.

use std::collections::HashMap;
use std::sync::OnceLock;

use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

use crate::binary_ecc::IrreduciblePoly;
use crate::hash::sha256::sha256;

/// Version of the canonical model JSON.
pub const VERSION: &str = "1";

/// Every name the registry knows, normalised, to its slug: generated from
/// `docs/curves/registry.json` by `scripts/build_curve_registry.py` and kept
/// under `src/` so that a checkout of `src/` alone still builds.
const ALIASES: &str = include_str!("curve_aliases.json");

/// A curve's ICV1 identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurveId {
    /// `ICV1:<field>:<trace>:<order>:<j>:<end>:<level>:<path>:<model12>`.
    pub icv1: String,
    /// `icv1-<field tag>-t<trace>-<model8>`, the name used in text.
    pub slug: String,
    /// The canonical model JSON the model hash is taken over.
    pub model_json: String,
}

fn hex_digest(s: &str) -> String {
    sha256(s.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn hex(v: &BigUint) -> String {
    format!("0x{v:x}")
}

/// `x^m + Σ x^i` over the low terms, as the integer whose bit `i` is the
/// coefficient of `x^i`.
pub fn modulus_integer(p: &IrreduciblePoly) -> BigUint {
    let mut f = BigUint::zero();
    f.set_bit(u64::from(p.degree), true);
    for &i in &p.low_terms {
        f.set_bit(u64::from(i), true);
    }
    f
}

fn deg(f: &BigUint) -> u64 {
    f.bits().saturating_sub(1)
}

fn poly_mod(mut a: BigUint, f: &BigUint) -> BigUint {
    let df = deg(f);
    while !a.is_zero() && deg(&a) >= df {
        a ^= f << (deg(&a) - df);
    }
    a
}

fn clmul(a: &BigUint, b: &BigUint) -> BigUint {
    let mut r = BigUint::zero();
    for i in 0..b.bits() {
        if b.bit(i) {
            r ^= a << i;
        }
    }
    r
}

/// Inverse of a non-zero `a` modulo the irreducible `f` over `GF(2)`.
fn poly_inverse(a: &BigUint, f: &BigUint) -> Option<BigUint> {
    let (mut r0, mut r1) = (f.clone(), poly_mod(a.clone(), f));
    let (mut s0, mut s1) = (BigUint::zero(), BigUint::one());
    if r1.is_zero() {
        return None;
    }
    while !r1.is_one() {
        let mut q = BigUint::zero();
        let mut r = r0.clone();
        while !r.is_zero() && deg(&r) >= deg(&r1) {
            let shift = deg(&r) - deg(&r1);
            q.set_bit(shift, true);
            r ^= &r1 << shift;
        }
        if r.is_zero() {
            return None; // f was not irreducible
        }
        let s = &s0 ^ clmul(&q, &s1);
        r0 = std::mem::replace(&mut r1, r);
        s0 = std::mem::replace(&mut s1, s);
    }
    Some(poly_mod(s1, f))
}

/// `|q + 1 − #E| ≤ 2√q`: false for a subgroup order passed as the group's.
fn hasse_ok(q: &BigUint, order: &BigUint) -> bool {
    let t = BigInt::from(q.clone()) + 1 - BigInt::from(order.clone());
    &t * &t <= BigInt::from(q.clone()) * 4
}

fn identity(
    field: &str,
    field_tag: &str,
    trace: &BigInt,
    order: &BigUint,
    j: &str,
    end: &str,
    model_json: String,
) -> CurveId {
    let model = hex_digest(&model_json);
    let icv1 = format!(
        "ICV1:{field}:{trace}:{order}:{j}:{end}:unk:r:{}",
        &model[..12]
    );
    let t = if trace < &BigInt::zero() {
        format!("tm{}", -trace)
    } else {
        format!("t{trace}")
    };
    let slug = format!("icv1-{field_tag}-{t}-{}", &model[..8]);
    CurveId {
        icv1,
        slug,
        model_json,
    }
}

/// `y² + xy = x³ + a·x² + b` over `GF(2^m) = GF(2)[x]/(modulus)`.
///
/// `a`, `b` are polynomial-basis elements (bit `i` is `x^i`) and `order`
/// is `#E(GF(2^m))`, the whole group.  `end` is the certified
/// discriminant of `End(E)`, or `None` for `unk`.  Returns `None` for a
/// singular curve, a reducible modulus or an order outside the Hasse
/// interval (a subgroup order passed by mistake).
pub fn binary(
    m: u32,
    modulus: &BigUint,
    a: &BigUint,
    b: &BigUint,
    order: &BigUint,
    end: Option<i64>,
) -> Option<CurveId> {
    if deg(modulus) != u64::from(m) {
        return None;
    }
    let (a, b) = (poly_mod(a.clone(), modulus), poly_mod(b.clone(), modulus));
    let q = BigUint::one() << m;
    if b.is_zero() || !hasse_ok(&q, order) {
        return None;
    }
    let modhash = hex_digest(&format!("f2m-modulus:{}", hex(modulus)));
    let field = format!("f2m-{m}-{}", &modhash[..8]);
    let model_json = format!(
        r#"{{"a":"{}","b":"{}","field":"{field}","form":"y^2+xy=x^3+a*x^2+b","modulus":"{}","v":"{VERSION}"}}"#,
        hex(&a),
        hex(&b),
        hex(modulus)
    );
    let trace = BigInt::from(q) + 1 - BigInt::from(order.clone());
    let j = hex(&poly_inverse(&b, modulus)?);
    let end = end.map_or_else(|| "unk".to_string(), |d| d.to_string());
    Some(identity(
        &field,
        &format!("f2m{m}"),
        &trace,
        order,
        &j,
        &end,
        model_json,
    ))
}

/// The Koblitz curve `K_a : y² + xy = x³ + a·x² + 1` over `GF(2^n)`.
///
/// `End(E) ⊇ Z[τ]` with `τ² − t₁τ + 2 = 0`, `t₁ = ±1`, whose
/// discriminant `−7` is fundamental: `Z[τ]` is already the maximal order
/// of `Q(√−7)`, so `End(E) = Z[τ]` and the identity certifies `end = −7`.
pub fn koblitz(a: u8, n: u32, modulus: &BigUint, order: &BigUint) -> Option<CurveId> {
    if a > 1 {
        return None;
    }
    binary(
        n,
        modulus,
        &BigUint::from(a),
        &BigUint::one(),
        order,
        Some(-7),
    )
}

/// `y² = x³ + a·x + b` over `GF(p)`, `p` an odd prime above 3.
fn prime_identity(
    p: &BigUint,
    a: &BigUint,
    b: &BigUint,
    order: &BigUint,
    j: &BigUint,
) -> Option<CurveId> {
    let (a, b) = (a % p, b % p);
    let a3 = a.modpow(&BigUint::from(3u8), p);
    let disc = (BigUint::from(4u8) * &a3 + BigUint::from(27u8) * &b * &b) % p;
    if disc.is_zero() || j >= p || !hasse_ok(p, order) {
        return None;
    }
    let field = format!("fp-{p}");
    let model_json = format!(
        r#"{{"a":"{a}","b":"{b}","field":"{field}","form":"y^2=x^3+a*x+b","p":"{p}","v":"{VERSION}"}}"#
    );
    let trace = BigInt::from(p.clone()) + 1 - BigInt::from(order.clone());
    Some(identity(
        &field,
        &format!("fp{}", p.bits()),
        &trace,
        order,
        &j.to_string(),
        "unk",
        model_json,
    ))
}

pub fn prime(p: &BigUint, a: &BigUint, b: &BigUint, order: &BigUint) -> Option<CurveId> {
    let (a, b) = (a % p, b % p);
    let a3 = a.modpow(&BigUint::from(3u8), p);
    let disc = (BigUint::from(4u8) * &a3 + BigUint::from(27u8) * &b * &b) % p;
    if disc.is_zero() {
        return None;
    }
    let inv = disc.modpow(&(p - BigUint::from(2u8)), p);
    let j = (BigUint::from(1728u32 * 4) * a3 * inv) % p;
    prime_identity(p, &a, &b, order, &j)
}

/// Build the prime-field identity when the caller has already recomputed and
/// checked the model's j-invariant.  This avoids a second full-width inversion
/// in streaming verifiers that have just evaluated [`Model::j`](crate::cryptanalysis::isogeny_walk::curve::Model::j).
/// The model is still rejected when it is singular, outside the Hasse interval,
/// or `j` is not a canonical field element.
pub fn prime_with_verified_j(
    p: &BigUint,
    a: &BigUint,
    b: &BigUint,
    order: &BigUint,
    j: &BigUint,
) -> Option<CurveId> {
    prime_identity(p, a, b, order, j)
}

/// `y² = x³ + a·x + b` over `GF(p^k) = GF(p)[t]/(f)`, `f = t^k + c_{k−1}t^{k−1}
/// + … + c₀` with `modulus = [c₀, …, c_{k−1}]`, `a` and `b` coefficient
/// lists in the basis `1, t, …, t^{k−1}`, and `order = #E(GF(p^k))`, the
/// whole group: `extension_id` of `scripts/curve_id.py`, for a prime that
/// fits a word (`p < 2^31`).  The caller has checked that `f` is
/// irreducible.  `None` for `p < 5`, `k < 2`, a singular curve or an
///   order outside the Hasse interval.
pub fn extension(
    p: u64,
    modulus: &[u64],
    a: &[u64],
    b: &[u64],
    order: &BigUint,
) -> Option<CurveId> {
    let k = modulus.len();
    if !(5..1 << 31).contains(&p) || k < 2 || a.len() != k || b.len() != k {
        return None;
    }
    let md: Vec<u64> = modulus.iter().map(|c| c % p).collect();
    let reduce = |u: Vec<u64>| -> Vec<u64> {
        let mut u: Vec<u64> = u.into_iter().map(|x| x % p).collect();
        for i in (k..u.len()).rev() {
            let c = u[i];
            if c != 0 {
                u[i] = 0;
                for j in 0..k {
                    let sub = c * md[j] % p;
                    u[i - k + j] = (u[i - k + j] + p - sub) % p;
                }
            }
        }
        u.resize(k.max(u.len()), 0);
        u.truncate(k);
        u
    };
    let mul = |u: &[u64], v: &[u64]| -> Vec<u64> {
        let mut prod = vec![0u64; u.len() + v.len() - 1];
        for (i, &x) in u.iter().enumerate() {
            for (j, &y) in v.iter().enumerate() {
                prod[i + j] = (prod[i + j] + x * y) % p;
            }
        }
        reduce(prod)
    };
    let a: Vec<u64> = a.iter().map(|x| x % p).collect();
    let b: Vec<u64> = b.iter().map(|x| x % p).collect();
    let a3 = mul(&mul(&a, &a), &a);
    let b2 = mul(&b, &b);
    let disc = reduce(a3.iter().zip(&b2).map(|(x, y)| 4 * x + 27 * y).collect());
    if disc.iter().all(|&x| x == 0) {
        return None;
    }
    let coeffs: Vec<String> = md.iter().map(u64::to_string).collect();
    let field = format!(
        "fpk-{p}-{k}-{}",
        &hex_digest(&format!("fpk-modulus:{p}:{}", coeffs.join(",")))[..8]
    );
    let q = BigUint::from(p).pow(k as u32);
    if !hasse_ok(&q, order) {
        return None;
    }
    let list = |v: &[u64]| -> String {
        let items: Vec<String> = v.iter().map(|x| format!("\"{x}\"")).collect();
        format!("[{}]", items.join(","))
    };
    let model_json = format!(
        r#"{{"a":{},"b":{},"field":"{field}","form":"y^2=x^3+a*x+b","k":"{k}","modulus":{},"p":"{p}","v":"{VERSION}"}}"#,
        list(&a),
        list(&b),
        list(&md)
    );
    // disc⁻¹ = disc^(q − 2), by square and multiply on the exponent's bits.
    let mut inv = reduce(vec![1]);
    let mut base = disc;
    let e = &q - BigUint::from(2u8);
    for i in 0..e.bits() {
        if e.bit(i) {
            inv = mul(&inv, &base);
        }
        base = mul(&base, &base);
    }
    let a3x: Vec<u64> = a3.iter().map(|x| 1728 * 4 % p * x % p).collect();
    let j = mul(&a3x, &inv);
    let jstr: Vec<String> = j.iter().map(u64::to_string).collect();
    let trace = BigInt::from(q) + 1 - BigInt::from(order.clone());
    Some(identity(
        &field,
        &format!("fp{}k{k}", 64 - p.leading_zeros()),
        &trace,
        order,
        &jstr.join(","),
        "unk",
        model_json,
    ))
}

/// The comparison key for a spelling: case, whitespace, braces,
/// underscores, surrounding backticks and subscript digits folded away.
/// Identical to `normalise_alias` in `scripts/curve_id.py`.
pub fn normalise(name: &str) -> String {
    name.trim()
        .trim_matches('`')
        .chars()
        .filter(|c| !c.is_whitespace() && !matches!(c, '{' | '}' | '_'))
        .map(|c| match c {
            '₀'..='₉' => char::from(b'0' + (c as u32 - '₀' as u32) as u8),
            _ => c.to_ascii_lowercase(),
        })
        .collect()
}

/// Every name the registry knows, normalised, mapped to its curve's slug.
fn aliases() -> &'static HashMap<String, String> {
    static MAP: OnceLock<HashMap<String, String>> = OnceLock::new();
    MAP.get_or_init(|| {
        let doc: serde_json::Value =
            serde_json::from_str(ALIASES).expect("curve_aliases.json parses");
        doc["aliases"]
            .as_object()
            .into_iter()
            .flatten()
            .filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_string())))
            .collect()
    })
}

/// The slug of the curve `name` denotes: a slug, an ICV1 string, a
/// standard name or a registered legacy spelling.  `None` when the
/// registry does not know the name.
pub fn resolve(name: &str) -> Option<&'static str> {
    aliases().get(&normalise(name)).map(String::as_str)
}

/// Whether two names denote one curve: equal as written, or resolved by
/// the registry to one slug.  This is how a rebuilt instance is matched
/// against the name a frozen report recorded.
pub fn same_curve(a: &str, b: &str) -> bool {
    if a == b {
        return true;
    }
    let ra = resolve(a).or_else(|| a.starts_with("icv1-").then_some(a));
    let rb = resolve(b).or_else(|| b.starts_with("icv1-").then_some(b));
    matches!((ra, rb), (Some(x), Some(y)) if x == y)
}

// A measured ecbench child is a fresh process that asks one question of
// the alias table: which of a frozen report's names is this instance.
// Building the map for that parses 2.8 MB of JSON (about 150 million
// instructions, 97% of an IC child), so `find_same_curve` reads the
// generated text instead.  It is `json.dumps(indent=0)` of an object sorted
// by name, one `"<name>": "<slug>",` per line (`scripts/curve_id.py`,
// `alias_map_text`), so a name is found by binary search over the lines.
// Anything the search cannot read exactly (a name JSON would escape, or an
// escaped name met on the way) goes to the map, so every answer is the
// map's.

/// The `"aliases"` object of the alias table, from the newline before its
/// first entry.
fn alias_text() -> &'static str {
    const OPEN: &str = "\"aliases\": {";
    ALIASES
        .find(OPEN)
        .map_or("", |i| &ALIASES[i + OPEN.len()..])
}

/// [`resolve`] of one normalised name, by binary search over the lines
/// of the table's text.
fn resolve_key(key: &str) -> Option<&'static str> {
    let verbatim = |s: &str| !s.chars().any(|c| c == '"' || c == '\\' || c.is_control());
    let map = || aliases().get(key).map(String::as_str);
    if key.is_empty() || !verbatim(key) {
        return map();
    }
    let text = alias_text();
    // Every entry that can match starts in [lo, hi), just after a newline.
    let (mut lo, mut hi) = (0, text.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        let Some(nl) = text[mid.max(1) - 1..].find('\n') else {
            hi = mid;
            continue;
        };
        let start = mid.max(1) - 1 + nl + 1;
        if start >= hi {
            hi = mid;
            continue;
        }
        let Some(name) = text[start..]
            .strip_prefix('"')
            .and_then(|t| t.split('"').next())
        else {
            hi = start; // the closing brace: past the last entry
            continue;
        };
        if !verbatim(name) || name.contains('\\') {
            return map();
        }
        match name.cmp(key) {
            std::cmp::Ordering::Less => lo = start + 1,
            std::cmp::Ordering::Greater => hi = start,
            std::cmp::Ordering::Equal => {
                let rest = &text[start + name.len() + 2..];
                let value = rest.strip_prefix(": \"")?;
                return value.split('"').next();
            }
        }
    }
    None
}

/// The first `(name, value)` whose name denotes the same curve as
/// `instance`, in the sense of [`same_curve`], without building the alias
/// map: one binary search per name.
pub fn find_same_curve<'n, T>(
    instance: &str,
    candidates: impl IntoIterator<Item = (&'n str, T)>,
) -> Option<T> {
    fn resolved(name: &str) -> Option<&str> {
        resolve_key(&normalise(name)).or_else(|| name.starts_with("icv1-").then_some(name))
    }
    let target = resolved(instance);
    candidates.into_iter().find_map(|(name, value)| {
        let hit = name == instance || (target.is_some() && resolved(name) == target);
        hit.then_some(value)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The binary search agrees with the map on every name the table
    /// holds, the first and last included, on names it does not hold
    /// (before the first, after the last, between two), and on names JSON
    /// would escape.
    #[test]
    fn the_binary_search_agrees_with_the_map() {
        let map = aliases();
        assert!(map.len() > 1000, "alias table has {} names", map.len());
        let mut keys: Vec<&String> = map.keys().collect();
        keys.sort();
        for key in &keys {
            assert_eq!(resolve_key(key), map.get(*key).map(String::as_str), "{key}");
            let between = format!("{key}~");
            assert_eq!(resolve_key(&between), map.get(&between).map(String::as_str));
        }
        for key in [
            "",
            "no-such-curve",
            "icv1-fp20-t0-00000000",
            "a\"b",
            "a\\b",
            "secp256k",
            "!",
            "~~~~",
            "0",
        ] {
            assert_eq!(
                resolve_key(key),
                map.get(key).map(String::as_str),
                "{key:?}"
            );
        }
    }

    /// `find_same_curve` returns what a search with [`same_curve`] returns,
    /// for every row of the pinned calibration table against instances
    /// named every way the table and the registry name them.
    #[test]
    fn find_same_curve_is_same_curve() {
        let doc: serde_json::Value =
            serde_json::from_str(include_str!("../../docs/ic/calibration.json")).unwrap();
        let rows: Vec<&str> = doc["instances"]
            .as_object()
            .unwrap()
            .keys()
            .filter_map(|k| k.split_once('/').map(|(_, n)| n))
            .collect();
        let mut instances: Vec<String> = rows.iter().map(|r| r.to_string()).collect();
        instances.extend(rows.iter().filter_map(|r| resolve(r).map(str::to_string)));
        instances.extend(rows.iter().map(|r| r.to_uppercase()));
        instances.extend(
            [
                "icv1-fp20-t0-00000000",
                "unknown curve",
                "",
                "secp256k1",
                "P-256",
            ]
            .map(str::to_string),
        );
        for instance in &instances {
            let expected = rows.iter().position(|r| same_curve(r, instance));
            let found = find_same_curve(instance, rows.iter().enumerate().map(|(i, r)| (*r, i)));
            assert_eq!(found, expected, "{instance:?}");
        }
    }
}
