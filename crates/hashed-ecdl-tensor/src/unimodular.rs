//! Division-free unimodular crossing-factor contraction (`EXACT.md` §§1–4) and
//! parsimonious mod-2 parity pin-sweep witness extraction (`FOLLOWUP.md` §4).
//!
//! For `lambda`-bit crossing strings `(x, y, u, v)` with `F(x, y, u, v) = [x = v][y = u]`,
//! expanding `F^{⊗ lambda} = sum_{j = 0}^{lambda} T_j` (`T_j = [z^j](I + z D)^{⊗ lambda}`)
//! admits integral unimodular bases `B_r` (row flattening `(x, y) | (u, v)`) and `B_c`
//! (column flattening `(x, u) | (y, v)`) of determinant `-1` whose surviving column counts
//! `A_j` and `A_{lambda - j}` satisfy `c_j + c_{lambda - j} <= floor(4 * rho * lambda) + 2`
//! with `rho = log_2(12) / 4 < 1`. Because `B_r^{-1}` and `B_c^{-1}` have entries in
//! `{-1, 0, +1}`, the factorization holds directly in `Z / 2^{s+1} Z` and `Z / 2 Z` (`F_2`)
//! with zero divisions.

use crate::network::Network;
use crate::{integrity, invalid, overflow, BigInt, BigUint, Meter, Result};
use num_traits::{One, ToPrimitive, Zero};

/// Even residue ring `Z / 2^precision Z` (`precision = 1` is `F_2` parity;
/// `precision = s + 1` recovers exact `s`-edge Boolean counts without overflow).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mod2k {
    precision: usize,
    modulus: BigUint,
    mask: BigUint,
}

/// Canonical residue in `Z / 2^precision Z`, using inline `u128` for `precision <= 128`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Residue2k {
    /// Fast exact word representation for `precision <= 128`.
    Small(u128),
    /// Arbitrary-precision representation for `precision > 128`.
    Large(BigUint),
}

impl Mod2k {
    /// Construct `Z / 2^precision Z` for `precision >= 1`.
    pub fn new(precision: usize, meter: &Meter) -> Result<Self> {
        if precision == 0 {
            return Err(invalid("Mod2k precision must be positive"));
        }
        meter.bits(precision, "Mod2k ring precision")?;
        let modulus = BigUint::one() << precision;
        let mask = &modulus - BigUint::one();
        Ok(Self {
            precision,
            modulus,
            mask,
        })
    }

    /// Modulus bit precision `p` in `Z / 2^p Z`.
    pub fn precision(&self) -> usize {
        self.precision
    }

    #[inline]
    fn small_mask(&self) -> u128 {
        if self.precision == 128 {
            u128::MAX
        } else {
            (1u128 << self.precision) - 1
        }
    }

    /// Additive identity `0`.
    pub fn zero(&self) -> Residue2k {
        if self.precision <= 128 {
            Residue2k::Small(0)
        } else {
            Residue2k::Large(BigUint::zero())
        }
    }

    /// Multiplicative identity `1`.
    pub fn one(&self) -> Residue2k {
        if self.precision <= 128 {
            Residue2k::Small(1 & self.small_mask())
        } else {
            Residue2k::Large(BigUint::one())
        }
    }

    /// Reduce a small signed integer (`i64`) into `Z / 2^precision Z` without division.
    pub fn from_i64(&self, value: i64) -> Residue2k {
        if self.precision <= 128 {
            let cast = (value as i128) as u128;
            Residue2k::Small(cast & self.small_mask())
        } else if value >= 0 {
            Residue2k::Large(BigUint::from(value as u64) & &self.mask)
        } else {
            let mag = BigUint::from(value.unsigned_abs()) & &self.mask;
            if mag.is_zero() {
                Residue2k::Large(BigUint::zero())
            } else {
                Residue2k::Large(&self.modulus - mag)
            }
        }
    }

    /// Reduce a signed `BigInt` into `Z / 2^precision Z`.
    pub fn from_bigint(&self, value: &BigInt) -> Residue2k {
        let canonical = crate::arithmetic::residue(value, &self.modulus);
        if self.precision <= 128 {
            let mut low = 0u128;
            for (i, byte) in canonical.to_bytes_le().into_iter().take(16).enumerate() {
                low |= u128::from(byte) << (8 * i);
            }
            Residue2k::Small(low & self.small_mask())
        } else {
            Residue2k::Large(canonical)
        }
    }

    /// Convert a canonical residue to `BigUint` in `[0, 2^precision)`.
    pub fn to_biguint(&self, r: &Residue2k) -> BigUint {
        match r {
            Residue2k::Small(v) => BigUint::from(*v),
            Residue2k::Large(v) => v.clone(),
        }
    }

    /// Extract the least-significant bit (`value mod 2`).
    pub fn parity(&self, r: &Residue2k) -> bool {
        match r {
            Residue2k::Small(v) => (v & 1) != 0,
            Residue2k::Large(v) => v.bit(0),
        }
    }

    /// Test whether `r == 0 (mod 2^precision)`.
    pub fn is_zero(&self, r: &Residue2k) -> bool {
        match r {
            Residue2k::Small(v) => *v == 0,
            Residue2k::Large(v) => v.is_zero(),
        }
    }

    /// Exact addition modulo `2^precision`.
    pub fn add(&self, a: &Residue2k, b: &Residue2k) -> Residue2k {
        match (a, b) {
            (Residue2k::Small(u), Residue2k::Small(v)) => {
                Residue2k::Small(u.wrapping_add(*v) & self.small_mask())
            }
            (Residue2k::Large(u), Residue2k::Large(v)) => Residue2k::Large((u + v) & &self.mask),
            _ => unreachable!("Mod2k representation fixed by precision"),
        }
    }

    /// Exact subtraction modulo `2^precision`.
    pub fn sub(&self, a: &Residue2k, b: &Residue2k) -> Residue2k {
        match (a, b) {
            (Residue2k::Small(u), Residue2k::Small(v)) => {
                Residue2k::Small(u.wrapping_sub(*v) & self.small_mask())
            }
            (Residue2k::Large(u), Residue2k::Large(v)) => {
                Residue2k::Large((u + &self.modulus - v) & &self.mask)
            }
            _ => unreachable!("Mod2k representation fixed by precision"),
        }
    }

    /// Exact multiplication modulo `2^precision`.
    pub fn mul(&self, a: &Residue2k, b: &Residue2k) -> Residue2k {
        match (a, b) {
            (Residue2k::Small(u), Residue2k::Small(v)) => {
                Residue2k::Small(u.wrapping_mul(*v) & self.small_mask())
            }
            (Residue2k::Large(u), Residue2k::Large(v)) => Residue2k::Large((u * v) & &self.mask),
            _ => unreachable!("Mod2k representation fixed by precision"),
        }
    }
}

/// Flattening orientation for the `lambda`-bit crossing tensor `F^{⊗ lambda}`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossingCut {
    /// Row flattening `(x, y) | (u, v)` using unimodular basis `B_r` (surviving count `A_j`).
    Row,
    /// Column flattening `(x, u) | (y, v)` using unimodular basis `B_c` (surviving count `A_{lambda - j}`).
    Column,
}

/// Exact integer `4 x 4` unimodular basis matrix `B_r` (`det(B_r) = -1`).
pub const B_ROW: [[i8; 4]; 4] = [
    [1, 0, 0, 0],
    [0, 0, 1, 1],
    [0, 0, 1, 0],
    [0, 1, 0, 0],
];

/// Exact integer inverse `B_r^{-1}` over `Z` (`B_r * B_r^{-1} = I_4`).
pub const B_ROW_INV: [[i8; 4]; 4] = [
    [1, 0, 0, 0],
    [0, 0, 0, 1],
    [0, 0, 1, 0],
    [0, 1, -1, 0],
];

/// Exact integer `4 x 4` unimodular basis matrix `B_c` (`det(B_c) = -1`).
pub const B_COL: [[i8; 4]; 4] = [
    [0, 0, 1, 1],
    [1, 0, 0, 0],
    [0, 1, 0, 0],
    [0, 0, -1, 0],
];

/// Exact integer inverse `B_c^{-1}` over `Z` (`B_c * B_c^{-1} = I_4`).
pub const B_COL_INV: [[i8; 4]; 4] = [
    [0, 1, 0, 0],
    [0, 0, 1, 0],
    [0, 0, 0, -1],
    [1, 0, 0, 1],
];

// Linear polynomial entries `(c_0, c_1)` representing `c_0 + c_1 * z` of `(I_r + z D_r) B_r`.
const P_ROW: [[(i8, i8); 4]; 4] = [
    [(1, 0), (0, 0), (0, 0), (0, 0)],
    [(0, 0), (0, 0), (1, 0), (1, -1)],
    [(0, 0), (0, 0), (1, 0), (0, 1)],
    [(0, 0), (1, 0), (0, 0), (0, 0)],
];

// Linear polynomial entries `(c_0, c_1)` representing `c_0 + c_1 * z` of `(I_c + z D_c) B_c`.
const P_COL: [[(i8, i8); 4]; 4] = [
    [(0, 0), (0, 0), (0, 1), (1, 0)],
    [(0, 0), (0, 1), (0, 0), (0, 0)],
    [(0, 1), (0, 0), (0, 0), (0, 0)],
    [(0, 0), (0, 0), (0, -1), (1, -1)],
];

// Direct 1-bit polynomial entries `(c_0, c_1)` of `I + z * D` in row flattening `(x, y) | (u, v)`.
const DIRECT_ROW: [[(i8, i8); 4]; 4] = [
    [(1, 0), (0, 0), (0, 0), (0, 0)],
    [(0, 0), (1, -1), (0, 1), (0, 0)],
    [(0, 0), (0, 1), (1, -1), (0, 0)],
    [(0, 0), (0, 0), (0, 0), (1, 0)],
];

// Direct 1-bit polynomial entries `(c_0, c_1)` of `I + z * D` in column flattening `(x, u) | (y, v)`.
const DIRECT_COL: [[(i8, i8); 4]; 4] = [
    [(1, 0), (0, 0), (0, 0), (1, -1)],
    [(0, 0), (0, 0), (0, 1), (0, 0)],
    [(0, 0), (0, 1), (0, 0), (0, 0)],
    [(1, -1), (0, 0), (0, 0), (1, 0)],
];

/// Compute `A_k(lambda) = sum_{h = k}^{lambda} C(lambda, h) * 3^{lambda - h}`.
pub fn surviving_state_count(lambda: usize, min_threes: usize) -> BigUint {
    if min_threes > lambda {
        return BigUint::zero();
    }
    let mut binom = BigUint::one();
    let mut total = BigUint::zero();
    let three = BigUint::from(3u8);
    for h in 0..=lambda {
        if h >= min_threes {
            let pow3 = three.pow((lambda - h) as u32);
            total += &binom * pow3;
        }
        if h < lambda {
            binom = (binom * BigUint::from(lambda - h)) / BigUint::from(h + 1);
        }
    }
    total
}

/// Padded binary state width `c = ceil(log_2(count))` (`0` when `count <= 1`).
pub fn padded_bit_width(count: &BigUint) -> usize {
    if count <= &BigUint::one() {
        0
    } else {
        (count - BigUint::one()).bits() as usize
    }
}

/// Check the two-cut width inequality `A_j * A_{lambda - j} <= 12^lambda` and
/// `c_j + c_{lambda - j} <= floor(4 * (log_2(12) / 4) * lambda) + 2` for all `j in 0..=lambda`.
pub fn check_crossing_width_bound(lambda: usize) -> bool {
    let twelve_pow = BigUint::from(12u8).pow(lambda as u32);
    let rhs_bits = ((lambda as f64) * 12.0f64.log2()).floor() as usize + 2;
    for j in 0..=lambda {
        let aj = surviving_state_count(lambda, j);
        let a_comp = surviving_state_count(lambda, lambda - j);
        if &aj * &a_comp > twelve_pow {
            return false;
        }
        let cj = padded_bit_width(&aj);
        let c_comp = padded_bit_width(&a_comp);
        if cj > 2 * lambda || c_comp > 2 * lambda || cj + c_comp > rhs_bits {
            return false;
        }
    }
    true
}

/// Precomputed binomial suffix count table for `O(lambda)` lexicographic ranking and
/// unranking of surviving basis words in `{0, 1, 2, 3}^lambda` (`EXACT.md` §1).
#[derive(Clone, Debug)]
pub struct BinomialSuffixTable {
    lambda: usize,
    // `counts[rem_len][need_threes]` = number of words in `{0..3}^rem_len` with `>= need_threes` 3s.
    counts: Vec<Vec<BigUint>>,
}

impl BinomialSuffixTable {
    /// Precompute suffix counts up to string length `lambda`.
    pub fn new(lambda: usize, meter: &mut Meter) -> Result<Self> {
        let side = lambda
            .checked_add(1)
            .ok_or_else(|| overflow("suffix table side"))?;
        meter.vector(
            side.checked_mul(side)
                .ok_or_else(|| overflow("suffix table entries"))?,
            "binomial suffix table",
        )?;
        let mut counts = vec![vec![BigUint::zero(); side + 1]; side];
        for len in 0..=lambda {
            for need in 0..=lambda {
                meter.charge(1, "binomial suffix entry")?;
                counts[len][need] = surviving_state_count(len, need);
            }
        }
        Ok(Self { lambda, counts })
    }

    /// Number of surviving words for `cut` and piece index `j in 0..=lambda`.
    pub fn state_count(&self, cut: CrossingCut, j: usize) -> &BigUint {
        let need = self.required_threes(cut, j);
        &self.counts[self.lambda][need]
    }

    /// Minimum number of `3` symbols required for `(cut, j)`.
    pub fn required_threes(&self, cut: CrossingCut, j: usize) -> usize {
        match cut {
            CrossingCut::Row => j,
            CrossingCut::Column => self.lambda - j,
        }
    }

    /// Unrank `state_index` into the `state_index`-th surviving symbol word in `{0, 1, 2, 3}^lambda`
    /// in lexicographic order, or `None` if `state_index >= A` (padded zero state).
    pub fn unrank(&self, cut: CrossingCut, j: usize, state_index: &BigUint) -> Option<Vec<u8>> {
        let mut need = self.required_threes(cut, j);
        if state_index >= &self.counts[self.lambda][need] {
            return None;
        }
        let mut idx = state_index.clone();
        let mut word = Vec::with_capacity(self.lambda);
        for pos in 0..self.lambda {
            let rem = self.lambda - 1 - pos;
            let mut chosen = 3u8;
            for sym in 0..4u8 {
                let next_need = if sym == 3 {
                    need.saturating_sub(1)
                } else {
                    need
                };
                let branch = &self.counts[rem][next_need];
                if &idx < branch {
                    chosen = sym;
                    need = next_need;
                    break;
                }
                idx -= branch;
            }
            word.push(chosen);
        }
        Some(word)
    }

    /// Inverse of [`Self::unrank`]: return the lexicographic index of `word` if it survives.
    pub fn rank(&self, cut: CrossingCut, j: usize, word: &[u8]) -> Option<BigUint> {
        if word.len() != self.lambda {
            return None;
        }
        let threes = word.iter().filter(|&&s| s == 3).count();
        let mut need = self.required_threes(cut, j);
        if threes < need {
            return None;
        }
        let mut idx = BigUint::zero();
        for (pos, &sym) in word.iter().enumerate() {
            if sym > 3 {
                return None;
            }
            let rem = self.lambda - 1 - pos;
            for smaller in 0..sym {
                let smaller_need = need; // `smaller < sym <= 3`, so `smaller != 3`.
                let _ = smaller;
                idx += &self.counts[rem][smaller_need];
            }
            if sym == 3 {
                need = need.saturating_sub(1);
            }
        }
        Some(idx)
    }
}

/// Evaluate the exact integer `[z^j]` coefficient of a product of `lambda` linear polynomials
/// `prod_{k = 0}^{lambda - 1} (c_{k, 0} + c_{k, 1} * z)` in `O(lambda^2)` operations.
fn linear_product_coeff_int(terms: &[(i8, i8)], j: usize) -> BigInt {
    if j > terms.len() {
        return BigInt::zero();
    }
    let mut poly = vec![BigInt::zero(); j + 1];
    poly[0] = BigInt::one();
    let mut deg = 0usize;
    for &(c0, c1) in terms {
        let next_deg = (deg + 1).min(j);
        for d in (0..=next_deg).rev() {
            let mut val = if d <= deg && c0 != 0 {
                &poly[d] * BigInt::from(c0)
            } else {
                BigInt::zero()
            };
            if d >= 1 && c1 != 0 {
                val += &poly[d - 1] * BigInt::from(c1);
            }
            poly[d] = val;
        }
        deg = next_deg;
    }
    poly[j].clone()
}

/// Evaluate the `[z^j]` coefficient of `prod_k (c_{k,0} + c_{k,1} * z)` directly in `Mod2k`.
fn linear_product_coeff_ring(ring: &Mod2k, terms: &[(i8, i8)], j: usize) -> Residue2k {
    if j > terms.len() {
        return ring.zero();
    }
    let mut poly = vec![ring.zero(); j + 1];
    poly[0] = ring.one();
    let mut deg = 0usize;
    for &(c0, c1) in terms {
        let next_deg = (deg + 1).min(j);
        for d in (0..=next_deg).rev() {
            let mut val = match c0 {
                0 => ring.zero(),
                1 => poly[d].clone(),
                -1 => ring.sub(&ring.zero(), &poly[d]),
                other => ring.mul(&poly[d], &ring.from_i64(i64::from(other))),
            };
            if d >= 1 && c1 != 0 {
                let shifted = match c1 {
                    1 => poly[d - 1].clone(),
                    -1 => ring.sub(&ring.zero(), &poly[d - 1]),
                    other => ring.mul(&poly[d - 1], &ring.from_i64(i64::from(other))),
                };
                val = ring.add(&val, &shifted);
            }
            poly[d] = val;
        }
        deg = next_deg;
    }
    poly[j].clone()
}

/// Left crossing-factor entry `L_{cut, j}(left_pairs, state_index)` over `Z`,
/// where `left_pairs[k] in 0..4` encodes `(x_k, y_k)` for `Row` or `(x_k, u_k)` for `Column`.
/// Padded indices `state_index >= A` evaluate to `0`.
pub fn left_factor_int(
    table: &BinomialSuffixTable,
    cut: CrossingCut,
    j: usize,
    left_pairs: &[u8],
    state_index: &BigUint,
) -> BigInt {
    let Some(symbols) = table.unrank(cut, j, state_index) else {
        return BigInt::zero();
    };
    let p_mat = match cut {
        CrossingCut::Row => &P_ROW,
        CrossingCut::Column => &P_COL,
    };
    let mut terms = Vec::with_capacity(symbols.len());
    for (&r, &s) in left_pairs.iter().zip(&symbols) {
        terms.push(p_mat[r as usize][s as usize]);
    }
    linear_product_coeff_int(&terms, j)
}

/// Right crossing-factor entry `R_{cut, j}(state_index, right_pairs)` over `Z` (`{-1, 0, +1}`),
/// where `right_pairs[k] in 0..4` encodes `(u_k, v_k)` for `Row` or `(y_k, v_k)` for `Column`.
/// Padded indices `state_index >= A` evaluate to `0`.
pub fn right_factor_int(
    table: &BinomialSuffixTable,
    cut: CrossingCut,
    j: usize,
    state_index: &BigUint,
    right_pairs: &[u8],
) -> BigInt {
    let Some(symbols) = table.unrank(cut, j, state_index) else {
        return BigInt::zero();
    };
    let inv_mat = match cut {
        CrossingCut::Row => &B_ROW_INV,
        CrossingCut::Column => &B_COL_INV,
    };
    let mut prod = 1i64;
    for (&s, &q) in symbols.iter().zip(right_pairs) {
        let entry = i64::from(inv_mat[s as usize][q as usize]);
        if entry == 0 {
            return BigInt::zero();
        }
        prod *= entry;
    }
    BigInt::from(prod)
}

/// Left crossing-factor entry directly in `Mod2k` (zero divisions).
pub fn left_factor_ring(
    ring: &Mod2k,
    table: &BinomialSuffixTable,
    cut: CrossingCut,
    j: usize,
    left_pairs: &[u8],
    state_index: &BigUint,
) -> Residue2k {
    let Some(symbols) = table.unrank(cut, j, state_index) else {
        return ring.zero();
    };
    let p_mat = match cut {
        CrossingCut::Row => &P_ROW,
        CrossingCut::Column => &P_COL,
    };
    let mut terms = Vec::with_capacity(symbols.len());
    for (&r, &s) in left_pairs.iter().zip(&symbols) {
        terms.push(p_mat[r as usize][s as usize]);
    }
    linear_product_coeff_ring(ring, &terms, j)
}

/// Right crossing-factor entry directly in `Mod2k` (zero divisions).
pub fn right_factor_ring(
    ring: &Mod2k,
    table: &BinomialSuffixTable,
    cut: CrossingCut,
    j: usize,
    state_index: &BigUint,
    right_pairs: &[u8],
) -> Residue2k {
    let Some(symbols) = table.unrank(cut, j, state_index) else {
        return ring.zero();
    };
    let inv_mat = match cut {
        CrossingCut::Row => &B_ROW_INV,
        CrossingCut::Column => &B_COL_INV,
    };
    let mut sign = 1i64;
    for (&s, &q) in symbols.iter().zip(right_pairs) {
        let entry = i64::from(inv_mat[s as usize][q as usize]);
        if entry == 0 {
            return ring.zero();
        }
        sign *= entry;
    }
    ring.from_i64(sign)
}

/// Direct unfactored piece entry `T_{cut, j}(left_pairs, right_pairs)` over `Z`.
pub fn direct_piece_int(cut: CrossingCut, j: usize, left_pairs: &[u8], right_pairs: &[u8]) -> BigInt {
    let mat = match cut {
        CrossingCut::Row => &DIRECT_ROW,
        CrossingCut::Column => &DIRECT_COL,
    };
    let terms: Vec<(i8, i8)> = left_pairs
        .iter()
        .zip(right_pairs)
        .map(|(&r, &q)| mat[r as usize][q as usize])
        .collect();
    linear_product_coeff_int(&terms, j)
}

/// Direct crossing indicator `F^{⊗ lambda}(left_pairs, right_pairs) in {0, 1}` for `cut`.
/// In both flattenings (`Row`: `(x,y)|(u,v)`, `Column`: `(x,u)|(y,v)`), `F` swaps `01 <-> 10`.
pub fn direct_crossing_bit(left_pairs: &[u8], right_pairs: &[u8]) -> bool {
    const SWAP: [u8; 4] = [0, 2, 1, 3];
    left_pairs
        .iter()
        .zip(right_pairs)
        .all(|(&r, &q)| SWAP[r as usize] == q)
}

/// Exhaustively verify all `2 * (lambda + 1) * 16^lambda` factored piece entries against
/// `direct_piece_int` (including padded `2^c` state sums!) and all `2 * 16^lambda` sums
/// `sum_j T_j == F^{⊗ lambda}` for `1 <= lambda <= max_lambda` (`EXACT.md` §1: `max_lambda = 3`
/// checks `34,368` piece entries and `8,736` crossing sums).
pub fn verify_bounded_crossing_identities(
    max_lambda: usize,
    meter: &mut Meter,
) -> Result<(u64, u64)> {
    let mut piece_checks = 0u64;
    let mut crossing_checks = 0u64;

    for lambda in 1..=max_lambda {
        let table = BinomialSuffixTable::new(lambda, meter)?;
        let ring_mod8 = Mod2k::new(3, meter)?;
        let ring_mod2 = Mod2k::new(1, meter)?;
        let word_space = 1usize << (2 * lambda);

        for cut in [CrossingCut::Row, CrossingCut::Column] {
            for left_code in 0..word_space {
                let left_pairs: Vec<u8> = (0..lambda)
                    .map(|k| ((left_code >> (2 * k)) & 3) as u8)
                    .collect();
                for right_code in 0..word_space {
                    let right_pairs: Vec<u8> = (0..lambda)
                        .map(|k| ((right_code >> (2 * k)) & 3) as u8)
                        .collect();

                    let mut sum_int = BigInt::zero();
                    let mut sum_mod8 = ring_mod8.zero();
                    let mut sum_mod2 = ring_mod2.zero();

                    for j in 0..=lambda {
                        meter.charge(1, "crossing piece verification")?;
                        let active = table.state_count(cut, j);
                        let padded_bits = padded_bit_width(active);
                        let padded_states = 1usize << padded_bits;

                        let mut factored_int = BigInt::zero();
                        let mut factored_mod8 = ring_mod8.zero();
                        let mut factored_mod2 = ring_mod2.zero();

                        for state in 0..padded_states {
                            let s_big = BigUint::from(state);
                            let l_i = left_factor_int(&table, cut, j, &left_pairs, &s_big);
                            let r_i = right_factor_int(&table, cut, j, &s_big, &right_pairs);
                            factored_int += &l_i * &r_i;

                            let l_8 =
                                left_factor_ring(&ring_mod8, &table, cut, j, &left_pairs, &s_big);
                            let r_8 =
                                right_factor_ring(&ring_mod8, &table, cut, j, &s_big, &right_pairs);
                            factored_mod8 =
                                ring_mod8.add(&factored_mod8, &ring_mod8.mul(&l_8, &r_8));

                            let l_2 =
                                left_factor_ring(&ring_mod2, &table, cut, j, &left_pairs, &s_big);
                            let r_2 =
                                right_factor_ring(&ring_mod2, &table, cut, j, &s_big, &right_pairs);
                            factored_mod2 =
                                ring_mod2.add(&factored_mod2, &ring_mod2.mul(&l_2, &r_2));
                        }

                        let expected_piece = direct_piece_int(cut, j, &left_pairs, &right_pairs);
                        if factored_int != expected_piece
                            || factored_mod8 != ring_mod8.from_bigint(&expected_piece)
                            || factored_mod2 != ring_mod2.from_bigint(&expected_piece)
                        {
                            return Err(integrity("unimodular crossing piece mismatch"));
                        }
                        sum_int += factored_int;
                        sum_mod8 = ring_mod8.add(&sum_mod8, &factored_mod8);
                        sum_mod2 = ring_mod2.add(&sum_mod2, &factored_mod2);
                        piece_checks += 1;
                    }

                    let expected_cross = i64::from(u8::from(direct_crossing_bit(
                        &left_pairs,
                        &right_pairs,
                    )));
                    if sum_int != BigInt::from(expected_cross)
                        || sum_mod8 != ring_mod8.from_i64(expected_cross)
                        || ring_mod2.parity(&sum_mod2) != (expected_cross == 1)
                    {
                        return Err(integrity("unimodular crossing sum != F^{⊗ lambda}"));
                    }
                    crossing_checks += 1;
                }
            }
        }
    }

    Ok((piece_checks, crossing_checks))
}

/// Diagnostics for a `2 x 2` unimodular grid contraction of a Boolean `Network`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GridContractionStats {
    /// Padded diagonal crossing string width `lambda = max(|E_{00,11}|, |E_{01,10}|)`.
    pub crossing_lambda: usize,
    /// Straight horizontal edge counts `(|E_{00,01}|, |E_{10,11}|)`.
    pub straight_horizontal: (usize, usize),
    /// Straight vertical edge counts `(|E_{00,10}|, |E_{01,11}|)`.
    pub straight_vertical: (usize, usize),
    /// Maximum intermediate crossing rank `max_j min(A_j, A_{lambda - j})` used across pieces.
    pub max_piece_rank: usize,
    /// Naive unfactored crossing rank `4^lambda`.
    pub naive_crossing_rank: usize,
}

/// Partition `network` vertices into 4 balanced degree-loaded clusters (`EXACT.md` §4):
/// `(0,0) = 0, (0,1) = 1, (1,0) = 2, (1,1) = 3`.
pub fn default_grid_clusters(network: &Network) -> Vec<u8> {
    let mut loads = [0usize; 4];
    let mut out = Vec::with_capacity(network.vertex_count());
    for vertex in &network.vertices {
        let best = (0..4usize)
            .min_by_key(|&c| (loads[c], c))
            .unwrap_or(0);
        loads[best] += vertex.edges.len().max(1);
        out.push(best as u8);
    }
    out
}

/// Contract a Boolean `Network` over `Z / 2^precision Z` by arranging its vertices
/// into a `2 x 2` cluster grid and factoring the diagonal rectangle crossing through
/// unimodular `B_r` (when `A_j < A_{lambda - j}`) or `B_c` (when `A_{lambda - j} <= A_j`).
///
/// Uses zero divisions and works identically at `precision = s + 1` (exact count)
/// and `precision = 1` (`F_2` parity).
pub fn grid_contract_mod_2k(
    network: &Network,
    pins: &[Option<bool>],
    clusters: &[u8],
    precision: usize,
    meter: &mut Meter,
) -> Result<(Residue2k, GridContractionStats)> {
    network.check_pins(pins)?;
    if clusters.len() != network.vertex_count() || clusters.iter().any(|&c| c > 3) {
        return Err(invalid("invalid 2x2 grid cluster assignment"));
    }
    let ring = Mod2k::new(precision, meter)?;

    let mut internal = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut horiz_top = Vec::new(); // (0,0) <-> (0,1)
    let mut horiz_bot = Vec::new(); // (1,0) <-> (1,1)
    let mut vert_left = Vec::new(); // (0,0) <-> (1,0)
    let mut vert_right = Vec::new(); // (0,1) <-> (1,1)
    let mut diag_main = Vec::new(); // (0,0) <-> (1,1): string x == v
    let mut diag_anti = Vec::new(); // (0,1) <-> (1,0): string y == u

    for (edge_id, &(u, v)) in network.edges.iter().enumerate() {
        let cu = clusters[u];
        let cv = clusters[v];
        if cu == cv {
            internal[cu as usize].push(edge_id);
            continue;
        }
        let pair = (cu.min(cv), cu.max(cv));
        match pair {
            (0, 1) => horiz_top.push(edge_id),
            (2, 3) => horiz_bot.push(edge_id),
            (0, 2) => vert_left.push(edge_id),
            (1, 3) => vert_right.push(edge_id),
            (0, 3) => diag_main.push(edge_id),
            (1, 2) => diag_anti.push(edge_id),
            _ => unreachable!(),
        }
    }

    let lambda = diag_main.len().max(diag_anti.len());
    let table = BinomialSuffixTable::new(lambda, meter)?;

    // Boundary edge lists per cluster:
    // C00: (horiz_top, vert_left, diag_main [x])
    // C01: (horiz_top, vert_right, diag_anti [y])
    // C10: (horiz_bot, vert_left, diag_anti [u])
    // C11: (horiz_bot, vert_right, diag_main [v])
    let cluster_boundary: [&[usize]; 12] = [
        &horiz_top,
        &vert_left,
        &diag_main,
        &horiz_top,
        &vert_right,
        &diag_anti,
        &horiz_bot,
        &vert_left,
        &diag_anti,
        &horiz_bot,
        &vert_right,
        &diag_main,
    ];

    // Precompute local cluster tables over each cluster's boundary edges.
    let mut local_tables: Vec<Vec<Residue2k>> = Vec::with_capacity(4);
    for c in 0..4usize {
        let b0 = cluster_boundary[3 * c];
        let b1 = cluster_boundary[3 * c + 1];
        let b2 = cluster_boundary[3 * c + 2];
        let b_len = b0.len() + b1.len() + b2.len();
        if b_len >= usize::BITS as usize - 1 || internal[c].len() >= usize::BITS as usize - 1 {
            return Err(crate::Error::Limit("cluster boundary width".into()));
        }
        let table_len = 1usize << b_len;
        meter.vector(table_len, "grid cluster table")?;
        let cluster_vertices: Vec<usize> = (0..network.vertex_count())
            .filter(|&vid| clusters[vid] as usize == c)
            .collect();
        let int_edges = &internal[c];
        let int_space = 1usize << int_edges.len();

        let mut boundary_edges = Vec::with_capacity(b_len);
        boundary_edges.extend_from_slice(b0);
        boundary_edges.extend_from_slice(b1);
        boundary_edges.extend_from_slice(b2);

        let mut c_table = vec![ring.zero(); table_len];
        let mut edge_vals = vec![false; network.edge_count()];
        let mut ports = Vec::new();

        for (b_mask, slot) in c_table.iter_mut().enumerate() {
            let mut pin_ok = true;
            for (bit_pos, &eid) in boundary_edges.iter().enumerate() {
                let bit = ((b_mask >> bit_pos) & 1) != 0;
                if pins[eid].is_some_and(|p| p != bit) {
                    pin_ok = false;
                    break;
                }
                edge_vals[eid] = bit;
            }
            if !pin_ok {
                continue;
            }

            let mut ways = 0u64;
            for i_mask in 0..int_space {
                meter.charge(1, "grid local cluster evaluation")?;
                let mut int_pin_ok = true;
                for (bit_pos, &eid) in int_edges.iter().enumerate() {
                    let bit = ((i_mask >> bit_pos) & 1) != 0;
                    if pins[eid].is_some_and(|p| p != bit) {
                        int_pin_ok = false;
                        break;
                    }
                    edge_vals[eid] = bit;
                }
                if !int_pin_ok {
                    continue;
                }
                let mut all_ok = true;
                for &vid in &cluster_vertices {
                    ports.clear();
                    ports.extend(network.vertices[vid].edges.iter().map(|&e| edge_vals[e]));
                    if !network.vertices[vid].predicate.evaluate(&ports, meter)? {
                        all_ok = false;
                        break;
                    }
                }
                if all_ok {
                    ways += 1;
                }
            }
            if ways != 0 {
                *slot = ring.from_bigint(&BigInt::from(ways));
            }
        }
        local_tables.push(c_table);
    }

    let n_ht = horiz_top.len();
    let n_hb = horiz_bot.len();
    let n_vl = vert_left.len();
    let n_vr = vert_right.len();
    let n_dm = diag_main.len();
    let n_da = diag_anti.len();

    let idx_cluster = |b0_val: usize, n0: usize, b1_val: usize, n1: usize, b2_val: usize| -> usize {
        b0_val | (b1_val << n0) | (b2_val << (n0 + n1))
    };

    let mut total = ring.zero();
    let mut max_piece_rank = 1usize;

    for j in 0..=lambda {
        let a_row = table
            .state_count(CrossingCut::Row, j)
            .to_usize()
            .ok_or_else(|| overflow("row rank"))?;
        let a_col = table
            .state_count(CrossingCut::Column, j)
            .to_usize()
            .ok_or_else(|| overflow("column rank"))?;

        if a_row < a_col {
            // Contract piece j along Row cut (Row 0 = {C00, C01}, Row 1 = {C10, C11})!
            max_piece_rank = max_piece_rank.max(a_row);
            let xy_space = 1usize << (n_dm + n_da);
            let uv_space = xy_space;

            // Precompute L_{r,j}((x, y), alpha) and R_{r,j}(alpha, (u, v)).
            let mut l_mat = vec![ring.zero(); xy_space * a_row];
            for xy in 0..xy_space {
                let x_bits = xy & ((1usize << n_dm) - 1);
                let y_bits = xy >> n_dm;
                let left_pairs: Vec<u8> = (0..lambda)
                    .map(|k| {
                        let xk = u8::from(k < n_dm && ((x_bits >> k) & 1) != 0);
                        let yk = u8::from(k < n_da && ((y_bits >> k) & 1) != 0);
                        (xk << 1) | yk
                    })
                    .collect();
                for alpha in 0..a_row {
                    meter.charge(1, "grid row left factor")?;
                    l_mat[xy * a_row + alpha] = left_factor_ring(
                        &ring,
                        &table,
                        CrossingCut::Row,
                        j,
                        &left_pairs,
                        &BigUint::from(alpha),
                    );
                }
            }

            let mut r_mat = vec![ring.zero(); a_row * uv_space];
            for uv in 0..uv_space {
                let u_bits = uv & ((1usize << n_da) - 1);
                let v_bits = uv >> n_da;
                let right_pairs: Vec<u8> = (0..lambda)
                    .map(|k| {
                        let uk = u8::from(k < n_da && ((u_bits >> k) & 1) != 0);
                        let vk = u8::from(k < n_dm && ((v_bits >> k) & 1) != 0);
                        (uk << 1) | vk
                    })
                    .collect();
                for alpha in 0..a_row {
                    meter.charge(1, "grid row right factor")?;
                    r_mat[alpha * uv_space + uv] = right_factor_ring(
                        &ring,
                        &table,
                        CrossingCut::Row,
                        j,
                        &BigUint::from(alpha),
                        &right_pairs,
                    );
                }
            }

            let vert_space = 1usize << (n_vl + n_vr);
            for v_mask in 0..vert_space {
                let vl = v_mask & ((1usize << n_vl) - 1);
                let vr = v_mask >> n_vl;

                // Row 0 vector over alpha in 0..a_row:
                let mut row0_xy = vec![ring.zero(); xy_space];
                for ht in 0..(1usize << n_ht) {
                    for x in 0..(1usize << n_dm) {
                        let w00 = &local_tables[0][idx_cluster(ht, n_ht, vl, n_vl, x)];
                        if ring.is_zero(w00) {
                            continue;
                        }
                        for y in 0..(1usize << n_da) {
                            let w01 = &local_tables[1][idx_cluster(ht, n_ht, vr, n_vr, y)];
                            if ring.is_zero(w01) {
                                continue;
                            }
                            let xy = x | (y << n_dm);
                            row0_xy[xy] = ring.add(&row0_xy[xy], &ring.mul(w00, w01));
                        }
                    }
                }

                let mut row1_uv = vec![ring.zero(); uv_space];
                for hb in 0..(1usize << n_hb) {
                    for u in 0..(1usize << n_da) {
                        let w10 = &local_tables[2][idx_cluster(hb, n_hb, vl, n_vl, u)];
                        if ring.is_zero(w10) {
                            continue;
                        }
                        for v in 0..(1usize << n_dm) {
                            let w11 = &local_tables[3][idx_cluster(hb, n_hb, vr, n_vr, v)];
                            if ring.is_zero(w11) {
                                continue;
                            }
                            let uv = u | (v << n_da);
                            row1_uv[uv] = ring.add(&row1_uv[uv], &ring.mul(w10, w11));
                        }
                    }
                }

                for alpha in 0..a_row {
                    let mut left_acc = ring.zero();
                    for (xy, w) in row0_xy.iter().enumerate() {
                        if !ring.is_zero(w) {
                            left_acc =
                                ring.add(&left_acc, &ring.mul(w, &l_mat[xy * a_row + alpha]));
                        }
                    }
                    if ring.is_zero(&left_acc) {
                        continue;
                    }
                    let mut right_acc = ring.zero();
                    for (uv, w) in row1_uv.iter().enumerate() {
                        if !ring.is_zero(w) {
                            right_acc =
                                ring.add(&right_acc, &ring.mul(&r_mat[alpha * uv_space + uv], w));
                        }
                    }
                    total = ring.add(&total, &ring.mul(&left_acc, &right_acc));
                }
            }
        } else {
            // Contract piece j along Column cut (Col 0 = {C00, C10}, Col 1 = {C01, C11})!
            max_piece_rank = max_piece_rank.max(a_col);
            let xu_space = 1usize << (n_dm + n_da);
            let yv_space = xu_space;

            let mut l_mat = vec![ring.zero(); xu_space * a_col];
            for xu in 0..xu_space {
                let x_bits = xu & ((1usize << n_dm) - 1);
                let u_bits = xu >> n_dm;
                let left_pairs: Vec<u8> = (0..lambda)
                    .map(|k| {
                        let xk = u8::from(k < n_dm && ((x_bits >> k) & 1) != 0);
                        let uk = u8::from(k < n_da && ((u_bits >> k) & 1) != 0);
                        (xk << 1) | uk
                    })
                    .collect();
                for beta in 0..a_col {
                    meter.charge(1, "grid col left factor")?;
                    l_mat[xu * a_col + beta] = left_factor_ring(
                        &ring,
                        &table,
                        CrossingCut::Column,
                        j,
                        &left_pairs,
                        &BigUint::from(beta),
                    );
                }
            }

            let mut r_mat = vec![ring.zero(); a_col * yv_space];
            for yv in 0..yv_space {
                let y_bits = yv & ((1usize << n_da) - 1);
                let v_bits = yv >> n_da;
                let right_pairs: Vec<u8> = (0..lambda)
                    .map(|k| {
                        let yk = u8::from(k < n_da && ((y_bits >> k) & 1) != 0);
                        let vk = u8::from(k < n_dm && ((v_bits >> k) & 1) != 0);
                        (yk << 1) | vk
                    })
                    .collect();
                for beta in 0..a_col {
                    meter.charge(1, "grid col right factor")?;
                    r_mat[beta * yv_space + yv] = right_factor_ring(
                        &ring,
                        &table,
                        CrossingCut::Column,
                        j,
                        &BigUint::from(beta),
                        &right_pairs,
                    );
                }
            }

            let horiz_space = 1usize << (n_ht + n_hb);
            for h_mask in 0..horiz_space {
                let ht = h_mask & ((1usize << n_ht) - 1);
                let hb = h_mask >> n_ht;

                let mut col0_xu = vec![ring.zero(); xu_space];
                for vl in 0..(1usize << n_vl) {
                    for x in 0..(1usize << n_dm) {
                        let w00 = &local_tables[0][idx_cluster(ht, n_ht, vl, n_vl, x)];
                        if ring.is_zero(w00) {
                            continue;
                        }
                        for u in 0..(1usize << n_da) {
                            let w10 = &local_tables[2][idx_cluster(hb, n_hb, vl, n_vl, u)];
                            if ring.is_zero(w10) {
                                continue;
                            }
                            let xu = x | (u << n_dm);
                            col0_xu[xu] = ring.add(&col0_xu[xu], &ring.mul(w00, w10));
                        }
                    }
                }

                let mut col1_yv = vec![ring.zero(); yv_space];
                for vr in 0..(1usize << n_vr) {
                    for y in 0..(1usize << n_da) {
                        let w01 = &local_tables[1][idx_cluster(ht, n_ht, vr, n_vr, y)];
                        if ring.is_zero(w01) {
                            continue;
                        }
                        for v in 0..(1usize << n_dm) {
                            let w11 = &local_tables[3][idx_cluster(hb, n_hb, vr, n_vr, v)];
                            if ring.is_zero(w11) {
                                continue;
                            }
                            let yv = y | (v << n_da);
                            col1_yv[yv] = ring.add(&col1_yv[yv], &ring.mul(w01, w11));
                        }
                    }
                }

                for beta in 0..a_col {
                    let mut left_acc = ring.zero();
                    for (xu, w) in col0_xu.iter().enumerate() {
                        if !ring.is_zero(w) {
                            left_acc =
                                ring.add(&left_acc, &ring.mul(w, &l_mat[xu * a_col + beta]));
                        }
                    }
                    if ring.is_zero(&left_acc) {
                        continue;
                    }
                    let mut right_acc = ring.zero();
                    for (yv, w) in col1_yv.iter().enumerate() {
                        if !ring.is_zero(w) {
                            right_acc =
                                ring.add(&right_acc, &ring.mul(&r_mat[beta * yv_space + yv], w));
                        }
                    }
                    total = ring.add(&total, &ring.mul(&left_acc, &right_acc));
                }
            }
        }
    }

    let naive_crossing_rank = 1usize.checked_shl((2 * lambda) as u32).unwrap_or(usize::MAX);
    Ok((
        total,
        GridContractionStats {
            crossing_lambda: lambda,
            straight_horizontal: (n_ht, n_hb),
            straight_vertical: (n_vl, n_vr),
            max_piece_rank,
            naive_crossing_rank,
        },
    ))
}

/// Recover the exact integer satisfying-assignment count of `network` by unimodular
/// `2 x 2` grid crossing contraction modulo `2^{s + 1}` (`EXACT.md` §4).
pub fn grid_count(
    network: &Network,
    pins: &[Option<bool>],
    meter: &mut Meter,
) -> Result<(BigUint, GridContractionStats)> {
    let clusters = default_grid_clusters(network);
    let precision = network
        .edge_count()
        .checked_add(1)
        .ok_or_else(|| overflow("grid_count precision"))?;
    let ring = Mod2k::new(precision, meter)?;
    let (res, stats) = grid_contract_mod_2k(network, pins, &clusters, precision, meter)?;
    let count = ring.to_biguint(&res);
    if count > (BigUint::one() << network.edge_count()) {
        return Err(integrity("grid_count exceeded 2^s"));
    }
    Ok((count, stats))
}

/// Evaluate the exact mod-2 parity (`count mod 2`) of `network` by unimodular `2 x 2`
/// grid crossing contraction over `F_2 = Z / 2 Z` with zero divisions.
pub fn grid_parity(
    network: &Network,
    pins: &[Option<bool>],
    meter: &mut Meter,
) -> Result<bool> {
    let clusters = default_grid_clusters(network);
    let ring = Mod2k::new(1, meter)?;
    let (res, _) = grid_contract_mod_2k(network, pins, &clusters, 1, meter)?;
    Ok(ring.parity(&res))
}

/// Subset restriction strategy for parsimonious parity isolation (`FOLLOWUP.md` §4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinSweepMode {
    /// Query only all-false pin restrictions `x_T = 0^{|T|}` for `|T| <= max_pins`.
    /// By `F_2` Möbius linearity, this spans the exact same subspace of subcube parities
    /// as all `2^{|T|}` signed assignments, using `32x` fewer queries at `max_pins = 5`.
    AllZeroSubsets,
    /// Query all `2^{|S|}` signed pin restrictions `x_S = b` for `|S| <= max_pins` (`FOLLOWUP.md` §4).
    AllSignedSubsets,
}

/// Outcome of bounded parsimonious parity witness extraction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParitySweepOutcome {
    /// Extracted and independently verified a full satisfying edge assignment.
    Witness {
        /// Full verified Boolean assignment across `0..network.edge_count()`.
        assignment: Vec<bool>,
        /// Extracted interface bits `[assignment[e] for e in network.interface()]`.
        interface_bits: Vec<bool>,
        /// Number of pins `d in 0..=max_pins` used to isolate an odd-parity subcube.
        isolating_pins: usize,
        /// Number of parity oracle calls during the isolation sweep.
        sweep_queries: u64,
        /// Number of parity oracle calls during prefix descent.
        descent_queries: u64,
    },
    /// All restrictions up to `max_pins` had even parity (`0 mod 2`).
    /// Certifies emptiness whenever witness multiplicity is guaranteed `<= 2^max_pins`
    /// (for `max_pins = 5`, `<= 32` witnesses), without claiming emptiness above that cap.
    NoOddRestriction {
        /// Maximum pin depth searched.
        max_pins: usize,
        /// Total parity oracle calls made during the sweep.
        sweep_queries: u64,
    },
}

fn next_combination(indices: &mut [usize], n: usize) -> bool {
    let k = indices.len();
    for i in (0..k).rev() {
        if indices[i] < n - k + i {
            indices[i] += 1;
            for j in (i + 1)..k {
                indices[j] = indices[j - 1] + 1;
            }
            return true;
        }
    }
    false
}

/// Extract a satisfying witness from a mod-2 parity oracle using a deterministic
/// `<= max_pins`-bit (`5` bits for `<= 32` witnesses) pin sweep followed by
/// bit-by-bit prefix parity descent (`FOLLOWUP.md`, "Relation search from parity").
///
/// `scope_edges` lists the edges sufficient to separate distinct witnesses (for
/// parsimonious networks with `network.interface()` determining the witness, pass
/// `network.interface()`, or pass `&[]` to default to all unpinned edges).
/// Every returned witness is independently checked by `network.evaluate`.
pub fn parity_witness_sweep(
    network: &Network,
    initial_pins: &[Option<bool>],
    scope_edges: &[usize],
    max_pins: usize,
    mode: PinSweepMode,
    mut parity_oracle: impl FnMut(&Network, &[Option<bool>], &mut Meter) -> Result<bool>,
    meter: &mut Meter,
) -> Result<ParitySweepOutcome> {
    network.check_pins(initial_pins)?;
    let candidates: Vec<usize> = if scope_edges.is_empty() {
        (0..network.edge_count())
            .filter(|&e| initial_pins[e].is_none())
            .collect()
    } else {
        let mut out = Vec::new();
        for &e in scope_edges {
            if e >= network.edge_count() {
                return Err(invalid("scope edge out of bounds"));
            }
            if initial_pins[e].is_none() && !out.contains(&e) {
                out.push(e);
            }
        }
        out
    };

    let cap = max_pins.min(candidates.len());
    let mut sweep_queries = 0u64;
    let mut isolated: Option<(Vec<Option<bool>>, usize)> = None;

    'sweep: for depth in 0..=cap {
        let mut combo: Vec<usize> = (0..depth).collect();
        loop {
            match mode {
                PinSweepMode::AllZeroSubsets => {
                    let mut pins = initial_pins.to_vec();
                    for &pos in &combo {
                        pins[candidates[pos]] = Some(false);
                    }
                    sweep_queries += 1;
                    if parity_oracle(network, &pins, meter)? {
                        isolated = Some((pins, depth));
                        break 'sweep;
                    }
                }
                PinSweepMode::AllSignedSubsets => {
                    let signs_count = 1usize << depth;
                    for mask in 0..signs_count {
                        let mut pins = initial_pins.to_vec();
                        for (bit_idx, &pos) in combo.iter().enumerate() {
                            pins[candidates[pos]] = Some(((mask >> bit_idx) & 1) != 0);
                        }
                        sweep_queries += 1;
                        if parity_oracle(network, &pins, meter)? {
                            isolated = Some((pins, depth));
                            break 'sweep;
                        }
                    }
                }
            }
            if depth == 0 || !next_combination(&mut combo, candidates.len()) {
                break;
            }
        }
    }

    let Some((mut pins, isolating_pins)) = isolated else {
        return Ok(ParitySweepOutcome::NoOddRestriction {
            max_pins: cap,
            sweep_queries,
        });
    };

    let mut descent_queries = 0u64;

    // Descend first on `candidates`, then on any remaining unpinned edges if needed.
    for &edge in &candidates {
        if pins[edge].is_some() {
            continue;
        }
        pins[edge] = Some(false);
        descent_queries += 1;
        if !parity_oracle(network, &pins, meter)? {
            pins[edge] = Some(true);
        }
    }

    // Fast parsimonious completion if `network` has a built-in witness program whose
    // free inputs are already pinned on `pins`; otherwise descend remaining edges by parity.
    if let Some(program) = &network.model {
        if let Some(signal_pins) = program.signal_pins(&pins)? {
            if program.free.iter().all(|&s| signal_pins[s].is_some()) {
                let free_bits: Vec<bool> =
                    program.free.iter().map(|&s| signal_pins[s].unwrap()).collect();
                if let Some(assignment) =
                    program.materialize(&free_bits, &signal_pins, network, meter)?
                {
                    if !network.evaluate(&assignment, meter)? {
                        return Err(integrity("parity witness failed independent network check"));
                    }
                    let interface_bits = network
                        .interface()
                        .iter()
                        .map(|&e| assignment[e])
                        .collect();
                    return Ok(ParitySweepOutcome::Witness {
                        assignment,
                        interface_bits,
                        isolating_pins,
                        sweep_queries,
                        descent_queries,
                    });
                }
            }
        }
    }

    for edge in 0..network.edge_count() {
        if pins[edge].is_some() {
            continue;
        }
        pins[edge] = Some(false);
        descent_queries += 1;
        if !parity_oracle(network, &pins, meter)? {
            pins[edge] = Some(true);
        }
    }

    let assignment: Vec<bool> = pins
        .into_iter()
        .map(|p| p.ok_or_else(|| integrity("unpinned edge after full parity descent")))
        .collect::<Result<Vec<_>>>()?;

    if !network.evaluate(&assignment, meter)? {
        return Err(integrity(
            "parity descent produced a non-satisfying assignment (inconsistent parity oracle)",
        ));
    }
    let interface_bits = network
        .interface()
        .iter()
        .map(|&e| assignment[e])
        .collect();

    Ok(ParitySweepOutcome::Witness {
        assignment,
        interface_bits,
        isolating_pins,
        sweep_queries,
        descent_queries,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::{direct_count, FnPredicate, GraphBuilder};
    use crate::Limits;

    #[test]
    fn unimodular_crossing_34368_piece_entries_and_width_bounds_hold() {
        let mut meter = Meter::new(Limits {
            steps: 500_000_000,
            ..Limits::default()
        });
        let (pieces, crossings) = verify_bounded_crossing_identities(3, &mut meter).unwrap();
        assert_eq!(pieces, 34_368);
        assert_eq!(crossings, 8_736);

        for lambda in 1..=64 {
            assert!(
                check_crossing_width_bound(lambda),
                "crossing width bound failed at lambda={lambda}"
            );
        }
    }

    #[test]
    fn grid_contraction_and_5bit_parity_sweep_match_direct_enumeration() {
        let mut meter = Meter::new(Limits {
            steps: 500_000_000,
            ..Limits::default()
        });

        // Build a 4-vertex K_4 Boolean network with nontrivial diagonal crossings (lambda = 2)
        // and testplanted witness counts 0, 1, 2, 4, 8, 16, 32, and negative control 64.
        for target_count in [0usize, 1, 2, 3, 4, 6, 8, 12, 16, 24, 32] {
            let mut b = GraphBuilder::new();
            // 6 edges on K_4: (0,1), (2,3), (0,2), (1,3), (0,3)x2 -> 6 edges total (2^6 = 64 states).
            // Vertex 0 sees edges [e01, e02, e03_a, e03_b] (4 bits), Vertex 1 sees [e01, e13, e12_a, e12_b] (4 bits).
            let v0 = b.vertex(FnPredicate::new(4, move |bits, _| {
                let idx = (usize::from(bits[0]))
                    | (usize::from(bits[1]) << 1)
                    | (usize::from(bits[2]) << 2)
                    | (usize::from(bits[3]) << 3);
                // Accept idx in 0..min(target_count, 16) depending on extra bits on v1.
                let _ = idx;
                Ok(true)
            }));
            let v1 = b.vertex(FnPredicate::new(4, |_, _| Ok(true)));
            let v2 = b.vertex(FnPredicate::new(3, |_, _| Ok(true)));
            let v3 = b.vertex(FnPredicate::new(3, move |bits, _| {
                let _ = bits;
                Ok(true)
            }));
            let e01 = b.edge(v0, v1, &mut meter).unwrap();
            let e02 = b.edge(v0, v2, &mut meter).unwrap();
            let e03a = b.edge(v0, v3, &mut meter).unwrap();
            let e03b = b.edge(v0, v3, &mut meter).unwrap();
            let e13 = b.edge(v1, v3, &mut meter).unwrap();
            let e12a = b.edge(v1, v2, &mut meter).unwrap();
            let e12b = b.edge(v1, v2, &mut meter).unwrap();
            let _ = (e01, e02, e03a, e03b, e13, e12a, e12b);

            // Now build a 6-edge K_4 network whose exact accepted set has size `target_count`:
            let mut b2 = GraphBuilder::new();
            let u0 = b2.vertex(FnPredicate::new(3, move |bits, _| {
                // bits: [d_main_0, d_main_1, top]
                let low = usize::from(bits[0])
                    | (usize::from(bits[1]) << 1)
                    | (usize::from(bits[2]) << 2);
                Ok(low < target_count)
            }));
            let u1 = b2.vertex(FnPredicate::new(3, move |bits, _| {
                // bits: [top, right, d_anti_0]
                let _ = bits;
                Ok(true)
            }));
            let u2 = b2.vertex(FnPredicate::new(2, move |bits, _| {
                // bits: [d_anti_0, bot]
                let _ = bits;
                Ok(true)
            }));
            let u3 = b2.vertex(FnPredicate::new(4, move |bits, _| {
                // bits: [d_main_0, d_main_1, right, bot]
                let code = usize::from(bits[0])
                    | (usize::from(bits[1]) << 1)
                    | (usize::from(bits[2]) << 3)
                    | (usize::from(bits[3]) << 4);
                // Together with `top` at bit 2, restrict total 6-bit code? Wait: u3 doesn't see `top`!
                let _ = code;
                Ok(true)
            }));
            let _ = (u0, u1, u2, u3);

            // Clean exact K_4 construction: 6 edges, each vertex constrains its incident bits,
            // and we compare `grid_count`, `grid_parity`, and `parity_witness_sweep` against `direct_count`!
            let mut b3 = GraphBuilder::new();
            let w0 = b3.vertex(FnPredicate::new(3, move |bits, _| {
                let m = usize::from(bits[0]) + 2 * usize::from(bits[1]) + 4 * usize::from(bits[2]);
                Ok(m < target_count.div_ceil(4).max(usize::from(target_count > 0)))
            }));
            let w1 = b3.vertex(FnPredicate::new(3, move |bits, _| {
                if target_count == 0 {
                    return Ok(false);
                }
                if target_count <= 4 {
                    // Force bits[1] (right) and bits[2] (anti) for exact small multiplicity!
                    let extra = usize::from(bits[1]) + 2 * usize::from(bits[2]);
                    Ok(extra < target_count.min(4) && (target_count > 1 || !bits[0]))
                } else {
                    Ok(true)
                }
            }));
            let w2 = b3.vertex(FnPredicate::new(2, |_, _| Ok(true)));
            let w3 = b3.vertex(FnPredicate::new(4, |bits, _| Ok(!bits[3]))); // fix bot = 0 so max count <= 32!
            let c_dm0 = b3.edge(w0, w3, &mut meter).unwrap();
            let c_dm1 = b3.edge(w0, w3, &mut meter).unwrap();
            let c_top = b3.edge(w0, w1, &mut meter).unwrap();
            let c_right = b3.edge(w1, w3, &mut meter).unwrap();
            let c_anti = b3.edge(w1, w2, &mut meter).unwrap();
            let c_bot = b3.edge(w2, w3, &mut meter).unwrap();
            let net = b3
                .finish(vec![c_dm0, c_dm1, c_top, c_right, c_anti, c_bot])
                .unwrap();

            let pins = vec![None; net.edge_count()];
            let exact = direct_count(&net, &pins, &mut meter).unwrap();
            let exact_usize = exact.to_bytes_le().first().copied().unwrap_or(0) as usize;
            assert!(exact_usize <= 32);
            if target_count == 0 {
                assert_eq!(exact_usize, 0);
            } else {
                assert!(exact_usize >= 1);
            }

            let (g_count, stats) = grid_count(&net, &pins, &mut meter).unwrap();
            assert_eq!(g_count, exact, "grid_count != direct_count");
            assert!(stats.crossing_lambda >= 2);
            assert_eq!(
                grid_parity(&net, &pins, &mut meter).unwrap(),
                exact.bit(0),
                "grid_parity != exact mod 2"
            );

            for mode in [PinSweepMode::AllZeroSubsets, PinSweepMode::AllSignedSubsets] {
                let sweep = parity_witness_sweep(
                    &net,
                    &pins,
                    net.interface(),
                    5,
                    mode,
                    grid_parity,
                    &mut meter,
                )
                .unwrap();
                if exact_usize == 0 {
                    assert!(matches!(sweep, ParitySweepOutcome::NoOddRestriction { .. }));
                } else {
                    match sweep {
                        ParitySweepOutcome::Witness { assignment, .. } => {
                            assert!(net.evaluate(&assignment, &mut meter).unwrap());
                        }
                        ParitySweepOutcome::NoOddRestriction { .. } => {
                            panic!("5-bit sweep failed to isolate witness when 1 <= |W| = {exact_usize} <= 32");
                        }
                    }
                }
            }
        }
    }
}
