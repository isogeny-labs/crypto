//! Joux–Vitse §3.3 adapted two-large-prime sieve over Fp abscissae.
//! Small roots leave a degree <= 2 cofactor which must split over Fp.
//! This component returns certified small-base rows; it does not perform
//! individual-log descent or claim a full DLP/rho cost.
use super::{
    fdeg, line_poly, relation_from_roots, sieve_line_threshold, sieve_setup, verify_rel, BaseEl,
    Counters, Ctx, FpRing, Line, SharedB, SieveCost, SieveRel, SieveSetup,
};
use crate::cryptanalysis::gaudry_cubic::{am, mm, sm, SparseRel};
use crate::cryptanalysis::jv_bitops;
use crate::cryptanalysis::jv_cover::{peval, Div, Fld, E2};
use crate::cryptanalysis::large_prime_filter::{
    filter_large_primes, replay_complete_relation, CompleteRelation, LargePrimeFilterOptions,
    LargePrimeFilterReport, PartialRelation,
};
use rand::rngs::StdRng;
use std::collections::HashMap;

#[path = "jv_sieve_two_large_prime_dlp.rs"]
mod dlp;
pub use dlp::{
    run_two_large_prime_dlp_on, run_two_large_prime_dlp_on_with_quotient_reuse,
    run_two_large_prime_dlp_on_with_structural_gate, ProtectedTargetProjection,
    TwoLargePrimeDlpOptions, VerifiedDescentRelation,
};
#[path = "jv_counted_linear.rs"]
mod counted_linear;
pub use counted_linear::CountedRelationSystem;

#[path = "jv_homogeneous_quotient.rs"]
mod homogeneous_quotient;

#[path = "jv_extension_remainder_dlp.rs"]
mod extension_remainder_dlp;
#[path = "jv_principal_relation.rs"]
mod principal_relation;
pub use extension_remainder_dlp::{
    run_extension_remainder_dlp_on, run_extension_remainder_dlp_on_with_solver,
    ExtensionRemainderDlpOptions,
};

/// Distinct roots of a nonzero polynomial of degree <= 2, or refusal.
/// Repeated roots are refused, rather than silently treated as simple roots.
fn low_degree_roots(ring: &FpRing, f: &[u64], sqrt: &[u64]) -> Option<Vec<u64>> {
    let p = ring.p;
    match fdeg(f) {
        0 => Some(Vec::new()),
        1 => {
            ring.count(1);
            Some(vec![mm(sm(0, f[0], p), ring.inv(f[1]), p)])
        }
        2 => {
            let d = sm(mm(f[1], f[1], p), mm(4 % p, mm(f[0], f[2], p), p), p);
            ring.count(3);
            let r = *sqrt.get(d as usize)?;
            if r == u64::MAX || r == 0 {
                return None;
            }
            let inv = ring.inv(am(f[2], f[2], p));
            let neg_b = sm(0, f[1], p);
            ring.count(2);
            let mut roots = vec![mm(am(neg_b, r, p), inv, p), mm(sm(neg_b, r, p), inv, p)];
            roots.sort_unstable();
            Some(roots)
        }
        _ => None,
    }
}

/// A Jacobian-verified original relation and its actual small/large split.
/// Fields are private so a caller cannot relabel arbitrary rows as verified.
#[derive(Clone, Debug)]
pub struct TwoLargePrimeRelation {
    full: SieveRel,
    partial: PartialRelation,
    modulus: u64,
}
impl TwoLargePrimeRelation {
    pub fn full(&self) -> &SieveRel {
        &self.full
    }
    pub fn partial(&self) -> &PartialRelation {
        &self.partial
    }
}

/// Reusable sieve configuration. The prefix is a deterministic reduced base;
/// every omitted Fp point is a large-prime vertex, identified by its abscissa.
pub struct TwoLargePrimeSieve {
    setup: SieveSetup,
    base: Vec<BaseEl>,
    by_x: HashMap<u64, usize>,
    small: usize,
}

pub struct FilteredTwoLargePrimeRelations {
    pub rows: Vec<SparseRel>,
    pub certificates: Vec<CompleteRelation>,
    pub report: LargePrimeFilterReport,
    pub replayed_rows: usize,
}

impl TwoLargePrimeSieve {
    pub fn new(ctx: &Ctx, base: &[BaseEl], small: usize, m: usize) -> Result<Self, &'static str> {
        if !(8..=16).contains(&m)
            || small == 0
            || small > base.len()
            || base.len() > u32::MAX as usize
            || ctx.f.f.p > 1_048_576
            || base.iter().any(|b| b.x >= ctx.f.f.p)
            || base.windows(2).any(|b| b[0].x >= b[1].x)
        {
            return Err("invalid reduced factor base or sieve bounds");
        }
        let fq = &ctx.f.f;
        if base.iter().any(|b| {
            b.y == E2::ZERO
                || fq.mul(&b.y, &b.y) != peval(fq, &ctx.cov.hx, &fq.from_fp(b.x))
                || b.d
                    != (Div {
                        u: vec![fq.neg(&fq.from_fp(b.x)), E2::ONE],
                        v: vec![b.y],
                    })
        }) {
            return Err("factor base is not bound to the cover");
        }
        Ok(Self {
            setup: sieve_setup(&ctx.f.f, &ctx.cov.hx, &base[..small], m),
            base: base.to_vec(),
            by_x: base.iter().enumerate().map(|(i, b)| (b.x, i)).collect(),
            small,
        })
    }
    pub fn setup(&self) -> &SieveSetup {
        &self.setup
    }
    pub fn small_columns(&self) -> usize {
        self.small
    }
    pub fn base(&self) -> &[BaseEl] {
        &self.base
    }
    pub fn sieve_line(
        &self,
        ring: &FpRing,
        line: &Line,
        ctr: &mut Counters,
        shared: Option<&SharedB>,
    ) -> (Vec<u64>, SieveCost) {
        assert_eq!(ring.p, self.setup.p);
        sieve_line_threshold(
            ring,
            &self.setup,
            line,
            ctr,
            None,
            shared,
            (self.setup.m - 2) as u8,
        )
    }

    fn split(&self, full: &SieveRel, id: u64, modulus: u64) -> Option<PartialRelation> {
        if !(3..1u64 << 63).contains(&modulus)
            || modulus.is_multiple_of(2)
            || full.terms.len() != self.setup.m
            || full.terms.windows(2).any(|t| t[0].0 >= t[1].0)
            || full
                .terms
                .iter()
                .any(|&(i, e)| i >= self.base.len() || ![-1, 1].contains(&e))
        {
            return None;
        }
        let mut small = Vec::new();
        let mut large = Vec::new();
        for &(i, e) in &full.terms {
            let value = if e == 1 { 1 } else { modulus - 1 };
            if i < self.small {
                small.push((i as u32, value));
            } else {
                large.push((self.base[i].x, value));
            }
        }
        (large.len() <= 2).then_some(PartialRelation {
            id,
            small,
            large,
            rhs: 0,
        })
    }

    /// Extract small roots deterministically, solve only the remaining
    /// quadratic, and verify the resulting full divisor relation in Jac(H).
    /// Refusals include nonsplit/repeated factors and exceptional ordinates.
    /// The caller supplies the prime logarithm modulus. Output rows are in
    /// those log coordinates; raw divisors need their cofactor projection.
    pub fn verified_relation(
        &self,
        ctx: &Ctx,
        ring: &FpRing,
        line: &Line,
        s: u64,
        id: u64,
        modulus: u64,
    ) -> Option<TwoLargePrimeRelation> {
        if ring.p != self.setup.p
            || ctx.f.f.p != self.setup.p
            || ctx.f.f.w != self.setup.w
            || ctx.cov.hx != self.setup.h
            || !(3..1u64 << 63).contains(&modulus)
            || s == 0
            || s >= self.setup.p
        {
            return None;
        }
        let f = line_poly(ring, line, s);
        if fdeg(&f) != self.setup.m as isize {
            return None;
        }
        let mut remainder = f.clone();
        let mut roots = Vec::new();
        for b in &self.base[..self.small] {
            jv_bitops::words(1);
            if ring.eval(&f, b.x) == 0 {
                roots.push(b.x);
                let (q, r) = ring.divrem(&remainder, &[sm(0, b.x, ring.p), 1]);
                if !r.is_empty() {
                    return None;
                }
                remainder = q;
            }
        }
        if roots.len() < self.setup.m - 2 {
            return None;
        }
        roots.extend(low_degree_roots(ring, &remainder, &self.setup.sqrt)?);
        roots.sort_unstable();
        jv_bitops::words(roots.len() as u64);
        if roots.len() != self.setup.m || roots.windows(2).any(|r| r[0] == r[1]) {
            return None;
        }
        let full = relation_from_roots(
            ring,
            &ctx.f.f,
            &self.setup,
            &self.base,
            &self.by_x,
            line,
            s,
            &roots,
        )?;
        let partial = self.split(&full, id, modulus)?;
        verify_rel(&ctx.jac(), &self.base, &full).then_some(TwoLargePrimeRelation {
            full,
            partial,
            modulus,
        })
    }

    /// Existing full-smooth materializer on the same reduced base. Keep
    /// enumeration and extraction RNGs separate for a matched line cohort.
    pub fn verified_full_smooth_relation(
        &self,
        ctx: &Ctx,
        ring: &FpRing,
        line: &Line,
        s: u64,
        id: u64,
        modulus: u64,
        rng: &mut StdRng,
    ) -> Option<TwoLargePrimeRelation> {
        if ring.p != self.setup.p
            || ctx.f.f.p != self.setup.p
            || ctx.f.f.w != self.setup.w
            || ctx.cov.hx != self.setup.h
            || s == 0
            || s >= self.setup.p
        {
            return None;
        }
        let full = super::relation_from_hit(
            ring,
            &ctx.f.f,
            &self.setup,
            &self.base,
            &self.by_x,
            line,
            s,
            rng,
        )?;
        let partial = self.split(&full, id, modulus)?;
        if !partial.large.is_empty() {
            return None;
        }
        verify_rel(&ctx.jac(), &self.base, &full).then_some(TwoLargePrimeRelation {
            full,
            partial,
            modulus,
        })
    }

    /// Recheck source binding, eliminate actual large-prime vertices and
    /// replay every output from original relations before emitting IC rows.
    /// The caller still needs rank selection, descent and scalar recovery.
    pub fn filter_checked(
        &self,
        ctx: &Ctx,
        relations: &[TwoLargePrimeRelation],
        options: &LargePrimeFilterOptions,
    ) -> Result<FilteredTwoLargePrimeRelations, &'static str> {
        let inputs = self.checked_inputs(ctx, relations, options)?;
        filter_and_replay(&inputs, options)
    }

    fn checked_inputs(
        &self,
        ctx: &Ctx,
        relations: &[TwoLargePrimeRelation],
        options: &LargePrimeFilterOptions,
    ) -> Result<Vec<PartialRelation>, &'static str> {
        if !(3..1u64 << 63).contains(&options.modulus)
            || options.modulus.is_multiple_of(2)
            || relations.len() > 4096
            || !(1..=4096).contains(&options.max_sources)
            || !(1..=4096).contains(&options.max_small_weight)
        {
            return Err("invalid modulus or graph resource bounds");
        }
        if ctx.f.f.p != self.setup.p || ctx.f.f.w != self.setup.w || ctx.cov.hx != self.setup.h {
            return Err("cover mismatch");
        }
        for r in relations {
            if r.modulus != options.modulus
                || self.split(&r.full, r.partial.id, options.modulus).as_ref() != Some(&r.partial)
                || !verify_rel(&ctx.jac(), &self.base, &r.full)
            {
                return Err("input relation binding or Jacobian verification failed");
            }
        }
        Ok(relations.iter().map(|r| r.partial.clone()).collect())
    }
}

fn filter_and_replay(
    inputs: &[PartialRelation],
    options: &LargePrimeFilterOptions,
) -> Result<FilteredTwoLargePrimeRelations, &'static str> {
    let (certificates, report) =
        filter_large_primes(inputs.to_vec(), options).map_err(|_| "graph refusal")?;
    for row in &certificates {
        let replay =
            replay_complete_relation(row, inputs, options.modulus).ok_or("witness refusal")?;
        if replay.small != row.small || replay.rhs != row.rhs {
            return Err("witness mismatch");
        }
    }
    let rows = certificates
        .iter()
        .map(|r| SparseRel {
            cols: r.small.iter().map(|&(c, v)| (c as usize, v)).collect(),
            rhs: r.rhs,
        })
        .collect();
    Ok(FilteredTwoLargePrimeRelations {
        rows,
        replayed_rows: certificates.len(),
        certificates,
        report,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::jv_cover::{factor_base, generate_spec};
    use crate::cryptanalysis::jv_sieve::{
        b_from_index, b_space, coprime_scramble, lines_for_b, relation_from_hit,
    };
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn quadratic_roots_match_exhaustive_fp_oracle_and_refuse_multiplicity() {
        let ring = FpRing::new(19);
        let mut sqrt = vec![u64::MAX; 19];
        for x in 0..19 {
            sqrt[(x * x % 19) as usize] = x;
        }
        for a in 1..19 {
            for b in 0..19 {
                for c in 0..19 {
                    let f = [c, b, a];
                    let expected: Vec<_> = (0..19).filter(|&x| ring.eval(&f, x) == 0).collect();
                    let roots = low_degree_roots(&ring, &f, &sqrt);
                    assert_eq!(roots, (expected.len() == 2).then_some(expected));
                }
            }
        }
        assert_eq!(low_degree_roots(&ring, &[], &sqrt), None);
        assert_eq!(low_degree_roots(&ring, &[7], &sqrt), Some(vec![]));
        assert_eq!(low_degree_roots(&ring, &[1, 2], &sqrt), Some(vec![9]));
    }

    #[test]
    fn actual_cover_partial_relations_agree_with_full_factorization_and_replay() {
        let spec = generate_spec(251, 2);
        let ctx = Ctx::new(&spec);
        let mut rng = StdRng::seed_from_u64(4);
        let base = factor_base(&ctx, &mut rng);
        let by_x = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
        let sieve = TwoLargePrimeSieve::new(&ctx, &base, base.len() * 3 / 4, 9).unwrap();
        let all_setup = sieve_setup(&ctx.f.f, &ctx.cov.hx, &base, 9);
        let ring = FpRing::new(spec.p);
        let scramble = coprime_scramble(b_space(sieve.setup()), &mut rng);
        let mut ctr = Counters::new(spec.p);
        let mut found = Vec::new();
        let mut has_large = false;
        'search: for k in 0..200_000 {
            let b = b_from_index(sieve.setup(), k, scramble);
            for line in lines_for_b(&ring, &ctx.f.f, sieve.setup(), &b, &mut rng) {
                for s in sieve.sieve_line(&ring, &line, &mut ctr, None).0 {
                    if let Some(r) =
                        sieve.verified_relation(&ctx, &ring, &line, s, found.len() as u64, spec.l)
                    {
                        assert_eq!(
                            relation_from_hit(
                                &ring, &ctx.f.f, &all_setup, &base, &by_x, &line, s, &mut rng
                            ),
                            Some(r.full.clone())
                        );
                        has_large |= !r.partial.large.is_empty();
                        found.push(r);
                        if found.len() >= 8 && has_large {
                            break 'search;
                        }
                    }
                }
            }
        }
        assert!(found.len() >= 8 && has_large);
        let options = LargePrimeFilterOptions {
            modulus: spec.l,
            ..Default::default()
        };
        let projected = sieve.filter_checked(&ctx, &found, &options).unwrap();
        assert_eq!(projected.replayed_rows, projected.rows.len());
        let mut damaged = found.clone();
        damaged[0].partial.rhs = 1;
        assert!(sieve.filter_checked(&ctx, &damaged, &options).is_err());
        let invalid = LargePrimeFilterOptions {
            modulus: 0,
            ..options
        };
        assert!(sieve.filter_checked(&ctx, &found, &invalid).is_err());
        let changed = LargePrimeFilterOptions {
            modulus: spec.l - 2,
            ..options
        };
        assert!(sieve.filter_checked(&ctx, &found, &changed).is_err());
        let other = Ctx::new(&generate_spec(251, 3));
        assert!(sieve.filter_checked(&other, &found, &options).is_err());
    }
}
