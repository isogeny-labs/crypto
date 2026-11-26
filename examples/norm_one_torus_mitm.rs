//! Five-term meet-in-the-middle decomposition on the production Prime31
//! norm-one-torus/Dickson factor base.
//!
//! The control converts every projective sum to affine with its own inversion.
//! The candidate uses Montgomery batch inversion for table construction and
//! fixed-size query blocks. Both return the five factor-base indices and those
//! indices are replayed with the library's separate affine group law outside
//! the timed interval.

#[path = "support/norm_one_torus_mitm.rs"]
mod common;

use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::{Duration, Instant};

use common::{certificate_bundle, result_digest, verify_results, Fixture};
use crypto_lib::cryptanalysis::ic_boundary::PrimePoint;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct OpCounts {
    multiplications: u64,
    squarings: u64,
    inversions: u64,
    additions: u64,
    doublings: u64,
}

impl OpCounts {
    #[inline]
    fn mul(&mut self, left: u64, right: u64, modulus: u64) -> u64 {
        self.multiplications += 1;
        mul_mod(left, right, modulus)
    }

    #[inline]
    fn square(&mut self, value: u64, modulus: u64) -> u64 {
        self.squarings += 1;
        mul_mod(value, value, modulus)
    }

    #[inline]
    fn inverse(&mut self, value: u64, modulus: u64) -> u64 {
        assert_ne!(value, 0);
        self.inversions += 1;
        inverse_mod(value, modulus)
    }
}

#[inline]
fn mul_mod(left: u64, right: u64, modulus: u64) -> u64 {
    if modulus == 2_147_483_647 {
        let product = left * right;
        let folded = (product & modulus) + (product >> 31);
        let folded = (folded & modulus) + (folded >> 31);
        if folded >= modulus {
            folded - modulus
        } else {
            folded
        }
    } else {
        ((u128::from(left) * u128::from(right)) % u128::from(modulus)) as u64
    }
}

#[inline]
fn add_mod(left: u64, right: u64, modulus: u64) -> u64 {
    let sum = left + right;
    if sum >= modulus {
        sum - modulus
    } else {
        sum
    }
}

#[inline]
fn sub_mod(left: u64, right: u64, modulus: u64) -> u64 {
    if left >= right {
        left - right
    } else {
        left + modulus - right
    }
}

fn inverse_mod(value: u64, modulus: u64) -> u64 {
    let (mut old_r, mut r) = (i128::from(value), i128::from(modulus));
    let (mut old_s, mut s) = (1i128, 0i128);
    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }
    assert_eq!(old_r, 1);
    old_s.rem_euclid(i128::from(modulus)) as u64
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Jacobian {
    x: u64,
    y: u64,
    z: u64,
}

impl Jacobian {
    const INFINITY: Self = Self { x: 0, y: 1, z: 0 };

    fn from_affine(point: PrimePoint) -> Self {
        if point.infinity {
            Self::INFINITY
        } else {
            Self {
                x: point.x,
                y: point.y,
                z: 1,
            }
        }
    }

    fn double(self, curve_a: u64, modulus: u64, ops: &mut OpCounts) -> Self {
        ops.doublings += 1;
        if self.z == 0 || self.y == 0 {
            return Self::INFINITY;
        }
        let xx = ops.square(self.x, modulus);
        let yy = ops.square(self.y, modulus);
        let yyyy = ops.square(yy, modulus);
        let x_plus_yy = add_mod(self.x, yy, modulus);
        let mut d = sub_mod(
            sub_mod(ops.square(x_plus_yy, modulus), xx, modulus),
            yyyy,
            modulus,
        );
        d = add_mod(d, d, modulus);
        let zz = ops.square(self.z, modulus);
        let zzzz = ops.square(zz, modulus);
        let three_xx = add_mod(add_mod(xx, xx, modulus), xx, modulus);
        let e = add_mod(three_xx, ops.mul(curve_a, zzzz, modulus), modulus);
        let f = ops.square(e, modulus);
        let x3 = sub_mod(f, add_mod(d, d, modulus), modulus);
        let eight_yyyy = (0..3).fold(yyyy, |value, _| add_mod(value, value, modulus));
        let y3 = sub_mod(
            ops.mul(e, sub_mod(d, x3, modulus), modulus),
            eight_yyyy,
            modulus,
        );
        let yz = ops.mul(self.y, self.z, modulus);
        let z3 = add_mod(yz, yz, modulus);
        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    fn add_mixed(self, other: PrimePoint, curve_a: u64, modulus: u64, ops: &mut OpCounts) -> Self {
        ops.additions += 1;
        if other.infinity {
            return self;
        }
        if self.z == 0 {
            return Self::from_affine(other);
        }
        let z1z1 = ops.square(self.z, modulus);
        let u2 = ops.mul(other.x, z1z1, modulus);
        let z1_cubed = ops.mul(self.z, z1z1, modulus);
        let s2 = ops.mul(other.y, z1_cubed, modulus);
        let h = sub_mod(u2, self.x, modulus);
        let delta = sub_mod(s2, self.y, modulus);
        if h == 0 {
            return if delta == 0 {
                self.double(curve_a, modulus, ops)
            } else {
                Self::INFINITY
            };
        }
        let hh = ops.square(h, modulus);
        let i = (0..2).fold(hh, |value, _| add_mod(value, value, modulus));
        let j = ops.mul(h, i, modulus);
        let r = add_mod(delta, delta, modulus);
        let v = ops.mul(self.x, i, modulus);
        let x3 = sub_mod(
            sub_mod(ops.square(r, modulus), j, modulus),
            add_mod(v, v, modulus),
            modulus,
        );
        let y1j = ops.mul(self.y, j, modulus);
        let y1j2 = add_mod(y1j, y1j, modulus);
        let y3 = sub_mod(ops.mul(r, sub_mod(v, x3, modulus), modulus), y1j2, modulus);
        let z_plus_h = add_mod(self.z, h, modulus);
        let z3 = sub_mod(
            sub_mod(ops.square(z_plus_h, modulus), z1z1, modulus),
            hh,
            modulus,
        );
        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    fn to_affine(self, modulus: u64, ops: &mut OpCounts) -> PrimePoint {
        if self.z == 0 {
            return PrimePoint::INFINITY;
        }
        from_inverse_z(self, ops.inverse(self.z, modulus), modulus, ops)
    }
}

fn from_inverse_z(point: Jacobian, inverse_z: u64, modulus: u64, ops: &mut OpCounts) -> PrimePoint {
    let inverse_z2 = ops.square(inverse_z, modulus);
    let x = ops.mul(point.x, inverse_z2, modulus);
    let inverse_z3 = ops.mul(inverse_z2, inverse_z, modulus);
    let y = ops.mul(point.y, inverse_z3, modulus);
    PrimePoint::affine(x, y)
}

fn batch_invert(values: &mut [u64], modulus: u64, ops: &mut OpCounts) {
    match values.len() {
        0 => return,
        1 => {
            values[0] = ops.inverse(values[0], modulus);
            return;
        }
        _ => {}
    }
    let mut prefixes = Vec::with_capacity(values.len());
    prefixes.push(values[0]);
    for &value in &values[1..] {
        prefixes.push(ops.mul(*prefixes.last().unwrap(), value, modulus));
    }
    let mut inverse = ops.inverse(*prefixes.last().unwrap(), modulus);
    for index in (1..values.len()).rev() {
        let value = values[index];
        values[index] = ops.mul(inverse, prefixes[index - 1], modulus);
        inverse = ops.mul(inverse, value, modulus);
    }
    values[0] = inverse;
}

fn normalize_batch<W: Copy>(
    states: &[(Jacobian, W)],
    modulus: u64,
    ops: &mut OpCounts,
) -> Vec<(PrimePoint, W)> {
    let finite: Vec<usize> = states
        .iter()
        .enumerate()
        .filter_map(|(index, (point, _))| (point.z != 0).then_some(index))
        .collect();
    let mut inverses: Vec<u64> = finite.iter().map(|&index| states[index].0.z).collect();
    batch_invert(&mut inverses, modulus, ops);
    let mut inverse_at = vec![0u64; states.len()];
    for (&index, inverse) in finite.iter().zip(inverses) {
        inverse_at[index] = inverse;
    }
    states
        .iter()
        .enumerate()
        .map(|(index, &(point, witness))| {
            let affine = if point.z == 0 {
                PrimePoint::INFINITY
            } else {
                from_inverse_z(point, inverse_at[index], modulus, ops)
            };
            (affine, witness)
        })
        .collect()
}

fn normalize_individual<W: Copy>(
    states: &[(Jacobian, W)],
    modulus: u64,
    ops: &mut OpCounts,
) -> Vec<(PrimePoint, W)> {
    states
        .iter()
        .map(|&(point, witness)| (point.to_affine(modulus, ops), witness))
        .collect()
}

fn pair_states(fixture: &Fixture, ops: &mut OpCounts) -> Vec<(Jacobian, [u16; 2])> {
    let mut states = Vec::new();
    for first in 0..fixture.base.len() {
        let left = Jacobian::from_affine(fixture.base[first]);
        for second in first..fixture.base.len() {
            states.push((
                left.add_mixed(
                    fixture.base[second],
                    fixture.instance.curve.a,
                    fixture.instance.curve.p,
                    ops,
                ),
                [first as u16, second as u16],
            ));
        }
    }
    states
}

fn negated_witness(mut witness: [u16; 3], neg_index: &[usize]) -> [u16; 3] {
    for index in &mut witness {
        *index = neg_index[usize::from(*index)] as u16;
    }
    witness.sort_unstable();
    witness
}

fn triple_states(
    fixture: &Fixture,
    signed_quotient: bool,
    ops: &mut OpCounts,
) -> Vec<(Jacobian, [u16; 3])> {
    let mut states = Vec::new();
    for first in 0..fixture.base.len() {
        let left = Jacobian::from_affine(fixture.base[first]);
        for second in first..fixture.base.len() {
            let pair = left.add_mixed(
                fixture.base[second],
                fixture.instance.curve.a,
                fixture.instance.curve.p,
                ops,
            );
            for third in second..fixture.base.len() {
                let witness = [first as u16, second as u16, third as u16];
                if signed_quotient && witness > negated_witness(witness, &fixture.neg_index) {
                    continue;
                }
                states.push((
                    pair.add_mixed(
                        fixture.base[third],
                        fixture.instance.curve.a,
                        fixture.instance.curve.p,
                        ops,
                    ),
                    witness,
                ));
            }
        }
    }
    states
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct PointKey {
    x: u64,
    y: u64,
    infinity: bool,
}

impl From<PrimePoint> for PointKey {
    fn from(point: PrimePoint) -> Self {
        Self {
            x: point.x,
            y: point.y,
            infinity: point.infinity,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Normalization {
    Individual,
    BatchSignedQuotient,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct XKey {
    x: u64,
    infinity: bool,
}

impl From<PrimePoint> for XKey {
    fn from(point: PrimePoint) -> Self {
        Self {
            x: point.x,
            infinity: point.infinity,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct QuotientEntry {
    point: PrimePoint,
    witness: [u16; 3],
}

enum RightTable {
    Full(HashMap<PointKey, [u16; 3]>),
    SignedQuotient(HashMap<XKey, QuotientEntry>),
}

struct Solver {
    left: Vec<(PrimePoint, [u16; 2])>,
    right: RightTable,
    normalization: Normalization,
    neg_index: Vec<usize>,
    curve_a: u64,
    modulus: u64,
}

impl Solver {
    fn new(fixture: &Fixture, normalization: Normalization) -> (Self, OpCounts) {
        let mut ops = OpCounts::default();
        let left_states = pair_states(fixture, &mut ops);
        let signed_quotient = matches!(normalization, Normalization::BatchSignedQuotient);
        let right_states = triple_states(fixture, signed_quotient, &mut ops);
        let left = match normalization {
            Normalization::Individual => {
                normalize_individual(&left_states, fixture.instance.curve.p, &mut ops)
            }
            Normalization::BatchSignedQuotient => {
                normalize_batch(&left_states, fixture.instance.curve.p, &mut ops)
            }
        };
        let right_points = match normalization {
            Normalization::Individual => {
                normalize_individual(&right_states, fixture.instance.curve.p, &mut ops)
            }
            Normalization::BatchSignedQuotient => {
                normalize_batch(&right_states, fixture.instance.curve.p, &mut ops)
            }
        };
        let right = match normalization {
            Normalization::Individual => {
                let mut table = HashMap::with_capacity(right_points.len());
                for (point, witness) in right_points {
                    table.entry(point.into()).or_insert(witness);
                }
                RightTable::Full(table)
            }
            Normalization::BatchSignedQuotient => {
                let mut table = HashMap::with_capacity(right_points.len());
                for (point, witness) in right_points {
                    table
                        .entry(point.into())
                        .or_insert(QuotientEntry { point, witness });
                }
                RightTable::SignedQuotient(table)
            }
        };
        (
            Self {
                left,
                right,
                normalization,
                neg_index: fixture.neg_index.clone(),
                curve_a: fixture.instance.curve.a,
                modulus: fixture.instance.curve.p,
            },
            ops,
        )
    }

    fn decomposition(
        &self,
        target: PrimePoint,
        block_size: usize,
        ops: &mut OpCounts,
    ) -> Option<Vec<usize>> {
        match self.normalization {
            Normalization::Individual => {
                let RightTable::Full(right) = &self.right else {
                    unreachable!()
                };
                for &(left, left_witness) in &self.left {
                    let complement = Jacobian::from_affine(target)
                        .add_mixed(negate(left, self.modulus), self.curve_a, self.modulus, ops)
                        .to_affine(self.modulus, ops);
                    if let Some(right_witness) = right.get(&complement.into()) {
                        return Some(
                            left_witness
                                .into_iter()
                                .chain(*right_witness)
                                .map(usize::from)
                                .collect(),
                        );
                    }
                }
                None
            }
            Normalization::BatchSignedQuotient => {
                let RightTable::SignedQuotient(right) = &self.right else {
                    unreachable!()
                };
                for block in self.left.chunks(block_size) {
                    let complements: Vec<_> = block
                        .iter()
                        .map(|&(left, witness)| {
                            (
                                Jacobian::from_affine(target).add_mixed(
                                    negate(left, self.modulus),
                                    self.curve_a,
                                    self.modulus,
                                    ops,
                                ),
                                witness,
                            )
                        })
                        .collect();
                    for ((point, left_witness), _) in
                        normalize_batch(&complements, self.modulus, ops)
                            .into_iter()
                            .zip(block)
                    {
                        if let Some(entry) = right.get(&point.into()) {
                            let right_witness = if point == entry.point {
                                entry.witness
                            } else {
                                assert_eq!(point, negate(entry.point, self.modulus));
                                negated_witness(entry.witness, &self.neg_index)
                            };
                            return Some(
                                left_witness
                                    .into_iter()
                                    .chain(right_witness)
                                    .map(usize::from)
                                    .collect(),
                            );
                        }
                    }
                }
                None
            }
        }
    }
}

fn negate(point: PrimePoint, modulus: u64) -> PrimePoint {
    if point.infinity || point.y == 0 {
        point
    } else {
        PrimePoint::affine(point.x, modulus - point.y)
    }
}

struct TimedRun {
    setup: Duration,
    query: Duration,
    setup_ops: OpCounts,
    query_ops: OpCounts,
    results: Vec<Option<Vec<usize>>>,
}

fn time_solver(fixture: &Fixture, block_size: usize, normalization: Normalization) -> TimedRun {
    let started = Instant::now();
    let (solver, setup_ops) = Solver::new(fixture, normalization);
    let setup = started.elapsed();
    let started = Instant::now();
    let mut query_ops = OpCounts::default();
    let results = fixture
        .targets
        .iter()
        .copied()
        .map(|target| solver.decomposition(target, block_size, &mut query_ops))
        .collect::<Vec<_>>();
    let query = started.elapsed();
    assert!(verify_results(fixture, &results));
    TimedRun {
        setup,
        query,
        setup_ops,
        query_ops,
        results,
    }
}

fn print_run(round: usize, label: &str, run: &TimedRun) {
    let total = run.setup + run.query;
    println!(
        "{round},{label},{:.3},{:.3},{:.3},{},{},{},{},{},{},{},{},{},{}",
        run.setup.as_secs_f64() * 1_000.0,
        run.query.as_secs_f64() * 1_000.0,
        total.as_secs_f64() * 1_000.0,
        run.setup_ops.inversions,
        run.query_ops.inversions,
        run.setup_ops.multiplications,
        run.query_ops.multiplications,
        run.setup_ops.squarings,
        run.query_ops.squarings,
        run.setup_ops.additions,
        run.query_ops.additions,
        run.results.iter().filter(|result| result.is_some()).count(),
        result_digest(&run.results),
    );
}

fn result_mask(results: &[Option<Vec<usize>>]) -> Vec<bool> {
    results.iter().map(Option::is_some).collect()
}

fn write_certificate(path: &str, bytes: &[u8]) {
    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(mut output) => output.write_all(bytes).expect("write certificate"),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            assert_eq!(fs::read(path).expect("read retained certificate"), bytes);
        }
        Err(error) => panic!("create certificate {path}: {error}"),
    }
}

fn main() {
    let arguments: Vec<_> = std::env::args().collect();
    assert_eq!(
        arguments.len(),
        5,
        "usage: norm_one_torus_mitm --manifest PATH --certificates PATH"
    );
    assert_eq!(arguments[1], "--manifest");
    assert_eq!(arguments[3], "--certificates");
    let (manifest, manifest_bytes) = common::read_manifest(&arguments[2]);
    let fixture = common::load_fixture(&manifest);
    let configuration = &manifest.configuration;

    let warm_individual = time_solver(
        &fixture,
        configuration.query_block_size,
        Normalization::Individual,
    );
    let warm_batch = time_solver(
        &fixture,
        configuration.query_block_size,
        Normalization::BatchSignedQuotient,
    );
    let expected_mask = result_mask(&warm_individual.results);
    assert_eq!(expected_mask, result_mask(&warm_batch.results));

    println!(
        "round,implementation,setup_ms,query_ms,total_ms,setup_inversions,query_inversions,setup_multiplications,query_multiplications,setup_squarings,query_squarings,setup_additions,query_additions,successes,result_digest"
    );
    let mut expected_batch = None;
    for round in 0..configuration.rounds {
        let order = if round % 2 == 0 {
            [
                Normalization::Individual,
                Normalization::BatchSignedQuotient,
            ]
        } else {
            [
                Normalization::BatchSignedQuotient,
                Normalization::Individual,
            ]
        };
        for normalization in order {
            let label = match normalization {
                Normalization::Individual => "individual",
                Normalization::BatchSignedQuotient => "batch-quotient",
            };
            let run = time_solver(&fixture, configuration.query_block_size, normalization);
            assert_eq!(expected_mask, result_mask(&run.results));
            if matches!(normalization, Normalization::BatchSignedQuotient) {
                if let Some(reference) = &expected_batch {
                    assert_eq!(&run.results, reference);
                } else {
                    expected_batch = Some(run.results.clone());
                }
            }
            print_run(round, label, &run);
        }
        for suffix in ["batch-a", "batch-b"] {
            let run = time_solver(
                &fixture,
                configuration.query_block_size,
                Normalization::BatchSignedQuotient,
            );
            assert_eq!(Some(&run.results), expected_batch.as_ref());
            print_run(round, suffix, &run);
        }
    }
    let expected = expected_batch.expect("at least one candidate round");
    assert!(
        expected.iter().filter(|result| result.is_some()).count() >= configuration.planted_targets
    );
    let bundle = certificate_bundle(&fixture, &manifest_bytes, &expected);
    let bytes = serde_json::to_vec_pretty(&bundle).expect("serialize certificates");
    write_certificate(&arguments[4], &bytes);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    use crypto_lib::cryptanalysis::ic_boundary::{
        fp_p31_subexp_mersenne_instance, CountedGroup, GroupOps,
    };
    use crypto_lib::cryptanalysis::ic_framework::pkm_semaev::dickson_factor_base;

    fn production_fixture() -> Fixture {
        let instance = fp_p31_subexp_mersenne_instance();
        let factor_base = dickson_factor_base(&instance, 6, 0, 0).unwrap();
        Fixture {
            instance,
            base: factor_base.points,
            neg_index: factor_base.neg_index,
            abscissae: factor_base.abscissae,
            targets: Vec::new(),
        }
    }

    #[test]
    fn projective_group_law_and_batch_normalization_match_affine() {
        let instance = fp_p31_subexp_mersenne_instance();
        let curve = &instance.curve;
        let generator = instance.generator_point();
        let points: Vec<_> = (1..=12)
            .map(|scalar| curve.mul(&mut GroupOps::default(), generator, scalar))
            .collect();
        let mut projective = Vec::new();
        let mut individual_ops = OpCounts::default();
        for (index, &left) in points.iter().enumerate() {
            let right = points[(index * 7 + 3) % points.len()];
            let sum =
                Jacobian::from_affine(left).add_mixed(right, curve.a, curve.p, &mut individual_ops);
            assert_eq!(
                sum.to_affine(curve.p, &mut individual_ops),
                curve.add(&mut GroupOps::default(), left, right)
            );
            projective.push((sum, index));
        }
        projective.push((Jacobian::INFINITY, points.len()));
        let mut batch_ops = OpCounts::default();
        let batch = normalize_batch(&projective, curve.p, &mut batch_ops);
        for ((point, witness), (state, expected_witness)) in batch.iter().zip(&projective) {
            assert_eq!(witness, expected_witness);
            assert_eq!(*point, state.to_affine(curve.p, &mut OpCounts::default()));
        }
        assert_eq!(batch_ops.inversions, 1);
    }

    #[test]
    fn signed_quotient_covers_the_complete_three_sum_table() {
        let fixture = production_fixture();
        let full = triple_states(&fixture, false, &mut OpCounts::default());
        let quotient = triple_states(&fixture, true, &mut OpCounts::default());
        assert_eq!(quotient.len() * 2, full.len());

        let quotient_points = normalize_batch(
            &quotient,
            fixture.instance.curve.p,
            &mut OpCounts::default(),
        );
        let quotient_keys: HashSet<_> = quotient_points
            .iter()
            .map(|(point, _)| XKey::from(*point))
            .collect();
        let full_points =
            normalize_batch(&full, fixture.instance.curve.p, &mut OpCounts::default());
        assert!(full_points
            .iter()
            .all(|(point, _)| quotient_keys.contains(&XKey::from(*point))));
    }
}
