//! Shifted subfield lines from PR #2686, in polynomial-basis coordinates.
//! This constructs the base only. The collector's exact MITM oracle is a
//! bounded correctness reference, not an algebraic C1 trial-cost model.
use crate::cryptanalysis::ic_boundary::BinaryInstance;

#[cfg(test)]
#[path = "trace_line_tests.rs"]
mod tests;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct Line {
    pub d: u32,
    pub multiplier: u64,
    pub basis: Vec<u64>,
    pub scanned: u64,
    pub trace_a: u64,
    pub native: BTreeMap<String, u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Evidence {
    pub basis: Vec<u64>,
    pub atoms: Vec<(u64, u64)>,
    pub columns: Vec<(u64, u64)>,
    pub small_columns: usize,
}

/// Replay every raw decomposition and projected linear equation with the
/// general Vec-backed arithmetic, independently of FastCurve's group law.
pub fn verify_relations(
    i: &BinaryInstance,
    target: crate::cryptanalysis::koblitz_fast::FastPoint,
    small_dimension: u32,
    evidence: &Evidence,
    relations: &[super::PartialRelation],
    expected_count: u64,
) -> Result<(), String> {
    use crate::binary_ecc::{
        curve::{point_add, point_neg, scalar_mul},
        BinaryPoint,
    };
    use num_bigint::BigUint;
    let curve = super::general_curve(i);
    if evidence.basis.is_empty() || evidence.basis.len().checked_mul(4) != Some(i.n as usize) {
        return Err("relation replay requires the declared n=4d line".into());
    }
    if small_dimension > evidence.basis.len() as u32 {
        return Err("small_dimension exceeds the declared line dimension".into());
    }
    validate_basis(i, &evidence.basis, evidence.basis.len() as u32)?;
    if relations.len() as u64 != expected_count || evidence.small_columns > evidence.columns.len() {
        return Err("relation transcript truncated or column boundary invalid".into());
    }
    let point = |&(x, y): &(u64, u64)| -> Result<BinaryPoint, String> {
        let p = crate::cryptanalysis::koblitz_fast::FastPoint::affine(x, y);
        if x >= (1 << i.n) || y >= (1 << i.n) || !curve.is_on_curve(&i.fast.lower(p)) {
            return Err("transcript point is off curve or outside field".into());
        }
        Ok(i.fast.lower(p))
    };
    let atoms = evidence
        .atoms
        .iter()
        .map(point)
        .collect::<Result<Vec<_>, _>>()?;
    let columns = evidence
        .columns
        .iter()
        .map(point)
        .collect::<Result<Vec<_>, _>>()?;
    let mut expected_atoms = Vec::new();
    let mut expected_by_key = BTreeMap::new();
    for index in 0..1u64 << evidence.basis.len() {
        let x = expand(&evidence.basis, index);
        for raw in i.points_with_x(x) {
            expected_atoms.push((raw.x, raw.y));
            let projected = scalar_mul(&curve, &i.fast.lower(raw), &BigUint::from(i.cofactor));
            if projected == BinaryPoint::Infinity {
                continue;
            }
            let negated = point_neg(&projected);
            let projected_fast = i.fast.lift(&projected);
            let negated_fast = i.fast.lift(&negated);
            let canonical =
                if (projected_fast.x, projected_fast.y) <= (negated_fast.x, negated_fast.y) {
                    projected_fast
                } else {
                    negated_fast
                };
            expected_by_key
                .entry(canonical.pack())
                .and_modify(|(small, _)| *small |= index < (1u64 << small_dimension))
                .or_insert((
                    index < (1u64 << small_dimension),
                    (canonical.x, canonical.y),
                ));
        }
    }
    if evidence.atoms != expected_atoms {
        return Err("raw atom table does not match the declared line".into());
    }
    let expected_small_columns = expected_by_key.values().filter(|(small, _)| *small).count();
    let expected_columns = expected_by_key
        .values()
        .filter(|(small, _)| *small)
        .chain(expected_by_key.values().filter(|(small, _)| !*small))
        .map(|(_, point)| *point)
        .collect::<Vec<_>>();
    if evidence.small_columns != expected_small_columns || evidence.columns != expected_columns {
        return Err(
            "column table or small-column boundary differs from the declared nested base".into(),
        );
    }
    let q = i.fast.lower(target);
    let mul = |p: &BinaryPoint, n: u64| scalar_mul(&curve, p, &BigUint::from(n));
    let projected = atoms.iter().map(|p| mul(p, i.cofactor)).collect::<Vec<_>>();
    if columns.iter().any(|p| !projected.contains(p)) {
        return Err("column table contains a point outside the projected line".into());
    }
    let mut previous = 0;
    for row in relations {
        if row.trial <= previous
            || row.a >= i.r
            || row.b == 0
            || row.b >= i.r
            || row.raw_terms.len() != 4
        {
            return Err("invalid trial order, query coefficients or summand count".into());
        }
        previous = row.trial;
        let large = row
            .columns
            .iter()
            .filter(|(col, _)| *col > evidence.small_columns)
            .count();
        let class = match large {
            0 => super::RelationClass::Full,
            1 => super::RelationClass::SingleLargePrime,
            2 => super::RelationClass::DoubleLargePrime,
            _ => return Err("more than two large primes".into()),
        };
        if row.class != class {
            return Err("wrong relation class".into());
        }
        let residual = point_add(&curve, &mul(&curve.generator, row.a), &mul(&q, row.b));
        let mut sum = BinaryPoint::Infinity;
        for &term in &row.raw_terms {
            let p = atoms.get(term).ok_or("raw term out of range")?;
            sum = point_add(&curve, &sum, p);
        }
        if sum != residual {
            return Err(format!("raw relation {} failed", row.trial));
        }
        let mut sum = BinaryPoint::Infinity;
        let mut previous_col = None;
        for &(col, coefficient) in &row.columns {
            if coefficient >= i.r || previous_col.is_some_and(|p| p >= col) {
                return Err("noncanonical relation columns".into());
            }
            previous_col = Some(col);
            let p = if col == evidence.small_columns {
                &q
            } else {
                let index = if col < evidence.small_columns {
                    col
                } else {
                    col - 1
                };
                columns.get(index).ok_or("projected column out of range")?
            };
            sum = point_add(&curve, &sum, &mul(p, coefficient));
        }
        let rhs = ((i.cofactor as u128 * row.a as u128) % i.r as u128) as u64;
        if sum != mul(&curve.generator, rhs) {
            return Err(format!("projected relation {} failed", row.trial));
        }
    }
    Ok(())
}

pub fn expand(basis: &[u64], index: u64) -> u64 {
    basis.iter().enumerate().fold(0, |v, (i, b)| {
        v ^ if (index >> i) & 1 != 0 { *b } else { 0 }
    })
}

pub fn validate_basis(i: &BinaryInstance, basis: &[u64], dimension: u32) -> Result<(), String> {
    if basis.len() != dimension as usize || dimension > i.n {
        return Err("abscissa basis length must equal envelope_dimension <= n".into());
    }
    let mut pivots = vec![0; i.n as usize];
    for &b in basis {
        if b >= (1u64 << i.n) {
            return Err("abscissa basis element outside field".into());
        }
        let mut v = b;
        for k in (0..i.n as usize).rev() {
            if (v >> k) & 1 == 0 {
                continue;
            }
            if pivots[k] == 0 {
                pivots[k] = v;
                break;
            }
            v ^= pivots[k];
        }
        if v == 0 {
            return Err("abscissa basis is linearly dependent".into());
        }
    }
    Ok(())
}

struct Field<'a> {
    i: &'a BinaryInstance,
    sq: u64,
    mul: u64,
    xor: u64,
    steps: u64,
}
impl Field<'_> {
    fn pow2(&mut self, mut x: u64, k: u32) -> u64 {
        for _ in 0..k {
            x = self.i.gf.sqr(x);
            self.sq += 1;
        }
        x
    }
    fn relative_trace(&mut self, mut x: u64, d: u32) -> u64 {
        let mut t = 0;
        for _ in 0..self.i.n / d {
            t ^= x;
            self.xor += 1;
            x = self.pow2(x, d);
        }
        t
    }
    fn product(&mut self, a: u64, b: u64) -> u64 {
        self.mul += 1;
        self.i.gf.mul(a, b)
    }
    // Explicit powering makes the field operations in inversions visible.
    fn inverse(&mut self, x: u64) -> u64 {
        let mut z = 1;
        for k in (0..self.i.n).rev() {
            z = self.pow2(z, 1);
            if k != 0 {
                z = self.product(z, x);
            }
        }
        z
    }
    fn subfield_basis(&mut self, d: u32) -> Result<Vec<u64>, String> {
        // Kernel of Frob^d + 1, by RREF over F_2. No exponential scan.
        let n = self.i.n as usize;
        let mut rows = vec![0u64; n];
        for col in 0..n {
            let v = self.pow2(1 << col, d) ^ (1 << col);
            self.xor += 1;
            for (r, row) in rows.iter_mut().enumerate() {
                *row |= ((v >> r) & 1) << col;
            }
        }
        let mut pivots = Vec::new();
        for col in 0..n {
            self.steps += 1;
            let rank = pivots.len();
            let Some(p) = (rank..n).find(|&r| (rows[r] >> col) & 1 != 0) else {
                continue;
            };
            rows.swap(rank, p);
            for r in 0..n {
                if r != rank && (rows[r] >> col) & 1 != 0 {
                    rows[r] ^= rows[rank];
                    self.xor += 1;
                }
            }
            pivots.push(col);
        }
        let mut basis = Vec::new();
        for free in (0..n).filter(|c| !pivots.contains(c)) {
            let mut v = 1u64 << free;
            for (r, &p) in pivots.iter().enumerate() {
                v |= ((rows[r] >> free) & 1) << p;
            }
            basis.push(v);
        }
        if basis.len() != d as usize {
            return Err("subfield kernel has wrong dimension".into());
        }
        Ok(basis)
    }
}

/// `standard` uses F_(2^d); `trace` uses the first qualifying u in a
/// preregistered integer scan. Exhaustion is an error, never a fallback.
pub fn construct(i: &BinaryInstance, d: u32, policy: &str, max_u: u64) -> Result<Line, String> {
    if d == 0 || d.checked_mul(4) != Some(i.n) || i.n > 62 {
        return Err("trace-line experiment requires n = 4d <= 62, d > 0".into());
    }
    if policy != "standard" && policy != "trace" {
        return Err("line is standard or trace".into());
    }
    if max_u == 0 || max_u >= (1u64 << i.n) {
        return Err("max_u must be in 1..2^n".into());
    }
    let mut f = Field {
        i,
        sq: 0,
        mul: 0,
        xor: 0,
        steps: 0,
    };
    let trace_a = f.relative_trace(i.a, 1);
    if policy == "trace" && trace_a != 0 {
        return Err("trace-selected line would lift to the twist: Tr(a) != 0".into());
    }
    let subfield = f.subfield_basis(d)?;
    let c = f.pow2(i.b, i.n - 1);
    let mut scanned = 0;
    let multiplier = if policy == "standard" {
        1
    } else {
        let mut found = None;
        for u in 1..=max_u {
            scanned += 1;
            if f.relative_trace(u, d) != 0 {
                continue;
            }
            let inv = f.inverse(u);
            let cu = f.product(c, inv);
            if f.relative_trace(cu, d) == 0 {
                found = Some(u);
                break;
            }
        }
        found.ok_or(
            "trace multiplier scan exhausted; increase only by a prospective protocol amendment",
        )?
    };
    let basis = subfield
        .into_iter()
        .map(|b| f.product(multiplier, b))
        .collect::<Vec<_>>();
    validate_basis(i, &basis, d)?;
    let native = BTreeMap::from([
        ("line_field_squarings_uncharged".into(), f.sq),
        ("line_field_multiplications_uncharged".into(), f.mul),
        ("line_word_xors_uncharged".into(), f.xor),
        ("line_kernel_columns_uncharged".into(), f.steps),
        ("line_scan_candidates_uncharged".into(), scanned),
    ]);
    Ok(Line {
        d,
        multiplier,
        basis,
        scanned,
        trace_a,
        native,
    })
}
