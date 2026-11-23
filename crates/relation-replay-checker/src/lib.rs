//! Separate wire/schema/order/arithmetic implementation. No producer dependency.
//! Exact ideal equality and Buchberger witnesses, with no basis construction.
pub mod identities;
pub mod packed;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};
const P: u64 = 2_147_483_647;
type Monomial = Vec<usize>;
type Polynomial = BTreeMap<Monomial, u64>;
type Result<T> = std::result::Result<T, String>;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InputTerm {
    exponents: Monomial,
    coefficient: i64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    prime: u64,
    variables: Vec<String>,
    polynomials: Vec<Vec<InputTerm>>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Term {
    exponents: Monomial,
    coefficient: u64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Multiplier {
    generator: usize,
    exponents: Monomial,
    coefficient: u64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Member {
    polynomial: Vec<Term>,
    proof: Vec<Multiplier>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Pair {
    left: usize,
    right: usize,
    #[serde(default)]
    product_criterion: bool,
    proof: Vec<Multiplier>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Witness {
    version: u32,
    definitions: Vec<Member>,
    basis: Vec<Member>,
    input_reductions: Vec<Vec<Multiplier>>,
    pair_reductions: Vec<Pair>,
}
#[derive(Default, Debug, Serialize)]
pub struct Stats {
    pub definitions: usize,
    pub basis_rows: usize,
    pub identities: usize,
    pub pairs: usize,
    pub product_pairs: usize,
    pub cofactor_terms: usize,
    pub referenced_term_products: usize,
}
#[derive(Debug, Serialize)]
pub struct Checked {
    pub basis: Vec<Vec<(Monomial, u64)>>,
    pub stats: Stats,
}
fn degree(m: &[usize]) -> Result<usize> {
    m.iter().try_fold(0usize, |sum, e| {
        sum.checked_add(*e)
            .ok_or_else(|| "exponent sum overflow".into())
    })
}
fn check_monomial(m: &[usize], n: usize, cap: usize) -> Result<()> {
    if m.len() != n || degree(m)? > cap {
        return Err("monomial shape/degree".into());
    }
    Ok(())
}
fn order(a: &[usize], b: &[usize]) -> Ordering {
    let degree_a: usize = a.iter().sum();
    let degree_b: usize = b.iter().sum();
    degree_a.cmp(&degree_b).then_with(|| {
        a.iter()
            .zip(b)
            .rev()
            .find_map(|(x, y)| if x == y { None } else { Some(y.cmp(x)) })
            .unwrap_or(Ordering::Equal)
    })
}
fn leading(f: &Polynomial) -> &Monomial {
    f.keys()
        .max_by(|a, b| order(a, b))
        .expect("nonzero validated row")
}
fn polynomial_degree(f: &Polynomial) -> usize {
    f.keys().map(|m| m.iter().sum()).max().unwrap_or(0)
}
fn divides(a: &[usize], b: &[usize]) -> bool {
    a.iter().zip(b).all(|(x, y)| x <= y)
}
fn add_term(f: &mut Polynomial, m: Monomial, c: u64) {
    if c == 0 {
        return;
    }
    let old = f.get(&m).copied().unwrap_or(0);
    let sum = old + c;
    let value = if sum >= P { sum - P } else { sum };
    if value == 0 {
        f.remove(&m);
    } else {
        f.insert(m, value);
    }
}
fn sum_monomials(a: &[usize], b: &[usize]) -> Result<Monomial> {
    a.iter()
        .zip(b)
        .map(|(x, y)| {
            x.checked_add(*y)
                .ok_or_else(|| "monomial product overflow".into())
        })
        .collect()
}
fn decode(f: &[Term], n: usize) -> Result<Polynomial> {
    if f.is_empty() || f.len() > 5000 {
        return Err("zero/oversized wire polynomial".into());
    }
    let mut out = Polynomial::new();
    for t in f {
        check_monomial(&t.exponents, n, 64)?;
        if t.coefficient == 0
            || t.coefficient >= P
            || out.insert(t.exponents.clone(), t.coefficient).is_some()
        {
            return Err("noncanonical/duplicate polynomial term".into());
        }
    }
    Ok(out)
}
fn original(input: &Input) -> Result<Vec<Polynomial>> {
    let n = input.variables.len();
    if input.prime != P
        || !(3..=16).contains(&n)
        || input.variables.iter().any(String::is_empty)
        || input.variables.iter().collect::<BTreeSet<_>>().len() != n
        || input.polynomials.is_empty()
        || input.polynomials.len() > 32
    {
        return Err("unsupported original field/shape".into());
    }
    let decoded: Vec<Polynomial> = input
        .polynomials
        .iter()
        .map(|f| {
            if f.len() > 32 {
                return Err("original term budget".into());
            }
            let mut out = Polynomial::new();
            for t in f {
                check_monomial(&t.exponents, n, 4)?;
                let c = t.coefficient.rem_euclid(P as i64) as u64;
                add_term(&mut out, t.exponents.clone(), c);
            }
            Ok(out)
        })
        .collect::<Result<_>>()?;
    // Wire generator indices refer to nonzero normalized original rows, as
    // specified by the original System interface. Zero equations add no ideal
    // generator; do not shift the remaining reference indices.
    let decoded: Vec<_> = decoded.into_iter().filter(|f| !f.is_empty()).collect();
    if decoded.is_empty() {
        return Err("need nonzero original equations".into());
    }
    Ok(decoded)
}
struct Arithmetic {
    n: usize,
    stats: Stats,
}
impl Arithmetic {
    fn combination(
        &mut self,
        terms: &[Multiplier],
        generators: &[Polynomial],
        bound: Option<&[usize]>,
    ) -> Result<Polynomial> {
        if terms.len() > 50_000 {
            return Err("identity cofactor budget".into());
        }
        self.stats.cofactor_terms = self
            .stats
            .cofactor_terms
            .checked_add(terms.len())
            .ok_or("term count overflow")?;
        if self.stats.cofactor_terms > 100_000 {
            return Err("total cofactor budget".into());
        }
        let mut seen = BTreeSet::new();
        let mut local_work = 0usize;
        let mut out = Polynomial::new();
        for t in terms {
            let g = generators
                .get(t.generator)
                .ok_or("future/out-of-range reference")?;
            check_monomial(&t.exponents, self.n, 64)?;
            if t.coefficient == 0
                || t.coefficient >= P
                || !seen.insert((t.generator, t.exponents.clone()))
                || polynomial_degree(g) + degree(&t.exponents)? > 64
            {
                return Err("malformed/duplicate cofactor".into());
            }
            local_work = local_work.checked_add(g.len()).ok_or("work overflow")?;
            self.stats.referenced_term_products = self
                .stats
                .referenced_term_products
                .checked_add(g.len())
                .ok_or("work overflow")?;
            if local_work > 1_000_000 || self.stats.referenced_term_products > 5_000_000 {
                return Err("identity/total arithmetic work budget".into());
            }
            if let Some(bound) = bound {
                if g.is_empty()
                    || order(&sum_monomials(leading(g), &t.exponents)?, bound) != Ordering::Less
                {
                    return Err("nonstandard S-pair representation".into());
                }
            }
            for (m, c) in g {
                let shifted = sum_monomials(m, &t.exponents)?;
                let product = c * t.coefficient % P;
                add_term(&mut out, shifted, product);
            }
        }
        Ok(out)
    }
    fn identity(
        &mut self,
        target: &Polynomial,
        terms: &[Multiplier],
        generators: &[Polynomial],
        bound: Option<&[usize]>,
    ) -> Result<()> {
        let rhs = self.combination(terms, generators, bound)?;
        if rhs != *target {
            return Err("polynomial identity mismatch".into());
        }
        self.stats.identities += 1;
        Ok(())
    }
}
/// Verify raw original/witness bytes through this crate's separate arithmetic.
/// Bounds are enforced before polynomial products. No producer function is used.
pub fn verify_json(input: &[u8], wire: &[u8]) -> Result<Checked> {
    if input.len() > 4 * 1024 * 1024 || wire.len() > 16 * 1024 * 1024 {
        return Err("input/wire byte cap".into());
    }
    let input: Input = serde_json::from_slice(input).map_err(|e| format!("input JSON: {e}"))?;
    let w: Witness = serde_json::from_slice(wire).map_err(|e| format!("witness JSON: {e}"))?;
    let n = input.variables.len();
    let originals = original(&input)?;
    if w.version != 1 || w.definitions.len() > n || w.basis.is_empty() || w.basis.len() > 256 {
        return Err("wire version/definition/basis shape".into());
    }
    if w.input_reductions.len() != originals.len()
        || w.pair_reductions.len() != w.basis.len() * (w.basis.len() - 1) / 2
    {
        return Err("missing input/S-pair coverage".into());
    }
    let mut arithmetic = Arithmetic {
        n,
        stats: Stats::default(),
    };
    let mut generators = originals.clone();
    for row in &w.definitions {
        let f = decode(&row.polynomial, n)?;
        arithmetic.identity(&f, &row.proof, &generators, None)?;
        generators.push(f);
        arithmetic.stats.definitions += 1;
    }
    let mut basis = Vec::new();
    let mut leads = Vec::new();
    for row in &w.basis {
        let f = decode(&row.polynomial, n)?;
        let lm = leading(&f).clone();
        if f[&lm] != 1 {
            return Err("non-monic basis".into());
        }
        arithmetic.identity(&f, &row.proof, &generators, None)?;
        basis.push(f);
        leads.push(lm);
    }
    for (i, f) in basis.iter().enumerate() {
        for m in f.keys() {
            if leads
                .iter()
                .enumerate()
                .any(|(j, lm)| j != i && divides(lm, m))
            {
                return Err("non-reduced basis".into());
            }
        }
    }
    for (f, terms) in originals.iter().zip(&w.input_reductions) {
        arithmetic.identity(f, terms, &basis, None)?;
    }
    let mut next = 0usize;
    for right in 0..basis.len() {
        for left in 0..right {
            let pair = &w.pair_reductions[next];
            next += 1;
            if pair.left != left || pair.right != right {
                return Err("missing/duplicate/out-of-order S-pair".into());
            }
            let a = &leads[left];
            let b = &leads[right];
            let lcm: Monomial = a.iter().zip(b).map(|(x, y)| (*x).max(*y)).collect();
            arithmetic.stats.pairs += 1;
            if pair.product_criterion {
                if !pair.proof.is_empty() || a.iter().zip(b).any(|(x, y)| *x != 0 && *y != 0) {
                    return Err("invalid product criterion".into());
                }
                arithmetic.stats.product_pairs += 1;
                continue;
            }
            let left_factor: Monomial = lcm.iter().zip(a).map(|(x, y)| x - y).collect();
            let right_factor: Monomial = lcm.iter().zip(b).map(|(x, y)| x - y).collect();
            let mut s = Polynomial::new();
            for (m, c) in &basis[left] {
                add_term(&mut s, sum_monomials(m, &left_factor)?, *c);
            }
            for (m, c) in &basis[right] {
                add_term(&mut s, sum_monomials(m, &right_factor)?, P - c);
            }
            arithmetic.identity(&s, &pair.proof, &basis, Some(&lcm))?;
        }
    }
    arithmetic.stats.basis_rows = basis.len();
    Ok(Checked {
        basis: basis.into_iter().map(|f| f.into_iter().collect()).collect(),
        stats: arithmetic.stats,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    fn fixture() -> (Value, Value) {
        let term = |m: Vec<usize>| json!({"exponents":m,"coefficient":1});
        let unit = |i| json!([{"generator":i,"exponents":[0,0,0],"coefficient":1}]);
        let polys: Vec<_> = (0..3)
            .map(|i| {
                let mut m = vec![0; 3];
                m[i] = 1;
                vec![term(m)]
            })
            .collect();
        (
            json!({"prime":P,"variables":["x","y","z"],"polynomials":polys}),
            json!({"version":1,"definitions":[],"basis":(0..3).map(|i|json!({"polynomial":polys[i],"proof":unit(i)})).collect::<Vec<_>>(),"input_reductions":(0..3).map(unit).collect::<Vec<_>>(),"pair_reductions":[{"left":0,"right":1,"product_criterion":true,"proof":[]},{"left":0,"right":2,"product_criterion":true,"proof":[]},{"left":1,"right":2,"product_criterion":true,"proof":[]}]}),
        )
    }
    fn check(s: &Value, w: &Value) -> Result<Checked> {
        verify_json(
            &serde_json::to_vec(s).unwrap(),
            &serde_json::to_vec(w).unwrap(),
        )
    }
    #[test]
    fn exact_field_wrap_cancellation_and_order_ties() {
        let mut f = Polynomial::new();
        add_term(&mut f, vec![0; 3], P - 1);
        add_term(&mut f, vec![0; 3], 1);
        assert!(f.is_empty());
        assert_eq!((P - 1) * (P - 1) % P, 1);
        assert_eq!(order(&[1, 0, 1], &[0, 2, 0]), Ordering::Less);
        assert_eq!(order(&[1, 1, 0], &[2, 0, 0]), Ordering::Less);
        assert_eq!(order(&[0, 0, 3], &[2, 0, 0]), Ordering::Greater);
    }
    #[test]
    fn whole_ideal_checks_accept_exact_coordinate_ideal_and_reject_tamper() {
        let (s, w) = fixture();
        let good = check(&s, &w).unwrap();
        assert_eq!(good.stats.pairs, 3);
        assert_eq!(good.stats.product_pairs, 3);
        let mut bad = w.clone();
        bad["basis"][0]["proof"][0]["coefficient"] = json!(2);
        assert!(check(&s, &bad).unwrap_err().contains("identity"));
        let mut bad = w.clone();
        bad["input_reductions"][0] = json!([]);
        assert!(check(&s, &bad).unwrap_err().contains("identity"));
        let mut bad = w.clone();
        bad["pair_reductions"][0]["right"] = json!(0);
        assert!(check(&s, &bad).unwrap_err().contains("S-pair"));
        let mut bad = w.clone();
        bad["basis"][0]["polynomial"][0]["coefficient"] = json!(P);
        assert!(check(&s, &bad).unwrap_err().contains("noncanonical"));
        let mut bad = w;
        bad["unknown"] = json!(true);
        assert!(check(&s, &bad).unwrap_err().contains("JSON"));
        let (mut normalized, w) = fixture();
        normalized["polynomials"][0][0]["coefficient"] = json!(P + 1);
        normalized["polynomials"]
            .as_array_mut()
            .unwrap()
            .insert(0, json!([]));
        assert!(check(&normalized, &w).is_ok());
    }
    #[test]
    fn wire_canonicality_forward_refs_and_unsupported_fields_reject() {
        let (s, w) = fixture();
        let mut bad = w.clone();
        let t = bad["basis"][0]["polynomial"][0].clone();
        bad["basis"][0]["polynomial"]
            .as_array_mut()
            .unwrap()
            .push(t);
        assert!(check(&s, &bad).unwrap_err().contains("duplicate"));
        let mut bad = w.clone();
        let t = bad["basis"][0]["proof"][0].clone();
        bad["basis"][0]["proof"].as_array_mut().unwrap().push(t);
        assert!(check(&s, &bad).unwrap_err().contains("duplicate"));
        let mut bad = w.clone();
        bad["basis"][0]["proof"][0]["generator"] = json!(3);
        assert!(check(&s, &bad).unwrap_err().contains("reference"));
        let mut bad = w.clone();
        bad["basis"][0]["polynomial"][0]["exponents"] = json!([usize::MAX, 1, 0]);
        assert!(check(&s, &bad).unwrap_err().contains("overflow"));
        let mut bad = s;
        bad["prime"] = json!(9007199254740881u64);
        assert!(check(&bad, &w).unwrap_err().contains("field"));
        let (mut bad, w) = fixture();
        bad["variables"] = json!(["x", "x", "z"]);
        assert!(check(&bad, &w).unwrap_err().contains("shape"));
    }
    #[test]
    fn exact_but_nonstandard_s_pair_identity_cannot_certify_non_gb() {
        let s = json!({"prime":P,"variables":["x","y","z"],"polynomials":[[{"exponents":[2,0,0],"coefficient":1},{"exponents":[0,1,0],"coefficient":-1}],[{"exponents":[1,1,0],"coefficient":1},{"exponents":[0,0,0],"coefficient":-1}]]});
        let original: Input = serde_json::from_value(s.clone()).unwrap();
        let polys = super::original(&original).unwrap();
        let unit = |i| json!([{"generator":i,"exponents":[0,0,0],"coefficient":1}]);
        let mut w = json!({"version":1,"definitions":[],"basis":polys.iter().enumerate().map(|(i,f)|json!({"polynomial":f.iter().map(|(m,c)|json!({"exponents":m,"coefficient":c})).collect::<Vec<_>>(),"proof":unit(i)})).collect::<Vec<_>>(),"input_reductions":[unit(0),unit(1)],"pair_reductions":[{"left":0,"right":1,"proof":[{"generator":0,"exponents":[0,1,0],"coefficient":1},{"generator":1,"exponents":[1,0,0],"coefficient":P-1}]}]});
        assert!(check(&s, &w).unwrap_err().contains("nonstandard"));
        w["pair_reductions"][0]["proof"] = json!([]);
        assert!(check(&s, &w).unwrap_err().contains("identity"));
        w["pair_reductions"][0]["product_criterion"] = json!(true);
        assert!(check(&s, &w).unwrap_err().contains("product criterion"));
    }
}
