// Comparison-only copy of the already committed projective/batch kernel.
//
// Kept separate so the public-precommit m=5 benchmark source remains byte
// identical. The comparison manifest pins this file independently.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::common::Fixture;
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
fn negate(point: PrimePoint, modulus: u64) -> PrimePoint {
    if point.infinity || point.y == 0 {
        point
    } else {
        PrimePoint::affine(point.x, modulus - point.y)
    }
}
