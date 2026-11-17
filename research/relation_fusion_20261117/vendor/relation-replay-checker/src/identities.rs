//! Strict original-input row identities, using the independent consumer arithmetic.
//! An identity receipt carries no basis or ideal-equality standing.
use super::{decode, degree, original, Arithmetic, Input, Member, Result, Stats};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Packet {
    version: u32,
    identities: Vec<Member>,
}

/// Successful identity-only verification. The fields cannot be constructed by callers.
#[derive(Debug)]
pub struct CheckedIdentities {
    stats: Stats,
}
impl CheckedIdentities {
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
}

/// Verify bounded identities against the original equations, never earlier rows.
/// No producer arithmetic or basis-construction function is called.
pub fn verify_json(input: &[u8], packet: &[u8]) -> Result<CheckedIdentities> {
    if input.len() > 4 * 1024 * 1024 || packet.len() > 1024 * 1024 {
        return Err("identity input/wire byte cap".into());
    }
    let input: Input = serde_json::from_slice(input).map_err(|e| format!("input JSON: {e}"))?;
    let packet: Packet =
        serde_json::from_slice(packet).map_err(|e| format!("identity JSON: {e}"))?;
    let n = input.variables.len();
    let originals = original(&input)?;
    if originals.len() != input.polynomials.len() {
        return Err("identity API rejects zero originals; indices never renumber".into());
    }
    if packet.version != 1 || packet.identities.len() > 32 {
        return Err("identity version/row budget".into());
    }
    // All limits are checked before any polynomial product.
    for row in &packet.identities {
        if row.polynomial.len() > 32 || row.proof.len() > 32 {
            return Err("identity polynomial/proof term budget".into());
        }
        for t in &row.polynomial {
            if t.exponents.len() != n || degree(&t.exponents)? > 6 {
                return Err("identity target shape/degree".into());
            }
        }
        for t in &row.proof {
            let g = originals
                .get(t.generator)
                .ok_or("identity original reference")?;
            if t.exponents.len() != n
                || degree(&t.exponents)? > 2
                || super::polynomial_degree(g) + degree(&t.exponents)? > 6
            {
                return Err("identity multiplier/product degree".into());
            }
        }
    }
    let mut arithmetic = Arithmetic {
        n,
        stats: Stats::default(),
    };
    for row in &packet.identities {
        let target = decode(&row.polynomial, n)?;
        arithmetic.identity(&target, &row.proof, &originals, None)?;
    }
    Ok(CheckedIdentities {
        stats: arithmetic.stats,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    fn fixture() -> (Value, Value) {
        (
            json!({"prime":2147483647,"variables":["x","y","z"],"polynomials":[[{"exponents":[1,0,0],"coefficient":1}]]}),
            json!({"version":1,"identities":[{"polynomial":[{"exponents":[2,0,0],"coefficient":1}],"proof":[{"generator":0,"exponents":[1,0,0],"coefficient":1}]}]}),
        )
    }
    fn check(i: &Value, w: &Value) -> Result<CheckedIdentities> {
        verify_json(
            &serde_json::to_vec(i).unwrap(),
            &serde_json::to_vec(w).unwrap(),
        )
    }
    #[test]
    fn valid_exact_identity_and_empty_batch() {
        let (i, mut w) = fixture();
        let c = check(&i, &w).unwrap();
        assert_eq!(c.stats().identities, 1);
        assert_eq!(c.stats().basis_rows, 0);
        assert_eq!(c.stats().cofactor_terms, 1);
        w["identities"] = json!([]);
        assert_eq!(check(&i, &w).unwrap().stats().identities, 0);
    }
    #[test]
    fn scalar_and_shift_tampers_refused() {
        let (i, w) = fixture();
        for part in ["polynomial", "proof"] {
            let mut v = w.clone();
            v["identities"][0][part][0]["coefficient"] = json!(2);
            assert!(check(&i, &v).is_err());
        }
        let mut v = w.clone();
        v["identities"][0]["proof"][0]["exponents"] = json!([0, 1, 0]);
        assert!(check(&i, &v).is_err());
    }
    #[test]
    fn schema_and_api_confusion_refused() {
        let (i, w) = fixture();
        assert!(super::super::verify_json(
            &serde_json::to_vec(&i).unwrap(),
            &serde_json::to_vec(&w).unwrap()
        )
        .is_err());
        for key in ["basis", "definitions", "pair_reductions"] {
            let mut v = w.clone();
            v[key] = json!([]);
            assert!(check(&i, &v).is_err());
        }
        let mut v = w.clone();
        v["version"] = json!(0);
        assert!(check(&i, &v).is_err());
        let mut v = w.clone();
        v["identities"][0]["proof"][0]["extra"] = json!(true);
        assert!(check(&i, &v).is_err());
    }
    #[test]
    fn duplicate_noncanonical_and_reference_terms_refused() {
        let (i, w) = fixture();
        for part in ["polynomial", "proof"] {
            let mut v = w.clone();
            let t = v["identities"][0][part][0].clone();
            v["identities"][0][part].as_array_mut().unwrap().push(t);
            assert!(check(&i, &v).is_err());
            for c in [0u64, 2147483647, u64::MAX] {
                let mut v = w.clone();
                v["identities"][0][part][0]["coefficient"] = json!(c);
                assert!(check(&i, &v).is_err());
            }
        }
        let mut v = w.clone();
        v["identities"][0]["proof"][0]["generator"] = json!(1);
        assert!(check(&i, &v).is_err());
    }
    #[test]
    fn field_zero_original_and_dimensions_refused() {
        let (i, w) = fixture();
        let mut v = i.clone();
        v["prime"] = json!(9007199254740881u64);
        assert!(check(&v, &w).is_err());
        let mut v = i.clone();
        v["polynomials"][0][0]["coefficient"] = json!(0);
        assert!(check(&v, &w).is_err());
        let mut v = i.clone();
        v["variables"] = json!(["x", "x", "z"]);
        assert!(check(&v, &w).is_err());
        let mut v = w.clone();
        v["identities"][0]["polynomial"][0]["exponents"] = json!([2, 0]);
        assert!(check(&i, &v).is_err());
    }
    #[test]
    fn row_term_degree_and_byte_caps_refused() {
        let (i, w) = fixture();
        let mut v = w.clone();
        v["identities"] = json!(vec![w["identities"][0].clone(); 33]);
        assert!(check(&i, &v).is_err());
        for part in ["polynomial", "proof"] {
            let mut v = w.clone();
            v["identities"][0][part] = json!(vec![w["identities"][0][part][0].clone(); 33]);
            assert!(check(&i, &v).is_err());
        }
        let mut v = w.clone();
        v["identities"][0]["proof"][0]["exponents"] = json!([3, 0, 0]);
        assert!(check(&i, &v).is_err());
        let mut v = w.clone();
        v["identities"][0]["polynomial"][0]["exponents"] = json!([7, 0, 0]);
        assert!(check(&i, &v).is_err());
        let mut v = w.clone();
        v["identities"][0]["proof"][0]["exponents"] = json!([usize::MAX, 1, 0]);
        assert!(check(&i, &v).is_err());
        assert!(verify_json(&vec![b' '; 4 * 1024 * 1024 + 1], b"{}").is_err());
        assert!(verify_json(b"{}", &vec![b' '; 1024 * 1024 + 1]).is_err());
    }
}
