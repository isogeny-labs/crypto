//! Identity, label and consistency helpers shared by the importers and the
//! API: canonical JSON and SHA-256 (EC1), ICV1 field hashes, LMFDB-style
//! Weil-polynomial labels, the Hasse bound, retired-name detection.

use anyhow::{bail, Result};
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{Signed, Zero};
use regex::Regex;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::sync::OnceLock;

pub fn sha256_hex(s: &str) -> String {
    hex::encode(Sha256::digest(s.as_bytes()))
}

/// Sorted-key compact UTF-8 JSON, the EC1 hash preimage convention
/// (`tools/curve_identity.py`, `canonical`). Floats are refused.
pub fn canonical_json(v: &Value) -> Result<String> {
    fn check(v: &Value) -> Result<()> {
        match v {
            Value::Number(n) => {
                let s = n.to_string();
                if s.contains(['.', 'e', 'E']) {
                    bail!("identity records require JSON without floats");
                }
                Ok(())
            }
            Value::Array(a) => a.iter().try_for_each(check),
            Value::Object(o) => o.values().try_for_each(check),
            _ => Ok(()),
        }
    }
    check(v)?;
    // serde_json::Map is a BTreeMap here (no preserve_order feature): keys
    // serialize in byte order, which is code-point order, as Python sorts them.
    Ok(serde_json::to_string(v)?)
}

/// EC1 curve identity digest: SHA-256 of {"field": field, "curve": curve minus curve_id}.
pub fn ec1_preimage(field: &Value, curve: &Value) -> Result<String> {
    let mut clean = curve.clone();
    if let Some(o) = clean.as_object_mut() {
        o.remove("curve_id");
    }
    canonical_json(&serde_json::json!({ "field": field, "curve": clean }))
}

/// Expected ICV1 field id for a binary modulus written in hex (`0x...`).
pub fn icv1_binary_field(m: u64, modulus_hex: &str) -> String {
    let h = sha256_hex(&format!("f2m-modulus:{modulus_hex}"));
    format!("f2m-{m}-{}", &h[..8])
}

/// Expected ICV1 field id for GF(p^k) with modulus low coefficients c_0..c_{k-1}.
pub fn icv1_extension_field(p: &BigInt, low: &[BigInt]) -> String {
    let coeffs: Vec<String> = low.iter().map(|c| c.mod_floor(p).to_string()).collect();
    let h = sha256_hex(&format!("fpk-modulus:{p}:{}", coeffs.join(",")));
    format!("fpk-{p}-{}-{}", low.len(), &h[..8])
}

/// One LMFDB base-26 coefficient: a = 0, b = 1, ..., ba = 26; a negative
/// value is prefixed by `a` (so -5 is `af`).
pub fn lmfdb_letters(a: &BigInt) -> String {
    fn digits(mut n: BigInt) -> String {
        if n.is_zero() {
            return "a".into();
        }
        let base = BigInt::from(26);
        let mut out = Vec::new();
        while !n.is_zero() {
            let (q, r) = n.div_mod_floor(&base);
            let d: u32 = r.try_into().unwrap_or(0);
            out.push(char::from(b'a' + d as u8));
            n = q;
        }
        out.iter().rev().collect()
    }
    if a.is_negative() {
        format!("a{}", digits(-a))
    } else {
        digits(a.clone())
    }
}

/// LMFDB-style isogeny-class label g.q.a_1_..._a_g for an L-polynomial
/// 1 + a_1 T + ... + q^g T^{2g} (ascending coefficients).
pub fn weil_label(g: usize, q: &BigInt, l_poly: &[BigInt]) -> Result<String> {
    if l_poly.len() != 2 * g + 1 {
        bail!("L-polynomial of genus {g} needs {} coefficients", 2 * g + 1);
    }
    let parts: Vec<String> = l_poly[1..=g].iter().map(lmfdb_letters).collect();
    Ok(format!("{g}.{q}.{}", parts.join("_")))
}

/// L(T) = 1 - t T + q T^2 of a genus-1 isogeny class.
pub fn genus1_l_poly(t: &BigInt, q: &BigInt) -> Vec<BigInt> {
    vec![BigInt::from(1), -t, q.clone()]
}

/// Hasse: t^2 <= 4q.
pub fn hasse_ok(t: &BigInt, q: &BigInt) -> bool {
    t * t <= BigInt::from(4) * q
}

pub fn pow(base: &BigInt, e: u32) -> BigInt {
    num_traits::pow(base.clone(), e as usize)
}

/// Exponents of the set bits of a binary modulus, descending (EC1 field record order).
pub fn modulus_exponents(modulus: &BigUint) -> Vec<u64> {
    (0..modulus.bits()).rev().filter(|&i| modulus.bit(i)).collect()
}

pub fn polynomial_shape(weight: usize) -> &'static str {
    match weight {
        3 => "trinomial",
        5 => "pentanomial",
        _ => "sparse",
    }
}

/// Forms retired by docs/curves/ICV1.md ("Names that are retired"). They
/// resolve but carry no model.
pub fn is_retired_name(s: &str) -> bool {
    static RE: OnceLock<Vec<Regex>> = OnceLock::new();
    let res = RE.get_or_init(|| {
        [
            r"^K_?[01₀₁]\s*/\s*(GF\(|F_)?2\^\d+\)?$",
            r"^k[01]n\d+$",
            r"^bench-\d+bit$",
            r"^generated-\d+bit-\d+$",
            r"^random-binary-n\d+-b\d+$",
            r"^E_\{[^}]*\}/GF\(2\^\d+\) over GF\(2\^\d+\)$",
        ]
        .iter()
        .map(|p| Regex::new(p).expect("static regex"))
        .collect()
    });
    res.iter().any(|r| r.is_match(s))
}

/// Largest integer every JSON client holds exactly (2^53 - 1, an IEEE double's mantissa).
pub const SAFE_INTEGER: i64 = 9_007_199_254_740_991;

/// Parse an integer that a producer wrote either as a JSON number or a string.
pub fn as_bigint(v: &Value) -> Option<BigInt> {
    match v {
        Value::Number(n) => n.to_string().parse().ok(),
        Value::String(s) => s.parse().ok(),
        _ => None,
    }
}

/// Fundamental-discriminant quadratic field data: (defining polynomial, LMFDB label).
pub fn imaginary_quadratic_field(d: &BigInt) -> Result<(Vec<BigInt>, String)> {
    if !d.is_negative() {
        bail!("not an imaginary quadratic discriminant: {d}");
    }
    let four = BigInt::from(4);
    let r = d.mod_floor(&four);
    let poly = if r.is_zero() {
        // x^2 - d/4
        vec![-(d / &four), BigInt::zero(), BigInt::from(1)]
    } else if r == BigInt::from(1) {
        // x^2 - x + (1 - d)/4
        vec![(BigInt::from(1) - d) / &four, BigInt::from(-1), BigInt::from(1)]
    } else {
        bail!("{d} is not a discriminant");
    };
    Ok((poly, format!("2.0.{}.1", -d)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bi(s: &str) -> BigInt {
        s.parse().unwrap()
    }

    #[test]
    fn lmfdb_labels_match_recorded_examples() {
        // LMFDB av_fq_isog rows read from ml-cryptanalysis/data/lmfdb and
        // research/ghs_hess_c_parameter_audit_20261026/lmfdb_548.json.
        assert_eq!(weil_label(1, &bi("8"), &[bi("1"), bi("-5"), bi("8")]).unwrap(), "1.8.af");
        let l: Vec<BigInt> = ["1", "0", "0", "35", "0", "0", "512"].iter().map(|s| bi(s)).collect();
        assert_eq!(weil_label(3, &bi("8"), &l).unwrap(), "3.8.a_a_bj");
        assert_eq!(lmfdb_letters(&bi("26")), "ba");
        assert_eq!(lmfdb_letters(&bi("0")), "a");
    }

    #[test]
    fn icv1_field_hashes_match_registry() {
        // docs/curves/ICV1.md examples and registry rows
        assert_eq!(icv1_binary_field(41, "0x20000000009"), "f2m-41-39c74c32");
        assert_eq!(icv1_binary_field(7, "0x83"), "f2m-7-e1a6f45a");
        assert_eq!(icv1_extension_field(&bi("5"), &[bi("2"), bi("0")]), "fpk-5-2-90ac2fda");
    }

    #[test]
    fn ec1_preimage_reproduces_registry_uid() {
        // docs/curves/registry.json, Koblitz a = 0 over GF(2^7): EC1N7Ce0hb6d297a2ca08
        let field = serde_json::json!({"characteristic": 2, "degree": 7, "representation": "polynomial",
            "modulus_exponents": [7, 1, 0],
            "element_encoding": "hex polynomial coefficient bitset, least significant bit is constant"});
        let curve = serde_json::json!({"model": "binary Weierstrass", "coefficients": [1, 0, 0, 0, 1],
            "subgroup_order": "29", "cofactor": 4, "generator": ["0x26", "0x5"],
            "target_group": "prime-order subgroup"});
        let pre = ec1_preimage(&field, &curve).unwrap();
        assert_eq!(sha256_hex(&pre), "b6d297a2ca085e9d7b8a6858df7421083e12e740c89b1647660b413e00ed02ff");
    }

    #[test]
    fn retired_names() {
        assert!(is_retired_name("K_0 / GF(2^41)"));
        assert!(is_retired_name("K_1/F_2^7"));
        assert!(is_retired_name("k0n41"));
        assert!(is_retired_name("bench-20bit"));
        assert!(is_retired_name("E_{0,2}/GF(2^2) over GF(2^6)"));
        assert!(!is_retired_name("sect163k1"));
        assert!(!is_retired_name("icv1-f2m7-t13-616700dd"));
    }

    #[test]
    fn hasse_and_quadratic_fields() {
        assert!(hasse_ok(&bi("13"), &bi("128")));
        assert!(!hasse_ok(&bi("23"), &bi("128")));
        let (poly, label) = imaginary_quadratic_field(&bi("-7")).unwrap();
        assert_eq!(poly, vec![bi("2"), bi("-1"), bi("1")]);
        assert_eq!(label, "2.0.7.1");
        let (poly, _) = imaginary_quadratic_field(&bi("-4")).unwrap();
        assert_eq!(poly, vec![bi("1"), bi("0"), bi("1")]);
    }
}
