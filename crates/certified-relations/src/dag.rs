//! Bounded shared intermediate rows for provenance-carrying Buchberger.
use super::*;
const NODE_CAP: usize = 512;
pub(crate) struct Output {
    pub definitions: Vec<Row>,
    pub basis: Vec<Row>,
}
struct Store {
    generators: Vec<Poly>,
    definitions: Vec<Row>,
    n: usize,
    p: u64,
    terms: usize,
    work: usize,
}
impl Store {
    fn retain(&mut self, row: Row) -> Result<Row, String> {
        if row.poly.is_empty()
            || row.poly.len() > 5000
            || degree(&row.poly) > 64
            || row.proof.len() > 50_000
            || self.definitions.len() >= NODE_CAP
        {
            return Err("dag row/node budget".into());
        }
        let mut work = 0usize;
        for ((i, m), c) in &row.proof {
            let g = self.generators.get(*i).ok_or("dag future reference")?;
            let d = m
                .iter()
                .try_fold(0usize, |a, b| a.checked_add(*b))
                .ok_or("dag exponent overflow")?;
            if m.len() != self.n || *c == 0 || *c >= self.p || d + degree(g) > 64 {
                return Err("dag cofactor shape/degree".into());
            }
            work = work.checked_add(g.len()).ok_or("dag work overflow")?;
        }
        self.terms = self
            .terms
            .checked_add(row.proof.len())
            .ok_or("dag terms overflow")?;
        self.work = self.work.checked_add(work).ok_or("dag work overflow")?;
        if work > 1_000_000
            || self.work > 5_000_000
            || self.terms > 100_000
            || !verify(&row, &self.generators, self.p)
        {
            return Err("dag exact membership/work budget".into());
        }
        let index = self.generators.len();
        self.generators.push(row.poly.clone());
        let reference = Row {
            poly: row.poly.clone(),
            proof: BTreeMap::from([((index, vec![0; self.n]), 1)]),
        };
        self.definitions.push(row);
        Ok(reference)
    }
}
pub(crate) fn solve(inputs: &[Poly], n: usize, p: u64) -> Result<Output, String> {
    if ![2147483647, 9007199254740881].contains(&p)
        || !(3..=quotient::MAX_VARIABLES).contains(&n)
        || !(4..=32).contains(&inputs.len())
        || inputs.iter().any(|f| f.len() > 32 || degree(f) > 4)
    {
        return Err("dag input budget".into());
    }
    let pairs = cheap::consequences(&inputs[..4], n, p);
    if pairs.len() != 2 {
        return Err("dag recognized initial pair roster".into());
    }
    let mut store = Store {
        generators: inputs.to_vec(),
        definitions: vec![],
        n,
        p,
        terms: 0,
        work: 0,
    };
    let mut rows = input_rows(inputs, n);
    for pair in pairs {
        rows.push(store.retain(pair.cubic_syzygy)?);
    }
    let mut basis = Vec::new();
    for row in rows {
        let r = monic(normal_form(row, &basis, p), p);
        if !r.poly.is_empty() {
            basis.push(store.retain(r)?);
        }
    }
    for round in 0..128 {
        let mut changed = false;
        let mut i = 0;
        while i < basis.len() {
            let others: Vec<_> = basis
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, r)| r.clone())
                .collect();
            let r = monic(normal_form(basis[i].clone(), &others, p), p);
            if r.poly.is_empty() {
                basis.remove(i);
                changed = true;
                continue;
            }
            if r.poly != basis[i].poly {
                basis[i] = store.retain(r)?;
                changed = true;
            }
            i += 1;
        }
        if !changed {
            break;
        }
        if round == 127 {
            return Err("dag initial interreduction budget".into());
        }
    }
    let mut pairs = VecDeque::new();
    for j in 0..basis.len() {
        for i in 0..j {
            pairs.push_back((i, j));
        }
    }
    let mut count = 0;
    while let Some((i, j)) = pairs.pop_front() {
        count += 1;
        if count > 10000 || basis.len() > 128 {
            return Err("dag pair/basis budget".into());
        }
        if lead(&basis[i].poly)
            .iter()
            .zip(lead(&basis[j].poly))
            .all(|(a, b)| *a == 0 || *b == 0)
        {
            continue;
        }
        let row = monic(
            normal_form(s_polynomial(&basis[i], &basis[j], p), &basis, p),
            p,
        );
        if !row.poly.is_empty() {
            let row = store.retain(row)?;
            let k = basis.len();
            for i in 0..k {
                pairs.push_back((i, k));
            }
            basis.push(row);
        }
    }
    let minimal: Vec<_> = basis
        .iter()
        .enumerate()
        .filter(|(i, g)| {
            !basis.iter().enumerate().any(|(j, h)| {
                i != &j
                    && divides(lead(&h.poly), lead(&g.poly))
                    && (lead(&h.poly) != lead(&g.poly) || j < *i)
            })
        })
        .map(|(_, g)| g.clone())
        .collect();
    let mut reduced = Vec::new();
    for (i, row) in minimal.iter().enumerate() {
        let others: Vec<_> = minimal
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, r)| r.clone())
            .collect();
        let r = monic(normal_form(row.clone(), &others, p), p);
        if r.poly == row.poly {
            reduced.push(row.clone());
        } else {
            reduced.push(store.retain(r)?);
        }
    }
    reduced.sort_by(|a, b| cmp(lead(&b.poly), lead(&a.poly)));
    // Includes original and all retained-generator containment, all final
    // membership and product/standard pair checks; no status-only acceptance.
    if !check_reference_product(&reduced, &store.generators, n, p) {
        return Err("dag final complete native checks".into());
    }
    Ok(Output {
        definitions: store.definitions,
        basis: reduced,
    })
}

/// Generic exact coefficient-DAG sparsification. Construction still performs
/// every original native check; transformed identities are checked again.
pub(crate) fn solve_pruned(inputs: &[Poly], n: usize, p: u64) -> Result<(Output, Value), String> {
    let original = solve(inputs, n, p)?;
    let count = inputs.len();
    let d = original.definitions.len();
    let mut polynomials = inputs.to_vec();
    polynomials.extend(original.definitions.iter().map(|r| r.poly.clone()));
    let mut rows: Vec<Option<Row>> = original
        .definitions
        .into_iter()
        .chain(original.basis)
        .map(Some)
        .collect();
    let mut live = vec![false; d];
    let mut stack = Vec::new();
    for row in rows.iter().skip(d).flatten() {
        for (i, _) in row.proof.keys() {
            if *i >= count {
                stack.push(*i - count);
            }
        }
    }
    while let Some(j) = stack.pop() {
        if j >= d {
            return Err("prune future reference".into());
        }
        if live[j] {
            continue;
        }
        live[j] = true;
        for (i, _) in rows[j].as_ref().unwrap().proof.keys() {
            if *i >= count {
                stack.push(*i - count);
            }
        }
    }
    let before_terms: usize = rows.iter().flatten().map(|r| r.proof.len()).sum();
    let work = |rs: &[Option<Row>]| {
        rs.iter()
            .flatten()
            .map(|r| {
                r.proof
                    .keys()
                    .map(|(i, _)| polynomials[*i].len())
                    .sum::<usize>()
            })
            .sum::<usize>()
    };
    let before_work = work(&rows);
    for j in 0..d {
        if !live[j] {
            rows[j] = None;
        }
    }
    let mut uses = vec![0usize; count + d];
    for row in rows.iter().flatten() {
        for (i, _) in row.proof.keys() {
            uses[*i] += 1;
        }
    }
    let mut aliases = 0usize;
    let mut unique = 0usize;
    let mut refused = 0usize;
    for j in (0..d).rev() {
        let Some(node) = rows[j].as_ref() else {
            continue;
        };
        let index = count + j;
        if uses[index] == 0 {
            let old = rows[j].take().unwrap();
            for (i, _) in old.proof.keys() {
                uses[*i] -= 1;
            }
            continue;
        }
        let alias = node.proof.len() == 1;
        if !alias && uses[index] != 1 {
            continue;
        }
        let inner = node.proof.clone();
        let mut edits = Vec::new();
        let mut allowed = true;
        for (k, outer) in rows
            .iter()
            .enumerate()
            .skip(j + 1)
            .filter_map(|(k, r)| r.as_ref().map(|r| (k, r)))
        {
            let factors: Vec<_> = outer
                .proof
                .range((index, vec![])..(index + 1, vec![]))
                .map(|((_, m), c)| (m.clone(), *c))
                .collect();
            if factors.is_empty() {
                continue;
            }
            let mut replacement = outer.proof.clone();
            for (m, _) in &factors {
                replacement.remove(&(index, m.clone()));
            }
            for (shift, coefficient) in factors {
                for ((i, m), c) in &inner {
                    let Some(mono) = m
                        .iter()
                        .zip(&shift)
                        .map(|(a, b)| a.checked_add(*b))
                        .collect::<Option<Vec<_>>>()
                    else {
                        allowed = false;
                        break;
                    };
                    let Some(deg) = mono.iter().try_fold(0usize, |a, b| a.checked_add(*b)) else {
                        allowed = false;
                        break;
                    };
                    if deg + degree(&polynomials[*i]) > 64 {
                        allowed = false;
                        break;
                    }
                    let key = (*i, mono);
                    let value =
                        (replacement.get(&key).copied().unwrap_or(0) + mul(*c, coefficient, p)) % p;
                    if value == 0 {
                        replacement.remove(&key);
                    } else {
                        replacement.insert(key, value);
                    }
                }
            }
            let products: usize = replacement.keys().map(|(i, _)| polynomials[*i].len()).sum();
            if replacement.len() > 50_000 || products > 1_000_000 {
                allowed = false;
            }
            edits.push((k, replacement));
            if !allowed {
                break;
            }
        }
        if !allowed {
            refused += 1;
            continue;
        }
        for (k, proof) in edits {
            let row = rows[k].as_mut().unwrap();
            for (i, _) in row.proof.keys() {
                uses[*i] -= 1;
            }
            for (i, _) in proof.keys() {
                uses[*i] += 1;
            }
            row.proof = proof;
        }
        if uses[index] != 0 {
            return Err("prune retained unresolved consumer".into());
        }
        let old = rows[j].take().unwrap();
        for (i, _) in old.proof.keys() {
            uses[*i] -= 1;
        }
        if alias {
            aliases += 1;
        } else {
            unique += 1;
        }
    }
    let after_terms: usize = rows.iter().flatten().map(|r| r.proof.len()).sum();
    let after_work = work(&rows);
    let mut mapping: Vec<Option<usize>> = (0..count)
        .map(Some)
        .chain(std::iter::repeat_n(None, d))
        .collect();
    let mut generators = inputs.to_vec();
    let mut definitions = Vec::new();
    let mut basis = Vec::new();
    for (j, row) in rows.into_iter().enumerate() {
        let Some(mut row) = row else { continue };
        row.proof = row
            .proof
            .into_iter()
            .map(|((i, m), c)| {
                Ok((
                    (
                        mapping
                            .get(i)
                            .and_then(|v| *v)
                            .ok_or("prune missing parent mapping")?,
                        m,
                    ),
                    c,
                ))
            })
            .collect::<Result<_, String>>()?;
        if !verify(&row, &generators, p) {
            return Err("prune transformed identity mismatch".into());
        }
        if j < d {
            mapping[count + j] = Some(generators.len());
            generators.push(row.poly.clone());
            definitions.push(row);
        } else {
            basis.push(row);
        }
    }
    if !check_reference_product(&basis, &generators, n, p) {
        return Err("prune complete native basis check".into());
    }
    let trace = json!({"before_nodes":d,"after_nodes":definitions.len(),"before_proof_terms":before_terms,"after_proof_terms":after_terms,"before_term_products":before_work,"after_term_products":after_work,"inlined_aliases":aliases,"inlined_unique":unique,"cap_refusals":refused});
    Ok((Output { definitions, basis }, trace))
}
