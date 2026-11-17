//! Exact bounded-Macaulay refutation certificates, with a separate identity checker.
//! Monomial mask zero means 1; an empty polynomial means 0. Boolean x_i^2=x_i.
//! This tool measures D_ref, not homogeneous regularity or an iterative F4 degree.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::io::Write;

const MAX_VARIABLES: u32 = 18;
const MAX_COLUMNS: usize = 4096;
const MAX_ROWS: usize = 4096;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct System {
    variables: u32,
    max_degree: u32,
    /// One sorted, duplicate-free list of squarefree monomial masks per generator.
    polynomials: Vec<Vec<u32>>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Recipe {
    generator: usize,
    multiplier: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Dual {
    degree: u32,
    /// Monomials whose coordinate in the dual vector is one.
    ones: Vec<u32>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Certificate {
    schema: String,
    input_sha256: String,
    metric: String,
    status: String,
    refutation_degree: Option<u32>,
    upper: Vec<Recipe>,
    lower: Option<Dual>,
    profiles: Vec<Profile>,
    exhaustive_boolean_roots: usize,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Profile {
    degree: u32,
    rows: usize,
    columns: usize,
    rank: usize,
}

fn degree(p: &[u32]) -> u32 {
    p.iter().map(|m| m.count_ones()).max().unwrap_or(0)
}
fn validate(s: &System) -> Result<(), String> {
    if s.variables > MAX_VARIABLES || s.max_degree > s.variables + 1 {
        return Err("variable/degree safety cap exceeded".into());
    }
    if s.polynomials.len() > 64 {
        return Err("generator cap exceeded".into());
    }
    for p in &s.polynomials {
        if p.windows(2).any(|w| w[0] >= w[1]) || p.iter().any(|&m| m >= (1 << s.variables)) {
            return Err("generators must have sorted unique in-range monomial masks".into());
        }
    }
    Ok(())
}
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn toggle(set: &mut BTreeSet<u32>, m: u32) {
    if !set.insert(m) {
        set.remove(&m);
    }
}
fn product(p: &[u32], m: u32) -> BTreeSet<u32> {
    let mut out = BTreeSet::new();
    for &term in p {
        toggle(&mut out, term | m);
    }
    out
}
fn recipes(s: &System, d: u32) -> Result<Vec<Recipe>, String> {
    let mut out = Vec::new();
    for (i, p) in s.polynomials.iter().enumerate() {
        if p.is_empty() || degree(p) > d {
            continue;
        }
        for m in 0u32..1 << s.variables {
            if m.count_ones() + degree(p) <= d {
                out.push(Recipe {
                    generator: i,
                    multiplier: m,
                });
                if out.len() > MAX_ROWS {
                    return Err(format!("UNKNOWN: row cap at degree {d}"));
                }
            }
        }
    }
    Ok(out)
}
fn bit(v: &[u64], i: usize) -> bool {
    v[i / 64] >> (i % 64) & 1 != 0
}
fn flip(v: &mut [u64], i: usize) {
    v[i / 64] ^= 1 << (i % 64);
}
fn xor(a: &mut [u64], b: &[u64]) {
    for (x, y) in a.iter_mut().zip(b) {
        *x ^= y;
    }
}
struct MatrixResult {
    profile: Profile,
    upper: Option<Vec<Recipe>>,
    dual: Option<Dual>,
}
fn matrix(s: &System, d: u32) -> Result<MatrixResult, String> {
    let rows = recipes(s, d)?;
    let mut cols: Vec<u32> = (0u32..1 << s.variables)
        .filter(|m| m.count_ones() <= d)
        .collect();
    // Constant is last: if it is a pivot, its row is exactly 1.
    cols.sort_by_key(|m| (std::cmp::Reverse(m.count_ones()), std::cmp::Reverse(*m)));
    if cols.len() > MAX_COLUMNS {
        return Err(format!("UNKNOWN: column cap at degree {d}"));
    }
    let mut positions = vec![usize::MAX; 1 << s.variables];
    for (i, &m) in cols.iter().enumerate() {
        positions[m as usize] = i;
    }
    let width = cols.len().div_ceil(64);
    let provenance_width = rows.len().div_ceil(64);
    type Pivot = Option<(Vec<u64>, Vec<u64>)>;
    let mut pivots: Vec<Pivot> = vec![None; cols.len()];
    let mut rank = 0;
    for (i, recipe) in rows.iter().enumerate() {
        let mut row = vec![0u64; width];
        for &term in &s.polynomials[recipe.generator] {
            flip(&mut row, positions[(term | recipe.multiplier) as usize]);
        }
        let mut provenance = vec![0u64; provenance_width];
        flip(&mut provenance, i);
        for (c, pivot) in pivots.iter_mut().enumerate() {
            if !bit(&row, c) {
                continue;
            }
            if let Some((basis, history)) = pivot {
                xor(&mut row, basis);
                xor(&mut provenance, history);
            } else {
                *pivot = Some((row, provenance));
                rank += 1;
                break;
            }
        }
    }
    let constant = positions[0];
    let upper = pivots[constant].as_ref().map(|(_, history)| {
        rows.iter()
            .enumerate()
            .filter(|(i, _)| bit(history, *i))
            .map(|(_, recipe)| recipe.clone())
            .collect()
    });
    let dual = if upper.is_none() {
        let mut values = vec![false; cols.len()];
        values[constant] = true;
        for c in (0..cols.len()).rev() {
            if let Some((row, _)) = &pivots[c] {
                values[c] = ((c + 1)..cols.len())
                    .filter(|&j| bit(row, j) && values[j])
                    .count()
                    % 2
                    == 1;
            }
        }
        Some(Dual {
            degree: d,
            ones: cols
                .iter()
                .enumerate()
                .filter(|(i, _)| values[*i])
                .map(|(_, &m)| m)
                .collect(),
        })
    } else {
        None
    };
    Ok(MatrixResult {
        profile: Profile {
            degree: d,
            rows: rows.len(),
            columns: cols.len(),
            rank,
        },
        upper,
        dual,
    })
}
fn roots(s: &System) -> usize {
    (0u32..1 << s.variables)
        .filter(|&x| {
            s.polynomials
                .iter()
                .all(|p| p.iter().filter(|&&m| m & x == m).count() % 2 == 0)
        })
        .count()
}
fn certify(s: &System, hash: String) -> Result<Certificate, String> {
    validate(s)?;
    let mut profiles = Vec::new();
    let mut lower = None;
    for d in 0..=s.max_degree {
        let result = matrix(s, d)?;
        profiles.push(result.profile);
        if let Some(upper) = result.upper {
            return Ok(Certificate {
                schema: "boolean-macaulay-certificate/v1".into(),
                input_sha256: hash,
                metric: "formal-generator-plus-multiplier-degree-in-Boolean-quotient".into(),
                status: "unsat_certified".into(),
                refutation_degree: Some(d),
                upper,
                lower,
                profiles,
                exhaustive_boolean_roots: roots(s),
            });
        }
        lower = result.dual;
    }
    Ok(Certificate {
        schema: "boolean-macaulay-certificate/v1".into(),
        input_sha256: hash,
        metric: "formal-generator-plus-multiplier-degree-in-Boolean-quotient".into(),
        status: "no_refutation_through_cap".into(),
        refutation_degree: None,
        upper: Vec::new(),
        lower,
        profiles,
        exhaustive_boolean_roots: roots(s),
    })
}
/// Uses set-valued polynomial multiplication, without trusting producer pivots/ranks.
fn verify(s: &System, hash: &str, c: &Certificate) -> Result<(), String> {
    validate(s)?;
    if c.schema != "boolean-macaulay-certificate/v1"
        || c.input_sha256 != hash
        || c.metric != "formal-generator-plus-multiplier-degree-in-Boolean-quotient"
    {
        return Err("schema/input/metric mismatch".into());
    }
    let root_count = roots(s);
    if root_count != c.exhaustive_boolean_roots {
        return Err("Boolean truth-table mismatch".into());
    }
    match c.refutation_degree {
        Some(d) => {
            if d > s.max_degree || c.status != "unsat_certified" || root_count != 0 {
                return Err("invalid UNSAT status/degree".into());
            }
            let mut sum = BTreeSet::new();
            for r in &c.upper {
                let p = s.polynomials.get(r.generator).ok_or("invalid generator")?;
                if r.multiplier >= 1 << s.variables || r.multiplier.count_ones() + degree(p) > d {
                    return Err("invalid multiplier degree".into());
                }
                for m in product(p, r.multiplier) {
                    toggle(&mut sum, m);
                }
            }
            if sum != BTreeSet::from([0]) {
                return Err("upper identity is not 1".into());
            }
            if d == 0 {
                if c.lower.is_some() {
                    return Err("unexpected lower certificate at D=0".into());
                }
            } else if c.lower.as_ref().map(|l| l.degree) != Some(d - 1) {
                return Err("missing adjacent lower-degree certificate".into());
            }
        }
        None => {
            if c.status != "no_refutation_through_cap"
                || !c.upper.is_empty()
                || c.lower.as_ref().map(|l| l.degree) != Some(s.max_degree)
            {
                return Err("invalid unresolved status".into());
            }
        }
    }
    if let Some(lower) = &c.lower {
        let values: BTreeSet<u32> = lower.ones.iter().copied().collect();
        if values.len() != lower.ones.len()
            || !values.contains(&0)
            || values
                .iter()
                .any(|&m| m >= 1 << s.variables || m.count_ones() > lower.degree)
        {
            return Err("invalid dual-vector domain/constant".into());
        }
        for r in recipes(s, lower.degree)? {
            if !product(&s.polynomials[r.generator], r.multiplier)
                .intersection(&values)
                .count()
                .is_multiple_of(2)
            {
                return Err("dual vector is not orthogonal to all lower-degree rows".into());
            }
        }
    }
    // Profiles are descriptive metadata, also rechecked rather than trusted.
    let expected_last = c.refutation_degree.unwrap_or(s.max_degree);
    if c.profiles.len() != expected_last as usize + 1 {
        return Err("incomplete profiles".into());
    }
    for (d, recorded) in c.profiles.iter().enumerate() {
        let actual = matrix(s, d as u32)?.profile;
        if (
            recorded.degree,
            recorded.rows,
            recorded.columns,
            recorded.rank,
        ) != (actual.degree, actual.rows, actual.columns, actual.rank)
        {
            return Err("profile metadata mismatch".into());
        }
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 || !matches!(args[1].as_str(), "certify" | "verify") {
        return Err("usage: degree-certificate certify|verify SYSTEM.json CERTIFICATE.json".into());
    }
    let bytes = fs::read(&args[2])?;
    let system: System = serde_json::from_slice(&bytes)?;
    let hash = digest(&bytes);
    if args[1] == "certify" {
        let cert = certify(&system, hash.clone())?;
        verify(&system, &hash, &cert)?;
        let output = serde_json::to_vec_pretty(&cert)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args[3])?;
        file.write_all(&output)?;
        file.write_all(b"\n")?;
        println!("{}", cert.status);
    } else {
        let cert: Certificate = serde_json::from_slice(&fs::read(&args[3])?)?;
        verify(&system, &hash, &cert)?;
        println!("verified");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> System {
        // xy=1, yz=1, xz=0: UNSAT, with degree-2 lower dual and degree-3 refutation.
        System {
            variables: 3,
            max_degree: 4,
            polynomials: vec![vec![0, 3], vec![0, 6], vec![5]],
        }
    }
    #[test]
    fn nonlinear_exact_degree_and_replay() {
        let s = fixture();
        let c = certify(&s, "fixture".into()).unwrap();
        assert_eq!(c.refutation_degree, Some(3));
        assert_eq!(c.lower.as_ref().unwrap().degree, 2);
        assert_eq!(c.exhaustive_boolean_roots, 0);
        verify(&s, "fixture", &c).unwrap();
    }
    #[test]
    fn rejects_corrupt_upper_and_lower() {
        let s = fixture();
        let mut c = certify(&s, "fixture".into()).unwrap();
        c.upper.clear();
        assert!(verify(&s, "fixture", &c).is_err());
        let mut c = certify(&s, "fixture".into()).unwrap();
        c.lower.as_mut().unwrap().ones.clear();
        assert!(verify(&s, "fixture", &c).is_err());
    }
    #[test]
    fn sat_and_unknown_are_never_unsat() {
        let s = System {
            variables: 2,
            max_degree: 3,
            polynomials: vec![vec![3]],
        };
        let c = certify(&s, "sat".into()).unwrap();
        assert_eq!(c.refutation_degree, None);
        assert_eq!(c.exhaustive_boolean_roots, 3);
        verify(&s, "sat", &c).unwrap();
        let mut s = fixture();
        s.max_degree = 2;
        let c = certify(&s, "capped".into()).unwrap();
        assert_eq!(c.refutation_degree, None);
        assert_eq!(c.exhaustive_boolean_roots, 0);
        verify(&s, "capped", &c).unwrap();
    }
    #[test]
    fn constant_contradiction_and_empty_system() {
        let s = System {
            variables: 0,
            max_degree: 0,
            polynomials: vec![vec![0]],
        };
        let c = certify(&s, "constant".into()).unwrap();
        assert_eq!(c.refutation_degree, Some(0));
        verify(&s, "constant", &c).unwrap();
        let s = System {
            variables: 2,
            max_degree: 2,
            polynomials: vec![],
        };
        let c = certify(&s, "empty".into()).unwrap();
        assert_eq!(c.exhaustive_boolean_roots, 4);
        verify(&s, "empty", &c).unwrap();
    }
    #[test]
    fn rejects_bad_binding_degrees_and_masks() {
        let mut s = fixture();
        let mut c = certify(&s, "fixture".into()).unwrap();
        assert!(verify(&s, "different", &c).is_err());
        c.lower.as_mut().unwrap().degree = 0;
        assert!(verify(&s, "fixture", &c).is_err());
        s.polynomials[0] = vec![0, 8];
        assert!(certify(&s, "bad".into()).is_err());
    }
    #[test]
    fn row_and_column_caps_are_explicit_errors() {
        let s = System {
            variables: 18,
            max_degree: 6,
            polynomials: vec![],
        };
        assert!(matches!(matrix(&s, 6), Err(e) if e.contains("column cap")));
    }
    #[test]
    fn all_two_variable_generator_pairs_match_truth_tables() {
        for a in 0u32..16 {
            for b in 0u32..16 {
                let polynomial = |bits: u32| (0..4).filter(|m| bits >> m & 1 == 1).collect();
                let s = System {
                    variables: 2,
                    max_degree: 3,
                    polynomials: vec![polynomial(a), polynomial(b)],
                };
                let c = certify(&s, "exhaustive-unit-fixture".into()).unwrap();
                assert_eq!(c.refutation_degree.is_some(), roots(&s) == 0);
                verify(&s, "exhaustive-unit-fixture", &c).unwrap();
            }
        }
    }
}
