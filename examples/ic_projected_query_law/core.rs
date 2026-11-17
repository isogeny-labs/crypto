//! Checked finite-group plumbing for projected IC, not a relation oracle or a cost model.
//! The backend must supply a correct abelian group and its certified cardinality.

use std::fmt::Debug;
use std::hash::Hash;

/// Bound for a supplied complete kernel; construction and storage are charged separately.
pub const MAX_SUPPLIED_KERNEL_SIZE: u64 = 65_536;

/// Arithmetic, membership checks and their accounting belong to the backend.
pub trait Group {
    type Point: Copy + Eq + Debug;
    /// Complete fixed-domain parameters, not a lossy hash or a user label.
    fn domain(&self) -> Vec<u64>;
    fn order(&self) -> u64;
    fn identity(&self) -> Self::Point;
    fn contains(&self, point: Self::Point) -> bool;
    fn add(&mut self, a: Self::Point, b: Self::Point) -> Self::Point;
    fn neg(&mut self, point: Self::Point) -> Self::Point;
    fn mul(&mut self, point: Self::Point, scalar: u64) -> Self::Point;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    UnsupportedPrimeOrder,
    UnsupportedKernelSize,
    NonCoprimeOrders,
    CardinalityMismatch,
    InvalidGenerator,
    InvalidKernelGenerator,
    KernelIsNotCyclicOfDeclaredOrder,
    InvalidKernelEnumeration,
    InvalidKernelElement,
    TooManyKernelCandidates,
    IncompleteKernelSpan { found: usize, expected: usize },
    InvalidTarget,
    OffsetOutOfRange,
    KernelIndexOutOfRange,
    QueryContextMismatch,
    InvalidRelationPoint,
    InvalidProjectedLog,
    InvalidFullCurveRelation,
    ScalarVerificationFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query<P> {
    target: P,
    offset: u64,
    kernel_index: usize,
    point: P,
    projected: P,
}

impl<P: Copy> Query<P> {
    pub fn point(&self) -> P {
        self.point
    }
    pub fn projected(&self) -> P {
        self.projected
    }
}

/// `log` is the supplied logarithm of [h]point, before applying `negative`.
#[derive(Clone, Copy, Debug)]
pub struct Term<P> {
    pub point: P,
    pub log: u64,
    pub negative: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Recovered {
    pub scalar: u64,
    pub projected_log_sum: u64,
    pub relation_terms: usize,
    pub direct_identity_hit: bool,
}

#[derive(Clone, Debug)]
pub struct Law<P> {
    domain: Vec<u64>,
    generator: P,
    r: u64,
    h: u64,
    h_inverse: u64,
    kernel: Vec<P>,
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

// Explicit small-order scope; no probable-prime flag or imported certificate is trusted.
fn supported_prime(n: u64) -> bool {
    if !(3..=u64::from(u32::MAX)).contains(&n) || n.is_multiple_of(2) {
        return false;
    }
    let mut d = 3;
    while d <= n / d {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 2;
    }
    true
}

fn times_mod(a: u64, b: u64, modulus: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(modulus)) as u64
}

fn power_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut value = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            value = times_mod(value, base, modulus);
        }
        exponent >>= 1;
        base = times_mod(base, base, modulus);
    }
    value
}

impl<P: Copy + Eq + Debug> Law<P> {
    fn checked_domain<G: Group<Point = P>>(
        group: &mut G,
        generator: P,
        r: u64,
        h: u64,
        maximum_kernel_size: u64,
    ) -> Result<u64, Error> {
        if !supported_prime(r) {
            return Err(Error::UnsupportedPrimeOrder);
        }
        if !(1..=maximum_kernel_size).contains(&h) {
            return Err(Error::UnsupportedKernelSize);
        }
        if gcd(r, h) != 1 {
            return Err(Error::NonCoprimeOrders);
        }
        if r.checked_mul(h) != Some(group.order()) {
            return Err(Error::CardinalityMismatch);
        }
        let zero = group.identity();
        if !group.contains(generator) || generator == zero || group.mul(generator, r) != zero {
            return Err(Error::InvalidGenerator);
        }
        let h_inverse = power_mod(h % r, r - 2, r);
        assert_eq!(times_mod(h, h_inverse, r), 1);
        Ok(h_inverse)
    }

    /// Certify exact generator/kernel orders, conditional on the backend's ambient order.
    /// A noncyclic kernel is rejected; a cyclic generator must actually be supplied.
    pub fn certify<G: Group<Point = P>>(
        group: &mut G,
        generator: P,
        r: u64,
        h: u64,
        kernel_generator: P,
    ) -> Result<Self, Error> {
        let h_inverse = Self::checked_domain(group, generator, r, h, 4096)?;
        let zero = group.identity();
        if !group.contains(kernel_generator) || group.mul(kernel_generator, h) != zero {
            return Err(Error::InvalidKernelGenerator);
        }
        let mut kernel = Vec::with_capacity(h as usize);
        let mut point = zero;
        for _ in 0..h {
            if !group.contains(point) || kernel.contains(&point) {
                return Err(Error::KernelIsNotCyclicOfDeclaredOrder);
            }
            if group.mul(point, h) != zero {
                return Err(Error::InvalidKernelGenerator);
            }
            kernel.push(point);
            point = group.add(point, kernel_generator);
        }
        if point != zero {
            return Err(Error::InvalidKernelGenerator);
        }
        Ok(Self {
            domain: group.domain(),
            generator,
            r,
            h,
            h_inverse,
            kernel,
        })
    }

    /// Check a complete supplied kernel, including noncyclic kernels.
    /// Given a correct abelian group of certified order h*r with prime r and
    /// gcd(h,r)=1, exactly h distinct points annihilated by h exhaust ker[h].
    /// This checks that census; it does not find it or certify the ambient order.
    /// Input order is retained as the public kernel-index map.
    pub fn certify_with_kernel<G: Group<Point = P>>(
        group: &mut G,
        generator: P,
        r: u64,
        h: u64,
        kernel: Vec<P>,
    ) -> Result<Self, Error>
    where
        P: Hash,
    {
        let h_inverse = Self::checked_domain(group, generator, r, h, MAX_SUPPLIED_KERNEL_SIZE)?;
        if kernel.len() != h as usize {
            return Err(Error::InvalidKernelEnumeration);
        }
        let zero = group.identity();
        let mut seen = std::collections::HashSet::with_capacity(kernel.len());
        for &point in &kernel {
            if !group.contains(point) || group.mul(point, h) != zero {
                return Err(Error::InvalidKernelElement);
            }
            if !seen.insert(point) {
                return Err(Error::InvalidKernelEnumeration);
            }
        }
        if !seen.contains(&zero) {
            return Err(Error::InvalidKernelEnumeration);
        }
        Ok(Self {
            domain: group.domain(),
            generator,
            r,
            h,
            h_inverse,
            kernel,
        })
    }

    /// Construct a complete kernel from supplied ambient points by projecting
    /// them with [r] and adjoining their cosets. This is conditional on the same
    /// certified ambient order as `certify_with_kernel`; no point search or
    /// randomness is supplied. Incomplete spans fail instead of becoming a
    /// purported full-domain query law. Input order fixes enumeration order.
    /// All construction and final certification group calls remain chargeable.
    pub fn certify_from_points<G: Group<Point = P>>(
        group: &mut G,
        generator: P,
        r: u64,
        h: u64,
        candidates: &[P],
    ) -> Result<Self, Error>
    where
        P: Hash,
    {
        Self::checked_domain(group, generator, r, h, MAX_SUPPLIED_KERNEL_SIZE)?;
        if candidates.len() > MAX_SUPPLIED_KERNEL_SIZE as usize {
            return Err(Error::TooManyKernelCandidates);
        }
        // Validate the complete supplied input, including unused trailing points.
        if candidates.iter().any(|&point| !group.contains(point)) {
            return Err(Error::InvalidKernelElement);
        }
        let zero = group.identity();
        let mut kernel = Vec::with_capacity(h as usize);
        let mut seen = std::collections::HashSet::with_capacity(h as usize);
        kernel.push(zero);
        seen.insert(zero);
        for &point in candidates {
            if kernel.len() == h as usize {
                break;
            }
            let projected = group.mul(point, r);
            if !group.contains(projected) || group.mul(projected, h) != zero {
                return Err(Error::InvalidKernelElement);
            }
            // The current kernel is a subgroup. Distinct successive cosets of
            // <projected> modulo it are disjoint until returning to the subgroup.
            let previous_size = kernel.len();
            let mut shift = projected;
            while !seen.contains(&shift) {
                if kernel.len() + previous_size > h as usize {
                    return Err(Error::InvalidKernelEnumeration);
                }
                for index in 0..previous_size {
                    let member = group.add(kernel[index], shift);
                    if !seen.insert(member) {
                        return Err(Error::InvalidKernelEnumeration);
                    }
                    kernel.push(member);
                }
                shift = group.add(shift, projected);
            }
        }
        if kernel.len() != h as usize {
            return Err(Error::IncompleteKernelSpan {
                found: kernel.len(),
                expected: h as usize,
            });
        }
        Self::certify_with_kernel(group, generator, r, h, kernel)
    }

    pub fn kernel(&self) -> &[P] {
        &self.kernel
    }

    /// The caller samples offset and kernel_index independently and uniformly.
    /// This function supplies no randomness and makes no empirical uniformity claim.
    pub fn query<G: Group<Point = P>>(
        &self,
        group: &mut G,
        target: P,
        offset: u64,
        kernel_index: usize,
    ) -> Result<Query<P>, Error> {
        if group.domain() != self.domain || self.r.checked_mul(self.h) != Some(group.order()) {
            return Err(Error::QueryContextMismatch);
        }
        if !group.contains(target) || group.mul(target, self.r) != group.identity() {
            return Err(Error::InvalidTarget);
        }
        if offset >= self.r {
            return Err(Error::OffsetOutOfRange);
        }
        let kernel_point = *self
            .kernel
            .get(kernel_index)
            .ok_or(Error::KernelIndexOutOfRange)?;
        let shift = group.mul(self.generator, offset);
        let point = group.add(target, shift);
        let point = group.add(point, kernel_point);
        let projected = group.mul(point, self.h);
        Ok(Query {
            target,
            offset,
            kernel_index,
            point,
            projected,
        })
    }

    /// Verify every supplied projected logarithm and the full signed curve relation,
    /// then verify the recovered scalar against the public target.
    /// This does not compute factor-base logarithms or discover relations.
    pub fn recover<G: Group<Point = P>>(
        &self,
        group: &mut G,
        query: &Query<P>,
        terms: &[Term<P>],
    ) -> Result<Recovered, Error> {
        if self.query(group, query.target, query.offset, query.kernel_index)? != *query {
            return Err(Error::QueryContextMismatch);
        }
        let mut point_sum = group.identity();
        let mut log_sum = 0;
        for term in terms {
            if !group.contains(term.point) {
                return Err(Error::InvalidRelationPoint);
            }
            if term.log >= self.r {
                return Err(Error::InvalidProjectedLog);
            }
            let projection = group.mul(term.point, self.h);
            if group.mul(self.generator, term.log) != projection {
                return Err(Error::InvalidProjectedLog);
            }
            let (point, log) = if term.negative {
                (group.neg(term.point), (self.r - term.log) % self.r)
            } else {
                (term.point, term.log)
            };
            point_sum = group.add(point_sum, point);
            log_sum = (log_sum + log) % self.r;
        }
        if point_sum != query.point {
            return Err(Error::InvalidFullCurveRelation);
        }
        let scalar = (times_mod(self.h_inverse, log_sum, self.r) + self.r - query.offset) % self.r;
        if group.mul(self.generator, scalar) != query.target {
            return Err(Error::ScalarVerificationFailed);
        }
        Ok(Recovered {
            scalar,
            projected_log_sum: log_sum,
            relation_terms: terms.len(),
            direct_identity_hit: query.point == group.identity(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    struct Cyclic(u64);
    impl Group for Cyclic {
        type Point = u64;
        fn domain(&self) -> Vec<u64> {
            vec![self.0]
        }
        fn order(&self) -> u64 {
            self.0
        }
        fn identity(&self) -> u64 {
            0
        }
        fn contains(&self, p: u64) -> bool {
            p < self.0
        }
        fn add(&mut self, a: u64, b: u64) -> u64 {
            (a + b) % self.0
        }
        fn neg(&mut self, p: u64) -> u64 {
            (self.0 - p) % self.0
        }
        fn mul(&mut self, p: u64, k: u64) -> u64 {
            times_mod(p, k, self.0)
        }
    }

    #[test]
    fn kernel_randomization_covers_each_ambient_point_once_for_every_target() {
        for (r, h) in [(3, 1), (7, 5), (11, 4), (13, 12)] {
            let mut group = Cyclic(r * h);
            let law = Law::certify(&mut group, h, r, h, r % (r * h)).unwrap();
            for d in 0..r {
                let target = group.mul(h, d);
                let mut points = BTreeSet::new();
                for k in 0..r {
                    for j in 0..h as usize {
                        let query = law.query(&mut group, target, k, j).unwrap();
                        assert!(points.insert(query.point()));
                        assert_eq!(query.projected(), times_mod(h, query.point(), r * h));
                    }
                }
                assert_eq!(points, (0..r * h).collect());
            }
        }
    }

    #[test]
    fn omitting_kernel_randomization_reaches_only_the_prime_subgroup() {
        let mut group = Cyclic(35);
        let law = Law::certify(&mut group, 5, 7, 5, 7).unwrap();
        let points: BTreeSet<_> = (0..7)
            .map(|k| law.query(&mut group, 10, k, 0).unwrap().point())
            .collect();
        assert_eq!(points, [0, 5, 10, 15, 20, 25, 30].into_iter().collect());
    }

    #[test]
    fn verified_signed_relations_recover_scalars_from_independently_looked_up_logs() {
        let mut group = Cyclic(35);
        let law = Law::certify(&mut group, 5, 7, 5, 7).unwrap();
        for d in 0..7 {
            for k in 0..7 {
                for j in 0..5 {
                    let target = group.mul(5, d);
                    let query = law.query(&mut group, target, k, j).unwrap();
                    // 9 - 12 + last = R. Logs come from a separate exhaustive subgroup table.
                    let last = (query.point() + 35 + 12 - 9) % 35;
                    let terms: Vec<_> = [(9, false), (12, true), (last, false)]
                        .into_iter()
                        .map(|(point, negative)| {
                            let projection = group.mul(point, 5);
                            let log = (0..7).find(|&log| group.mul(5, log) == projection).unwrap();
                            Term {
                                point,
                                log,
                                negative,
                            }
                        })
                        .collect();
                    assert_eq!(law.recover(&mut group, &query, &terms).unwrap().scalar, d);
                }
            }
        }
    }

    #[test]
    fn invalid_logs_full_relations_targets_and_ranges_fail_closed() {
        let mut group = Cyclic(35);
        let law = Law::certify(&mut group, 5, 7, 5, 7).unwrap();
        let query = law.query(&mut group, 10, 1, 2).unwrap();
        assert_eq!(law.query(&mut group, 1, 0, 0), Err(Error::InvalidTarget));
        assert_eq!(
            law.query(&mut group, 10, 7, 0),
            Err(Error::OffsetOutOfRange)
        );
        assert_eq!(
            law.query(&mut group, 10, 0, 5),
            Err(Error::KernelIndexOutOfRange)
        );
        let point = query.point();
        let projection = group.mul(point, 5);
        let log = (0..7).find(|&v| group.mul(5, v) == projection).unwrap();
        assert_eq!(
            law.recover(
                &mut group,
                &query,
                &[Term {
                    point,
                    log: (log + 1) % 7,
                    negative: false
                }]
            ),
            Err(Error::InvalidProjectedLog)
        );
        assert_eq!(
            law.recover(&mut group, &query, &[]),
            Err(Error::InvalidFullCurveRelation)
        );
        assert_eq!(
            law.recover(
                &mut group,
                &query,
                &[Term {
                    point: 35,
                    log: 0,
                    negative: false
                }]
            ),
            Err(Error::InvalidRelationPoint)
        );
        let mut foreign = query.clone();
        foreign.point = (foreign.point + 1) % 35;
        assert_eq!(
            law.recover(&mut group, &foreign, &[]),
            Err(Error::QueryContextMismatch)
        );
    }

    #[test]
    fn identity_hit_is_verified_without_inventing_a_factor_base_relation() {
        let mut group = Cyclic(35);
        let law = Law::certify(&mut group, 5, 7, 5, 7).unwrap();
        let query = law.query(&mut group, 15, 4, 0).unwrap();
        assert_eq!(query.point(), 0);
        let answer = law.recover(&mut group, &query, &[]).unwrap();
        assert_eq!(answer.scalar, 3);
        assert!(answer.direct_identity_hit);
        assert_eq!(answer.relation_terms, 0);
    }

    #[test]
    fn a_certificate_cannot_be_reused_with_a_different_group_domain() {
        let mut group = Cyclic(35);
        let law = Law::certify(&mut group, 5, 7, 5, 7).unwrap();
        let query = law.query(&mut group, 10, 1, 2).unwrap();
        let mut other = Cyclic(70);
        assert_eq!(
            law.query(&mut other, 0, 0, 0),
            Err(Error::QueryContextMismatch)
        );
        assert_eq!(
            law.recover(&mut other, &query, &[]),
            Err(Error::QueryContextMismatch)
        );
    }

    struct NoncyclicKernel;
    impl Group for NoncyclicKernel {
        type Point = (u8, u8, u8);
        fn domain(&self) -> Vec<u64> {
            vec![7, 2, 2]
        }
        fn order(&self) -> u64 {
            28
        }
        fn identity(&self) -> Self::Point {
            (0, 0, 0)
        }
        fn contains(&self, p: Self::Point) -> bool {
            p.0 < 7 && p.1 < 2 && p.2 < 2
        }
        fn add(&mut self, p: Self::Point, q: Self::Point) -> Self::Point {
            ((p.0 + q.0) % 7, p.1 ^ q.1, p.2 ^ q.2)
        }
        fn neg(&mut self, p: Self::Point) -> Self::Point {
            ((7 - p.0) % 7, p.1, p.2)
        }
        fn mul(&mut self, p: Self::Point, k: u64) -> Self::Point {
            (
                (u64::from(p.0) * k % 7) as u8,
                p.1 * (k % 2) as u8,
                p.2 * (k % 2) as u8,
            )
        }
    }

    #[test]
    fn noncyclic_or_incompatible_kernel_and_invalid_orders_are_rejected() {
        assert_eq!(
            Law::certify(&mut NoncyclicKernel, (1, 0, 0), 7, 4, (0, 1, 0)).unwrap_err(),
            Error::KernelIsNotCyclicOfDeclaredOrder
        );
        assert_eq!(
            Law::certify(&mut Cyclic(49), 7, 7, 7, 7).unwrap_err(),
            Error::NonCoprimeOrders
        );
        assert_eq!(
            Law::certify(&mut Cyclic(35), 1, 7, 5, 7).unwrap_err(),
            Error::InvalidGenerator
        );
        assert_eq!(
            Law::certify(&mut Cyclic(35), 5, 7, 5, 1).unwrap_err(),
            Error::InvalidKernelGenerator
        );
        assert_eq!(
            Law::certify(&mut Cyclic(35), 5, 9, 5, 7).unwrap_err(),
            Error::UnsupportedPrimeOrder
        );
        assert_eq!(
            Law::certify(&mut Cyclic(36), 5, 7, 5, 7).unwrap_err(),
            Error::CardinalityMismatch
        );
    }

    #[test]
    fn a_complete_noncyclic_kernel_covers_the_group_and_recovers_every_target() {
        let kernel: Vec<_> = (0..2)
            .flat_map(|a| (0..2).map(move |b| (0, a, b)))
            .collect();
        let law = Law::certify_with_kernel(&mut NoncyclicKernel, (1, 0, 0), 7, 4, kernel.clone())
            .unwrap();
        assert_eq!(law.kernel(), kernel);
        for d in 0..7 {
            let mut seen = BTreeSet::new();
            for k in 0..7 {
                for index in 0..4 {
                    let query = law
                        .query(&mut NoncyclicKernel, (d, 0, 0), k, index)
                        .unwrap();
                    assert!(seen.insert(query.point()));
                    let log = (4 * u64::from(query.point().0)) % 7;
                    let recovered = law
                        .recover(
                            &mut NoncyclicKernel,
                            &query,
                            &[Term {
                                point: query.point(),
                                log,
                                negative: false,
                            }],
                        )
                        .unwrap();
                    assert_eq!(recovered.scalar, u64::from(d));
                }
            }
            let expected: BTreeSet<_> = (0..7)
                .flat_map(|x| (0..2).flat_map(move |a| (0..2).map(move |b| (x, a, b))))
                .collect();
            assert_eq!(seen, expected);
        }
    }

    #[test]
    fn incomplete_duplicate_or_non_kernel_censuses_are_rejected() {
        for bad in [vec![(0, 0, 0); 3], vec![(0, 0, 0); 4]] {
            assert_eq!(
                Law::certify_with_kernel(&mut NoncyclicKernel, (1, 0, 0), 7, 4, bad).unwrap_err(),
                Error::InvalidKernelEnumeration
            );
        }
        for bad in [(1, 1, 1), (0, 2, 0)] {
            assert_eq!(
                Law::certify_with_kernel(
                    &mut NoncyclicKernel,
                    (1, 0, 0),
                    7,
                    4,
                    vec![(0, 0, 0), (0, 0, 1), (0, 1, 0), bad]
                )
                .unwrap_err(),
                Error::InvalidKernelElement
            );
        }
        assert_eq!(
            Law::certify_with_kernel(
                &mut Cyclic(7 * (MAX_SUPPLIED_KERNEL_SIZE + 1)),
                MAX_SUPPLIED_KERNEL_SIZE + 1,
                7,
                MAX_SUPPLIED_KERNEL_SIZE + 1,
                vec![]
            )
            .unwrap_err(),
            Error::UnsupportedKernelSize
        );
    }

    #[test]
    fn supplied_kernel_supports_cofactor_4232_without_claiming_a_curve_certificate() {
        let (r, h) = (991, 4232);
        let mut group = Cyclic(r * h);
        let kernel: Vec<_> = (0..h).map(|i| i * r).collect();
        let law = Law::certify_with_kernel(&mut group, h, r, h, kernel.clone()).unwrap();
        assert_eq!(law.kernel(), kernel);
        let constructed = Law::certify_from_points(&mut group, h, r, h, &[1]).unwrap();
        assert_eq!(constructed.kernel(), law.kernel());
        let law = constructed;
        let query = law.query(&mut group, h * 123, 456, h as usize - 1).unwrap();
        let recovered = law
            .recover(
                &mut group,
                &query,
                &[Term {
                    point: query.point(),
                    log: query.point() % r,
                    negative: false,
                }],
            )
            .unwrap();
        assert_eq!(recovered.scalar, 123);
    }

    #[test]
    fn projected_generators_construct_a_noncyclic_kernel_in_deterministic_order() {
        let candidates = [(2, 1, 0), (3, 0, 1), (0, 1, 0)];
        let law =
            Law::certify_from_points(&mut NoncyclicKernel, (1, 0, 0), 7, 4, &candidates).unwrap();
        assert_eq!(law.kernel(), [(0, 0, 0), (0, 1, 0), (0, 0, 1), (0, 1, 1)]);
        let reversed = Law::certify_from_points(
            &mut NoncyclicKernel,
            (1, 0, 0),
            7,
            4,
            &[(3, 0, 1), (2, 1, 0)],
        )
        .unwrap();
        assert_eq!(
            reversed.kernel(),
            [(0, 0, 0), (0, 0, 1), (0, 1, 0), (0, 1, 1)]
        );
        assert_eq!(
            law.kernel().iter().copied().collect::<BTreeSet<_>>(),
            reversed.kernel().iter().copied().collect()
        );
    }

    #[test]
    fn projected_generators_construct_multiple_cosets_and_the_trivial_kernel() {
        let law = Law::certify_from_points(&mut Cyclic(35), 5, 7, 5, &[2]).unwrap();
        assert_eq!(law.kernel(), [0, 14, 28, 7, 21]);
        let extended = Law::certify_from_points(&mut Cyclic(84), 12, 7, 12, &[3, 1]).unwrap();
        assert_eq!(
            extended.kernel(),
            [0, 21, 42, 63, 7, 28, 49, 70, 14, 35, 56, 77]
        );
        let trivial = Law::certify_from_points(&mut Cyclic(7), 1, 7, 1, &[]).unwrap();
        assert_eq!(trivial.kernel(), [0]);
    }

    #[test]
    fn an_incomplete_projected_span_is_not_a_complete_kernel_certificate() {
        for candidates in [vec![], vec![(1, 0, 0)], vec![(2, 1, 0), (3, 1, 0)]] {
            let expected_size = if candidates.last().is_some_and(|p| p.1 == 1) {
                2
            } else {
                1
            };
            assert_eq!(
                Law::certify_from_points(&mut NoncyclicKernel, (1, 0, 0), 7, 4, &candidates)
                    .unwrap_err(),
                Error::IncompleteKernelSpan {
                    found: expected_size,
                    expected: 4
                }
            );
        }
        // Invalid trailing input cannot disappear behind an earlier complete span.
        assert_eq!(
            Law::certify_from_points(&mut Cyclic(35), 5, 7, 5, &[1, 35]).unwrap_err(),
            Error::InvalidKernelElement
        );
        assert_eq!(
            Law::certify_from_points(
                &mut Cyclic(35),
                5,
                7,
                5,
                &vec![0; MAX_SUPPLIED_KERNEL_SIZE as usize + 1]
            )
            .unwrap_err(),
            Error::TooManyKernelCandidates
        );
    }
}
