//! Independent exact whole-certificate replay with bounded packed monomials.
//! Wire schema and obligations match the legacy consumer; no producer calls.
use super::{Checked, Input, Member, Multiplier, Result, Stats, Term, Witness};
use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

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
thread_local! {
    static HEAP_WORK: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((0,0)) };
}
/// Prototype structural observations only: product pushes and final map entries.
/// These exclude parsing, other maps, native checks and all application costs.
pub fn heap_work() -> (usize, usize) {
    HEAP_WORK.with(|v| v.get())
}
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
    heap: bool,
    heap_products: usize,
    heap_final_entries: usize,
}
impl<const P: u64> Arithmetic<P> {
    fn combination_heap(
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
        let mut streams = Vec::with_capacity(terms.len());
        let mut heap = BinaryHeap::with_capacity(terms.len());
        for (index, t) in terms.iter().enumerate() {
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
            let mut stream = g.iter();
            if let Some((m, c)) = stream.next() {
                let product = ((u128::from(*c) * u128::from(t.coefficient)) % u128::from(P)) as u64;
                heap.push(Reverse((m.product(shift)?, index, product)));
                self.heap_products += 1;
            }
            streams.push((stream, shift, t.coefficient));
        }
        // Fixed shifts preserve total degree order and reverse packed-bit order:
        // every lane/product is bounded, so adding the same bits cannot carry or
        // overflow. Thus each generator iterator remains sorted. One heap head
        // per cofactor permits exact aggregation before final map insertion.
        let mut out = Poly::new();
        let mut pending: Option<(M, u64)> = None;
        while let Some(Reverse((m, index, c))) = heap.pop() {
            match pending.as_mut() {
                Some((last, sum)) if *last == m => {
                    let value = *sum + c;
                    *sum = if value >= P { value - P } else { value };
                }
                _ => {
                    if let Some((last, sum)) = pending.take() {
                        if sum != 0 {
                            out.insert(last, sum);
                        }
                    }
                    pending = Some((m, c));
                }
            }
            let (stream, shift, coefficient) = &mut streams[index];
            if let Some((next, value)) = stream.next() {
                let product =
                    ((u128::from(*value) * u128::from(*coefficient)) % u128::from(P)) as u64;
                heap.push(Reverse((next.product(*shift)?, index, product)));
                self.heap_products += 1;
            }
        }
        if let Some((m, c)) = pending {
            if c != 0 {
                out.insert(m, c);
            }
        }
        self.heap_final_entries += out.len();
        Ok(out)
    }
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
        let actual = if self.heap {
            self.combination_heap(terms, generators, bound)?
        } else {
            self.combination(terms, generators, bound)?
        };
        if actual != *target {
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
fn verify<const P: u64>(input: Input, w: Witness, dag: bool, heap: bool) -> Result<Checked> {
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
        heap,
        heap_products: 0,
        heap_final_entries: 0,
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
    HEAP_WORK.with(|v| v.set((math.heap_products, math.heap_final_entries)));
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
    verify_wire_with_strategy(input, wire, dag, false)
}
/// Prototype opt-in: same full v1 checks with heap aggregation of term products.
pub fn verify_json_heap(input: &[u8], wire: &[u8]) -> Result<Checked> {
    verify_wire_with_strategy(input, wire, false, true)
}
/// Prototype opt-in: same full v2 checks, budgets and acyclic/GB obligations.
pub fn verify_dag_json_heap(input: &[u8], wire: &[u8]) -> Result<Checked> {
    verify_wire_with_strategy(input, wire, true, true)
}
fn verify_wire_with_strategy(input: &[u8], wire: &[u8], dag: bool, heap: bool) -> Result<Checked> {
    HEAP_WORK.with(|v| v.set((0, 0)));
    if input.len() > 4 * 1024 * 1024 || wire.len() > 16 * 1024 * 1024 {
        return Err("packed input/wire byte cap".into());
    }
    let input: Input =
        serde_json::from_slice(input).map_err(|e| format!("packed input JSON:{e}"))?;
    let witness: Witness =
        serde_json::from_slice(wire).map_err(|e| format!("packed witness JSON:{e}"))?;
    match input.prime {
        2147483647 => verify::<2147483647>(input, witness, dag, heap),
        9007199254740881 => verify::<9007199254740881>(input, witness, dag, heap),
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

#[cfg(test)]
mod heap_tests {
    use super::*;
    use serde_json::json;
    fn arithmetic<const P: u64>(n: usize) -> Arithmetic<P> {
        Arithmetic {
            n,
            stats: Stats::default(),
            heap: true,
            heap_products: 0,
            heap_final_entries: 0,
        }
    }
    fn term(i: usize, m: Vec<usize>, c: u64) -> Multiplier {
        Multiplier {
            generator: i,
            exponents: m,
            coefficient: c,
        }
    }
    fn differential<const P: u64>(n: usize, g: &[Poly], terms: &[Multiplier], bound: Option<M>) {
        let mut a = arithmetic::<P>(n);
        let mut b = arithmetic::<P>(n);
        assert_eq!(
            a.combination(terms, g, bound),
            b.combination_heap(terms, g, bound)
        );
        assert_eq!(
            serde_json::to_value(&a.stats).unwrap(),
            serde_json::to_value(&b.stats).unwrap()
        );
    }
    fn cancellation<const P: u64>() {
        let g = Poly::from([
            (M::read(&[0, 0, 0], 3, 64).unwrap(), P - 1),
            (M::read(&[2, 0, 0], 3, 64).unwrap(), 7),
        ]);
        let terms = [term(0, vec![0, 1, 0], 1), term(1, vec![0, 1, 0], P - 1)];
        let mut a = arithmetic::<P>(3);
        let result = a
            .combination_heap(&terms, &[g.clone(), g.clone()], None)
            .unwrap();
        assert!(result.is_empty());
        assert_eq!(a.heap_products, 4);
        assert_eq!(a.heap_final_entries, 0);
        differential::<P>(3, &[g.clone(), g], &terms, None);
    }
    #[test]
    fn cancellation_large_products_and_empty_combination() {
        cancellation::<2147483647>();
        cancellation::<9007199254740881>();
        differential::<2147483647>(3, &[], &[], None);
    }
    fn orders<const P: u64>() {
        for n in [3, 4, 6, 16] {
            for axis in 0..n {
                let mut m = vec![0; n];
                m[axis] = 63;
                let mut shift = vec![0; n];
                shift[axis] = 1;
                let g = Poly::from([
                    (M::read(&vec![0; n], n, 64).unwrap(), P - 1),
                    (M::read(&m, n, 64).unwrap(), P - 2),
                ]);
                differential::<P>(
                    n,
                    &[g.clone(), g],
                    &[term(0, shift, 2), term(1, vec![0; n], P - 1)],
                    None,
                );
            }
        }
    }
    #[test]
    fn fixed_shifts_preserve_order_through_degree64_and_dimension16() {
        orders::<2147483647>();
        orders::<9007199254740881>();
    }
    #[test]
    fn rejects_exact_same_shape_reference_duplicate_and_standardness_errors() {
        let g = Poly::from([(M::read(&[1, 0, 0], 3, 64).unwrap(), 1)]);
        for terms in [
            vec![term(9, vec![0, 0, 0], 1)],
            vec![term(0, vec![0, 0, 0], 0)],
            vec![term(0, vec![0], 1)],
            vec![term(0, vec![64, 0, 0], 1)],
            vec![term(0, vec![0, 0, 0], 1), term(0, vec![0, 0, 0], 1)],
        ] {
            let mut a = arithmetic::<2147483647>(3);
            let mut b = arithmetic::<2147483647>(3);
            assert_eq!(
                a.combination(&terms, &[g.clone()], None).unwrap_err(),
                b.combination_heap(&terms, &[g.clone()], None).unwrap_err()
            );
        }
        let terms = [term(0, vec![0, 0, 0], 1)];
        let bound = Some(M::read(&[1, 0, 0], 3, 64).unwrap());
        differential::<2147483647>(3, &[g], &terms, bound);
    }
    #[test]
    fn all_work_and_cofactor_budgets_fail_closed() {
        let g = Poly::from([(M::read(&[1, 0, 0], 3, 64).unwrap(), 1)]);
        let terms = [term(0, vec![0, 0, 0], 1)];
        for (cofactors, products) in [(100000, 0), (0, 5000000)] {
            let mut a = arithmetic::<2147483647>(3);
            let mut b = arithmetic::<2147483647>(3);
            a.stats.cofactor_terms = cofactors;
            b.stats.cofactor_terms = cofactors;
            a.stats.referenced_term_products = products;
            b.stats.referenced_term_products = products;
            assert_eq!(
                a.combination(&terms, &[g.clone()], None).unwrap_err(),
                b.combination_heap(&terms, &[g.clone()], None).unwrap_err()
            );
        }
        let many: Vec<_> = (0..50001).map(|_| term(0, vec![0, 0, 0], 1)).collect();
        let mut a = arithmetic::<2147483647>(3);
        let mut b = arithmetic::<2147483647>(3);
        assert_eq!(
            a.combination(&many, &[g.clone()], None).unwrap_err(),
            b.combination_heap(&many, &[g], None).unwrap_err()
        );
    }
    #[test]
    fn full_both_field_replay_and_nonstandard_zero_pair() {
        for p in [2147483647u64, 9007199254740881] {
            let input = json!({"prime":p,"variables":["x","y","z"],"polynomials":[[{"exponents":[1,0,0],"coefficient":p-1}],[{"exponents":[0,1,0],"coefficient":p-1}]]});
            let row = |m: Vec<usize>, i| json!({"polynomial":[{"exponents":m,"coefficient":1}],"proof":[{"generator":i,"exponents":[0,0,0],"coefficient":p-1}]});
            let mut w = json!({"version":1,"definitions":[],"basis":[row(vec![1,0,0],0),row(vec![0,1,0],1)],"input_reductions":[[{"generator":0,"exponents":[0,0,0],"coefficient":p-1}],[{"generator":1,"exponents":[0,0,0],"coefficient":p-1}]],"pair_reductions":[{"left":0,"right":1,"product_criterion":true,"proof":[]}]});
            let ib = serde_json::to_vec(&input).unwrap();
            let wb = serde_json::to_vec(&w).unwrap();
            assert_eq!(
                serde_json::to_value(verify_json(&ib, &wb).unwrap()).unwrap(),
                serde_json::to_value(verify_json_heap(&ib, &wb).unwrap()).unwrap()
            );
            w["pair_reductions"][0]["product_criterion"] = json!(false);
            w["pair_reductions"][0]["proof"] = json!([{"generator":0,"exponents":[0,1,0],"coefficient":1},{"generator":1,"exponents":[1,0,0],"coefficient":p-1}]);
            let wb = serde_json::to_vec(&w).unwrap();
            assert!(verify_json_heap(&ib, &wb)
                .unwrap_err()
                .contains("nonstandard"));
        }
    }
    #[test]
    fn dag512_coverage_and_future_reference_boundaries() {
        let ib=serde_json::to_vec(&json!({"prime":2147483647,"variables":["x","y","z"],"polynomials":[[{"exponents":[1,0,0],"coefficient":1}]]})).unwrap();
        let row = |i| json!({"polynomial":[{"exponents":[1,0,0],"coefficient":1}],"proof":[{"generator":i,"exponents":[0,0,0],"coefficient":1}]});
        let mut w = json!({"version":2,"definitions":(0..512).map(row).collect::<Vec<_>>(),"basis":[row(512)],"input_reductions":[[{"generator":0,"exponents":[0,0,0],"coefficient":1}]],"pair_reductions":[]});
        let wb = serde_json::to_vec(&w).unwrap();
        assert_eq!(
            serde_json::to_value(verify_dag_json(&ib, &wb).unwrap()).unwrap(),
            serde_json::to_value(verify_dag_json_heap(&ib, &wb).unwrap()).unwrap()
        );
        w["definitions"][1]["proof"][0]["generator"] = json!(2);
        assert!(verify_dag_json_heap(&ib, &serde_json::to_vec(&w).unwrap()).is_err());
        w["definitions"][1] = row(1);
        w["definitions"].as_array_mut().unwrap().push(row(512));
        assert!(verify_dag_json_heap(&ib, &serde_json::to_vec(&w).unwrap()).is_err());
    }
}
