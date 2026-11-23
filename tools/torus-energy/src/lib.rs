//! Exact calibration of a birthday-block gap test, not a Curve25519 solver.
//! Statistical guarantees require independent uniform tuple samples and the
//! probability argument in the study. This crate does not certify that law.
use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::BTreeSet;

pub type Result<T> = std::result::Result<T, String>;
pub const BLOCKS: u32 = 8192;
pub const COLLISION_THRESHOLD: u32 = BLOCKS / 24;
pub const ERROR_BITS: usize = 20;

#[derive(Clone, Debug)]
pub struct Tail {
    pub numerator: BigUint,
    pub denominator: BigUint,
}
impl Tail {
    pub fn at_most_power_of_two(&self, bits: usize) -> bool {
        (&self.numerator << bits) <= self.denominator
    }
}

/// Exact binomial tail for event successes <= threshold (lower=true) or
/// successes > threshold, with independent success probability 1/d.
/// Every recurrence division and normalization is checked using integers.
pub fn binomial_tail(trials: u32, d: u32, threshold: u32, lower: bool) -> Result<Tail> {
    if trials > 32768 || !(2..=256).contains(&d) || threshold > trials {
        return Err("unsupported fixed calibration parameters".into());
    }
    let mut term = BigUint::from(d - 1).pow(trials);
    let mut all = BigUint::zero();
    let mut selected = BigUint::zero();
    for k in 0..=trials {
        all += &term;
        if (k <= threshold) == lower {
            selected += &term;
        }
        if k != trials {
            let numerator = &term * (trials - k);
            let divisor = BigUint::from(k + 1) * (d - 1);
            if &numerator % &divisor != BigUint::zero() {
                return Err("nonintegral binomial recurrence".into());
            }
            term = numerator / divisor;
        }
    }
    let denominator = BigUint::from(d).pow(trials);
    if all != denominator || selected > denominator {
        return Err("binomial normalization failed".into());
    }
    Ok(Tail {
        numerator: selected,
        denominator,
    })
}

pub fn calibration() -> Result<(Tail, Tail)> {
    // Worst-case good-base block probability <=1/32; reject above floor(m/24).
    let false_reject = binomial_tail(BLOCKS, 32, COLLISION_THRESHOLD, false)?;
    // Worst-case bad-base block probability >=1/17; accept at/below threshold.
    let false_accept = binomial_tail(BLOCKS, 17, COLLISION_THRESHOLD, true)?;
    if !false_reject.at_most_power_of_two(ERROR_BITS)
        || !false_accept.at_most_power_of_two(ERROR_BITS)
    {
        return Err("registered exact error threshold not met".into());
    }
    Ok((false_reject, false_accept))
}

#[derive(Clone, Debug)]
pub struct Plan {
    pub w0: BigUint,
    pub block_samples: BigUint,
    pub sample_ceiling: BigUint,
    pub completeness_lhs: BigUint,
    pub soundness_rhs: BigUint,
}
impl Plan {
    /// Good C<=100W0 versus bad C>=576W0; no promise in the gap.
    /// The inequalities, not an assumed floating-point square root, bind n.
    pub fn new(w0: BigUint) -> Result<Self> {
        if w0 < BigUint::from(230400u32) {
            return Err("this calibration requires W0 >= 230400".into());
        }
        let square_root = w0.sqrt();
        if &square_root * &square_root > w0 || (&square_root + 1u32).pow(2) <= w0 {
            return Err("invalid integer square root".into());
        }
        let n = square_root / 40u32;
        if n < BigUint::from(2u32) {
            return Err("block too short".into());
        }
        let completeness_lhs = BigUint::from(1600u32) * n.pow(2);
        let soundness_rhs = BigUint::from(2304u32) * (&n - 1u32).pow(2);
        if completeness_lhs > w0 || soundness_rhs < w0 {
            return Err("birthday-block thresholds do not cover energy gap".into());
        }
        let sample_ceiling = &n * BLOCKS;
        Ok(Self {
            w0,
            block_samples: n,
            sample_ceiling,
            completeness_lhs,
            soundness_rhs,
        })
    }
}

/// Exact full-label comparison. Supplying a hash digest alone is not a proof
/// that distinct curve points have distinct labels. This is a finite control
/// primitive; large-base sample collection is deliberately not launched here.
pub fn block_has_collision<K: Ord>(samples: &[K], expected: usize) -> Result<bool> {
    if expected < 2 || samples.len() != expected {
        return Err("incomplete or invalid block".into());
    }
    let mut seen = BTreeSet::new();
    for item in samples {
        if !seen.insert(item) {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn accept_blocks(flags: &[bool]) -> Result<bool> {
    if flags.len() != BLOCKS as usize {
        return Err("fixed block count not met".into());
    }
    Ok(flags.iter().filter(|b| **b).count() <= COLLISION_THRESHOLD as usize)
}

pub fn actual_w0() -> BigUint {
    let l = BigUint::from(4u32) * BigUint::from(60824497u64) * BigUint::from(213156431u64);
    BigUint::from(16u32) * &l * (&l - 1u32) * (&l - 2u32) * (&l - 3u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;
    use num_rational::BigRational as Q;
    fn q(a: u64, b: u64) -> Q {
        Q::new(BigInt::from(a), BigInt::from(b))
    }

    #[test]
    fn exact_binomial_matches_direct_small_counts_and_limits() {
        for n in 0..=8u32 {
            for d in 2..=4u32 {
                for threshold in 0..=n {
                    let mut accepted = 0u64;
                    for mut code in 0..(d as u64).pow(n) {
                        let mut successes = 0;
                        for _ in 0..n {
                            successes += u32::from(code % d as u64 == 0);
                            code /= d as u64;
                        }
                        if successes <= threshold {
                            accepted += 1;
                        }
                    }
                    let lower = binomial_tail(n, d, threshold, true).unwrap();
                    let upper = binomial_tail(n, d, threshold, false).unwrap();
                    assert_eq!(lower.numerator, BigUint::from(accepted));
                    assert_eq!(&lower.numerator + &upper.numerator, lower.denominator);
                }
            }
        }
        assert!(binomial_tail(32769, 32, 1, true).is_err());
        assert!(binomial_tail(2, 1, 1, true).is_err());
    }

    #[test]
    fn exact_fixed_calibration_and_actual_plan() {
        let (good, bad) = calibration().unwrap();
        assert!(good.at_most_power_of_two(20));
        assert!(bad.at_most_power_of_two(20));
        let plan = Plan::new(actual_w0()).unwrap();
        assert!(plan.completeness_lhs <= plan.w0);
        assert!(plan.soundness_rhs >= plan.w0);
        assert_eq!(plan.sample_ceiling, &plan.block_samples * BLOCKS);
        assert!(Plan::new(BigUint::from(230399u32)).is_err());
        for w in [230400u64, 278783, 278784, 319999, 320000, 400000, 1000000] {
            assert!(Plan::new(BigUint::from(w)).is_ok());
        }
    }

    #[test]
    fn exact_collision_moments_match_full_finite_enumeration() {
        // Uniform sample indices into these multisets induce uniform,
        // concentrated, heavy-atom and singleton output distributions.
        for labels in [
            vec![0],
            vec![0, 1],
            vec![0, 0, 1],
            vec![0, 0, 0, 1],
            vec![0, 0, 1, 2],
            vec![0, 1, 2, 3],
        ] {
            let domain = labels.len() as u64;
            let mut counts = std::collections::BTreeMap::<u8, u64>::new();
            for x in &labels {
                *counts.entry(*x).or_default() += 1;
            }
            let q2 = q(counts.values().map(|x| x * x).sum(), domain.pow(2));
            let q3 = q(counts.values().map(|x| x * x * x).sum(), domain.pow(3));
            assert!(&q3 * &q3 <= &q2 * &q2 * &q2);
            for n in 2..=6u32 {
                let total = domain.pow(n);
                let mut sum = 0u64;
                let mut sumsq = 0u64;
                let mut nonzero = 0u64;
                for mut code in 0..total {
                    let mut sample = vec![];
                    for _ in 0..n {
                        sample.push(labels[(code % domain) as usize]);
                        code /= domain;
                    }
                    let mut z = 0u64;
                    for i in 0..sample.len() {
                        for j in 0..i {
                            z += u64::from(sample[i] == sample[j]);
                        }
                    }
                    assert_eq!(block_has_collision(&sample, n as usize).unwrap(), z > 0);
                    sum += z;
                    sumsq += z * z;
                    nonzero += u64::from(z > 0);
                }
                let mean = q(sum, total);
                let second = q(sumsq, total);
                let variance = &second - &mean * &mean;
                let pairs = (n as u64) * (n as u64 - 1) / 2;
                let triples = (n as u64) * (n as u64 - 1) * (n as u64 - 2);
                assert_eq!(mean, q(pairs, 1) * &q2);
                assert_eq!(
                    variance,
                    q(pairs, 1) * (&q2 - &q2 * &q2) + q(triples, 1) * (&q3 - &q2 * &q2)
                );
                assert!(q(nonzero, total) <= mean);
                if !second.is_zero() {
                    assert!(q(nonzero, total) * second >= &mean * &mean);
                }
            }
        }
    }

    #[test]
    fn fixed_frame_and_decision_boundaries() {
        assert!(block_has_collision(&[1u8, 2], 3).is_err());
        assert!(!block_has_collision(&[1u8, 2, 3], 3).unwrap());
        assert!(block_has_collision(&[1u8, 2, 1], 3).unwrap());
        assert!(accept_blocks(&[false; 4]).is_err());
        let mut flags = vec![false; BLOCKS as usize];
        for f in flags.iter_mut().take(COLLISION_THRESHOLD as usize) {
            *f = true;
        }
        assert!(accept_blocks(&flags).unwrap());
        flags[COLLISION_THRESHOLD as usize] = true;
        assert!(!accept_blocks(&flags).unwrap());
    }
}
