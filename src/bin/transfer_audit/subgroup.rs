//! The subgroup-preservation certificate.
//!
//! A transfer is useful only if the prime-order subgroup `<G>` of order `r`
//! maps injectively: `phi(G)` has order exactly `r` and the kernel of `phi`
//! meets `<G>` trivially.  For prime `r` the second condition is the same as
//! `phi(G) != O`, because the kernel meets the subgroup in a subgroup of
//! `<G>`.  The checker also demands the map be a homomorphism on sampled
//! elements and commute with scalars, since a map that is not a homomorphism
//! carries no discrete logarithm.
//!
//! The checker is generic over [`Group`], so the same code certifies a
//! trace map `E(F_(2^n)) -> E(F_(2^l))`, a cover pullback
//! `E(F_q) -> Jac(H)(F_q)` and the tiny algebraic groups used in tests.

use num_bigint::BigUint;
use num_traits::One;
use serde::Serialize;

use crate::certificate::Evidence;
use crypto_lib::binary_ecc::curve::{point_add, scalar_mul};
use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, HyperellipticCurve, MumfordDivisor};

/// The operations the checker needs from a finite abelian group.
pub trait Group {
    type El: Clone + PartialEq;
    fn identity(&self) -> Self::El;
    fn add(&self, a: &Self::El, b: &Self::El) -> Self::El;
    /// `[k]a`.
    fn mul(&self, k: &BigUint, a: &Self::El) -> Self::El;
}

/// `E(F_(2^m))`.
pub struct BinaryEc<'a>(pub &'a BinaryCurve);

impl Group for BinaryEc<'_> {
    type El = BinaryPoint;
    fn identity(&self) -> BinaryPoint {
        BinaryPoint::Infinity
    }
    fn add(&self, a: &BinaryPoint, b: &BinaryPoint) -> BinaryPoint {
        point_add(self.0, a, b)
    }
    fn mul(&self, k: &BigUint, a: &BinaryPoint) -> BinaryPoint {
        scalar_mul(self.0, a, k)
    }
}

/// `Jac(H)(F_(2^m))` in Mumford representation.
pub struct BinaryJac<'a>(pub &'a HyperellipticCurve);

impl Group for BinaryJac<'_> {
    type El = MumfordDivisor;
    fn identity(&self) -> MumfordDivisor {
        MumfordDivisor::identity(self.0.m)
    }
    fn add(&self, a: &MumfordDivisor, b: &MumfordDivisor) -> MumfordDivisor {
        a.add(b, self.0)
    }
    fn mul(&self, k: &BigUint, a: &MumfordDivisor) -> MumfordDivisor {
        a.scalar_mul(k, self.0)
    }
}

/// `Z/n` under addition; the smallest groups on which the certificate's
/// logic can be checked by hand.
#[cfg(test)]
pub struct ZMod(pub u64);

#[cfg(test)]
impl Group for ZMod {
    type El = u64;
    fn identity(&self) -> u64 {
        0
    }
    fn add(&self, a: &u64, b: &u64) -> u64 {
        (a + b) % self.0
    }
    fn mul(&self, k: &BigUint, a: &u64) -> u64 {
        let n = BigUint::from(self.0);
        let v = (k % &n) * BigUint::from(*a) % n;
        v.iter_u64_digits().next().unwrap_or(0)
    }
}

/// What the checker established.
#[derive(Clone, Debug, Serialize)]
pub struct SubgroupReport {
    pub status: Evidence,
    /// `G != O` and `[r]G = O`: with `r` prime, `G` has order exactly `r`.
    pub source_generator_order_is_r: bool,
    pub r_is_prime: bool,
    pub image_is_identity: bool,
    /// `phi(G) != O` and `[r]phi(G) = O`.
    pub image_order_is_r: bool,
    /// Kernel of `phi` meets `<G>` nontrivially (for prime `r`: `phi(G) = O`).
    pub kernel_meets_subgroup: bool,
    pub additivity_passed: usize,
    pub additivity_total: usize,
    pub scalar_equivariance_passed: usize,
    pub scalar_equivariance_total: usize,
    pub failures: Vec<String>,
}

/// Inputs of one check.  `samples` are elements of the source group (not
/// necessarily of `<G>`) used for the additivity test; `scalars` for the
/// equivariance test.
pub struct SubgroupInputs<'a, S: Group, D: Group> {
    pub src: &'a S,
    pub dst: &'a D,
    pub map: &'a dyn Fn(&S::El) -> Result<D::El, String>,
    pub g: &'a S::El,
    pub r: &'a BigUint,
    pub r_is_prime: bool,
    pub samples: &'a [S::El],
    pub scalars: &'a [BigUint],
}

/// Run the certificate.  `Verified` only if every check passes;
/// `Refuted` when the map is defined but loses the subgroup or is not a
/// homomorphism; `NotEvaluated` when the premise (a generator of prime
/// order `r`) or the map itself fails to evaluate.
pub fn check_subgroup_preservation<S: Group, D: Group>(
    inp: &SubgroupInputs<S, D>,
) -> SubgroupReport {
    let mut rep = SubgroupReport {
        status: Evidence::NotEvaluated,
        source_generator_order_is_r: false,
        r_is_prime: inp.r_is_prime,
        image_is_identity: false,
        image_order_is_r: false,
        kernel_meets_subgroup: false,
        additivity_passed: 0,
        additivity_total: 0,
        scalar_equivariance_passed: 0,
        scalar_equivariance_total: 0,
        failures: Vec::new(),
    };
    if !inp.r_is_prime {
        rep.failures.push(
            "r is not prime: the kernel-intersection test below is the prime-order test".into(),
        );
        return rep;
    }
    let src_id = inp.src.identity();
    rep.source_generator_order_is_r =
        *inp.g != src_id && inp.src.mul(inp.r, inp.g) == src_id && !inp.r.is_one();
    if !rep.source_generator_order_is_r {
        rep.failures
            .push("source generator does not have exact order r".into());
        return rep;
    }
    let image = match (inp.map)(inp.g) {
        Ok(v) => v,
        Err(e) => {
            rep.failures
                .push(format!("map failed on the generator: {e}"));
            return rep;
        }
    };
    let dst_id = inp.dst.identity();
    rep.image_is_identity = image == dst_id;
    rep.kernel_meets_subgroup = rep.image_is_identity;
    rep.image_order_is_r = !rep.image_is_identity && inp.dst.mul(inp.r, &image) == dst_id;
    if rep.image_is_identity {
        rep.failures
            .push("phi(G) = O: the kernel meets <G> in all of <G>".into());
    } else if !rep.image_order_is_r {
        rep.failures.push(
            "[r]phi(G) != O: phi(G) does not have order r, so phi is not a homomorphism on <G>"
                .into(),
        );
    }

    // Additivity on consecutive pairs of samples (cyclically).
    let n = inp.samples.len();
    if n >= 2 {
        for i in 0..n {
            let a = &inp.samples[i];
            let b = &inp.samples[(i + 1) % n];
            rep.additivity_total += 1;
            let sum = inp.src.add(a, b);
            match ((inp.map)(a), (inp.map)(b), (inp.map)(&sum)) {
                (Ok(fa), Ok(fb), Ok(fs)) => {
                    if inp.dst.add(&fa, &fb) == fs {
                        rep.additivity_passed += 1;
                    } else {
                        rep.failures
                            .push(format!("phi(a+b) != phi(a)+phi(b) on sample pair {i}"));
                    }
                }
                _ => rep.failures.push(format!("map failed on sample pair {i}")),
            }
        }
    }
    // Scalar equivariance on the generator.
    for k in inp.scalars {
        rep.scalar_equivariance_total += 1;
        let kg = inp.src.mul(k, inp.g);
        match (inp.map)(&kg) {
            Ok(v) if v == inp.dst.mul(k, &image) => rep.scalar_equivariance_passed += 1,
            Ok(_) => rep
                .failures
                .push("phi([k]G) != [k]phi(G) for a sampled k".into()),
            Err(e) => rep.failures.push(format!("map failed on [k]G: {e}")),
        }
    }

    let homomorphism_ok = rep.additivity_passed == rep.additivity_total
        && rep.scalar_equivariance_passed == rep.scalar_equivariance_total;
    rep.status = if rep.image_order_is_r && !rep.kernel_meets_subgroup && homomorphism_ok {
        Evidence::Verified
    } else {
        Evidence::Refuted
    };
    // A zero scalar or an empty sample set cannot refute: they are not run.
    if inp.scalars.is_empty() && inp.samples.len() < 2 && rep.status == Evidence::Verified {
        rep.failures
            .push("no homomorphism samples supplied: image order only".into());
    }
    rep
}

#[cfg(test)]
mod tests {
    use super::*;

    fn big(v: u64) -> BigUint {
        BigUint::from(v)
    }

    /// `Z/15` has the subgroup `<3>` of prime order 5.  Multiplication by 1
    /// and by 3 are homomorphisms whose kernels ({0}, {0,5,10}) miss `<3>`;
    /// multiplication by 5 has kernel `<3>` itself.
    #[test]
    fn algebraic_maps_on_z15_by_hand() {
        let g = ZMod(15);
        let samples = [1u64, 2, 7, 11];
        let scalars = [big(2), big(3), big(4)];
        let mk = |m: u64| move |x: &u64| Ok::<u64, String>((m * x) % 15);
        for (mult, expect) in [
            (1u64, Evidence::Verified),
            (3, Evidence::Verified),
            (5, Evidence::Refuted),
        ] {
            let f = mk(mult);
            let rep = check_subgroup_preservation(&SubgroupInputs {
                src: &g,
                dst: &g,
                map: &f,
                g: &3u64,
                r: &big(5),
                r_is_prime: true,
                samples: &samples,
                scalars: &scalars,
            });
            assert_eq!(rep.status, expect, "multiplier {mult}: {:?}", rep.failures);
            assert_eq!(rep.kernel_meets_subgroup, mult == 5);
        }
    }

    /// A map that is not a homomorphism is rejected even when it sends the
    /// generator to a point of order r.
    #[test]
    fn non_homomorphism_is_refuted() {
        let g = ZMod(15);
        let f = |x: &u64| Ok::<u64, String>(if *x == 3 { 3 } else { (x * x) % 15 });
        let rep = check_subgroup_preservation(&SubgroupInputs {
            src: &g,
            dst: &g,
            map: &f,
            g: &3u64,
            r: &big(5),
            r_is_prime: true,
            samples: &[1, 2, 7, 11],
            scalars: &[big(2)],
        });
        assert_eq!(rep.status, Evidence::Refuted);
        assert!(!rep.failures.is_empty());
    }

    #[test]
    fn invalid_premises_are_not_evaluated() {
        let g = ZMod(15);
        let f = |x: &u64| Ok::<u64, String>(*x);
        // 4 is not prime.
        let rep = check_subgroup_preservation(&SubgroupInputs {
            src: &g,
            dst: &g,
            map: &f,
            g: &3u64,
            r: &big(4),
            r_is_prime: false,
            samples: &[],
            scalars: &[],
        });
        assert_eq!(rep.status, Evidence::NotEvaluated);
        // 1 is not of order 5.
        let rep = check_subgroup_preservation(&SubgroupInputs {
            src: &g,
            dst: &g,
            map: &f,
            g: &1u64,
            r: &big(5),
            r_is_prime: true,
            samples: &[],
            scalars: &[],
        });
        assert_eq!(rep.status, Evidence::NotEvaluated);
    }
}
