//! Incremental residual construction. This only samples aG+bQ: it does not
//! store generic collisions or recover scalars independently of IC relations.
use super::*;
use std::collections::HashSet;

pub(super) struct RAddingSampler {
    jumps: Vec<(u64, u64, Pt3)>,
    a: u64,
    b: u64,
    point: Pt3,
    started: bool,
    rng: StdRng,
    visited: HashSet<Pt3>,
    epoch_limit: usize,
    pub(super) restarts: u64,
}

impl RAddingSampler {
    pub fn new(inst: &Instance3, seed: u64, max_epoch: usize) -> Self {
        let c = &inst.curve;
        // Separate RNG domain: the fresh sampler and its oracle stream stay
        // byte-for-byte unchanged when this treatment is not selected.
        let mut rng = StdRng::seed_from_u64(seed ^ 0x5341_4d50_4c45_5233);
        let mut jumps = Vec::with_capacity(32);
        while jumps.len() < 32 {
            let a = rng.gen_range(0..c.n);
            let b = rng.gen_range(1..c.n);
            let point = c.add(&c.mul(&c.g, a), &c.mul(&inst.q, b));
            if !point.inf {
                jumps.push((a, b, point));
            }
        }
        let a = rng.gen_range(0..c.n);
        let b = rng.gen_range(1..c.n);
        let point = c.add(&c.mul(&c.g, a), &c.mul(&inst.q, b));
        let mut visited = HashSet::new();
        visited.insert(point);
        Self {
            jumps,
            a,
            b,
            point,
            started: false,
            rng,
            visited,
            epoch_limit: (c.n as f64).sqrt().ceil().clamp(2.0, max_epoch as f64) as usize,
            restarts: 0,
        }
    }

    fn restart(&mut self, c: &Curve3, q: &Pt3) {
        self.a = self.rng.gen_range(0..c.n);
        self.b = self.rng.gen_range(1..c.n);
        self.point = c.add(&c.mul(&c.g, self.a), &c.mul(q, self.b));
        self.visited.clear();
        self.visited.insert(self.point);
        self.restarts += 1;
    }

    pub fn next(&mut self, c: &Curve3, q: &Pt3) -> (u64, u64, Pt3) {
        if self.started {
            if self.visited.len() >= self.epoch_limit {
                self.restart(c, q);
                return (self.a, self.b, self.point);
            }
            let j = (mix64(
                self.point.x.0[0]
                    ^ self.point.x.0[1].wrapping_mul(0x9e37)
                    ^ self.point.x.0[2].rotate_left(17),
            ) % 32) as usize;
            let (a, b, point) = self.jumps[j];
            let next = c.add(&self.point, &point);
            // Store only point IDs, never previous (a,b) coefficients. A
            // repeat triggers a uniform restart; it cannot recover a scalar.
            if !self.visited.insert(next) {
                self.restart(c, q);
            } else {
                self.point = next;
                self.a = ((self.a as u128 + a as u128) % c.n as u128) as u64;
                self.b = ((self.b as u128 + b as u128) % c.n as u128) as u64;
            }
        }
        self.started = true;
        (self.a, self.b, self.point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incremental_residuals_keep_exact_group_invariant() {
        let inst = generate_instance3(7, 3);
        let mut sampler = RAddingSampler::new(&inst, 3, 1024);
        for _ in 0..200 {
            let before = inst.curve.ops();
            let restarts_before = sampler.restarts;
            let (a, b, point) = sampler.next(&inst.curve, &inst.q);
            if sampler.restarts == restarts_before {
                assert!(inst.curve.ops() - before <= 1);
            }
            assert_eq!(
                point,
                inst.curve.add(
                    &inst.curve.mul(&inst.curve.g, a),
                    &inst.curve.mul(&inst.q, b)
                )
            );
        }
        assert!(sampler.restarts > 0);
    }

    #[test]
    fn fresh_sampler_keeps_observed_pilot_prefix() {
        let inst = generate_instance3(67, 3);
        let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(3));
        let opts = GaudryOptions {
            max_residuals: 10000,
            solver: Solver::SymmetricPairOnly,
            small_base: (base.len() as f64).powf(2.0 / 3.0).ceil() as usize,
            max_large_primes: 2,
            ..GaudryOptions::default()
        };
        let got =
            run_gaudry_with_eliminator(&inst, &base, 3, &opts, LargePrimeBackend::SymbolicGraph);
        assert_eq!(got.correct, Some(true));
        assert_eq!(
            got.lp_input_digest.as_deref(),
            Some("a3cb66b5c75d1b0b7bd1114041e5519907e5985919ca22ae2cf4f49915de85ac")
        );
        assert_eq!(
            (got.residuals, got.group_ops, got.oracle_fp_muls),
            (1884, 93145, 267577)
        );
        assert_eq!(got.sampler_setup_group_ops, Some(0));
        assert!(got.sampling_group_ops.unwrap() > got.residuals);
    }

    #[test]
    fn walk_oracles_match_between_protected_backends_and_recover() {
        let inst = generate_instance3(7, 3);
        let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(3));
        for solver in [Solver::SymmetricPairOnly, Solver::SymmetricMeetInTheMiddle] {
            let opts = GaudryOptions {
                max_residuals: 20000,
                solver,
                small_base: 1,
                max_large_primes: 2,
                ..GaudryOptions::default()
            };
            let p = run_gaudry_with_sampler(
                &inst,
                &base,
                5,
                &opts,
                LargePrimeBackend::Pivot,
                ResidualSampler3::RAdding32,
            );
            let g = run_gaudry_with_sampler(
                &inst,
                &base,
                5,
                &opts,
                LargePrimeBackend::SymbolicGraph,
                ResidualSampler3::RAdding32,
            );
            assert_eq!(p.correct, Some(true), "{p:?}");
            assert_eq!(g.correct, Some(true), "{g:?}");
            assert_eq!(p.lp_input_digest, g.lp_input_digest);
            assert_eq!(p.residuals, g.residuals);
            assert_eq!(p.sampling_group_ops, g.sampling_group_ops);
            let bits = 64 - inst.curve.n.leading_zeros() as u64;
            assert!(
                g.sampling_group_ops.unwrap()
                    <= g.residuals + g.sampler_restarts.unwrap() * (4 * bits + 2)
            );
            assert!(g.sampler_setup_group_ops.unwrap() > 0);
        }
    }

    #[test]
    fn prepared_oracle_preserves_walk_stream_and_long_epochs_recover() {
        let inst = generate_instance3(127, 37);
        let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(37));
        for sampler in [ResidualSampler3::RAdding32, ResidualSampler3::RAddingLong] {
            let mut opts = GaudryOptions {
                max_residuals: 100000,
                solver: Solver::SymmetricPairOnly,
                small_base: (base.len() as f64).powf(2.0 / 3.0).ceil() as usize,
                max_large_primes: 2,
                ..GaudryOptions::default()
            };
            let reference = run_gaudry_with_sampler(
                &inst,
                &base,
                37,
                &opts,
                LargePrimeBackend::SymbolicGraph,
                sampler,
            );
            opts.solver = Solver::PreparedSymmetricPairOnly;
            let prepared = run_gaudry_with_sampler(
                &inst,
                &base,
                37,
                &opts,
                LargePrimeBackend::SymbolicGraph,
                sampler,
            );
            assert_eq!(reference.correct, Some(true));
            assert_eq!(prepared.correct, Some(true));
            assert_eq!(reference.lp_input_digest, prepared.lp_input_digest);
            assert_eq!(reference.group_ops, prepared.group_ops);
            assert_eq!(reference.residuals, prepared.residuals);
            assert!(prepared.oracle_fp_muls < reference.oracle_fp_muls);
            assert!(prepared.precompute_fp_muls > reference.precompute_fp_muls);
            let stats = prepared.relation_point_stats.unwrap();
            assert_eq!(
                stats.decomposed_residuals,
                stats.distinct_abscissae + stats.repeated_abscissae
            );
        }
    }
}
