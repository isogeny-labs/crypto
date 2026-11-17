//! Independent exact whole-certificate replay with bounded packed monomials.
//! Wire schema and obligations match the legacy consumer; no producer calls.
use super::{Checked, Input, Member, Multiplier, Result, Stats, Term, Witness};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct M {
    bits: u128,
    degree: u8,
}
impl Ord for M {
    fn cmp(&self, other: &Self) -> Ordering {
        self.degree
            .cmp(&other.degree)
            .then_with(|| other.bits.cmp(&self.bits))
    }
}
impl PartialOrd for M {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl M {
    fn read(exponents: &[usize], n: usize, cap: usize) -> Result<Self> {
        if exponents.len() != n {
            return Err("packed monomial dimension".into());
        }
        let mut bits = 0u128;
        let mut degree = 0usize;
        for (i, e) in exponents.iter().enumerate() {
            degree = degree.checked_add(*e).ok_or("packed degree overflow")?;
            if degree > cap || *e > 64 {
                return Err("packed degree budget".into());
            }
            bits |= (*e as u128) << (7 * i);
        }
        Ok(Self {
            bits,
            degree: degree as u8,
        })
    }
    fn product(self, rhs: Self) -> Result<Self> {
        let degree = u16::from(self.degree) + u16::from(rhs.degree);
        if degree > 64 {
            return Err("packed product degree".into());
        }
        // Total degree <=64 proves every seven-bit lane is carry-free.
        Ok(Self {
            bits: self.bits + rhs.bits,
            degree: degree as u8,
        })
    }
    fn vector(self, n: usize) -> Vec<usize> {
        (0..n)
            .map(|i| ((self.bits >> (7 * i)) & 127) as usize)
            .collect()
    }
    fn divides(self, rhs: Self, n: usize) -> bool {
        (0..n).all(|i| ((self.bits >> (7 * i)) & 127) <= ((rhs.bits >> (7 * i)) & 127))
    }
    fn coprime(self, rhs: Self, n: usize) -> bool {
        (0..n).all(|i| ((self.bits >> (7 * i)) & 127) == 0 || ((rhs.bits >> (7 * i)) & 127) == 0)
    }
    fn lcm(self, rhs: Self, n: usize) -> Result<Self> {
        Self::read(
            &(0..n)
                .map(|i| ((self.bits >> (7 * i)) & 127).max((rhs.bits >> (7 * i)) & 127) as usize)
                .collect::<Vec<_>>(),
            n,
            64,
        )
    }
    fn quotient(self, rhs: Self) -> Self {
        Self {
            bits: self.bits - rhs.bits,
            degree: self.degree - rhs.degree,
        }
    }
}
type Poly = BTreeMap<M, u64>;
fn put<const P: u64>(out: &mut Poly, m: M, c: u64) {
    let old = out.get(&m).copied().unwrap_or(0);
    let sum = old + c;
    let value = if sum >= P { sum - P } else { sum };
    if value == 0 {
        out.remove(&m);
    } else {
        out.insert(m, value);
    }
}
fn decode<const P: u64>(terms: &[Term], n: usize) -> Result<Poly> {
    if terms.is_empty() || terms.len() > 5000 {
        return Err("packed polynomial term budget".into());
    }
    let mut result = Poly::new();
    for t in terms {
        let m = M::read(&t.exponents, n, 64)?;
        if t.coefficient == 0 || t.coefficient >= P || result.insert(m, t.coefficient).is_some() {
            return Err("packed noncanonical/duplicate term".into());
        }
    }
    Ok(result)
}
struct Arithmetic<const P: u64> {
    n: usize,
    stats: Stats,
}
impl<const P: u64> Arithmetic<P> {
    fn combination(
        &mut self,
        terms: &[Multiplier],
        generators: &[Poly],
        bound: Option<M>,
    ) -> Result<Poly> {
        if terms.len() > 50_000 {
            return Err("packed cofactor budget".into());
        }
        self.stats.cofactor_terms = self
            .stats
            .cofactor_terms
            .checked_add(terms.len())
            .ok_or("packed count overflow")?;
        if self.stats.cofactor_terms > 100_000 {
            return Err("packed total cofactor budget".into());
        }
        let mut seen = BTreeSet::new();
        let mut local = 0usize;
        let mut out = Poly::new();
        for t in terms {
            let g = generators
                .get(t.generator)
                .ok_or("packed future/out-of-range reference")?;
            let shift = M::read(&t.exponents, self.n, 64)?;
            if t.coefficient == 0 || t.coefficient >= P || !seen.insert((t.generator, shift)) {
                return Err("packed noncanonical/duplicate cofactor".into());
            }
            let lm = *g.last_key_value().ok_or("packed zero generator")?.0;
            let shifted_lead = lm.product(shift)?;
            if bound.is_some_and(|limit| shifted_lead >= limit) {
                return Err("packed nonstandard S-pair representation".into());
            }
            local = local.checked_add(g.len()).ok_or("packed work overflow")?;
            self.stats.referenced_term_products = self
                .stats
                .referenced_term_products
                .checked_add(g.len())
                .ok_or("packed work overflow")?;
            if local > 1_000_000 || self.stats.referenced_term_products > 5_000_000 {
                return Err("packed product work budget".into());
            }
            for (m, c) in g {
                let product = ((u128::from(*c) * u128::from(t.coefficient)) % u128::from(P)) as u64;
                put::<P>(&mut out, m.product(shift)?, product);
            }
        }
        Ok(out)
    }
    fn identity(
        &mut self,
        target: &Poly,
        terms: &[Multiplier],
        generators: &[Poly],
        bound: Option<M>,
    ) -> Result<()> {
        if self.combination(terms, generators, bound)? != *target {
            return Err("packed polynomial identity mismatch".into());
        }
        self.stats.identities += 1;
        Ok(())
    }
    fn member(&mut self, row: &Member, generators: &[Poly]) -> Result<Poly> {
        let f = decode::<P>(&row.polynomial, self.n)?;
        self.identity(&f, &row.proof, generators, None)?;
        Ok(f)
    }
}
fn verify<const P: u64>(input: Input, w: Witness, dag: bool) -> Result<Checked> {
    let n = input.variables.len();
    if input.prime != P
        || !(3..=16).contains(&n)
        || input.variables.iter().any(String::is_empty)
        || input.variables.iter().collect::<BTreeSet<_>>().len() != n
        || input.polynomials.is_empty()
        || input.polynomials.len() > 32
    {
        return Err("packed original field/shape".into());
    }
    let mut originals = Vec::new();
    for f in input.polynomials {
        if f.len() > 32 {
            return Err("packed original term budget".into());
        }
        let mut out = Poly::new();
        for t in f {
            put::<P>(
                &mut out,
                M::read(&t.exponents, n, 4)?,
                t.coefficient.rem_euclid(P as i64) as u64,
            );
        }
        if !out.is_empty() {
            originals.push(out);
        }
    }
    if originals.is_empty()
        || w.version != if dag { 2 } else { 1 }
        || w.definitions.len() > if dag { 512 } else { n }
        || w.basis.is_empty()
        || w.basis.len() > 256
    {
        return Err("packed wire shape/version".into());
    }
    if w.input_reductions.len() != originals.len()
        || w.pair_reductions.len() != w.basis.len() * (w.basis.len() - 1) / 2
    {
        return Err("packed missing input/S-pair coverage".into());
    }
    let mut math = Arithmetic::<P> {
        n,
        stats: Stats::default(),
    };
    let mut generators = originals.clone();
    for row in w.definitions {
        let f = math.member(&row, &generators)?;
        generators.push(f);
        math.stats.definitions += 1;
    }
    let mut basis = Vec::new();
    let mut leads = Vec::new();
    for row in w.basis {
        let f = math.member(&row, &generators)?;
        let (lm, c) = f.last_key_value().ok_or("packed zero basis")?;
        if *c != 1 {
            return Err("packed nonmonic basis".into());
        }
        leads.push(*lm);
        basis.push(f);
    }
    for (i, f) in basis.iter().enumerate() {
        if f.keys().any(|m| {
            leads
                .iter()
                .enumerate()
                .any(|(j, lm)| i != j && lm.divides(*m, n))
        }) {
            return Err("packed nonreduced basis".into());
        }
    }
    for (f, proof) in originals.iter().zip(&w.input_reductions) {
        math.identity(f, proof, &basis, None)?;
    }
    let mut next = 0;
    for right in 0..basis.len() {
        for left in 0..right {
            let pair = &w.pair_reductions[next];
            next += 1;
            if pair.left != left || pair.right != right {
                return Err("packed missing/duplicate/out-of-order pair".into());
            }
            math.stats.pairs += 1;
            let a = leads[left];
            let b = leads[right];
            if pair.product_criterion {
                if !pair.proof.is_empty() || !a.coprime(b, n) {
                    return Err("packed invalid product criterion".into());
                }
                math.stats.product_pairs += 1;
                continue;
            }
            let lcm = a.lcm(b, n)?;
            let af = lcm.quotient(a);
            let bf = lcm.quotient(b);
            let mut s = Poly::new();
            for (m, c) in &basis[left] {
                put::<P>(&mut s, m.product(af)?, *c);
            }
            for (m, c) in &basis[right] {
                put::<P>(&mut s, m.product(bf)?, P - c);
            }
            math.identity(&s, &pair.proof, &basis, Some(lcm))?;
        }
    }
    math.stats.basis_rows = basis.len();
    Ok(Checked {
        basis: basis
            .into_iter()
            .map(|f| {
                let mut terms: Vec<_> = f.into_iter().map(|(m, c)| (m.vector(n), c)).collect();
                terms.sort();
                terms
            })
            .collect(),
        stats: math.stats,
    })
}
/// Full exact replay on either fixed prime. This is a GB receipt, not row-only.
pub fn verify_json(input: &[u8], wire: &[u8]) -> Result<Checked> {
    verify_wire(input, wire, false)
}
/// Explicit version2 API. Every one of at most512 acyclic nodes is checked;
/// all prior polynomial/byte/cofactor/work budgets and full basis checks remain.
pub fn verify_dag_json(input: &[u8], wire: &[u8]) -> Result<Checked> {
    verify_wire(input, wire, true)
}
fn verify_wire(input: &[u8], wire: &[u8], dag: bool) -> Result<Checked> {
    if input.len() > 4 * 1024 * 1024 || wire.len() > 16 * 1024 * 1024 {
        return Err("packed input/wire byte cap".into());
    }
    let input: Input =
        serde_json::from_slice(input).map_err(|e| format!("packed input JSON:{e}"))?;
    let witness: Witness =
        serde_json::from_slice(wire).map_err(|e| format!("packed witness JSON:{e}"))?;
    match input.prime {
        2147483647 => verify::<2147483647>(input, witness, dag),
        9007199254740881 => verify::<9007199254740881>(input, witness, dag),
        _ => Err("packed unsupported prime".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn whole_certificate_large_products_and_nonstandard_zero_pair() {
        use serde_json::{json, Value};
        for p in [2147483647u64, 9007199254740881] {
            let input = json!({"prime":p,"variables":["x","y","z"],"polynomials":[[{"exponents":[1,0,0],"coefficient":p-1}],[{"exponents":[0,1,0],"coefficient":p-1}]]});
            let member = |m: Vec<usize>, i| json!({"polynomial":[{"exponents":m,"coefficient":1}],"proof":[{"generator":i,"exponents":[0,0,0],"coefficient":p-1}]});
            let mut wire = json!({"version":1,"definitions":[],"basis":[member(vec![1,0,0],0),member(vec![0,1,0],1)],"input_reductions":[[{"generator":0,"exponents":[0,0,0],"coefficient":p-1}],[{"generator":1,"exponents":[0,0,0],"coefficient":p-1}]],"pair_reductions":[{"left":0,"right":1,"product_criterion":true,"proof":[]}]});
            let ib = serde_json::to_vec(&input).unwrap();
            let check = |w: &Value| verify_json(&ib, &serde_json::to_vec(w).unwrap());
            assert_eq!(check(&wire).unwrap().stats.identities, 4);
            // This representation equals the zero S-polynomial exactly but is
            // invalid because both summands attain the pair LCM.
            wire["pair_reductions"][0]["product_criterion"] = json!(false);
            wire["pair_reductions"][0]["proof"] = json!([{"generator":0,"exponents":[0,1,0],"coefficient":1},{"generator":1,"exponents":[1,0,0],"coefficient":p-1}]);
            assert!(check(&wire).unwrap_err().contains("nonstandard"));
            if p == 2147483647 {
                assert!(
                    super::super::verify_json(&ib, &serde_json::to_vec(&wire).unwrap()).is_err()
                );
            }
        }
    }
    #[test]
    fn packed_order_and_products_match_vector_definition() {
        for n in [3, 4, 6, 16] {
            for i in 0..n {
                for j in 0..n {
                    let mut a = vec![0; n];
                    a[i] = 2;
                    let mut b = vec![0; n];
                    b[j] = 3;
                    let x = M::read(&a, n, 64).unwrap();
                    let y = M::read(&b, n, 64).unwrap();
                    assert_eq!(x.cmp(&y), super::super::order(&a, &b));
                    let sum: Vec<_> = a.iter().zip(&b).map(|(a, b)| a + b).collect();
                    assert_eq!(x.product(y).unwrap().vector(n), sum);
                    let l = x.lcm(y, n).unwrap();
                    assert!(x.divides(l, n));
                    assert_eq!(l.quotient(x).product(x).unwrap(), l);
                }
            }
        }
        assert!(M::read(&[usize::MAX, 1, 0], 3, 64).is_err());
        assert!(M::read(&[64, 0, 0], 3, 64)
            .unwrap()
            .product(M::read(&[1, 0, 0], 3, 64).unwrap())
            .is_err());
    }
    #[test]
    fn same_degree_reverse_lex_order_is_exact() {
        let mut vectors = Vec::new();
        for a in 0..5 {
            for b in 0..5 {
                for c in 0..5 {
                    vectors.push(vec![a, b, c]);
                }
            }
        }
        for a in &vectors {
            for b in &vectors {
                assert_eq!(
                    M::read(a, 3, 64).unwrap().cmp(&M::read(b, 3, 64).unwrap()),
                    super::super::order(a, b)
                );
            }
        }
    }
    #[test]
    fn explicit_dag_version_and_acyclic_node_budget() {
        use serde_json::json;
        let input = json!({"prime":2147483647,"variables":["x","y","z"],"polynomials":[[{"exponents":[1,0,0],"coefficient":1}]]});
        let row = |i| json!({"polynomial":[{"exponents":[1,0,0],"coefficient":1}],"proof":[{"generator":i,"exponents":[0,0,0],"coefficient":1}]});
        let mut wire = json!({"version":2,"definitions":[row(0),row(1)],"basis":[row(2)],"input_reductions":[[{"generator":0,"exponents":[0,0,0],"coefficient":1}]],"pair_reductions":[]});
        let ib = serde_json::to_vec(&input).unwrap();
        let check = |v: &serde_json::Value| verify_dag_json(&ib, &serde_json::to_vec(v).unwrap());
        assert_eq!(check(&wire).unwrap().stats.definitions, 2);
        assert!(verify_json(&ib, &serde_json::to_vec(&wire).unwrap()).is_err());
        let original = wire.clone();
        wire["definitions"][0]["proof"][0]["generator"] = json!(1);
        assert!(check(&wire).is_err());
        wire = original.clone();
        wire["definitions"][1]["proof"][0]["generator"] = json!(2);
        assert!(check(&wire).is_err());
        wire = original.clone();
        wire["definitions"] = json!(vec![row(0); 512]);
        assert_eq!(check(&wire).unwrap().stats.definitions, 512);
        wire["definitions"].as_array_mut().unwrap().push(row(0));
        assert!(check(&wire).is_err());
        wire = original;
        wire["version"] = json!(1);
        assert!(check(&wire).is_err());
    }
}
