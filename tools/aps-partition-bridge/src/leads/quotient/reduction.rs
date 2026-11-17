use crypto_lib::cryptanalysis::f4_fp::{self, Ordering, Poly};
use std::collections::{BTreeMap, BTreeSet};

fn add_term(out: &mut BTreeMap<Vec<u32>, u64>, e: Vec<u32>, c: u64, p: u64) {
    let value = (out.get(&e).copied().unwrap_or(0) + c) % p;
    if value == 0 {
        out.remove(&e);
    } else {
        out.insert(e, value);
    }
}

/// Normalise's monic scaling is undone: preserve values, not just zero sets.
fn canonical(terms: BTreeMap<Vec<u32>, u64>, p: u64) -> Poly {
    let source: Vec<_> = terms.iter().map(|(e, c)| (e.clone(), *c)).collect();
    let mut result = f4_fp::normalise(&source, p, Ordering::Grevlex);
    if let Some((e, _)) = result.first() {
        let scale = terms[e];
        for (_, coefficient) in &mut result {
            *coefficient = (*coefficient * scale) % p;
        }
    }
    result
}

fn vanish(xs: &[u64], p: u64) -> Vec<u64> {
    let mut g = vec![1];
    for &x in xs {
        let mut next = vec![0; g.len() + 1];
        for (j, &coefficient) in g.iter().enumerate() {
            next[j] = (next[j] + coefficient * ((p - x) % p)) % p;
            next[j + 1] = (next[j + 1] + coefficient) % p;
        }
        g = next;
    }
    g
}

/// Divide by monic polynomials in separate variables, giving a unique remainder.
pub(super) fn remainder(poly: &Poly, parts: &[Vec<u64>], p: u64) -> Result<(Poly, usize), String> {
    if !(5..(1u64 << 32)).contains(&p) || parts.len() != 4 {
        return Err("bounded quotient requires four domains and 5<=p<2^32".into());
    }
    let mut seen = BTreeSet::new();
    if parts.iter().any(|xs| xs.is_empty() || xs.len() > 2)
        || parts.iter().flatten().any(|&x| x >= p || !seen.insert(x))
        || poly.iter().any(|(e, _)| e.len() != 4)
    {
        return Err("invalid, repeated or overlapping coordinate domains".into());
    }
    let mut terms = BTreeMap::new();
    for (e, c) in poly {
        add_term(&mut terms, e.clone(), c % p, p);
    }
    let mut rewrites = 0;
    for (axis, xs) in parts.iter().enumerate() {
        let g = vanish(xs, p);
        let degree = xs.len() as u32;
        loop {
            let next = terms
                .iter()
                .find(|(e, _)| e[axis] >= degree)
                .map(|(e, c)| (e.clone(), *c));
            let Some((e, coefficient)) = next else {
                break;
            };
            terms.remove(&e);
            for (j, &lower) in g[..xs.len()].iter().enumerate() {
                let mut reduced = e.clone();
                reduced[axis] = reduced[axis] - degree + j as u32;
                add_term(&mut terms, reduced, coefficient * ((p - lower) % p) % p, p);
            }
            rewrites += 1;
        }
    }
    Ok((canonical(terms, p), rewrites))
}
