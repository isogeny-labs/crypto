//! Fixed-degree raw/closed spaces with original-ideal and bounded-derivation witnesses.
//! This does not compute an F4/F5 runtime or assert a degree of regularity.
use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Raw,
    Closed,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Limits {
    pub max_columns: usize,
    pub max_rows: usize,
    pub max_sources: usize,
    pub max_polynomial_terms: usize,
    pub max_proof_terms: usize,
    pub max_retained_proof_terms: usize,
    pub max_certificate_expansion_terms: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_columns: 4096,
            max_rows: 200_000,
            max_sources: 2000,
            max_polynomial_terms: 4096,
            max_proof_terms: 100_000,
            max_retained_proof_terms: 2_000_000,
            max_certificate_expansion_terms: 2_000_000,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "index", rename_all = "snake_case")]
pub enum Origin {
    Original(usize),
    Extra(usize),
    Basis(usize),
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reduction {
    pub basis_index: usize,
    pub coefficient: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Derivation {
    pub source: Origin,
    pub multiplier: Mono,
    pub reductions: Vec<Reduction>,
    pub scale: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoefficientTerm {
    pub exponents: Mono,
    pub coefficient: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MembershipTerm {
    pub input_index: usize,
    pub multiplier: Mono,
    pub coefficient: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedRow {
    pub polynomial: Vec<CoefficientTerm>,
    pub membership: Vec<MembershipTerm>,
    pub derivation: Derivation,
    pub certificate_max_product_degree: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub prime: u64,
    pub variables: usize,
    pub degree: usize,
    pub mode: Mode,
    pub status: String,
    pub closure_completed: bool,
    pub columns: usize,
    pub rows_processed: usize,
    pub reductions: usize,
    pub closure_rounds: usize,
    pub basis: Vec<CertifiedRow>,
}
#[derive(Clone, Debug, Serialize)]
pub struct Verified {
    /// None for a censored snapshot; no difference of lower bounds is called a gain.
    pub exact_space_rank: Option<usize>,
    pub rank_lower_bound: usize,
    pub original_memberships_verified: bool,
    pub bounded_derivations_verified: bool,
}
#[derive(Clone, Debug, Serialize)]
pub struct Counterfactual {
    pub baseline: Snapshot,
    pub augmented: Snapshot,
    pub rank_gain: Option<usize>,
}
#[derive(Clone)]
struct Source {
    row: Row,
    origin: Origin,
}
fn checked_degree(m: &Mono) -> Result<usize, String> {
    m.iter().try_fold(0usize, |v, e| {
        v.checked_add(*e).ok_or_else(|| "degree overflow".into())
    })
}
fn validate_poly(f: &Poly, n: usize, p: u64) -> Result<(), String> {
    if f.iter().any(|(m, c)| m.len() != n || *c == 0 || *c >= p) {
        return Err("noncanonical polynomial".into());
    }
    for m in f.keys() {
        checked_degree(m)?;
    }
    Ok(())
}
fn proof_peak(row: &Row, inputs: &[Poly], n: usize, p: u64) -> Result<usize, String> {
    validate_poly(&row.poly, n, p)?;
    let mut peak = 0;
    for ((i, m), c) in &row.proof {
        if *i >= inputs.len() || m.len() != n || *c == 0 || *c >= p {
            return Err("invalid original-generator witness".into());
        }
        let md = checked_degree(m)?;
        let fd = inputs[*i]
            .keys()
            .map(checked_degree)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .max()
            .unwrap_or(0);
        peak = peak.max(md.checked_add(fd).ok_or("certificate degree overflow")?);
    }
    Ok(peak)
}
fn expansion_fits(row: &Row, inputs: &[Poly], limits: Limits) -> bool {
    row.proof.len() <= limits.max_proof_terms
        && row.poly.len() <= limits.max_polynomial_terms
        && row
            .proof
            .keys()
            .try_fold(0usize, |n, (i, _)| n.checked_add(inputs[*i].len()))
            .is_some_and(|n| n <= limits.max_certificate_expansion_terms)
}
fn validate(
    inputs: &[Poly],
    n: usize,
    p: u64,
    d: usize,
    extra: &[Row],
    limits: Limits,
) -> Result<(), String> {
    if n == 0 || n > 16 || d > 16 || p > i64::MAX as u64 || !prime(p) {
        return Err("need1..16variables,degree<=16 and a prime below2^63".into());
    }
    for f in inputs {
        validate_poly(f, n, p)?;
        if f.len() > limits.max_polynomial_terms {
            return Err("input polynomial budget".into());
        }
    }
    for row in extra {
        proof_peak(row, inputs, n, p)?;
        if !expansion_fits(row, inputs, limits) {
            return Err("extra certificate budget".into());
        }
    }
    Ok(())
}
fn encoded(row: &Row, derivation: Derivation, inputs: &[Poly], n: usize, p: u64) -> CertifiedRow {
    CertifiedRow {
        polynomial: row
            .poly
            .iter()
            .map(|(m, c)| CoefficientTerm {
                exponents: m.clone(),
                coefficient: *c,
            })
            .collect(),
        membership: row
            .proof
            .iter()
            .map(|((i, m), c)| MembershipTerm {
                input_index: *i,
                multiplier: m.clone(),
                coefficient: *c,
            })
            .collect(),
        derivation,
        certificate_max_product_degree: proof_peak(row, inputs, n, p)
            .expect("validated producer provenance"),
    }
}
/// No reference basis enters construction or termination. Extra rows retain
/// ORIGINAL-input witnesses while being admitted as generators of this presentation.
pub fn snapshot(
    inputs: &[Poly],
    n: usize,
    p: u64,
    d: usize,
    extra: &[Row],
    mode: Mode,
    limits: Limits,
) -> Result<Snapshot, String> {
    validate(inputs, n, p, d, extra, limits)?;
    for row in extra {
        if !verify(row, inputs, p) {
            return Err("extra is not certified in the original ideal".into());
        }
    }
    let mut out = Snapshot {
        prime: p,
        variables: n,
        degree: d,
        mode,
        status: "complete".into(),
        closure_completed: true,
        columns: columns(n, d, false),
        rows_processed: 0,
        reductions: 0,
        closure_rounds: 0,
        basis: vec![],
    };
    if out.columns > limits.max_columns {
        out.status = "column_budget".into();
        out.closure_completed = false;
        return Ok(out);
    }
    if inputs
        .len()
        .checked_add(extra.len())
        .is_none_or(|v| v > limits.max_sources)
    {
        out.status = "source_or_proof_budget".into();
        out.closure_completed = false;
        return Ok(out);
    }
    let mut sources: Vec<Source> = input_rows(inputs, n)
        .into_iter()
        .enumerate()
        .map(|(i, row)| Source {
            row,
            origin: Origin::Original(i),
        })
        .collect();
    sources.extend(extra.iter().cloned().enumerate().map(|(i, row)| Source {
        row,
        origin: Origin::Extra(i),
    }));
    let mut retained = sources
        .iter()
        .try_fold(0usize, |v, s| v.checked_add(s.row.proof.len()))
        .ok_or("proof storage overflow")?;
    if sources.len() > limits.max_sources || retained > limits.max_retained_proof_terms {
        out.status = "source_or_proof_budget".into();
        out.closure_completed = false;
        return Ok(out);
    }
    let mut seen: BTreeSet<Poly> = sources.iter().map(|s| s.row.poly.clone()).collect();
    let mut processed = BTreeSet::new();
    let mut basis: Vec<Row> = vec![];
    let mut pivots: BTreeMap<Mono, usize> = BTreeMap::new();
    let mut hit = None;
    'round: loop {
        let mut did_work = false;
        let source_count = sources.len();
        for sid in 0..source_count {
            if sources[sid].row.poly.is_empty() || degree(&sources[sid].row.poly) > d {
                continue;
            }
            for shift in monomials(n, d - degree(&sources[sid].row.poly), false) {
                if processed.contains(&(sid, shift.clone())) {
                    continue;
                }
                if out.rows_processed >= limits.max_rows {
                    hit = Some("row_budget");
                    break 'round;
                }
                processed.insert((sid, shift.clone()));
                out.rows_processed += 1;
                if !did_work {
                    out.closure_rounds += 1;
                    did_work = true;
                }
                let mut row = row_shift(&sources[sid].row, &shift);
                let mut reductions = vec![];
                if !expansion_fits(&row, inputs, limits) {
                    hit = Some("polynomial_or_proof_budget");
                    break 'round;
                }
                while !row.poly.is_empty() {
                    let lm = lead(&row.poly).clone();
                    let Some(&i) = pivots.get(&lm) else {
                        break;
                    };
                    let c = row.poly[&lm];
                    if row
                        .proof
                        .len()
                        .checked_add(basis[i].proof.len())
                        .is_none_or(|v| v > limits.max_proof_terms)
                        || row
                            .poly
                            .len()
                            .checked_add(basis[i].poly.len())
                            .is_none_or(|v| v > limits.max_polynomial_terms)
                    {
                        hit = Some("reduction_reservation_budget");
                        break 'round;
                    }
                    subtract_row(&mut row, &basis[i], c, p);
                    out.reductions += 1;
                    reductions.push(Reduction {
                        basis_index: i,
                        coefficient: c,
                    });
                }
                if row.poly.is_empty() {
                    continue;
                }
                let scale = inv(row.poly[lead(&row.poly)], p);
                super::scale(&mut row, scale, p);
                if !expansion_fits(&row, inputs, limits) {
                    hit = Some("verification_budget");
                    break 'round;
                }
                let append = mode == Mode::Closed && !seen.contains(&row.poly);
                if append && sources.len() >= limits.max_sources {
                    hit = Some("source_budget");
                    break 'round;
                }
                let copies = 1 + usize::from(append);
                let Some(next) = row
                    .proof
                    .len()
                    .checked_mul(copies)
                    .and_then(|v| v.checked_add(retained))
                else {
                    hit = Some("retained_proof_budget");
                    break 'round;
                };
                if next > limits.max_retained_proof_terms {
                    hit = Some("retained_proof_budget");
                    break 'round;
                }
                if !verify(&row, inputs, p) {
                    return Err("producer original provenance failure".into());
                }
                let id = basis.len();
                pivots.insert(lead(&row.poly).clone(), id);
                retained = next;
                out.basis.push(encoded(
                    &row,
                    Derivation {
                        source: sources[sid].origin.clone(),
                        multiplier: shift,
                        reductions,
                        scale,
                    },
                    inputs,
                    n,
                    p,
                ));
                basis.push(row.clone());
                if append {
                    seen.insert(row.poly.clone());
                    sources.push(Source {
                        row,
                        origin: Origin::Basis(id),
                    });
                }
            }
        }
        if !did_work {
            break;
        }
    }
    if let Some(reason) = hit {
        out.status = reason.into();
        out.closure_completed = false;
    }
    Ok(out)
}

// Separate wide-arithmetic replay. It never calls producer row arithmetic or Space.
fn q_add(out: &mut Poly, rhs: &Poly, c: u64, p: u64) {
    for (m, a) in rhs {
        let v = ((out.get(m).copied().unwrap_or(0) as u128 + *a as u128 * c as u128) % p as u128)
            as u64;
        if v == 0 {
            out.remove(m);
        } else {
            out.insert(m.clone(), v);
        }
    }
}
fn q_shift(f: &Poly, m: &Mono) -> Result<Poly, String> {
    f.iter()
        .map(|(a, c)| {
            let e = a
                .iter()
                .zip(m)
                .map(|(x, y)| x.checked_add(*y).ok_or("exponent overflow".to_string()))
                .collect::<Result<Vec<_>, _>>()?;
            Ok((e, *c))
        })
        .collect()
}
fn q_inv(a: u64, p: u64) -> u64 {
    let (mut x, mut e, mut v) = (a, p - 2, 1u64);
    while e > 0 {
        if e & 1 != 0 {
            v = ((v as u128 * x as u128) % p as u128) as u64;
        }
        x = ((x as u128 * x as u128) % p as u128) as u64;
        e >>= 1;
    }
    v
}
fn q_lm(f: &Poly) -> Option<&Mono> {
    f.keys().max_by(|a, b| {
        a.iter()
            .sum::<usize>()
            .cmp(&b.iter().sum::<usize>())
            .then_with(|| a.cmp(b))
    })
}
fn q_reduce(mut f: Poly, basis: &BTreeMap<Mono, Poly>, p: u64) -> Poly {
    while let Some(lm) = q_lm(&f).cloned() {
        let Some(g) = basis.get(&lm) else {
            break;
        };
        let c = f[&lm];
        q_add(&mut f, g, if c == 0 { 0 } else { p - c }, p);
    }
    f
}
fn q_insert(f: Poly, basis: &mut BTreeMap<Mono, Poly>, p: u64) -> bool {
    let mut r = q_reduce(f, basis, p);
    let Some(lm) = q_lm(&r).cloned() else {
        return false;
    };
    let reciprocal = q_inv(r[&lm], p);
    for c in r.values_mut() {
        *c = ((*c as u128 * reciprocal as u128) % p as u128) as u64;
    }
    basis.insert(lm, r);
    true
}
fn q_monomials(n: usize, d: usize, limit: usize) -> Result<Vec<Mono>, String> {
    let mut all = BTreeSet::from([vec![0; n]]);
    let mut frontier = vec![vec![0; n]];
    for _ in 0..d {
        let mut next = vec![];
        for m in frontier {
            for j in 0..n {
                let mut a = m.clone();
                a[j] += 1;
                if all.insert(a.clone()) {
                    if all.len() > limit {
                        return Err("replay column budget".into());
                    }
                    next.push(a);
                }
            }
        }
        frontier = next;
    }
    Ok(all.into_iter().collect())
}
fn decode_row(row: &CertifiedRow, n: usize, p: u64) -> Result<Row, String> {
    let mut f = Poly::new();
    for t in &row.polynomial {
        if t.exponents.len() != n
            || t.coefficient == 0
            || t.coefficient >= p
            || f.insert(t.exponents.clone(), t.coefficient).is_some()
        {
            return Err("noncanonical encoded polynomial".into());
        }
        checked_degree(&t.exponents)?;
    }
    let mut proof = Proof::new();
    for t in &row.membership {
        if t.multiplier.len() != n
            || t.coefficient == 0
            || t.coefficient >= p
            || proof
                .insert((t.input_index, t.multiplier.clone()), t.coefficient)
                .is_some()
        {
            return Err("noncanonical encoded witness".into());
        }
    }
    Ok(Row { poly: f, proof })
}
fn q_membership(
    row: &Row,
    inputs: &[Poly],
    n: usize,
    p: u64,
    limits: Limits,
) -> Result<(), String> {
    proof_peak(row, inputs, n, p)?;
    if !expansion_fits(row, inputs, limits) {
        return Err("replay witness budget".into());
    }
    let mut f = Poly::new();
    for ((i, m), c) in &row.proof {
        q_add(&mut f, &q_shift(&inputs[*i], m)?, *c, p);
    }
    if f != row.poly {
        return Err("original-ideal witness fails".into());
    }
    Ok(())
}
/// Establishes both inclusions using a degree-bounded derivation and a separately
/// row-reduced space closed under all admitted source products/multiplications.
pub fn replay(
    inputs: &[Poly],
    n: usize,
    p: u64,
    extra: &[Row],
    s: &Snapshot,
    limits: Limits,
) -> Result<Verified, String> {
    validate(inputs, n, p, s.degree, extra, limits)?;
    if s.prime != p
        || s.variables != n
        || s.closure_completed != (s.status == "complete")
        || s.rows_processed > limits.max_rows
        || s.basis.len() > s.rows_processed
        || s.basis.len() > limits.max_columns
        || s.columns != columns(n, s.degree, false)
    {
        return Err("snapshot metadata mismatch".into());
    }
    for e in extra {
        q_membership(e, inputs, n, p, limits)?;
    }
    let mut previous: Vec<Row> = vec![];
    let mut basis = BTreeMap::new();
    for (i, encoded) in s.basis.iter().enumerate() {
        let row = decode_row(encoded, n, p)?;
        q_membership(&row, inputs, n, p, limits)?;
        if row.poly.is_empty()
            || degree(&row.poly) > s.degree
            || proof_peak(&row, inputs, n, p)? != encoded.certificate_max_product_degree
        {
            return Err("row degree or certificate metric".into());
        }
        let step = &encoded.derivation;
        let source = match step.source {
            Origin::Original(j) => inputs.get(j).ok_or("bad original origin")?,
            Origin::Extra(j) => &extra.get(j).ok_or("bad extra origin")?.poly,
            Origin::Basis(j) => {
                if s.mode != Mode::Closed || j >= i {
                    return Err("bad derived origin".into());
                }
                &previous[j].poly
            }
        };
        if step.multiplier.len() != n
            || source.is_empty()
            || degree(source)
                .checked_add(checked_degree(&step.multiplier)?)
                .is_none_or(|d| d > s.degree)
            || step.scale == 0
            || step.scale >= p
        {
            return Err("derivation leaves degree bound".into());
        }
        let mut f = q_shift(source, &step.multiplier)?;
        for sub in &step.reductions {
            if sub.basis_index >= i || sub.coefficient >= p {
                return Err("bad reduction reference".into());
            }
            q_add(
                &mut f,
                &previous[sub.basis_index].poly,
                if sub.coefficient == 0 {
                    0
                } else {
                    p - sub.coefficient
                },
                p,
            );
        }
        for c in f.values_mut() {
            *c = ((*c as u128 * step.scale as u128) % p as u128) as u64;
        }
        if f != row.poly || !q_insert(row.poly.clone(), &mut basis, p) {
            return Err("bounded derivation or independence fails".into());
        }
        previous.push(row);
    }
    if s.closure_completed {
        let mons = q_monomials(n, s.degree, limits.max_columns)?;
        if mons.len() != s.columns {
            return Err("column census mismatch".into());
        }
        for f in inputs.iter().chain(extra.iter().map(|r| &r.poly)) {
            if f.is_empty() || degree(f) > s.degree {
                continue;
            }
            if s.mode == Mode::Raw {
                for m in &mons {
                    if checked_degree(m)? <= s.degree - degree(f)
                        && !q_reduce(q_shift(f, m)?, &basis, p).is_empty()
                    {
                        return Err("raw source product missing".into());
                    }
                }
            } else if !q_reduce(f.clone(), &basis, p).is_empty() {
                return Err("initial generator missing".into());
            }
        }
        if s.mode == Mode::Closed {
            // The verifier's graded-lex echelon basis spans every low-degree
            // part of this vector space, even if transmitted rows did not.
            for f in basis.values() {
                if degree(f) < s.degree {
                    for j in 0..n {
                        let mut x = vec![0; n];
                        x[j] = 1;
                        if !q_reduce(q_shift(f, &x)?, &basis, p).is_empty() {
                            return Err("space is not multiplication-closed".into());
                        }
                    }
                }
            }
        }
    }
    Ok(Verified {
        exact_space_rank: s.closure_completed.then_some(previous.len()),
        rank_lower_bound: previous.len(),
        original_memberships_verified: true,
        bounded_derivations_verified: true,
    })
}
pub fn counterfactual(
    inputs: &[Poly],
    n: usize,
    p: u64,
    d: usize,
    candidate: &Row,
    mode: Mode,
    limits: Limits,
) -> Result<Counterfactual, String> {
    let baseline = snapshot(inputs, n, p, d, &[], mode, limits)?;
    let augmented = snapshot(
        inputs,
        n,
        p,
        d,
        std::slice::from_ref(candidate),
        mode,
        limits,
    )?;
    let a = replay(inputs, n, p, &[], &baseline, limits)?;
    let b = replay(
        inputs,
        n,
        p,
        std::slice::from_ref(candidate),
        &augmented,
        limits,
    )?;
    let rank_gain = match (a.exact_space_rank, b.exact_space_rank) {
        (Some(x), Some(y)) => Some(
            y.checked_sub(x)
                .ok_or("certified consequence reduced space rank")?,
        ),
        _ => None,
    };
    Ok(Counterfactual {
        baseline,
        augmented,
        rank_gain,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(p: u64) -> (Vec<Poly>, Row, Row) {
        let f = vec![
            Poly::from([(vec![2, 0], 1)]),
            Poly::from([(vec![2, 0], 1), (vec![1, 0], 1)]),
        ];
        let x = Row {
            poly: Poly::from([(vec![1, 0], 1)]),
            proof: Proof::from([((0, vec![0, 0]), p - 1), ((1, vec![0, 0]), 1)]),
        };
        let xy = Row {
            poly: Poly::from([(vec![1, 1], 1)]),
            proof: Proof::from([((0, vec![0, 1]), p - 1), ((1, vec![0, 1]), 1)]),
        };
        (f, x, xy)
    }
    #[test]
    fn raw_gain_can_be_zero_gain_after_closure() {
        for p in [2147483647, 9007199254740881] {
            let (f, _, xy) = fixture(p);
            let raw = counterfactual(&f, 2, p, 2, &xy, Mode::Raw, Limits::default()).unwrap();
            let closed = counterfactual(&f, 2, p, 2, &xy, Mode::Closed, Limits::default()).unwrap();
            assert_eq!(
                (
                    raw.baseline.basis.len(),
                    raw.augmented.basis.len(),
                    raw.rank_gain
                ),
                (2, 3, Some(1))
            );
            assert_eq!(
                (
                    closed.baseline.basis.len(),
                    closed.augmented.basis.len(),
                    closed.rank_gain
                ),
                (3, 3, Some(0))
            );
            assert!(closed
                .baseline
                .basis
                .iter()
                .any(|r| r.certificate_max_product_degree > 2));
        }
    }
    #[test]
    fn genuinely_earlier_consequence_retains_original_generator_witnesses() {
        for p in [2147483647, 9007199254740881] {
            let (f, x, _) = fixture(p);
            let c = counterfactual(&f, 2, p, 1, &x, Mode::Closed, Limits::default()).unwrap();
            assert_eq!(c.rank_gain, Some(1));
            assert_eq!(c.augmented.basis.len(), 1);
            assert_eq!(c.augmented.basis[0].certificate_max_product_degree, 2);
            assert!(c.augmented.basis[0]
                .membership
                .iter()
                .all(|t| t.input_index < f.len()));
            let mut bad = x.clone();
            bad.proof.clear();
            assert!(snapshot(&f, 2, p, 1, &[bad], Mode::Closed, Limits::default()).is_err());
        }
    }
    #[test]
    fn an_original_ideal_witness_alone_does_not_admit_a_low_degree_row() {
        let p = 2147483647;
        let (f, x, _) = fixture(p);
        let mut forged = snapshot(
            &f,
            2,
            p,
            1,
            std::slice::from_ref(&x),
            Mode::Closed,
            Limits::default(),
        )
        .unwrap();
        // x really is in the original ideal, but no original generator is degree1.
        forged.basis[0].derivation.source = Origin::Original(1);
        assert!(replay(&f, 2, p, &[], &forged, Limits::default()).is_err());
        let raw = snapshot(&f, 2, p, 2, &[], Mode::Closed, Limits::default()).unwrap();
        let mut wrong_mode = raw;
        wrong_mode.mode = Mode::Raw;
        assert!(replay(&f, 2, p, &[], &wrong_mode, Limits::default()).is_err());
    }
    #[test]
    fn producer_censoring_never_becomes_an_exact_rank_or_gain() {
        let p = 2147483647;
        let (f, x, _) = fixture(p);
        let limits = Limits {
            max_rows: 1,
            ..Limits::default()
        };
        let c = counterfactual(&f, 2, p, 2, &x, Mode::Closed, limits).unwrap();
        assert!(!c.baseline.closure_completed);
        assert_eq!(c.rank_gain, None);
        assert_eq!(
            replay(&f, 2, p, &[], &c.baseline, limits)
                .unwrap()
                .exact_space_rank,
            None
        );
        let mut false_complete = c.baseline.clone();
        false_complete.closure_completed = true;
        false_complete.status = "complete".into();
        assert!(replay(&f, 2, p, &[], &false_complete, limits).is_err());
        let small = Limits {
            max_columns: 1,
            ..Limits::default()
        };
        let s = snapshot(&f, 2, p, 2, &[], Mode::Closed, small).unwrap();
        assert_eq!(s.status, "column_budget");
        assert_eq!(
            replay(&f, 2, p, &[], &s, small).unwrap().exact_space_rank,
            None
        );
    }
    #[test]
    fn degree_derivation_and_membership_tampering_is_rejected() {
        let p = 2147483647;
        let (f, _, _) = fixture(p);
        let s = snapshot(&f, 2, p, 2, &[], Mode::Closed, Limits::default()).unwrap();
        replay(&f, 2, p, &[], &s, Limits::default()).unwrap();
        let mut bad = s.clone();
        bad.basis[0].membership[0].coefficient = 2;
        assert!(replay(&f, 2, p, &[], &bad, Limits::default()).is_err());
        let mut bad = s.clone();
        bad.basis[0].derivation.multiplier = vec![3, 0];
        assert!(replay(&f, 2, p, &[], &bad, Limits::default()).is_err());
        let mut bad = s.clone();
        bad.basis[0].derivation.source = Origin::Basis(0);
        assert!(replay(&f, 2, p, &[], &bad, Limits::default()).is_err());
        let mut bad = s.clone();
        bad.basis[0].certificate_max_product_degree += 1;
        assert!(replay(&f, 2, p, &[], &bad, Limits::default()).is_err());
        let mut bad = s.clone();
        bad.basis.push(s.basis[0].clone());
        bad.rows_processed += 1;
        assert!(replay(&f, 2, p, &[], &bad, Limits::default()).is_err());
    }
    #[test]
    fn empty_constant_and_validation_edges() {
        let p = 2147483647;
        for mode in [Mode::Raw, Mode::Closed] {
            let s = snapshot(&[], 2, p, 3, &[], mode, Limits::default()).unwrap();
            assert_eq!(
                replay(&[], 2, p, &[], &s, Limits::default())
                    .unwrap()
                    .exact_space_rank,
                Some(0)
            );
            let f = vec![Poly::from([(vec![0, 0], 5)])];
            let s = snapshot(&f, 2, p, 2, &[], mode, Limits::default()).unwrap();
            assert_eq!(
                replay(&f, 2, p, &[], &s, Limits::default())
                    .unwrap()
                    .exact_space_rank,
                Some(6)
            );
        }
        assert!(snapshot(&[], 0, p, 2, &[], Mode::Raw, Limits::default()).is_err());
        assert!(snapshot(&[], 1, 15, 2, &[], Mode::Raw, Limits::default()).is_err());
        assert!(snapshot(&[], 1, p, 17, &[], Mode::Raw, Limits::default()).is_err());
        assert!(snapshot(
            &[Poly::from([(vec![1, 0], p)])],
            2,
            p,
            2,
            &[],
            Mode::Raw,
            Limits::default()
        )
        .is_err());
    }
}
