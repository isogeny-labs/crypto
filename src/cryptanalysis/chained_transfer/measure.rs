//! End-to-end cost of solving a transferred ECDLP, measured in counted group
//! operations against a matched Pollard-rho reference on the same order-`r`
//! subgroup.
//!
//! Two destinations are measured:
//! - another elliptic curve reached by an isogeny hop, solved by rho;
//! - a genus-2 Jacobian reached by the bielliptic pull-back, solved by a
//!   self-contained single-large-prime genus-2 index calculus
//!   ([`genus2_index_calculus`]) whose every Cantor operation is counted.
//!
//! The comparison answers the task's primary question quantitatively: is the
//! *complete* discrete-log computation in the destination cheaper than solving
//! it directly? Group operations are the common unit; field-operation ratios
//! (a Cantor step costs several elliptic additions) only widen any gap.

use super::ec::{Curve, Pt};
use super::field::Fp;
use crate::cryptanalysis::genus2_fp::{Divisor, Genus2Curve};
use crate::cryptanalysis::ic_boundary::{rho_reference, CountedGroup, GroupOps, RhoResult};

// ── Counted elliptic group over F_p (points may be in F_{p²}, but the
//    subgroup we solve is F_p-rational) ────────────────────────────────────

/// Elliptic curve wrapped as a [`CountedGroup`] on `Pt`.
pub struct EcGroup(pub Curve);

impl CountedGroup for EcGroup {
    type Elt = Pt;
    fn identity(&self) -> Pt {
        None
    }
    fn is_identity(&self, p: &Pt) -> bool {
        p.is_none()
    }
    fn add(&self, ops: &mut GroupOps, p: Pt, q: Pt) -> Pt {
        ops.adds += 1;
        self.0.add(&p, &q)
    }
    fn double(&self, ops: &mut GroupOps, p: Pt) -> Pt {
        ops.doubles += 1;
        self.0.add(&p, &p)
    }
    fn neg(&self, p: Pt) -> Pt {
        self.0.neg(&p)
    }
    fn key(&self, p: &Pt) -> u64 {
        match p {
            None => 0,
            Some((x, y)) => {
                x.a.wrapping_mul(0x9E37_79B9_7F4A_7C15)
                    ^ y.a.wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
                    ^ x.b.wrapping_mul(0x1656_67B1_9E37_79F9)
                    ^ y.b.wrapping_mul(0x27D4_EB2F_1656_67C5)
            }
        }
    }
}

/// Genus-2 Jacobian wrapped as a [`CountedGroup`] on `Divisor`.
pub struct JacGroup(pub Genus2Curve);

impl CountedGroup for JacGroup {
    type Elt = Divisor;
    fn identity(&self) -> Divisor {
        Divisor::IDENTITY
    }
    fn is_identity(&self, d: &Divisor) -> bool {
        d.is_identity()
    }
    fn add(&self, ops: &mut GroupOps, p: Divisor, q: Divisor) -> Divisor {
        ops.adds += 1;
        self.0.add(&p, &q)
    }
    fn double(&self, ops: &mut GroupOps, p: Divisor) -> Divisor {
        ops.doubles += 1;
        self.0.double(&p)
    }
    fn neg(&self, p: Divisor) -> Divisor {
        self.0.neg(&p)
    }
    fn key(&self, d: &Divisor) -> u64 {
        let u = d.u_coeffs();
        let v = d.v_coeffs();
        let mut k = (d.degree() as u64).wrapping_add(0x9E37_79B9_7F4A_7C15);
        for c in u.iter().chain(v.iter()) {
            k = (k ^ *c).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
            k ^= k >> 29;
        }
        k
    }
}

/// Counted Pollard rho on an elliptic subgroup. The full result is returned so
/// the caller can check `recovered` against the planted log.
pub fn rho_on_ec(c: &Curve, gen: Pt, target: Pt, r: u64, seed: u64) -> RhoResult {
    let g = EcGroup(*c);
    rho_reference(&g, gen, target, r, seed, 200 * r)
}

/// Counted Pollard rho on a genus-2 Jacobian subgroup (full result, as above).
pub fn rho_on_jac(c: &Genus2Curve, gen: Divisor, target: Divisor, r: u64, seed: u64) -> RhoResult {
    let g = JacGroup(c.clone());
    rho_reference(&g, gen, target, r, seed, 200 * r)
}

// ── Genus-2 single-large-prime index calculus ───────────────────────────────

/// Outcome of a genus-2 index-calculus solve.
#[derive(Clone, Debug)]
pub struct IcOutcome {
    pub recovered: Option<u64>,
    /// Cantor group operations charged across relation collection.
    pub group_ops: u64,
    /// Relation trials attempted.
    pub trials: u64,
    /// Smooth relations kept.
    pub relations: u64,
    /// Factor-base size.
    pub fb_size: usize,
    /// Field multiplications charged to the linear-algebra phase (Z/r).
    pub la_field_ops: u64,
}

/// Factorise a reduced class into degree-1 places over `F_p`, returning the
/// signed exponent of each place (±1), or `None` if it does not split into
/// base places.
fn smooth_decompose(
    k: &Fp,
    d: &Divisor,
    index: &dyn Fn(u64) -> Option<usize>,
) -> Option<Vec<(usize, i64)>> {
    if d.is_identity() {
        return Some(vec![]);
    }
    let p = k.p;
    let u = d.u_coeffs();
    let v = d.v_coeffs();
    let deg = d.degree();
    let roots: Vec<u64> = match deg {
        1 => vec![(p - u[0]) % p],
        2 => {
            let (c0, c1) = (u[0], u[1]);
            // x² + c1 x + c0
            let disc = k.sub(k.mul(c1, c1), k.mul(4, c0));
            let s = k.sqrt(disc)?;
            let inv2 = k.inv(2);
            let r1 = k.mul(k.add(k.neg(c1), s), inv2);
            let r2 = k.mul(k.sub(k.neg(c1), s), inv2);
            if r1 == r2 {
                return None; // ramified; skip
            }
            vec![r1, r2]
        }
        _ => return None,
    };
    let mut out = vec![];
    for x in roots {
        let mut y = 0u64;
        for &cf in v.iter().rev() {
            y = k.add(k.mul(y, x), cf);
        }
        let i = index(x)?;
        // canonical y0 = min(y, p−y); class uses +1 if y == y0, else −1.
        let y0 = y.min((p - y) % p);
        out.push((i, if y == y0 { 1 } else { -1 }));
    }
    Some(out)
}

/// Relations collected over the degree-1 factor base.
struct Relations {
    /// `(exponents over the factor base, alpha, beta)` with
    /// `[alpha]gen + [beta]target = Σ e_i (P_i − ∞)`.
    rows: Vec<(Vec<i64>, u64, u64)>,
    nfb: usize,
    ops: u64,
    trials: u64,
}

fn collect_relations(
    c: &Genus2Curve,
    gen: &Divisor,
    target: &Divisor,
    r: u64,
    fb_size: usize,
    max_trials: u64,
    seed: u64,
) -> Relations {
    let p = c.p();
    let k = Fp::new(p).expect("genus2_fp curve has a prime field");
    // Factor base: the first `fb_size` abscissae `x` with an affine point. Only
    // the index of each place matters here; the canonical sign is recomputed in
    // `smooth_decompose`.
    let mut nfb = 0usize;
    let mut xindex = std::collections::HashMap::new();
    for x in 0..p {
        if nfb >= fb_size {
            break;
        }
        if k.sqrt(c.eval_f(x)).is_some() {
            xindex.insert(x, nfb);
            nfb += 1;
        }
    }
    let index = |x: u64| xindex.get(&x).copied();

    let mut ops = 0u64;
    let mut rows: Vec<(Vec<i64>, u64, u64)> = vec![];
    let mut rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };
    let mut trials = 0u64;
    let want = nfb + 8;
    while rows.len() < want && trials < max_trials {
        trials += 1;
        let alpha = next() % r;
        let beta = next() % r;
        // R = [alpha]gen + [beta]target, counting Cantor ops.
        let (ra, oa) = scalar_counted(c, gen, alpha);
        let (rb, ob) = scalar_counted(c, target, beta);
        ops += oa + ob + 1;
        let rr = c.add(&ra, &rb);
        if let Some(decomp) = smooth_decompose(&k, &rr, &index) {
            // Keep only full weight-2 relations over the degree-1 base.
            if rr.degree() == 2 && decomp.len() == 2 {
                let mut row = vec![0i64; nfb];
                for (i, e) in decomp {
                    row[i] += e;
                }
                rows.push((row, alpha, beta));
            }
        }
    }
    Relations {
        rows,
        nfb,
        ops,
        trials,
    }
}

/// The relation rows of the old collector, exposed so that an instrument can
/// count the elimination itself (`instrument::solve_linear_counted`). Same
/// collector, same random stream as [`genus2_index_calculus`].
pub struct Genus2Rows {
    /// `(exponents over the factor base, alpha, beta)`.
    pub rows: Vec<(Vec<i64>, u64, u64)>,
    pub fb_size: usize,
    pub group_ops: u64,
    pub trials: u64,
}

/// Collect the old collector's relations without eliminating them.
pub fn genus2_collect_rows(
    c: &Genus2Curve,
    gen: &Divisor,
    target: &Divisor,
    r: u64,
    fb_size: usize,
    max_trials: u64,
    seed: u64,
) -> Genus2Rows {
    let rel = collect_relations(c, gen, target, r, fb_size, max_trials, seed);
    Genus2Rows {
        rows: rel.rows,
        fb_size: rel.nfb,
        group_ops: rel.ops,
        trials: rel.trials,
    }
}

/// Single-large-prime-free genus-2 index calculus restricted to the cyclic
/// `r`-part. `gen` has order `r`, `target = [x]gen`. Returns whatever the
/// elimination yields (F4: no reference value is consulted, so a wrong log is
/// returned as wrong and judged afterwards by a certificate) and the counted
/// cost. `fb_size` degree-1 places form the factor base.
pub fn genus2_index_calculus(
    c: &Genus2Curve,
    gen: &Divisor,
    target: &Divisor,
    r: u64,
    fb_size: usize,
    max_trials: u64,
    seed: u64,
) -> IcOutcome {
    let rel = collect_relations(c, gen, target, r, fb_size, max_trials, seed);
    let (recovered, la_ops) = solve_linear_free(&rel.rows, rel.nfb, r);
    IcOutcome {
        recovered,
        group_ops: rel.ops,
        trials: rel.trials,
        relations: rel.rows.len() as u64,
        fb_size: rel.nfb,
        la_field_ops: la_ops,
    }
}

/// The pre-F4 path: the same relations, but the elimination accepts a
/// candidate only if it equals a BSGS answer (`target_is_x`, uncharged). A
/// wrong elimination gives `None`, never a wrong log. Kept for older callers
/// and as a cross-check; not independent evidence that elimination works.
pub fn genus2_index_calculus_oracle(
    c: &Genus2Curve,
    gen: &Divisor,
    target: &Divisor,
    r: u64,
    fb_size: usize,
    max_trials: u64,
    seed: u64,
) -> IcOutcome {
    let rel = collect_relations(c, gen, target, r, fb_size, max_trials, seed);
    let (recovered, la_ops) = solve_linear(&rel.rows, rel.nfb, r, target_is_x(c, gen, target, r));
    IcOutcome {
        recovered,
        group_ops: rel.ops,
        trials: rel.trials,
        relations: rel.rows.len() as u64,
        fb_size: rel.nfb,
        la_field_ops: la_ops,
    }
}

/// `[n]d` counting every Cantor add/double.
fn scalar_counted(c: &Genus2Curve, d: &Divisor, mut n: u64) -> (Divisor, u64) {
    let mut acc = Divisor::IDENTITY;
    let mut base = *d;
    let mut ops = 0u64;
    while n > 0 {
        if n & 1 == 1 {
            // The first addition into the identity is a copy, not a group
            // operation (Genus2Curve::add returns early on the identity), so it
            // is not charged; rho's scalar multiplication counts the same way.
            if !acc.is_identity() {
                ops += 1;
            }
            acc = c.add(&acc, &base);
        }
        n >>= 1;
        if n > 0 {
            base = c.double(&base);
            ops += 1;
        }
    }
    (acc, ops)
}

/// Reference value of `x` for a self-check of the linear algebra (computed by
/// BSGS on the Jacobian subgroup; cost not charged to IC).
fn target_is_x(c: &Genus2Curve, gen: &Divisor, target: &Divisor, r: u64) -> Option<u64> {
    // Baby-step giant-step, O(√r), only to validate IC output in tests.
    let m = (r as f64).sqrt().ceil() as u64 + 1;
    let mut table = std::collections::HashMap::new();
    let mut cur = Divisor::IDENTITY;
    for j in 0..m {
        table.entry(cur).or_insert(j);
        cur = c.add(&cur, gen);
    }
    let factor = c.mul(gen, (r - m % r) % r); // [-m]gen
    let mut gamma = *target;
    for i in 0..=m {
        if let Some(&j) = table.get(&gamma) {
            let x = (i * m + j) % r;
            return Some(x);
        }
        gamma = c.add(&gamma, &factor);
    }
    None
}

/// Gaussian elimination over `Z/r` on the factor-base columns; once a row's
/// fb-part is zeroed, `α + β·x ≡ 0`, giving `x = −α/β`. `reference` lets the
/// caller cross-check (tests only). Returns `(x, field_mults_charged)`.
fn solve_linear(
    rows: &[(Vec<i64>, u64, u64)],
    fb_size: usize,
    r: u64,
    reference: Option<u64>,
) -> (Option<u64>, u64) {
    solve_linear_inner(rows, fb_size, r, reference)
}

/// Oracle-free elimination (F4): the first zero-factor-base row with
/// `beta ≠ 0` gives `x = −α/β`, returned as is. `None` only when elimination
/// finds no such row. Nothing about the true log is consulted.
pub fn solve_linear_free(
    rows: &[(Vec<i64>, u64, u64)],
    fb_size: usize,
    r: u64,
) -> (Option<u64>, u64) {
    solve_linear_inner(rows, fb_size, r, None)
}

fn solve_linear_inner(
    rows: &[(Vec<i64>, u64, u64)],
    fb_size: usize,
    r: u64,
    reference: Option<u64>,
) -> (Option<u64>, u64) {
    if rows.is_empty() {
        return (None, 0);
    }
    let rr = r as i128;
    let red = |v: i128| ((v % rr + rr) % rr) as u64;
    // Augmented matrix: columns 0..fb_size are places, then alpha, beta.
    let mut m: Vec<Vec<u64>> = rows
        .iter()
        .map(|(row, a, b)| {
            let mut v: Vec<u64> = row.iter().map(|&e| red(e as i128)).collect();
            v.push(*a);
            v.push(*b);
            v
        })
        .collect();
    let ncols = fb_size + 2;
    let mut ops = 0u64;
    // Modular inverse mod the prime r (Fermat).
    let finv = |a: u64| -> u64 {
        let (mut b, mut e, mut res) = (a % r, r - 2, 1u64);
        while e > 0 {
            if e & 1 == 1 {
                res = ((res as u128 * b as u128) % r as u128) as u64;
            }
            b = ((b as u128 * b as u128) % r as u128) as u64;
            e >>= 1;
        }
        res
    };
    let mut pivot_row = 0usize;
    for col in 0..fb_size {
        let Some(sel) = (pivot_row..m.len()).find(|&i| m[i][col] != 0) else {
            continue;
        };
        m.swap(pivot_row, sel);
        let inv = finv(m[pivot_row][col]);
        for c in 0..ncols {
            m[pivot_row][c] = m[pivot_row][c] * inv % r;
            ops += 1;
        }
        for i in 0..m.len() {
            if i != pivot_row && m[i][col] != 0 {
                let f = m[i][col];
                for c in 0..ncols {
                    m[i][c] = (m[i][c] + r - f * m[pivot_row][c] % r) % r;
                    ops += 1;
                }
            }
        }
        pivot_row += 1;
        if pivot_row >= m.len() {
            break;
        }
    }
    // A row with all fb columns zero and beta ≠ 0 gives the relation.
    for row in &m {
        if row[..fb_size].iter().all(|&e| e == 0) {
            let (a, b) = (row[fb_size], row[fb_size + 1]);
            if b != 0 {
                let x = (r - a % r) % r * finv(b) % r;
                ops += 1;
                if reference.is_none_or(|rx| rx == x) {
                    return (Some(x), ops);
                }
            }
        }
    }
    (None, ops)
}

/// Build a generator of the cyclic `r`-part of `J(C)` and a target `[x]gen`.
/// `order = #J(C)(F_p)`; `r` must divide it with `r² ∤ order`.
pub fn jac_instance(
    c: &Genus2Curve,
    order: u64,
    r: u64,
    x: u64,
    seed: u64,
) -> Option<(Divisor, Divisor)> {
    if !order.is_multiple_of(r) || (order / r).is_multiple_of(r) {
        return None;
    }
    let cof = order / r;
    use rand::{rngs::StdRng, SeedableRng};
    let mut rng = StdRng::seed_from_u64(seed);
    for _ in 0..400 {
        let d = c.mul(&c.random(&mut rng), cof);
        if !d.is_identity() && c.mul(&d, r).is_identity() {
            return Some((d, c.mul(&d, x)));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genus2_ic_recovers_log_and_is_counted() {
        // y² = x⁵ + 3 over F_p, p ≡ 1 mod 5 so #J has usable prime factors.
        for p in [101u64, 251, 1009] {
            let c = Genus2Curve::x5_plus_b(p, 3).unwrap();
            let order = c.jacobian_order().unwrap();
            // pick the largest prime factor r of #J with r² ∤ #J
            let r = largest_cyclic_prime(order);
            if r < 20 {
                continue;
            }
            if !order.is_multiple_of(r) || (order / r).is_multiple_of(r) {
                continue;
            }
            let x = (p * 7 + 13) % r;
            let Some((gen, target)) = jac_instance(&c, order, r, x, p) else {
                continue;
            };
            let out = genus2_index_calculus(&c, &gen, &target, r, p as usize, 2_000_000, p);
            assert_eq!(out.recovered, Some(x), "IC recovers the log (p={p}, r={r})");
            assert!(out.group_ops > 0);
            // rho on the same subgroup, for the comparison.
            let rho = rho_on_jac(&c, gen, target, r, p);
            assert_eq!(rho.recovered, Some(x), "rho recovers the log");
            assert!(rho.group_ops.adds + rho.group_ops.doubles > 0);
        }
    }

    #[test]
    fn oracle_free_elimination_reports_a_wrong_log_as_wrong() {
        // One doctored relation: empty factor-base part, alpha = 1, beta = 1
        // gives x = -1 mod r. The oracle-free path returns it; the oracle path
        // (reference = some other value) returns None. Correctness is judged
        // outside, by a certificate.
        let r = 101u64;
        let rows = vec![(vec![0i64, 0], 1u64, 1u64)];
        let (free, _) = solve_linear_free(&rows, 2, r);
        assert_eq!(free, Some(r - 1));
        let (gated, _) = solve_linear(&rows, 2, r, Some(5));
        assert_eq!(gated, None);
        let (empty, _) = solve_linear_free(&[], 2, r);
        assert_eq!(empty, None);
    }

    #[test]
    fn oracle_free_and_oracle_paths_agree_on_a_real_instance() {
        let c = Genus2Curve::x5_plus_b(251, 3).unwrap();
        let order = c.jacobian_order().unwrap();
        let r = largest_cyclic_prime(order);
        if r < 20 || (order / r).is_multiple_of(r) {
            return;
        }
        let x = 17 % r;
        let (g, t) = jac_instance(&c, order, r, x, 3).unwrap();
        let a = genus2_index_calculus(&c, &g, &t, r, 251, 2_000_000, 3);
        let b = genus2_index_calculus_oracle(&c, &g, &t, r, 251, 2_000_000, 3);
        assert_eq!(a.recovered, Some(x));
        assert_eq!(b.recovered, Some(x));
        assert_eq!(a.group_ops, b.group_ops);
    }

    fn largest_cyclic_prime(mut n: u64) -> u64 {
        let mut best = 1;
        let mut d = 2;
        while d * d <= n {
            if n.is_multiple_of(d) {
                let mut e = 0;
                while n.is_multiple_of(d) {
                    n /= d;
                    e += 1;
                }
                if e == 1 {
                    best = best.max(d);
                }
            }
            d += 1;
        }
        if n > 1 {
            best = best.max(n);
        }
        best
    }
}
