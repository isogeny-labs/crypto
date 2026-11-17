//! Exact, bounded affine Macaulay closure with provenance and top-form Hilbert traces.
//! This measures a specified closure-to-grevlex-basis threshold, not F4/F5 timing.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::time::Instant;

mod monomial;
use monomial::MonomialOps;
pub mod packed_constructor;
pub mod cheap;
mod dag;
pub mod projection;
pub mod quotient;
pub mod replay;
pub mod workload;

// Only the explicit native-owned factory selects these mathematically identical
// kernel variants. Legacy public paths start in mode zero on every thread.
thread_local! {
    static NATIVE_KERNELS: std::cell::Cell<u16> = const { std::cell::Cell::new(0) };
}
fn native_kernel_bits() -> u16 {
    NATIVE_KERNELS.with(|v| v.get())
}
fn with_native_kernel_bits<T>(bits: u16, work: impl FnOnce() -> T) -> T {
    assert!(bits <= 127 || bits == 231 || bits == 487);
    struct Restore(u16);
    impl Drop for Restore {
        fn drop(&mut self) {
            NATIVE_KERNELS.with(|v| v.set(self.0));
        }
    }
    let previous = NATIVE_KERNELS.with(|v| v.replace(bits));
    let _restore = Restore(previous);
    work()
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct Counters {
    pub modular_products: u64,
    pub nontrivial_inversions: u64,
    pub coefficient_updates: u64,
}
#[cfg(feature = "count-field-ops")]
thread_local! {
    static COUNTERS: std::cell::Cell<Counters> = std::cell::Cell::new(Counters::default());
}
pub fn reset_counters() {
    #[cfg(feature = "count-field-ops")]
    COUNTERS.with(|c| c.set(Counters::default()));
}
pub fn counters() -> Counters {
    #[cfg(feature = "count-field-ops")]
    {
        COUNTERS.with(|c| c.get())
    }
    #[cfg(not(feature = "count-field-ops"))]
    {
        Counters::default()
    }
}

pub type Mono = Vec<usize>;
pub type Poly = BTreeMap<Mono, u64>;
pub type Proof = BTreeMap<(usize, Mono), u64>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub poly: Poly,
    pub proof: Proof,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Term {
    pub exponents: Mono,
    pub coefficient: i64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct System {
    pub prime: u64,
    pub variables: Vec<String>,
    pub polynomials: Vec<Vec<Term>>,
}

fn mul(a: u64, b: u64, p: u64) -> u64 {
    #[cfg(feature = "count-field-ops")]
    COUNTERS.with(|cell| {
        let mut c = cell.get();
        c.modular_products += 1;
        cell.set(c);
    });
    const M31: u64 = (1 << 31) - 1;
    if p == M31 && (a | b) <= M31 {
        // x < 2^62; 2^31 == 1 mod M31. The folded sum is < 2*M31.
        // The operand guard preserves the old semantics for arbitrary u64s.
        let x = a * b;
        let folded = (x & M31) + (x >> 31);
        if folded >= M31 {
            folded - M31
        } else {
            folded
        }
    } else {
        ((a as u128 * b as u128) % p as u128) as u64
    }
}
fn pow(mut a: u64, mut b: u64, p: u64) -> u64 {
    let mut result = 1;
    while b > 0 {
        if b & 1 == 1 {
            result = mul(result, a, p);
        }
        a = mul(a, a, p);
        b >>= 1;
    }
    result
}
fn inv(a: u64, p: u64) -> u64 {
    assert!(a > 0 && a < p);
    if a == 1 {
        return 1;
    }
    #[cfg(feature = "count-field-ops")]
    COUNTERS.with(|cell| {
        let mut c = cell.get();
        c.nontrivial_inversions += 1;
        cell.set(c);
    });
    pow(a, p - 2, p)
}
fn minus(a: u64, p: u64) -> u64 {
    if a == 0 {
        0
    } else {
        p - a
    }
}
pub fn prime(p: u64) -> bool {
    if p < 2 {
        return false;
    }
    for q in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if p == q {
            return true;
        }
        if p.is_multiple_of(q) {
            return false;
        }
    }
    let s = (p - 1).trailing_zeros();
    let d = (p - 1) >> s;
    // Deterministic Miller--Rabin bases for the entire u64 range.
    'base: for a in [2u64, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        if a.is_multiple_of(p) {
            continue;
        }
        let mut x = pow(a % p, d, p);
        if x == 1 || x == p - 1 {
            continue;
        }
        for _ in 1..s {
            x = mul(x, x, p);
            if x == p - 1 {
                continue 'base;
            }
        }
        return false;
    }
    true
}
include!("kernels/arithmetic.rs");
/// A separate textbook Buchberger path, bounded; never supplies rows to the tracer.
/// Every reference row carries membership in the original ideal, every input reduces
/// to zero, and all S-polynomials reduce to zero (Buchberger criterion).
pub fn reference(inputs: &[Poly], n: usize, p: u64) -> Result<Vec<Row>, String> {
    let mut basis: Vec<Row> = Vec::new();
    for row in input_rows(inputs, n) {
        let row = monic(normal_form(row, &basis, p), p);
        if !row.poly.is_empty() {
            basis.push(row);
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
            return Err("reference_budget: pairs or basis cap".into());
        }
        let row = monic(
            normal_form(s_polynomial(&basis[i], &basis[j], p), &basis, p),
            p,
        );
        if !row.poly.is_empty() {
            let k = basis.len();
            for i in 0..k {
                pairs.push_back((i, k));
            }
            basis.push(row);
        }
    }
    // Retain a minimal leading-monomial set, then fully interreduce it.
    let minimal: Vec<Row> = basis
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
        let others: Vec<Row> = minimal
            .iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, g)| g.clone())
            .collect();
        reduced.push(monic(normal_form(row.clone(), &others, p), p));
    }
    reduced.sort_by(|a, b| cmp(lead(&b.poly), lead(&a.poly)));
    if !check_reference(&reduced, inputs, n, p) {
        return Err("reference certificate check failed".into());
    }
    Ok(reduced)
}
/// Matched engine ablation: only initial interreduction differs.
pub fn reference_interreduce(inputs: &[Poly], n: usize, p: u64) -> Result<Vec<Row>, String> {
    let mut basis: Vec<Row> = Vec::new();
    for row in input_rows(inputs, n) {
        let row = monic(normal_form(row, &basis, p), p);
        if !row.poly.is_empty() {
            basis.push(row);
        }
    }
    // Same generators and ideal, but eliminate obsolete initial rows before pairs.
    for round in 0..128 {
        let mut changed = false;
        let mut i = 0;
        while i < basis.len() {
            let others: Vec<Row> = basis
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, r)| r.clone())
                .collect();
            let reduced = monic(normal_form(basis[i].clone(), &others, p), p);
            if reduced.poly.is_empty() {
                basis.remove(i);
                changed = true;
                continue;
            }
            if reduced.poly != basis[i].poly {
                basis[i] = reduced;
                changed = true;
            }
            i += 1;
        }
        if !changed {
            break;
        }
        if round == 127 {
            return Err("initial_interreduction_budget".into());
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
            return Err("reference_budget: pairs or basis cap".into());
        }
        let row = monic(
            normal_form(s_polynomial(&basis[i], &basis[j], p), &basis, p),
            p,
        );
        if !row.poly.is_empty() {
            let k = basis.len();
            for i in 0..k {
                pairs.push_back((i, k));
            }
            basis.push(row);
        }
    }
    // Retain a minimal leading-monomial set, then fully interreduce it.
    let minimal: Vec<Row> = basis
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
        let others: Vec<Row> = minimal
            .iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, g)| g.clone())
            .collect();
        reduced.push(monic(normal_form(row.clone(), &others, p), p));
    }
    reduced.sort_by(|a, b| cmp(lead(&b.poly), lead(&a.poly)));
    if !check_reference(&reduced, inputs, n, p) {
        return Err("reference certificate check failed".into());
    }
    Ok(reduced)
}
/// Initial interreduction plus the standard product criterion.
pub fn reference_criteria(inputs: &[Poly], n: usize, p: u64) -> Result<Vec<Row>, String> {
    let mut basis: Vec<Row> = Vec::new();
    for row in input_rows(inputs, n) {
        let row = monic(normal_form(row, &basis, p), p);
        if !row.poly.is_empty() {
            basis.push(row);
        }
    }
    // Same generators and ideal, but eliminate obsolete initial rows before pairs.
    for round in 0..128 {
        let mut changed = false;
        let mut i = 0;
        while i < basis.len() {
            let others: Vec<Row> = basis
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, r)| r.clone())
                .collect();
            let reduced = monic(normal_form(basis[i].clone(), &others, p), p);
            if reduced.poly.is_empty() {
                basis.remove(i);
                changed = true;
                continue;
            }
            if reduced.poly != basis[i].poly {
                basis[i] = reduced;
                changed = true;
            }
            i += 1;
        }
        if !changed {
            break;
        }
        if round == 127 {
            return Err("initial_interreduction_budget".into());
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
            return Err("reference_budget: pairs or basis cap".into());
        }
        // Product criterion. Final certificate checks still reduce every S-pair.
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
            let k = basis.len();
            for i in 0..k {
                pairs.push_back((i, k));
            }
            basis.push(row);
        }
    }
    // Retain a minimal leading-monomial set, then fully interreduce it.
    let minimal: Vec<Row> = basis
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
        let others: Vec<Row> = minimal
            .iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, g)| g.clone())
            .collect();
        reduced.push(monic(normal_form(row.clone(), &others, p), p));
    }
    reduced.sort_by(|a, b| cmp(lead(&b.poly), lead(&a.poly)));
    if !check_reference(&reduced, inputs, n, p) {
        return Err("reference certificate check failed".into());
    }
    Ok(reduced)
}
include!("kernels/check.rs");

pub fn check_reference_legacy(basis: &[Row], inputs: &[Poly], n: usize, p: u64) -> bool {
    if basis.is_empty()
        || basis
            .iter()
            .any(|g| g.poly.is_empty() || !verify(g, inputs, p))
    {
        return false;
    }
    if input_rows(inputs, n)
        .into_iter()
        .any(|r| !normal_form(r, basis, p).poly.is_empty())
    {
        return false;
    }
    for j in 0..basis.len() {
        for i in 0..j {
            if !normal_form(s_polynomial(&basis[i], &basis[j], p), basis, p)
                .poly
                .is_empty()
            {
                return false;
            }
        }
    }
    true
}

#[derive(Default)]
struct Space {
    pivots: BTreeMap<Mono, Row>,
    reductions: usize,
}
impl Space {
    fn reduce(&mut self, mut row: Row, p: u64) -> Row {
        while !row.poly.is_empty() {
            let m = lead(&row.poly);
            let c = row.poly[m];
            if let Some(g) = self.pivots.get(m) {
                subtract_row(&mut row, g, c, p);
                self.reductions += 1;
            } else {
                break;
            }
        }
        row
    }
    fn add(&mut self, row: Row, p: u64) -> Option<Row> {
        let row = monic(self.reduce(row, p), p);
        if row.poly.is_empty() {
            None
        } else {
            self.pivots.insert(lead(&row.poly).clone(), row.clone());
            Some(row)
        }
    }
    fn contains(&mut self, f: &Poly, p: u64) -> bool {
        self.reduce(
            Row {
                poly: f.clone(),
                proof: Proof::new(),
            },
            p,
        )
        .poly
        .is_empty()
    }
}
fn columns(n: usize, d: usize, exact: bool) -> usize {
    let top = if exact { n + d - 1 } else { n + d };
    let k = if exact { n - 1 } else { n };
    let mut result = 1usize;
    for i in 0..k {
        let Some(v) = result.checked_mul(top - i) else {
            return usize::MAX;
        };
        result = v / (i + 1);
    }
    result
}
fn monomials(n: usize, d: usize, exact: bool) -> Vec<Mono> {
    if native_kernel_bits() & 1 != 0 {
        return monomials_ordered(n, d, exact);
    }
    monomials_legacy(n, d, exact)
}
fn monomials_legacy(n: usize, d: usize, exact: bool) -> Vec<Mono> {
    fn recurse(n: usize, left: usize, prefix: &mut Mono, out: &mut Vec<Mono>) {
        if n == 1 {
            let mut m = prefix.clone();
            m.push(left);
            out.push(m);
            return;
        }
        for e in 0..=left {
            prefix.push(e);
            recurse(n - 1, left - e, prefix, out);
            prefix.pop();
        }
    }
    let mut result = Vec::new();
    for k in if exact { d..=d } else { 0..=d } {
        recurse(n, k, &mut Vec::new(), &mut result);
    }
    result.sort_by(|a, b| cmp(b, a));
    result
}
fn monomials_ordered(n: usize, d: usize, exact: bool) -> Vec<Mono> {
    assert!(n > 0);
    fn emit(index: usize, left: usize, m: &mut Mono, out: &mut Vec<Mono>) {
        if index == 0 {
            m[0] = left;
            out.push(m.clone());
            return;
        }
        for e in 0..=left {
            m[index] = e;
            emit(index - 1, left - e, m, out);
        }
    }
    let mut out = Vec::new();
    let mut m = vec![0; n];
    for degree in (if exact { d..=d } else { 0..=d }).rev() {
        emit(n - 1, degree, &mut m, &mut out);
    }
    out
}
fn certificate_degree(row: &Row, inputs: &[Poly]) -> usize {
    row.proof
        .keys()
        .map(|(i, m)| degree(&inputs[*i]) + m.iter().sum::<usize>())
        .max()
        .unwrap_or(0)
}
#[derive(Clone, Debug, Serialize)]
pub struct Event {
    pub degree: usize,
    pub closure_completed: bool,
    pub columns: usize,
    pub new_rows_processed: usize,
    pub new_row_nonzeros: usize,
    pub rank: usize,
    pub new_pivots: usize,
    pub row_reductions: usize,
    pub degree_falls: usize,
    pub closure_rounds: usize,
    pub new_pivot_degree_histogram: BTreeMap<usize, usize>,
    pub max_new_certificate_product_degree: usize,
    pub reference_gb_contained: bool,
}
#[derive(Clone, Debug, Serialize)]
pub struct Run {
    pub status: String,
    pub solving_degree: Option<usize>,
    pub trace: Vec<Event>,
}
/// Close at each degree, using only rank-increasing derived sources.
/// Completion oracle checks the certified reference, not a pinning heuristic.
pub fn trace(
    inputs: &[Poly],
    n: usize,
    p: u64,
    reference: &[Row],
    cap: usize,
    max_columns: usize,
) -> (Run, Vec<(usize, Row)>) {
    let mut space = Space::default();
    let mut sources = input_rows(inputs, n);
    let mut seen: BTreeSet<Poly> = inputs.iter().map(|f| canonical(f, p)).collect();
    let mut processed = BTreeSet::new();
    let mut events = Vec::new();
    let mut archive = Vec::new();
    let mut status = "degree_cap";
    for d in inputs.iter().map(degree).min().unwrap_or(0)..=cap {
        let cols = columns(n, d, false);
        if cols > max_columns {
            status = "column_budget";
            break;
        }
        let rank_before = space.pivots.len();
        let reductions_before = space.reductions;
        let (mut rows, mut nnz, mut falls, mut rounds, mut cert) = (0, 0, 0, 0, 0);
        let mut histogram = BTreeMap::new();
        let mut budget_hit = false;
        'closure: loop {
            let mut pending = Vec::new();
            for (sid, source) in sources.iter().enumerate() {
                if degree(&source.poly) > d {
                    continue;
                }
                for m in monomials(n, d - degree(&source.poly), false) {
                    if processed.insert((sid, m.clone())) {
                        pending.push((sid, m));
                    }
                }
            }
            if pending.is_empty() {
                break;
            }
            rounds += 1;
            for (sid, m) in pending {
                if rows
                    + events
                        .iter()
                        .map(|e: &Event| e.new_rows_processed)
                        .sum::<usize>()
                    >= 200000
                    || sources.len() >= 2000
                {
                    status = "row_or_source_budget";
                    budget_hit = true;
                    break 'closure;
                }
                let row = row_shift(&sources[sid], &m);
                let original_degree = degree(&row.poly);
                rows += 1;
                nnz += row.poly.len();
                if let Some(row) = space.add(row, p) {
                    assert!(verify(&row, inputs, p), "invalid provenance");
                    cert = cert.max(certificate_degree(&row, inputs));
                    let rd = degree(&row.poly);
                    *histogram.entry(rd).or_insert(0) += 1;
                    if rd < original_degree {
                        falls += 1;
                    }
                    if seen.insert(canonical(&row.poly, p)) {
                        sources.push(row.clone());
                        archive.push((d, row));
                    }
                }
            }
        }
        let before_oracle = space.reductions;
        let complete = !budget_hit && reference.iter().all(|r| space.contains(&r.poly, p));
        space.reductions = before_oracle;
        events.push(Event {
            degree: d,
            closure_completed: !budget_hit,
            columns: cols,
            new_rows_processed: rows,
            new_row_nonzeros: nnz,
            rank: space.pivots.len(),
            new_pivots: space.pivots.len() - rank_before,
            row_reductions: space.reductions - reductions_before,
            degree_falls: falls,
            closure_rounds: rounds,
            new_pivot_degree_histogram: histogram,
            max_new_certificate_product_degree: cert,
            reference_gb_contained: complete,
        });
        if complete {
            return (
                Run {
                    status: "complete".into(),
                    solving_degree: Some(d),
                    trace: events,
                },
                archive,
            );
        }
        if budget_hit {
            break;
        }
    }
    (
        Run {
            status: status.into(),
            solving_degree: None,
            trace: events,
        },
        archive,
    )
}
#[derive(Clone, Debug, Serialize)]
pub struct HilbertEvent {
    pub degree: usize,
    pub monomials: usize,
    pub rank: usize,
    pub hilbert_function: usize,
}
#[derive(Clone, Debug, Serialize)]
pub struct Hilbert {
    pub status: String,
    pub top_form_d_reg: Option<usize>,
    pub trace: Vec<HilbertEvent>,
}
pub fn hilbert(inputs: &[Poly], n: usize, p: u64, cap: usize, max_columns: usize) -> Hilbert {
    let tops: Vec<Poly> = inputs
        .iter()
        .map(|f| {
            f.iter()
                .filter(|(m, _)| m.iter().sum::<usize>() == degree(f))
                .map(|(m, c)| (m.clone(), *c))
                .collect()
        })
        .collect();
    let mut events = Vec::new();
    let mut status = "not_vanished_within_cap";
    for d in 0..=cap {
        let cols = columns(n, d, true);
        if cols > max_columns {
            status = "column_budget";
            break;
        }
        let mut space = Space::default();
        for f in &tops {
            if degree(f) <= d {
                for m in monomials(n, d - degree(f), true) {
                    space.add(
                        Row {
                            poly: shift(f, &m),
                            proof: Proof::new(),
                        },
                        p,
                    );
                }
            }
        }
        let h = cols - space.pivots.len();
        events.push(HilbertEvent {
            degree: d,
            monomials: cols,
            rank: space.pivots.len(),
            hilbert_function: h,
        });
        if h == 0 {
            return Hilbert {
                status: "vanished".into(),
                top_form_d_reg: Some(d),
                trace: events,
            };
        }
    }
    Hilbert {
        status: status.into(),
        top_form_d_reg: None,
        trace: events,
    }
}
fn encode(f: &Poly) -> Vec<Term> {
    f.iter()
        .map(|(m, c)| Term {
            exponents: m.clone(),
            coefficient: *c as i64,
        })
        .collect()
}
impl System {
    pub fn decode(&self) -> Result<Vec<Poly>, String> {
        if self.prime > i64::MAX as u64 || !prime(self.prime) {
            return Err("modulus must be prime and fit a signed 63-bit integer".into());
        }
        if self.variables.is_empty()
            || self.variables.len() > 16
            || self.variables.iter().any(|s| s.is_empty())
            || self.variables.iter().collect::<BTreeSet<_>>().len() != self.variables.len()
        {
            return Err("need 1..16 distinct nonempty variable names".into());
        }
        let mut result = Vec::new();
        for terms in &self.polynomials {
            let mut f = Poly::new();
            for t in terms {
                if t.exponents.len() != self.variables.len()
                    || t.exponents.iter().any(|e| *e > 4096)
                {
                    return Err("invalid exponents".into());
                }
                let c = (t.coefficient as i128).rem_euclid(self.prime as i128) as u64;
                axpy(
                    &mut f,
                    &BTreeMap::from([(t.exponents.clone(), c)]),
                    1,
                    self.prime,
                );
            }
            if !f.is_empty() {
                result.push(f);
            }
        }
        if result.is_empty() {
            Err("need nonzero equations".into())
        } else {
            Ok(result)
        }
    }
}
fn evaluate(inputs: &[Poly], n: usize, p: u64, gb: &[Row], cap: usize) -> Value {
    let (run, _) = trace(inputs, n, p, gb, cap, 5000);
    let mut value = serde_json::to_value(&run).unwrap();
    value["top_forms"] = json!(hilbert(inputs, n, p, cap, 5000));
    value
}
fn score(run: &Value) -> (u64, u64, u64, u64) {
    (
        run["solving_degree"].as_u64().unwrap_or(u64::MAX),
        run["top_forms"]["top_form_d_reg"]
            .as_u64()
            .unwrap_or(u64::MAX),
        run["trace"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["new_rows_processed"].as_u64().unwrap())
            .sum(),
        run["trace"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["new_row_nonzeros"].as_u64().unwrap())
            .sum(),
    )
}
/// Full same-ideal ablation; no newly imposed constraints or hidden field equations.
pub fn analyze(system: &System, cap: usize, limit: usize) -> Result<Value, String> {
    if cap > 16 || limit > 32 {
        return Err("development budgets: max degree 16, max candidates 32".into());
    }
    let inputs = system.decode()?;
    let n = system.variables.len();
    let p = system.prime;
    let t = Instant::now();
    let gb = reference(&inputs, n, p)?;
    let oracle_seconds = t.elapsed().as_secs_f64();
    let (run, archive) = trace(&inputs, n, p, &gb, cap, 5000);
    let mut baseline = json!(run);
    baseline["top_forms"] = json!(hilbert(&inputs, n, p, cap, 5000));
    let mut seen: BTreeSet<Poly> = inputs.iter().map(|f| canonical(f, p)).collect();
    let mut pool: Vec<(usize, Row)> = archive
        .into_iter()
        .filter(|(_, r)| seen.insert(canonical(&r.poly, p)))
        .collect();
    pool.sort_by_key(|(d, r)| (degree(&r.poly), r.poly.len(), *d, canonical(&r.poly, p)));
    pool.truncate(limit);
    let mut candidates = Vec::new();
    for (i, (discovered, row)) in pool.iter().enumerate() {
        assert!(verify(row, &inputs, p));
        let mut augmented = inputs.clone();
        augmented.push(row.poly.clone());
        let run = evaluate(&augmented, n, p, &gb, cap);
        let proof: Vec<Value> = row
            .proof
            .iter()
            .map(|((j, m), c)| json!({"input":j,"multiplier_exponents":m,"coefficient":c}))
            .collect();
        candidates.push(json!({"id":format!("c{i}"),"polynomial":encode(&row.poly),"degree":degree(&row.poly),"terms":row.poly.len(),
            "discovered_at_degree":discovered,"certificate_max_product_degree":certificate_degree(row,&inputs),"membership_certificate":proof,"single_addition":run}));
    }
    let mut all_inputs = inputs.clone();
    all_inputs.extend(pool.iter().map(|(_, r)| r.poly.clone()));
    let all_run = evaluate(&all_inputs, n, p, &gb, cap);
    let mut loo = Vec::new();
    for i in 0..pool.len() {
        let mut augmented = inputs.clone();
        augmented.extend(
            pool.iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, (_, r))| r.poly.clone()),
        );
        loo.push(json!({"removed":format!("c{i}"),"solve":evaluate(&augmented,n,p,&gb,cap)}));
    }
    let mut selected = Vec::new();
    let mut steps = Vec::new();
    let mut current = baseline.clone();
    loop {
        let mut choices = Vec::new();
        for (i, (_, r)) in pool.iter().enumerate() {
            if selected.contains(&i) {
                continue;
            }
            let mut augmented = inputs.clone();
            augmented.extend(selected.iter().map(|j: &usize| pool[*j].1.poly.clone()));
            augmented.push(r.poly.clone());
            let run = evaluate(&augmented, n, p, &gb, cap);
            choices.push((score(&run), i, run));
        }
        let Some((best, i, run)) = choices.into_iter().min_by_key(|(s, i, _)| (*s, *i)) else {
            break;
        };
        if best >= score(&current) {
            break;
        }
        selected.push(i);
        steps.push(json!({"added":format!("c{i}"),"solve":run}));
        current = run;
    }
    let peak = baseline["trace"]
        .as_array()
        .unwrap()
        .iter()
        .chain(current["trace"].as_array().unwrap())
        .filter_map(|t| t["degree"].as_u64())
        .max();
    Ok(
        json!({"schema":"relation-degree-trace/v1","prime":p,"variables":system.variables,"input":system.polynomials,
        "definition":"Bounded affine Macaulay closure threshold to a certified reduced grevlex GB; not a solver-independent runtime claim.",
        "reference_groebner_basis":gb.iter().map(|g| encode(&g.poly)).collect::<Vec<_>>(),"reference_gb_max_degree":gb.iter().map(|g| degree(&g.poly)).max(),
        "reference_oracle_seconds_diagnostic":oracle_seconds,"baseline":baseline,
        "candidate_discovery":{"method":"baseline closure","max_degree_paid":run.trace.last().map(|t| t.degree),"rows_paid":run.trace.iter().map(|t| t.new_rows_processed).sum::<usize>(),"free_online_preprocessing":false},
        "candidates":candidates,"all_candidates":all_run,"leave_one_out":loo,"greedy_selected":selected.iter().map(|i| format!("c{i}")).collect::<Vec<_>>(),
        "greedy_trace":steps,"greedy_final":current,"end_to_end_complete":baseline["status"] == "complete" && current["status"] == "complete","end_to_end_peak_matrix_degree":peak,
        "claim_status":"producer output; no signed independent PCC verdict"}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordered_monomials_match_complete_legacy_vectors() {
        for n in 1..=16 {
            for d in 0..=if n <= 6 { 6 } else { 3 } {
                for exact in [false, true] {
                    assert_eq!(
                        monomials_ordered(n, d, exact),
                        monomials_legacy(n, d, exact),
                        "n={n}, d={d}, exact={exact}"
                    );
                }
            }
        }
    }
    #[test]
    fn cached_division_preserves_remainder_provenance_and_field_counts() {
        for p in [2147483647, 9007199254740881] {
            for n in [1, 3, 8, 16] {
                let z = vec![0; n];
                let mut a = z.clone();
                a[0] = 2;
                let mut b = z.clone();
                b[n - 1] = 1;
                let basis = input_rows(
                    &[
                        BTreeMap::from([(a, 3), (b, p - 1), (z.clone(), 7)]),
                        BTreeMap::from([(z.clone(), 5)]),
                    ],
                    n,
                );
                let mut shift = z.clone();
                shift[0] = 1;
                let mut row = row_shift(&basis[0], &shift);
                subtract_row(&mut row, &basis[1], 11, p);
                reset_counters();
                let old = normal_form_legacy(row.clone(), &basis, p);
                let old_counts = serde_json::to_value(counters()).unwrap();
                reset_counters();
                let cached = normal_form_cached(row, &basis, p);
                let cached_counts = serde_json::to_value(counters()).unwrap();
                assert_eq!(old, cached);
                assert_eq!(old_counts, cached_counts);
                let empty = Row {
                    poly: Poly::new(),
                    proof: Proof::new(),
                };
                assert_eq!(
                    normal_form_cached(empty.clone(), std::slice::from_ref(&empty), p),
                    empty
                );
                let with_empty = vec![basis[0].clone(), empty.clone()];
                assert_eq!(
                    normal_form_cached(basis[0].clone(), &with_empty, p),
                    normal_form_legacy(basis[0].clone(), &with_empty, p)
                );
                assert!(std::panic::catch_unwind(|| normal_form_cached(
                    basis[0].clone(),
                    std::slice::from_ref(&empty),
                    p
                ))
                .is_err());
                assert!(std::panic::catch_unwind(|| normal_form_legacy(
                    basis[0].clone(),
                    std::slice::from_ref(&empty),
                    p
                ))
                .is_err());
            }
        }
    }
    #[test]
    fn native_kernel_scope_restores_after_nested_error_and_panic() {
        assert_eq!(native_kernel_bits(), 0);
        let error: Result<(), &str> = with_native_kernel_bits(1, || {
            assert_eq!(native_kernel_bits(), 1);
            with_native_kernel_bits(3, || assert_eq!(native_kernel_bits(), 3));
            assert_eq!(native_kernel_bits(), 1);
            assert_eq!(std::thread::spawn(native_kernel_bits).join().unwrap(), 0);
            Err("expected")
        });
        assert!(error.is_err());
        assert_eq!(native_kernel_bits(), 0);
        assert!(
            std::panic::catch_unwind(|| with_native_kernel_bits(2, || panic!("scope probe")))
                .is_err()
        );
        assert_eq!(native_kernel_bits(), 0);
    }

    #[test]
    fn guarded_m31_products_match_wide_remainder_at_boundaries_and_fallbacks() {
        let m = (1u64 << 31) - 1;
        let values = [0, 1, 2, m / 2, m - 2, m - 1, m, m + 1, 1 << 32, u64::MAX];
        for p in [3, 101, m, 9007199254740881, 18446744073709551557] {
            for a in values {
                for b in values {
                    assert_eq!(
                        mul(a, b, p),
                        ((a as u128 * b as u128) % p as u128) as u64,
                        "a={a}, b={b}, p={p}"
                    );
                }
            }
        }
    }

    #[test]
    fn guarded_m31_products_match_wide_remainder_on_fixed_differential_corpus() {
        let p = (1u64 << 31) - 1;
        let mut state = 20261116u64;
        for _ in 0..65536 {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let a = state & p;
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let b = (state >> 17) & p;
            assert_eq!(mul(a, b, p), ((a as u128 * b as u128) % p as u128) as u64);
        }
    }

    #[cfg(feature = "count-field-ops")]
    #[test]
    fn counters_separate_products_updates_and_inversions() {
        reset_counters();
        assert_eq!(mul(7, 9, 101), 63);
        assert_eq!(inv(1, 101), 1);
        let mut target = BTreeMap::new();
        axpy(&mut target, &BTreeMap::from([(vec![1], 8)]), 2, 101);
        let c = counters();
        assert_eq!(c.modular_products, 2);
        assert_eq!(c.coefficient_updates, 1);
        assert_eq!(c.nontrivial_inversions, 0);
    }
    fn simple() -> System {
        System {
            prime: 101,
            variables: vec!["x".into(), "y".into()],
            polynomials: vec![
                vec![
                    Term {
                        exponents: vec![2, 0],
                        coefficient: 1,
                    },
                    Term {
                        exponents: vec![0, 1],
                        coefficient: -1,
                    },
                ],
                vec![
                    Term {
                        exponents: vec![1, 1],
                        coefficient: 1,
                    },
                    Term {
                        exponents: vec![0, 0],
                        coefficient: -1,
                    },
                ],
            ],
        }
    }
    #[test]
    fn interreduced_engine_matches_raw_and_projection_bases() {
        for p in [2147483647, 9007199254740881] {
            let fixture = workload::generate(p, 3, 7).unwrap();
            let f = fixture.system.decode().unwrap();
            let baseline = reference(&f, 3, p).unwrap();
            let raw = reference_interreduce(&f, 3, p).unwrap();
            let augmented = projection::augment(&f, 3, p).0;
            let fast = reference_interreduce(&augmented, 3, p).unwrap();
            let key = |b: Vec<Row>| b.into_iter().map(|r| r.poly).collect::<BTreeSet<_>>();
            let expected = key(baseline);
            assert_eq!(expected, key(raw));
            assert_eq!(expected, key(fast));
        }
    }
    #[test]
    fn fast_verifier_and_product_criterion_keep_all_contracts() {
        let system = simple();
        let f = system.decode().unwrap();
        let valid = reference(&f, 2, 101).unwrap();
        assert!(check_reference(&valid, &f, 2, 101));
        assert!(check_reference_legacy(&valid, &f, 2, 101));
        let mut forged = valid.clone();
        forged[0].proof.clear();
        assert!(!check_reference(&forged, &f, 2, 101));
        assert!(!check_reference_legacy(&forged, &f, 2, 101));
        let missing = &valid[..1];
        assert!(!check_reference(missing, &f, 2, 101));
        assert!(!check_reference_legacy(missing, &f, 2, 101));
        for p in [2147483647, 9007199254740881] {
            let fixture = workload::generate(p, 3, 7).unwrap();
            let raw = fixture.system.decode().unwrap();
            let expected = reference(&raw, 3, p).unwrap();
            let augmented = projection::augment(&raw, 3, p).0;
            let actual = reference_criteria(&augmented, 3, p).unwrap();
            assert_eq!(
                expected
                    .iter()
                    .map(|r| r.poly.clone())
                    .collect::<BTreeSet<_>>(),
                actual.iter().map(|r| r.poly.clone()).collect()
            );
            assert!(check_reference_legacy(&actual, &augmented, 3, p));
        }
    }
    #[test]
    fn certified_consequence_changes_threshold() {
        let system = simple();
        let f = system.decode().unwrap();
        let gb = reference(&f, 2, 101).unwrap();
        assert_eq!(trace(&f, 2, 101, &gb, 5, 5000).0.solving_degree, Some(3));
        let g = BTreeMap::from([(vec![0, 2], 1), (vec![1, 0], 100)]);
        assert!(verify(
            &Row {
                poly: g.clone(),
                proof: BTreeMap::from([((1, vec![1, 0]), 1), ((0, vec![0, 1]), 100)])
            },
            &f,
            101
        ));
        let mut augmented = f;
        augmented.push(g);
        assert_eq!(
            trace(&augmented, 2, 101, &gb, 5, 5000).0.solving_degree,
            Some(2)
        );
    }
    #[test]
    fn hilbert_control_and_nonartinian() {
        let f = vec![
            BTreeMap::from([(vec![2, 0], 1)]),
            BTreeMap::from([(vec![0, 2], 1)]),
        ];
        let h = hilbert(&f, 2, 101, 5, 5000);
        assert_eq!(
            h.trace
                .iter()
                .map(|t| t.hilbert_function)
                .collect::<Vec<_>>(),
            vec![1, 2, 1, 0]
        );
        assert_eq!(h.top_form_d_reg, Some(3));
        assert_eq!(
            hilbert(&simple().decode().unwrap(), 2, 101, 5, 5000).top_form_d_reg,
            None
        );
    }
    #[test]
    fn caps_do_not_become_degrees() {
        let f = simple().decode().unwrap();
        let gb = reference(&f, 2, 101).unwrap();
        let r = trace(&f, 2, 101, &gb, 2, 5000).0;
        assert_eq!(r.status, "degree_cap");
        assert_eq!(r.solving_degree, None);
        assert_eq!(trace(&f, 2, 101, &gb, 3, 2).0.status, "column_budget");
    }

    #[test]
    fn partial_source_budget_retains_incomplete_degree() {
        let f = BTreeMap::from([(vec![1], 1)]);
        let inputs = vec![f.clone(); 2000];
        let gb = reference(&[f], 1, 101).unwrap();
        let run = trace(&inputs, 1, 101, &gb, 3, 5000).0;
        assert_eq!(run.status, "row_or_source_budget");
        assert_eq!(run.solving_degree, None);
        assert_eq!(run.trace.len(), 1);
        assert!(!run.trace[0].closure_completed);
        assert!(!run.trace[0].reference_gb_contained);
    }
    #[test]
    fn prime_and_exact_53_bit_arithmetic() {
        for p in [2147483647, 9007199254740881] {
            assert!(prime(p));
            assert_eq!(mul(p - 1, p - 1, p), 1);
            assert_eq!(mul(1234567, inv(1234567, p), p), 1);
        }
        for n in [0, 1, 4, 341, 561, 2u64.pow(53), 341550071728321] {
            assert!(!prime(n));
        }
    }
    #[test]
    fn reject_bad_modulus_and_input() {
        let mut s = simple();
        s.prime = 2u64.pow(53);
        assert!(s.decode().is_err());
        s.prime = 101;
        s.variables[1] = "x".into();
        assert!(s.decode().is_err());
        s.variables[1] = "y".into();
        s.polynomials[0][0].exponents = vec![1];
        assert!(s.decode().is_err());
    }
    #[test]
    fn reference_rejects_missing_or_forged_basis() {
        let f = simple().decode().unwrap();
        let mut gb = reference(&f, 2, 101).unwrap();
        assert!(check_reference(&gb, &f, 2, 101));
        gb[0].proof.clear();
        assert!(!check_reference(&gb, &f, 2, 101));
        assert!(!check_reference(&[], &f, 2, 101));
    }
    #[test]
    fn three_variable_linear_system() {
        let f: Vec<Poly> = (0..3)
            .map(|i| {
                let mut m = vec![0; 3];
                m[i] = 1;
                BTreeMap::from([(m, 1)])
            })
            .collect();
        let gb = reference(&f, 3, 101).unwrap();
        assert_eq!(trace(&f, 3, 101, &gb, 3, 5000).0.solving_degree, Some(1));
    }
    #[test]
    fn fixed_ec_systems_match_historical_regressions_and_certificates() {
        let historical: Value =
            serde_json::from_str(include_str!("../historical/python-trace.json")).unwrap();
        for (i, input) in [
            include_str!("../fixtures/system-2147483647.json"),
            include_str!("../fixtures/system-9007199254740881.json"),
        ]
        .iter()
        .enumerate()
        {
            let system: System = serde_json::from_str(input).unwrap();
            let f = system.decode().unwrap();
            let gb = reference(&f, 2, system.prime).unwrap();
            let old = &historical["reports"][i];
            // Independent term expansion of f3 at the recorded constants,
            // and evaluation at the recorded x-coordinate witness.
            for (j, equation) in f.iter().enumerate() {
                let p = system.prime;
                let z = old["system"]["f3_constants"][j].as_u64().unwrap();
                let z2 = mul(z, z, p);
                let expected = BTreeMap::from([
                    (vec![2, 2], 1),
                    (vec![2, 1], minus(mul(2, z, p), p)),
                    (vec![1, 2], minus(mul(2, z, p), p)),
                    (vec![2, 0], z2),
                    (vec![0, 2], z2),
                    (vec![1, 1], minus((mul(2, z2, p) + 4) % p, p)),
                    (vec![1, 0], minus((mul(4, z, p) + 12) % p, p)),
                    (vec![0, 1], minus((mul(4, z, p) + 12) % p, p)),
                    (vec![0, 0], (4 + p - mul(12, z, p)) % p),
                ]);
                assert_eq!(*equation, expected);
                let witness = &old["system"]["witness"];
                let value = equation.iter().fold(0u64, |total, (m, c)| {
                    let term = mul(
                        *c,
                        mul(
                            pow(witness[0].as_u64().unwrap(), m[0] as u64, p),
                            pow(witness[1].as_u64().unwrap(), m[1] as u64, p),
                            p,
                        ),
                        p,
                    );
                    (total + term) % p
                });
                assert_eq!(value, 0);
            }
            let old_gb: System = System {
                prime: system.prime,
                variables: system.variables.clone(),
                polynomials: serde_json::from_value(old["reference_groebner_basis"].clone())
                    .unwrap(),
            };
            let actual: BTreeSet<_> = gb
                .iter()
                .map(|r| canonical(&r.poly, system.prime))
                .collect();
            let expected: BTreeSet<_> = old_gb
                .decode()
                .unwrap()
                .iter()
                .map(|f| canonical(f, system.prime))
                .collect();
            assert_eq!(actual, expected);
            let report = analyze(&system, 8, 8).unwrap();
            assert_eq!(report["baseline"]["solving_degree"], 5);
            assert_eq!(report["greedy_final"]["solving_degree"], 4);
            assert_eq!(report["end_to_end_peak_matrix_degree"], 5);
            assert_eq!(report["greedy_final"]["top_forms"]["top_form_d_reg"], 4);
            assert_eq!(report["greedy_selected"], json!(["c1", "c0"]));
            let removed = report["leave_one_out"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["removed"] == "c1")
                .unwrap();
            assert_eq!(removed["solve"]["solving_degree"], 5);
            for candidate in report["candidates"].as_array().unwrap() {
                let terms: Vec<Term> =
                    serde_json::from_value(candidate["polynomial"].clone()).unwrap();
                let poly: Poly = terms
                    .into_iter()
                    .map(|t| (t.exponents, t.coefficient as u64))
                    .collect();
                let proof: Proof = candidate["membership_certificate"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|t| {
                        (
                            (
                                t["input"].as_u64().unwrap() as usize,
                                serde_json::from_value(t["multiplier_exponents"].clone()).unwrap(),
                            ),
                            t["coefficient"].as_u64().unwrap(),
                        )
                    })
                    .collect();
                assert!(verify(&Row { poly, proof }, &f, system.prime));
            }
        }
    }
    #[test]
    fn cached_division_matches_on_genuinely_non_groebner_nonmonic_lists() {
        for p in [2147483647,9007199254740881] {for n in [3,8,16] {
            let z=vec![0;n];let mut x2=z.clone();x2[0]=2;let mut y=z.clone();y[1]=1;let mut xy=z.clone();xy[0]=1;xy[1]=1;
            let basis=input_rows(&[BTreeMap::from([(x2,3),(y,3)]),BTreeMap::from([(xy,5),(z,5)])],n);
            let target=s_polynomial(&basis[0],&basis[1],p);
            reset_counters();let old=normal_form_legacy(target.clone(),&basis,p);let before=serde_json::to_value(counters()).unwrap();
            assert!(!old.poly.is_empty());
            reset_counters();let cached=normal_form_cached(target,&basis,p);let after=serde_json::to_value(counters()).unwrap();
            assert_eq!(old,cached);assert_eq!(before,after);
        }}
    }

    #[test]
    fn bounded_sum_matches_full_width_expression() {
        let boundary=[0,1,2,2147483645,2147483646,2147483647,2147483648,9007199254740880,u64::MAX];
        for p in [2,3,2147483647,9007199254740881,u64::MAX] {
            for a in boundary {for b in boundary {assert_eq!(sum_mod(a,b,p),((a as u128+b as u128)%p as u128) as u64);}}
        }
        let mut state=0x35ac91f217u64;
        for i in 0..65536 {
            state=state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);let a=if i%2==0 {state&2147483647}else{state};
            state=state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);let b=if i%2==0 {state&2147483647}else{state};
            for p in [2147483647,9007199254740881] {assert_eq!(sum_mod(a,b,p),((a as u128+b as u128)%p as u128) as u64);}
        }
        assert!(std::panic::catch_unwind(||sum_mod(1,1,0)).is_err());
    }
    #[test]
    fn native_updates_preserve_maps_proofs_and_field_counters() {
        let values=[0,1,2,2147483646,2147483647,9007199254740880,u64::MAX];
        for p in [2,3,2147483647,9007199254740881,u64::MAX] {
            for x in values {for y in values {for scale in values {
                let a=BTreeMap::from([(vec![0,0,0],x),(vec![1,0,0],y),(vec![0,1,0],0)]);
                let b=BTreeMap::from([(vec![0,0,0],y),(vec![1,0,0],x),(vec![0,0,1],x)]);
                reset_counters();let mut expected=a.clone();axpy_legacy(&mut expected,&b,scale,p);let counts=serde_json::to_value(counters()).unwrap();
                for bits in [7,15] {
                    reset_counters();let mut actual=a.clone();with_native_kernel_bits(bits,||axpy(&mut actual,&b,scale,p));assert_eq!(actual,expected);assert_eq!(serde_json::to_value(counters()).unwrap(),counts);
                    let mut proof:Proof=a.iter().map(|(m,c)|((2,m.clone()),*c)).collect();let proof_b:Proof=b.iter().map(|(m,c)|((2,m.clone()),*c)).collect();with_native_kernel_bits(bits,||axpy(&mut proof,&proof_b,scale,p));assert_eq!(proof,expected.iter().map(|(m,c)|((2,m.clone()),*c)).collect());
                    let mut empty=BTreeMap::new();with_native_kernel_bits(bits,||axpy(&mut empty,&b,scale,p));let mut old_empty=BTreeMap::new();axpy_legacy(&mut old_empty,&b,scale,p);assert_eq!(empty,old_empty);
                    let mut unchanged=a.clone();with_native_kernel_bits(bits,||axpy(&mut unchanged,&BTreeMap::new(),scale,p));assert_eq!(unchanged,a);
                }
            }}}
        }
    }

    #[test]
    fn cached_degree_lead_preserves_sparse_maps_ties_empty_and_singleton() {
        for n in 1..=16 {
            for degree in 0..=3 {
                for exact in [false,true] {
                    let keys=monomials_legacy(n,degree,exact);
                    for stride in 1..=5 {for offset in 0..stride {
                        let poly:Poly=keys.iter().enumerate().filter(|(i,_)|i%stride==offset).map(|(i,m)|(m.clone(),i as u64%31)).collect();
                        if !poly.is_empty() {
                            assert_eq!(lead_legacy(&poly),lead_with_cached_degrees(&poly));
                            assert_eq!(lead_legacy(&poly),with_native_kernel_bits(23,||lead(&poly)));
                        }
                    }}
                }
            }
        }
        let singleton=Poly::from([(vec![usize::MAX,1],1)]);
        assert_eq!(lead_legacy(&singleton),lead_with_cached_degrees(&singleton));
        assert!(std::panic::catch_unwind(|| {let _=lead_legacy(&Poly::new());}).is_err());
        assert!(std::panic::catch_unwind(|| {let _=lead_with_cached_degrees(&Poly::new());}).is_err());
        assert_eq!(native_kernel_bits(),0);
    }

    fn compare_streamed_normal_form(target:Row,basis:&[Row],p:u64){
        reset_counters();let old=with_native_kernel_bits(7,||normal_form_cached(target.clone(),basis,p));let before=serde_json::to_value(counters()).unwrap();
        reset_counters();let new=with_native_kernel_bits(39,||normal_form_cached(target,basis,p));let after=serde_json::to_value(counters()).unwrap();
        assert_eq!(new,old);assert_eq!(after,before);assert_eq!(native_kernel_bits(),0);
    }
    #[test]
    fn streamed_reducers_keep_nonmonic_non_groebner_remainders_proofs_and_counts(){
        for p in [2147483647,9007199254740881] {for n in [3,8,16] {
            let z=vec![0;n];let mut x2=z.clone();x2[0]=2;let mut y=z.clone();y[1]=1;let mut xy=z.clone();xy[0]=1;xy[1]=1;let mut x=z.clone();x[0]=1;
            let basis=input_rows(&[BTreeMap::from([(x2,3),(y,3)]),BTreeMap::from([(xy,5),(z.clone(),5)])],n);
            let sp=s_polynomial(&basis[0],&basis[1],p);assert!(!with_native_kernel_bits(7,||normal_form_cached(sp.clone(),&basis,p)).poly.is_empty());
            for target in [basis[0].clone(),row_shift(&basis[1],&x),sp,Row{poly:Poly::new(),proof:Proof::new()}] {compare_streamed_normal_form(target,&basis,p);}
        }}
    }
    #[test]
    fn streamed_reducers_fall_back_before_invalid_shift_collisions(){
        let basis=input_rows(&[Poly::from([(vec![1],2),(vec![1,0],1)])],2);
        assert!(!can_stream_shift(&basis[0],&vec![0,0]));
        let target=Row{poly:Poly::from([(vec![1,0],1)]),proof:Proof::new()};
        compare_streamed_normal_form(target,&basis,2147483647);
    }
    #[test]
    fn streamed_overflow_keeps_shift_failure_before_inverse_and_restores_scope(){
        let g=Row{poly:Poly::from([(vec![1],3)]),proof:Proof::from([((0,vec![usize::MAX]),1)])};
        let target=Row{poly:Poly::from([(vec![2],1)]),proof:Proof::new()};
        assert!(!can_stream_shift(&g,&vec![1]));
        let mut observed=Vec::new();
        for bits in [7,39] {
            reset_counters();assert!(std::panic::catch_unwind(||{let _=with_native_kernel_bits(bits,||normal_form_cached(target.clone(),std::slice::from_ref(&g),2147483647));}).is_err());
            observed.push(serde_json::to_value(counters()).unwrap());assert_eq!(native_kernel_bits(),0);
        }
        assert_eq!(observed[0],observed[1]);assert_eq!(observed[1]["nontrivial_inversions"],0);
    }

}
