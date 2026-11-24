//! Constant-residual S3 solved in the spans of x1+x2 and x1*x2.
//! Existing symmetric S3 algebra, with exact root/lift checks and conservative
//! treatment of rank-deficient affine solution spaces; no SAT assumption.
use crate::binary_ecc::{
    curve::{point_add, point_neg},
    BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly, TraceZeroVariety,
};
use crate::cryptanalysis::trace_zero_word_field::{WordField, WordPoint};
use crate::cryptanalysis::{
    binary_semaev::binary_semaev_s3,
    trace_zero_relations::{
        TraceZeroRelationDisposition as Disposition, TraceZeroRelationLimits as Limits,
        TraceZeroRelationReport as Report,
    },
};
use num_bigint::BigUint;
use std::{collections::BTreeMap, time::Instant};

#[derive(Clone, Debug, Default)]
pub struct TraceZeroLinearizedReport {
    pub relation: Report,
    pub source_dimension: usize,
    pub product_dimension: usize,
    pub linear_columns: usize,
    pub systems: usize,
    pub maximum_rank: usize,
    pub maximum_nullity: usize,
    pub projected_columns: usize,
    pub projected_systems: usize,
    pub projected_rows: usize,
    pub projected_inconsistent: usize,
}

type WordMask = [u64; 3];

/// Direct addressing for small polynomial-coordinate windows, with the general
/// ordered dictionary retained for arbitrary factor bases. Both expose keys in
/// exactly the original lexicographic order and retain every supplied index.
enum BaseIndex {
    Dense(Vec<IndexBucket>),
    Tree(BTreeMap<WordMask, Vec<usize>>),
}
#[derive(Clone)]
enum IndexBucket {
    Inline { len: usize, ids: [usize; 2] },
    Spill(Vec<usize>),
}
impl IndexBucket {
    const EMPTY: Self = Self::Inline {
        len: 0,
        ids: [0; 2],
    };
    #[inline]
    fn as_slice(&self) -> &[usize] {
        match self {
            Self::Inline { len, ids } => &ids[..*len],
            Self::Spill(ids) => ids,
        }
    }
    #[inline]
    fn push(&mut self, i: usize) {
        match self {
            Self::Inline { len, ids } if *len < 2 => {
                ids[*len] = i;
                *len += 1;
            }
            Self::Inline { ids, .. } => {
                let mut out = Vec::with_capacity(4);
                out.extend_from_slice(ids);
                out.push(i);
                *self = Self::Spill(out);
            }
            Self::Spill(ids) => ids.push(i),
        }
    }
}
enum BaseEntry<'a> {
    Dense(&'a mut IndexBucket),
    Tree(std::collections::btree_map::Entry<'a, WordMask, Vec<usize>>),
}
impl BaseIndex {
    fn new(base: &[BinaryPoint]) -> Self {
        if base.len() >= 64 {
            let maximum = base.iter().try_fold(0u64, |maximum, p| {
                let BinaryPoint::Affine { x, .. } = p else {
                    return None;
                };
                let bits = x.raw_bits();
                let low = bits.first().copied().unwrap_or(0);
                (low < 65536 && bits.iter().skip(1).all(|&w| w == 0)).then_some(maximum.max(low))
            });
            if let Some(maximum) = maximum {
                let slots = (maximum as usize + 1).next_power_of_two();
                // Bound empty-bucket storage as well as the coordinate width.
                if slots <= base.len().saturating_mul(8) {
                    return Self::Dense(vec![IndexBucket::EMPTY; slots]);
                }
            }
        }
        Self::Tree(BTreeMap::new())
    }
    #[inline]
    fn entry(&mut self, key: WordMask) -> BaseEntry<'_> {
        match self {
            Self::Dense(buckets) => {
                debug_assert_eq!((key[0], key[1]), (0, 0));
                BaseEntry::Dense(&mut buckets[key[2] as usize])
            }
            Self::Tree(tree) => BaseEntry::Tree(tree.entry(key)),
        }
    }
    #[inline]
    fn get(&self, key: &WordMask) -> Option<&[usize]> {
        match self {
            Self::Dense(buckets) => {
                if key[0] != 0 || key[1] != 0 || key[2] >= buckets.len() as u64 {
                    return None;
                }
                let ids = buckets[key[2] as usize].as_slice();
                (!ids.is_empty()).then_some(ids)
            }
            Self::Tree(tree) => tree.get(key).map(Vec::as_slice),
        }
    }
    fn keys(&self) -> Vec<WordMask> {
        match self {
            Self::Dense(buckets) => buckets
                .iter()
                .enumerate()
                .filter(|(_, ids)| !ids.as_slice().is_empty())
                .map(|(x, _)| [0, 0, x as u64])
                .collect(),
            Self::Tree(tree) => tree.keys().copied().collect(),
        }
    }
}
impl BaseEntry<'_> {
    #[inline]
    fn first(&self) -> Option<usize> {
        match self {
            Self::Dense(ids) => ids.as_slice().first().copied(),
            Self::Tree(std::collections::btree_map::Entry::Occupied(entry)) => {
                entry.get().first().copied()
            }
            Self::Tree(std::collections::btree_map::Entry::Vacant(_)) => None,
        }
    }
    #[inline]
    fn push(self, i: usize) {
        match self {
            Self::Dense(ids) => ids.push(i),
            Self::Tree(entry) => entry.or_default().push(i),
        }
    }
}

fn words(x: &F2mElement) -> WordMask {
    let mut a = [0; 3];
    a[..x.raw_bits().len()].copy_from_slice(x.raw_bits());
    a
}

fn parity(a: &WordMask, b: &WordMask) -> u64 {
    (((a[0] & b[0]) ^ (a[1] & b[1]) ^ (a[2] & b[2])).count_ones() & 1) as u64
}

/// Conjugate transposed multiplication to ordinary multiplication. For
/// f=X^n+sum X^t, N(r)=(I+S)reverse_n(r), S(a)=sum_{t>0} a>>(n-t).
/// N M_h^T = M_h N. These sparse moduli need at most two inverse folds.
/// This changes the construction cost, not any projected equation.
struct AdjointMap {
    n: usize,
    shifts: [usize; 3],
}
impl AdjointMap {
    fn new(c: &BinaryCurve) -> Option<Self> {
        let mut terms = c.irreducible.low_terms.clone();
        terms.sort_unstable();
        let n = c.m as usize;
        let shifts = match (c.m, c.irreducible.degree, terms.as_slice()) {
            (83, 83, [0, 1, 2, 45]) => [82, 81, 38],
            (131, 131, [0, 1, 2, 13]) => [130, 129, 118],
            _ => return None,
        };
        Some(Self { n, shifts })
    }
    fn shr(a: WordMask, shift: usize) -> WordMask {
        let word = shift / 64;
        let bit = shift % 64;
        std::array::from_fn(|i| {
            let j = i + word;
            if j >= 3 {
                return 0;
            }
            (a[j] >> bit)
                ^ if bit != 0 && j + 1 < 3 {
                    a[j + 1] << (64 - bit)
                } else {
                    0
                }
        })
    }
    fn reverse(&self, a: WordMask) -> WordMask {
        Self::shr(
            [
                a[2].reverse_bits(),
                a[1].reverse_bits(),
                a[0].reverse_bits(),
            ],
            192 - self.n,
        )
    }
    fn sparse(&self, a: WordMask) -> WordMask {
        let mut out = [0; 3];
        for shift in self.shifts {
            let term = Self::shr(a, shift);
            for i in 0..3 {
                out[i] ^= term[i];
            }
        }
        out
    }
    fn forward(&self, r: WordMask) -> WordMask {
        let a = self.reverse(r);
        let s = self.sparse(a);
        std::array::from_fn(|i| a[i] ^ s[i])
    }
    fn inverse(&self, a: WordMask) -> WordMask {
        let mut result = a;
        let mut term = self.sparse(a);
        while term != [0; 3] {
            for i in 0..3 {
                result[i] ^= term[i];
            }
            term = self.sparse(term);
        }
        self.reverse(result)
    }
}

/// Byte combination tables for the first sixteen projected equations.
/// Four 16-bit coefficient rows share each word; RHS bits remain separate.
/// This evaluates the existing F2-linear maps, without changing their order.
struct PackedRowMap {
    coefficients: Vec<[u64; 4]>,
    rhs: Vec<u16>,
    rows: usize,
    bits: usize,
}
impl PackedRowMap {
    fn new(equations: &[Vec<WordMask>], rhs: &[WordMask], n: u32) -> Option<Self> {
        let width = equations.first()?.len();
        if width > 16 {
            return None;
        }
        let rows = equations.len().min(16);
        let bits = if n <= 96 { 4 } else { 8 };
        let entries = 1usize << bits;
        let groups = (n as usize).div_ceil(bits);
        let mut coefficients = vec![[0; 4]; groups * entries];
        let mut rhs_table = vec![0u16; groups * entries];
        for group in 0..groups {
            let mut basis = [[0u64; 4]; 8];
            let mut rhs_basis = [0u16; 8];
            for b in 0..bits {
                let bit = group * bits + b;
                if bit >= n as usize {
                    continue;
                }
                for row in 0..rows {
                    for col in 0..width {
                        let value = (equations[row][col][bit / 64] >> (bit % 64)) & 1;
                        basis[b][row / 4] |= value << ((row % 4) * 16 + col);
                    }
                    rhs_basis[b] |= (((rhs[row][bit / 64] >> (bit % 64)) & 1) as u16) << row;
                }
            }
            for value in 1usize..entries {
                let prev = group * entries + (value & (value - 1));
                let b = value.trailing_zeros() as usize;
                coefficients[group * entries + value] =
                    std::array::from_fn(|i| coefficients[prev][i] ^ basis[b][i]);
                rhs_table[group * entries + value] = rhs_table[prev] ^ rhs_basis[b];
            }
        }
        Some(Self {
            coefficients,
            rhs: rhs_table,
            rows,
            bits,
        })
    }
    fn evaluate(&self, z: WordMask, target: WordMask) -> ([u64; 4], u16) {
        if self.bits == 4 {
            self.evaluate_bits::<4>(z, target)
        } else {
            self.evaluate_bits::<8>(z, target)
        }
    }
    #[inline]
    fn evaluate_bits<const BITS: usize>(&self, z: WordMask, target: WordMask) -> ([u64; 4], u16) {
        let mut coefficients = [0; 4];
        let mut rhs = 0;
        let entries = 1usize << BITS;
        let groups_per_word = 64 / BITS;
        // Extract each field word once; only the last word may have fewer
        // than eight byte tables. No dynamic field-word index per byte.
        for (word, tables) in self
            .coefficients
            .chunks(groups_per_word * entries)
            .enumerate()
        {
            let mut zi = z[word];
            let mut ti = target[word];
            for (group, table) in tables.chunks_exact(entries).enumerate() {
                let c = table[(zi as usize) & (entries - 1)];
                for i in 0..4 {
                    coefficients[i] ^= c[i];
                }
                rhs ^= self.rhs
                    [(word * groups_per_word + group) * entries + ((ti as usize) & (entries - 1))];
                zi >>= BITS;
                ti >>= BITS;
            }
        }
        (coefficients, rhs)
    }
}

/// Eliminate a packed prefix in original row order. Each inserted pivot is
/// applied to every later row in parallel; already-consumed lanes may change
/// because their original pivots have been saved. Rank and the first
/// contradiction/full-rank index are the same as sequential elimination.
#[cfg(target_arch = "x86_64")]
unsafe fn eliminate_packed_prefix(
    coefficients: [u64; 4],
    targets: u16,
    width: usize,
    rows: usize,
    pivots: &mut [u64; 64],
) -> (usize, usize, bool) {
    use std::arch::x86_64::*;
    debug_assert!(width < 16 && rows <= 16);
    let zero = _mm_setzero_si128();
    let rhs = _mm_set1_epi16((1u16 << width) as i16);
    let t = _mm_set1_epi16(targets as i16);
    let mask0 = _mm_setr_epi16(1, 2, 4, 8, 16, 32, 64, 128);
    let mask1 = _mm_setr_epi16(256, 512, 1024, 2048, 4096, 8192, 16384, -32768);
    let mut lo = _mm_xor_si128(
        _mm_loadu_si128(coefficients.as_ptr().cast()),
        _mm_andnot_si128(_mm_cmpeq_epi16(_mm_and_si128(t, mask0), zero), rhs),
    );
    let mut hi = _mm_xor_si128(
        _mm_loadu_si128(coefficients.as_ptr().add(2).cast()),
        _mm_andnot_si128(_mm_cmpeq_epi16(_mm_and_si128(t, mask1), zero), rhs),
    );
    let coefficient_mask = (1u64 << width) - 1;
    let mut rank = 0;
    macro_rules! consume {
        ($j:literal, $v:ident, $lane:literal) => {{
            if $j >= rows {
                return (rank, rows, false);
            }
            let row = _mm_extract_epi16::<$lane>($v) as u64;
            let c = row & coefficient_mask;
            if c == 0 {
                if row >> width != 0 {
                    return (rank, $j + 1, true);
                }
            } else {
                let bit = 63 - c.leading_zeros() as usize;
                pivots[bit] = row;
                rank += 1;
                if rank == width {
                    return (rank, $j + 1, false);
                }
                let bitmask = _mm_set1_epi16((1u16 << bit) as i16);
                let pivot = _mm_set1_epi16(row as i16);
                let m0 = _mm_cmpeq_epi16(_mm_and_si128(lo, bitmask), bitmask);
                let m1 = _mm_cmpeq_epi16(_mm_and_si128(hi, bitmask), bitmask);
                lo = _mm_xor_si128(lo, _mm_and_si128(m0, pivot));
                hi = _mm_xor_si128(hi, _mm_and_si128(m1, pivot));
            }
        }};
    }
    consume!(0, lo, 0);
    consume!(1, lo, 1);
    consume!(2, lo, 2);
    consume!(3, lo, 3);
    consume!(4, lo, 4);
    consume!(5, lo, 5);
    consume!(6, lo, 6);
    consume!(7, lo, 7);
    consume!(8, hi, 0);
    consume!(9, hi, 1);
    consume!(10, hi, 2);
    consume!(11, hi, 3);
    consume!(12, hi, 4);
    consume!(13, hi, 5);
    consume!(14, hi, 6);
    consume!(15, hi, 7);
    (rank, rows, false)
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
enum ProjectedResult {
    Inconsistent,
    Unique(F2mElement, F2mElement),
    Deficient,
}

/// Eliminate v by Artin--Schreier before solving. For the actual residual
/// (z,y), t=y+zH(z) satisfies t²+zt=1. Since Tr(u)=0, every S3 candidate is
/// v=t+zH(u²)+epsilon*z. Membership v in W supplies only d+1 unknown bits.
/// Maps z -> L_j(zH(x_i²)) are precomputed; rows cost packed parity tests.
struct ProductProjection {
    equations: Vec<Vec<WordMask>>,
    rhs: Vec<WordMask>,
    w_pivots: Vec<(usize, WordMask)>,
    halves: Vec<F2mElement>,
    w_support: WordMask,
    row_map: Option<PackedRowMap>,
    half_combinations: Vec<[[u64; 3]; 16]>,
}
impl ProductProjection {
    fn reduce_with(pivots: &[(usize, WordMask)], mut a: WordMask) -> WordMask {
        for (bit, row) in pivots {
            if a[bit / 64] & (1u64 << (bit % 64)) != 0 {
                for j in 0..3 {
                    a[j] ^= row[j];
                }
            }
        }
        a
    }
    #[cfg(test)]
    fn new(xs: &[F2mElement], ws: &[F2mElement], c: &BinaryCurve) -> Self {
        Self::new_with_field(xs, ws, c, &WordField::new(c))
    }
    fn new_with_field(
        xs: &[F2mElement],
        ws: &[F2mElement],
        c: &BinaryCurve,
        field: &WordField,
    ) -> Self {
        Self::build(xs, ws, c, field, true)
    }
    fn build(
        xs: &[F2mElement],
        ws: &[F2mElement],
        c: &BinaryCurve,
        field: &WordField,
        adjoint: bool,
    ) -> Self {
        assert!(xs.len() < 63 && c.m <= 192);
        let pivots: Vec<_> = ws
            .iter()
            .rev()
            .map(|w| (w.degree().unwrap() as usize, words(w)))
            .collect();
        let coordinates: Vec<_> = (0..c.m as usize)
            .filter(|i| !pivots.iter().any(|(p, _)| p == i))
            .collect();
        let mut rhs = vec![[0; 3]; coordinates.len()];
        for bit in 0..c.m as usize {
            let mut unit = [0; 3];
            unit[bit / 64] = 1u64 << (bit % 64);
            let projected = Self::reduce_with(&pivots, unit);
            for (j, &coord) in coordinates.iter().enumerate() {
                if projected[coord / 64] & (1u64 << (coord % 64)) != 0 {
                    rhs[j][bit / 64] |= 1u64 << (bit % 64);
                }
            }
        }
        let halves: Vec<_> = xs
            .iter()
            .map(|x| {
                x.square(&c.irreducible)
                    .half_trace(&c.irreducible)
                    .expect("odd field")
            })
            .collect();
        let mut equations = vec![vec![[0; 3]; xs.len() + 1]; coordinates.len()];
        if let Some(map) = adjoint.then(|| AdjointMap::new(c)).flatten() {
            for (row, mask) in equations.iter_mut().zip(&rhs) {
                let numerator = map.forward(*mask);
                for (i, h) in halves.iter().enumerate() {
                    row[i] = map.inverse(field.mul(numerator, words(h)));
                }
            }
        } else {
            for (i, h) in halves.iter().enumerate() {
                let mut multiple = words(h);
                for bit in 0..c.m as usize {
                    let image = Self::reduce_with(&pivots, multiple);
                    for (j, &coord) in coordinates.iter().enumerate() {
                        if image[coord / 64] & (1u64 << (coord % 64)) != 0 {
                            equations[j][i][bit / 64] |= 1u64 << (bit % 64);
                        }
                    }
                    multiple = field.times_x(multiple);
                }
            }
        }
        for (row, mask) in equations.iter_mut().zip(&rhs) {
            row[xs.len()] = *mask;
        }
        let mut w_support = [0; 3];
        for (_, w) in &pivots {
            for i in 0..3 {
                w_support[i] |= w[i];
            }
        }
        let row_map = PackedRowMap::new(&equations, &rhs, c.m);
        let half_combinations = halves
            .chunks(4)
            .map(|chunk| {
                let mut table = [[0; 3]; 16];
                for value in 1usize..16 {
                    let bit = value.trailing_zeros() as usize;
                    let image = chunk.get(bit).map(words).unwrap_or([0; 3]);
                    let previous = table[value & (value - 1)];
                    table[value] = std::array::from_fn(|i| previous[i] ^ image[i]);
                }
                table
            })
            .collect();
        Self {
            equations,
            rhs,
            w_pivots: pivots,
            halves,
            w_support,
            row_map,
            half_combinations,
        }
    }

    #[cfg(test)]
    fn solve(
        &self,
        xs: &[F2mElement],
        c: &BinaryCurve,
        z: &F2mElement,
        y: &F2mElement,
    ) -> (ProjectedResult, usize) {
        self.solve_compact(xs, c, words(z), words(y), &WordField::new(c))
    }
    fn solve_compact(
        &self,
        xs: &[F2mElement],
        c: &BinaryCurve,
        z: WordMask,
        y: WordMask,
        field: &WordField,
    ) -> (ProjectedResult, usize) {
        let width = xs.len() + 1;
        let coefficient_mask = (1u64 << width) - 1;
        let mut pivots = [0u64; 64];
        let mut rank = 0;
        let zw = z;
        let offset = field.add(y, field.mul(z, field.half(z)));
        let target = offset;
        // Keep packed lookup and the generic parity fallback in separate
        // loops; no repeated Option/zip/row-map branch in the hot prefix.
        macro_rules! insert_row {
            ($row:expr, $j:expr) => {{
                let mut row = $row;
                loop {
                    let coefficients = row & coefficient_mask;
                    if coefficients == 0 {
                        if row >> width != 0 {
                            return (ProjectedResult::Inconsistent, $j + 1);
                        }
                        break;
                    }
                    let bit = 63 - coefficients.leading_zeros() as usize;
                    if pivots[bit] == 0 {
                        pivots[bit] = row;
                        rank += 1;
                        break;
                    }
                    row ^= pivots[bit];
                }
                if rank == width {
                    return self.finish_unique(xs, c, zw, offset, field, &pivots, $j + 1);
                }
            }};
        }
        let packed_rows = if let Some(map) = &self.row_map {
            let (coefficients, mut targets) = map.evaluate(zw, target);
            let mut consumed_by_vector = false;
            #[cfg(target_arch = "x86_64")]
            if width < 16 {
                // SAFETY: SSE2 is mandatory on x86-64; four words contain
                // exactly sixteen16-bit lanes, width<16 and map.rows<=16.
                let (head_rank, used, inconsistent) = unsafe {
                    eliminate_packed_prefix(coefficients, targets, width, map.rows, &mut pivots)
                };
                if inconsistent {
                    return (ProjectedResult::Inconsistent, used);
                }
                rank = head_rank;
                if rank == width {
                    return self.finish_unique(xs, c, zw, offset, field, &pivots, used);
                }
                consumed_by_vector = true;
            }
            if !consumed_by_vector {
                for j in 0..map.rows {
                    let row = ((coefficients[j / 4] >> ((j % 4) * 16)) & 0xffff)
                        | (((targets & 1) as u64) << width);
                    targets >>= 1;
                    insert_row!(row, j);
                }
            }
            map.rows
        } else {
            0
        };
        for j in packed_rows..self.equations.len() {
            let row = self.equations[j]
                .iter()
                .enumerate()
                .fold(parity(&self.rhs[j], &target) << width, |r, (i, col)| {
                    r | (parity(col, &zw) << i)
                });
            insert_row!(row, j);
        }
        (ProjectedResult::Deficient, self.equations.len())
    }
    fn finish_unique(
        &self,
        xs: &[F2mElement],
        c: &BinaryCurve,
        z: WordMask,
        offset: WordMask,
        field: &WordField,
        pivots: &[u64; 64],
        rows: usize,
    ) -> (ProjectedResult, usize) {
        let width = xs.len() + 1;
        // These independent rows force one coefficient vector. Check
        // ALL remaining W conditions with one exact remainder test.
        let mut solution = 0u64;
        for (i, &pivot) in pivots.iter().enumerate().take(width) {
            if ((pivot >> width) ^ ((pivot & solution).count_ones() as u64)) & 1 != 0 {
                solution |= 1u64 << i;
            }
        }
        let mut h = if solution & (1u64 << xs.len()) != 0 {
            [1, 0, 0]
        } else {
            [0; 3]
        };
        let mut coefficients = solution;
        for table in &self.half_combinations {
            let term = table[(coefficients & 15) as usize];
            h = field.add(h, term);
            coefficients >>= 4;
        }
        let product_words = field.add(offset, field.mul(z, h));
        if product_words
            .iter()
            .zip(&self.w_support)
            .any(|(v, s)| *v & !*s != 0)
            || Self::reduce_with(&self.w_pivots, product_words) != [0; 3]
        {
            return (ProjectedResult::Inconsistent, rows);
        }
        let mut u = F2mElement::zero(c.m);
        for (i, x) in xs.iter().enumerate() {
            if solution & (1u64 << i) != 0 {
                u = u.add(x);
            }
        }
        return (
            ProjectedResult::Unique(u, F2mElement::from_words(&product_words, c.m)),
            rows,
        );
    }
}

#[cfg(test)]
fn basis(values: impl IntoIterator<Item = BigUint>, n: u32) -> Vec<F2mElement> {
    let mut pivots: Vec<Option<BigUint>> = vec![None; n as usize];
    for mut v in values {
        for bit in (0..n as usize).rev() {
            if v.bit(bit as u64) {
                if let Some(p) = &pivots[bit] {
                    v ^= p;
                } else {
                    pivots[bit] = Some(v);
                    break;
                }
            }
        }
    }
    pivots
        .into_iter()
        .flatten()
        .map(|v| F2mElement::from_biguint(&v, n))
        .collect()
}
fn linear_solutions(
    columns: &[F2mElement],
    target: &F2mElement,
    n: u32,
) -> Option<(BigUint, Vec<BigUint>, usize)> {
    // Three words cover every supported field (n <= 131). Retain the
    // arbitrary-width path for coefficient spaces wider than 192 bits.
    if n <= 192 && columns.len() <= 192 {
        return packed_linear_solutions(columns, target, n);
    }
    generic_linear_solutions(columns, target, n)
}
fn generic_linear_solutions(
    columns: &[F2mElement],
    target: &F2mElement,
    n: u32,
) -> Option<(BigUint, Vec<BigUint>, usize)> {
    let mut pivots: Vec<Option<(BigUint, BigUint)>> = vec![None; n as usize];
    let mut kernel = Vec::new();
    let mut rank = 0;
    for (j, col) in columns.iter().enumerate() {
        let mut v = col.to_biguint();
        let mut label = BigUint::from(1u32) << j;
        let mut installed = false;
        for bit in (0..n as usize).rev() {
            if v.bit(bit as u64) {
                if let Some((p, l)) = &pivots[bit] {
                    v ^= p;
                    label ^= l;
                } else {
                    pivots[bit] = Some((v.clone(), label.clone()));
                    rank += 1;
                    installed = true;
                    break;
                }
            }
        }
        if !installed {
            kernel.push(label);
        }
    }
    let mut v = target.to_biguint();
    let mut solution = BigUint::default();
    for bit in (0..n as usize).rev() {
        if v.bit(bit as u64) {
            let (p, l) = pivots[bit].as_ref()?;
            v ^= p;
            solution ^= l;
        }
    }
    Some((solution, kernel, rank))
}

fn packed_linear_solutions(
    columns: &[F2mElement],
    target: &F2mElement,
    n: u32,
) -> Option<(BigUint, Vec<BigUint>, usize)> {
    fn packed(x: &F2mElement) -> [u64; 3] {
        let mut a = [0; 3];
        a[..x.raw_bits().len()].copy_from_slice(x.raw_bits());
        a
    }
    fn leading(a: &[u64; 3]) -> Option<usize> {
        (0..3)
            .rev()
            .find(|&i| a[i] != 0)
            .map(|i| i * 64 + 63 - a[i].leading_zeros() as usize)
    }
    fn big(a: &[u64; 3]) -> BigUint {
        BigUint::from_slice(
            &a.iter()
                .flat_map(|w| [*w as u32, (*w >> 32) as u32])
                .collect::<Vec<_>>(),
        )
    }
    let mut rows = [[0u64; 6]; 192];
    let mut present = [false; 192];
    let mut rank = 0;
    let mut kernel = Vec::new();
    for (j, col) in columns.iter().enumerate() {
        let mut a = packed(col);
        let mut label = [0u64; 3];
        label[j / 64] = 1u64 << (j % 64);
        loop {
            let Some(bit) = leading(&a) else {
                kernel.push(big(&label));
                break;
            };
            debug_assert!(bit < n as usize);
            if !present[bit] {
                present[bit] = true;
                rows[bit][..3].copy_from_slice(&a);
                rows[bit][3..].copy_from_slice(&label);
                rank += 1;
                break;
            }
            for w in 0..3 {
                a[w] ^= rows[bit][w];
                label[w] ^= rows[bit][3 + w];
            }
        }
    }
    let mut a = packed(target);
    let mut solution = [0u64; 3];
    while let Some(bit) = leading(&a) {
        if !present[bit] {
            return None;
        }
        for w in 0..3 {
            a[w] ^= rows[bit][w];
            solution[w] ^= rows[bit][3 + w];
        }
    }
    Some((big(&solution), kernel, rank))
}

/// Montgomery's simultaneous inversion, applied only to ordinary additions.
/// Preserve every input position; identity/doubling/opposite cases use the
/// original group routine. Computing a batch charges even unused tail work.
fn residual_batch(c: &BinaryCurve, target: &BinaryPoint, base: &[BinaryPoint]) -> Vec<BinaryPoint> {
    let BinaryPoint::Affine { x: tx, y: ty } = target else {
        return base.iter().map(point_neg).collect();
    };
    let irr = &c.irreducible;
    let mut out = vec![BinaryPoint::Infinity; base.len()];
    let mut ordinary = Vec::new();
    let mut product = F2mElement::one(c.m);
    for (i, p) in base.iter().enumerate() {
        match p {
            BinaryPoint::Affine { x, y } if x != tx => {
                let den = tx.add(x);
                ordinary.push((i, product.clone(), den.clone(), ty.add(&y.add(x))));
                product = product.mul(&den, irr);
            }
            _ => out[i] = point_add(c, target, &point_neg(p)),
        }
    }
    if ordinary.is_empty() {
        return out;
    }
    let mut inverse = product
        .flt_inverse(irr)
        .expect("nonzero residual denominators");
    for (i, prefix, den, numerator) in ordinary.into_iter().rev() {
        let inv_den = inverse.mul(&prefix, irr);
        inverse = inverse.mul(&den, irr);
        let lambda = numerator.mul(&inv_den, irr);
        let x = lambda.square(irr).add(&lambda).add(&den).add(&c.a);
        let y = lambda.mul(&tx.add(&x), irr).add(&x).add(ty);
        crate::cryptanalysis::group_op_count::count_adds(1);
        out[i] = BinaryPoint::Affine { x, y };
    }
    out
}

/// Monic specialization: avoid inverting the known leading coefficient 1.
/// The variety admits odd fields, so half-trace gives both actual roots.
fn monic_roots(
    u: &F2mElement,
    product: &F2mElement,
    n: u32,
    irr: &IrreduciblePoly,
) -> Vec<F2mElement> {
    if u.is_zero() {
        return vec![product.square_k_times(n - 1, irr)];
    }
    let inv = u
        .flt_inverse(irr)
        .expect("nonzero monic linear coefficient");
    let arg = product.mul(&inv.square(irr), irr);
    if arg.trace(irr) != 0 {
        return Vec::new();
    }
    let root = u.mul(&arg.half_trace(irr).expect("odd field"), irr);
    vec![root.clone(), root.add(u)]
}
fn x_key(x: &F2mElement) -> WordMask {
    let w = words(x);
    [w[2], w[1], w[0]]
}
fn word_basis(values: impl IntoIterator<Item = WordMask>, n: u32) -> Vec<F2mElement> {
    let mut pivots: Vec<Option<WordMask>> = vec![None; n as usize];
    for mut v in values {
        while let Some(word) = (0..3).rev().find(|&i| v[i] != 0) {
            let bit = word * 64 + 63 - v[word].leading_zeros() as usize;
            if let Some(p) = pivots[bit] {
                for i in 0..3 {
                    v[i] ^= p[i];
                }
            } else {
                pivots[bit] = Some(v);
                break;
            }
        }
    }
    pivots
        .into_iter()
        .flatten()
        .map(|w| F2mElement::from_words(&w, n))
        .collect()
}

/// For each actual final point R, solve the necessary linear equation
/// v²+z*v+z²*u²+b=0 with u in span(B_x), v in span(B_x*B_x),
/// z=x(T-R). Factor X²+uX+v and replay actual supplied point signs.
/// All candidates of a deficient affine space are required for Refuted.
pub fn decompose_trace_zero_linearized(
    v: &TraceZeroVariety,
    base: &[BinaryPoint],
    target: &BinaryPoint,
    limits: Limits,
) -> TraceZeroLinearizedReport {
    let mut out = TraceZeroLinearizedReport::default();
    let deadline = limits.wall_budget.map(|d| Instant::now() + d);
    if !v.contains(target) {
        out.relation.reason = Some("invalid linearized target/base");
        return out;
    }
    if limits.max_models == 0 || limits.max_patterns == 0 || limits.max_variables == 0 {
        out.relation.reason = Some(
            if base
                .iter()
                .any(|p| matches!(p, BinaryPoint::Infinity) || !v.contains(p))
            {
                "invalid linearized target/base"
            } else {
                "zero linearized budget"
            },
        );
        return out;
    }
    if base.is_empty() {
        out.relation.disposition = Disposition::Refuted;
        return out;
    }
    let c = v.curve();
    let compact = (c.m >= 31 && base.len() >= 64).then(|| WordField::new(c));
    let mut by_x = BaseIndex::new(base);
    for (i, p) in base.iter().enumerate() {
        let BinaryPoint::Affine { x: fx, y: fy } = p else {
            out.relation.reason = Some("invalid linearized target/base");
            return out;
        };
        if fx.m_value() != c.m || fy.m_value() != c.m {
            out.relation.reason = Some("invalid linearized target/base");
            return out;
        }
        let x = x_key(fx);
        // Hold one entry search across admission instead of finding the
        // same key again for insertion. No entry is inserted until admitted.
        let entry = by_x.entry(x);
        let known_sign = if let Some(first) = entry.first() {
            let BinaryPoint::Affine { y: known, .. } = &base[first] else {
                unreachable!()
            };
            fy == known
                || if let Some(f) = &compact {
                    words(fy) == f.add(words(known), words(fx))
                } else {
                    *fy == known.add(fx)
                }
        } else {
            false
        };
        // is_in_4e performs the field-width and curve-equation checks itself.
        // Once one ordinate is admitted, its identical/opposite lift is also
        // admitted; every supplied sign/index is still retained below.
        if !known_sign
            && !compact
                .as_ref()
                .map_or_else(|| c.is_in_4e(p) == Some(true), |f| f.admit(p))
        {
            out.relation.reason = Some("invalid linearized target/base");
            return out;
        }
        entry.push(i);
    }
    let xs = word_basis(by_x.keys().into_iter().map(|w| [w[2], w[1], w[0]]), c.m);
    let products: Vec<_> = xs
        .iter()
        .enumerate()
        .flat_map(|(i, x)| xs[i..].iter().map(|y| words(&x.mul(y, &c.irreducible))))
        .collect();
    let ws = word_basis(products, c.m);
    out.source_dimension = xs.len();
    out.product_dimension = ws.len();
    out.linear_columns = xs.len() + ws.len();
    if out.linear_columns > limits.max_variables as usize {
        out.relation.reason = Some("linear-column allocation cap");
        return out;
    }
    let xs_squared: Vec<_> = xs.iter().map(|x| x.square(&c.irreducible)).collect();
    let ws_squared: Vec<_> = ws.iter().map(|w| w.square(&c.irreducible)).collect();
    // Small bases keep scalar admission/search costs. Larger bases amortize
    // each inversion over at most 256 consecutive residuals, in original order.
    // Unused tail residuals are computed and charged even for an early answer.
    let batch_size = if base.len() >= 64 { 256 } else { 1 };
    let mut residuals = Vec::new();
    let mut residual_start = 0;
    let use_projection =
        c.m >= 31 && base.len() >= 64 && xs.len() < 63 && out.linear_columns < c.m as usize;
    let mut projection = None;
    let mut incomplete = false;
    for (r, p) in base.iter().enumerate() {
        if deadline.is_some_and(|d| Instant::now() >= d) {
            out.relation.reason = Some("linearized total deadline");
            return out;
        }
        if r == 0 || r == residual_start + residuals.len() {
            residual_start = r;
            // An immediate relation should not pay for 31 unused residuals.
            residuals = if r == 0 || batch_size == 1 {
                vec![WordPoint::from_point(&point_add(c, target, &point_neg(p)))]
            } else if let Some(f) = &compact {
                f.residuals(c, target, &base[r..base.len().min(r + batch_size)])
            } else {
                residual_batch(c, target, &base[r..base.len().min(r + batch_size)])
                    .iter()
                    .map(WordPoint::from_point)
                    .collect()
            };
        }
        let residual = residuals[r - residual_start];
        out.relation.patterns_attempted += 1;
        let mut residual_incomplete = false;
        let found = if residual == WordPoint::Infinity {
            base.iter().enumerate().find_map(|(i, q)| {
                let BinaryPoint::Affine { x, .. } = q else {
                    unreachable!()
                };
                let negative = point_neg(q);
                by_x.get(&x_key(x))
                    .and_then(|ids| ids.iter().copied().find(|&j| base[j] == negative))
                    .map(|j| (i, j))
            })
        } else {
            let WordPoint::Affine { x: zw, y: yw } = residual else {
                unreachable!()
            };
            out.systems += 1;
            let projected = if r > 0 && use_projection {
                let p = projection.get_or_insert_with(|| {
                    ProductProjection::new_with_field(
                        &xs,
                        &ws,
                        c,
                        compact.as_ref().expect("projected compact workspace"),
                    )
                });
                out.projected_columns = xs.len() + 1;
                out.projected_systems += 1;
                let (answer, rows) = p.solve_compact(
                    &xs,
                    c,
                    zw,
                    yw,
                    compact.as_ref().expect("projected compact workspace"),
                );
                out.projected_rows += rows;
                Some(answer)
            } else {
                None
            };
            let mut unique_pair = None;
            let (solution, kernel, rank) = match projected {
                Some(ProjectedResult::Inconsistent) => {
                    out.projected_inconsistent += 1;
                    out.relation.patterns_refuted += 1;
                    continue;
                }
                Some(ProjectedResult::Unique(u, v)) => {
                    unique_pair = Some((u, v));
                    (BigUint::default(), Vec::new(), out.linear_columns)
                }
                Some(ProjectedResult::Deficient) | None => {
                    let z = F2mElement::from_words(&zw, c.m);
                    let z2 = z.square(&c.irreducible);
                    let columns: Vec<_> = xs_squared
                        .iter()
                        .map(|x2| z2.mul(x2, &c.irreducible))
                        .chain(
                            ws.iter()
                                .zip(&ws_squared)
                                .map(|(w, w2)| w2.add(&z.mul(w, &c.irreducible))),
                        )
                        .collect();
                    let Some(answer) = linear_solutions(&columns, &c.b, c.m) else {
                        out.relation.patterns_refuted += 1;
                        continue;
                    };
                    answer
                }
            };
            let z = F2mElement::from_words(&zw, c.m);
            out.maximum_rank = out.maximum_rank.max(rank);
            out.maximum_nullity = out.maximum_nullity.max(kernel.len());
            let available = limits.max_models.saturating_sub(out.relation.models);
            let total = 1usize.checked_shl(kernel.len() as u32);
            let count = total.unwrap_or(usize::MAX).min(available);
            if total.is_none_or(|t| count < t) {
                incomplete = true;
                residual_incomplete = true;
            }
            let mut pair = None;
            for selector in 0..count {
                if deadline.is_some_and(|d| Instant::now() >= d) {
                    out.relation.reason = Some("linearized coefficient deadline");
                    return out;
                }
                let mut coefficients = solution.clone();
                for (j, k) in kernel.iter().enumerate().take(usize::BITS as usize) {
                    if selector >> j & 1 != 0 {
                        coefficients ^= k;
                    }
                }
                let (u, product) = if let Some(pair) = &unique_pair {
                    pair.clone()
                } else {
                    let mut u = F2mElement::zero(c.m);
                    let mut product = F2mElement::zero(c.m);
                    for (j, x) in xs.iter().enumerate() {
                        if coefficients.bit(j as u64) {
                            u = u.add(x);
                        }
                    }
                    for (j, w) in ws.iter().enumerate() {
                        if coefficients.bit((xs.len() + j) as u64) {
                            product = product.add(w);
                        }
                    }
                    (u, product)
                };
                out.relation.models += 1;
                let roots = monic_roots(&u, &product, c.m, &c.irreducible);
                for x in roots {
                    let y = x.add(&u);
                    let (Some(is), Some(js)) = (by_x.get(&x_key(&x)), by_x.get(&x_key(&y))) else {
                        continue;
                    };
                    if !binary_semaev_s3(&x, &y, &z, &c.b, &c.irreducible).is_zero() {
                        out.relation.reason = Some("linearized S3 replay failed");
                        return out;
                    }
                    for &i in is {
                        for &j in js {
                            if point_add(c, &base[i], &base[j]) == residual.point(c.m) {
                                pair = Some((i, j));
                                break;
                            }
                        }
                        if pair.is_some() {
                            break;
                        }
                    }
                    if pair.is_some() {
                        break;
                    }
                }
                if pair.is_some() {
                    break;
                }
            }
            pair
        };
        if let Some((i, j)) = found {
            if point_add(c, &point_add(c, &base[i], &base[j]), p) != *target {
                out.relation.reason = Some("linearized original replay failed");
                return out;
            }
            out.relation.indices = Some(vec![i, j, r]);
            out.relation.disposition = Disposition::Found;
            return out;
        }
        if !residual_incomplete {
            out.relation.patterns_refuted += 1;
        }
    }
    if incomplete {
        out.relation.reason = Some("linearized affine-model cap");
    } else {
        out.relation.disposition = Disposition::Refuted;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_index_preserves_tree_keys_duplicates_and_first_indices() {
        let base: Vec<_> = (0..256u32)
            .map(|i| BinaryPoint::Affine {
                x: F2mElement::from_biguint(&((i * 73) % 127).into(), 131),
                y: F2mElement::zero(131),
            })
            .collect();
        let mut dense = BaseIndex::new(&base);
        assert!(matches!(dense, BaseIndex::Dense(_)));
        let mut tree = BaseIndex::Tree(BTreeMap::new());
        for (i, p) in base.iter().enumerate() {
            let BinaryPoint::Affine { x, .. } = p else {
                unreachable!()
            };
            let key = x_key(x);
            let a = dense.entry(key);
            let b = tree.entry(key);
            assert_eq!(a.first(), b.first());
            a.push(i);
            b.push(i);
        }
        assert_eq!(dense.keys(), tree.keys());
        for x in 0..257u64 {
            assert_eq!(dense.get(&[0, 0, x]), tree.get(&[0, 0, x]));
        }
        for key in [[0, 1, 0], [1, 0, 0], [0, 0, u64::MAX]] {
            assert_eq!(dense.get(&key), tree.get(&key));
        }
    }

    #[test]
    fn direct_index_bounds_storage_and_falls_back_for_other_shapes() {
        let point = |x: u64| BinaryPoint::Affine {
            x: F2mElement::from_biguint(&x.into(), 131),
            y: F2mElement::zero(131),
        };
        assert!(matches!(
            BaseIndex::new(&vec![point(1); 63]),
            BaseIndex::Tree(_)
        ));
        assert!(matches!(
            BaseIndex::new(&vec![point(65536); 64]),
            BaseIndex::Tree(_)
        ));
        assert!(matches!(
            BaseIndex::new(&vec![point(65535); 64]),
            BaseIndex::Tree(_)
        ));
        assert!(matches!(
            BaseIndex::new(&vec![BinaryPoint::Infinity; 64]),
            BaseIndex::Tree(_)
        ));
        let high = BinaryPoint::Affine {
            x: F2mElement::from_bit_positions(&[64], 131),
            y: F2mElement::zero(131),
        };
        assert!(matches!(
            BaseIndex::new(&vec![high; 64]),
            BaseIndex::Tree(_)
        ));
        let BaseIndex::Dense(buckets) = BaseIndex::new(&vec![point(255); 64]) else {
            panic!("exact storage boundary should qualify")
        };
        assert_eq!(buckets.len(), 256);
        // Actual d12 trace-zero windows reach polynomial degree12, with about
        // 2000 admitted signed points, and therefore need8192 direct slots.
        let BaseIndex::Dense(buckets) = BaseIndex::new(&vec![point(0x1ffc); 1994]) else {
            panic!("actual d12 occupancy must select direct addressing")
        };
        assert_eq!(buckets.len(), 8192);
    }

    #[test]
    fn direct_index_two_common_signs_stay_inline_and_extra_indices_spill_in_order() {
        let mut bucket = IndexBucket::EMPTY;
        for i in [7, 1] {
            bucket.push(i);
        }
        assert!(matches!(bucket, IndexBucket::Inline { len: 2, .. }));
        assert_eq!(bucket.as_slice(), &[7, 1]);
        for i in [7, 4, 2] {
            bucket.push(i);
        }
        assert!(matches!(bucket, IndexBucket::Spill(_)));
        assert_eq!(bucket.as_slice(), &[7, 1, 7, 4, 2]);
    }

    fn adjoint_curve(n: u32) -> BinaryCurve {
        BinaryCurve {
            m: n,
            irreducible: IrreduciblePoly {
                degree: n,
                low_terms: if n == 83 {
                    vec![0, 1, 2, 45]
                } else {
                    vec![0, 1, 2, 13]
                },
            },
            a: F2mElement::zero(n),
            b: F2mElement::one(n),
            generator: BinaryPoint::Infinity,
            order: 1u32.into(),
            cofactor: 4u32.into(),
        }
    }

    #[test]
    fn adjoint_matches_reference_multiplication_on_every_bilinear_basis_triple() {
        for n in [83, 131] {
            // Only the field is used; no group operations or order assertions.
            let c = adjoint_curve(n);
            let map = AdjointMap::new(&c).unwrap();
            let field = WordField::new(&c);
            let basis: Vec<_> = (0..n)
                .map(|i| F2mElement::from_bit_positions(&[i], n))
                .collect();
            let products: Vec<Vec<_>> = basis
                .iter()
                .map(|h| {
                    basis
                        .iter()
                        .map(|z| words(&h.mul(z, &c.irreducible)))
                        .collect()
                })
                .collect();
            for (j, r) in basis.iter().enumerate() {
                let r = words(r);
                assert_eq!(map.inverse(map.forward(r)), r);
                assert_eq!(map.forward(map.inverse(r)), r);
                for (i, h) in basis.iter().enumerate() {
                    let mask = map.inverse(field.mul(map.forward(r), words(h)));
                    for k in 0..n as usize {
                        assert_eq!(
                            (mask[k / 64] >> (k % 64)) & 1,
                            (products[i][k][j / 64] >> (j % 64)) & 1,
                            "n{n}/r{j}/h{i}/z{k}"
                        );
                    }
                }
            }
            let mut wrong = c.clone();
            wrong.irreducible.low_terms.push(3);
            assert!(AdjointMap::new(&wrong).is_none());
        }
        assert!(AdjointMap::new(&small_curve()).is_none());
    }

    #[test]
    fn adjoint_projector_preserves_every_coefficient_and_quotient_coordinate() {
        for n in [83, 131] {
            let c = adjoint_curve(n);
            let field = WordField::new(&c);
            for d in [1, 3, 8, 12, 17] {
                let xs: Vec<_> = (0..d)
                    .map(|i| F2mElement::from_bit_positions(&[i, i + 19], n))
                    .collect();
                let ws: Vec<_> = (0..d + 3)
                    .map(|i| F2mElement::from_bit_positions(&[i, i + 29], n))
                    .collect();
                let old = ProductProjection::build(&xs, &ws, &c, &field, false);
                let new = ProductProjection::build(&xs, &ws, &c, &field, true);
                assert_eq!(old.equations, new.equations, "n{n}/d{d}");
                assert_eq!(old.rhs, new.rhs);
                assert_eq!(old.w_pivots, new.w_pivots);
                assert_eq!(old.halves, new.halves);
                assert_eq!(old.w_support, new.w_support);
            }
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn vector_prefix_matches_scalar_rank_and_stopping_row_on_arbitrary_matrices() {
        let mut seed = 0xabc20261124u64;
        for width in 1..16 {
            for rows in [0, 1, 7, 16] {
                for sample in 0..128 {
                    let mut values = [0u64; 16];
                    let mut coefficients = [0u64; 4];
                    let mut targets = 0u16;
                    let mask = (1u64 << width) - 1;
                    for j in 0..rows {
                        seed ^= seed << 13;
                        seed ^= seed >> 7;
                        seed ^= seed << 17;
                        let row = if sample == 0 {
                            0
                        } else if sample == 1 {
                            1u64 << width
                        } else {
                            seed & ((1u64 << (width + 1)) - 1)
                        };
                        values[j] = row;
                        coefficients[j / 4] |= (row & mask) << (16 * (j % 4));
                        targets |= ((row >> width) as u16) << j;
                    }
                    let mut scalar = [0u64; 64];
                    let mut rank = 0;
                    let mut used = rows;
                    let mut inconsistent = false;
                    'outer: for (j, mut row) in values.into_iter().take(rows).enumerate() {
                        loop {
                            let c = row & mask;
                            if c == 0 {
                                if row >> width != 0 {
                                    used = j + 1;
                                    inconsistent = true;
                                    break 'outer;
                                }
                                break;
                            }
                            let bit = 63 - c.leading_zeros() as usize;
                            if scalar[bit] == 0 {
                                scalar[bit] = row;
                                rank += 1;
                                break;
                            }
                            row ^= scalar[bit];
                        }
                        if rank == width {
                            used = j + 1;
                            break;
                        }
                    }
                    let mut vector = [0u64; 64];
                    let actual = unsafe {
                        eliminate_packed_prefix(coefficients, targets, width, rows, &mut vector)
                    };
                    assert_eq!(
                        actual,
                        (rank, used, inconsistent),
                        "w{width}/rows{rows}/sample{sample}"
                    );
                    if rank == width && !inconsistent {
                        let mut solution = 0u64;
                        for (i, pivot) in vector.iter().enumerate().take(width) {
                            if ((pivot >> width) ^ ((pivot & solution).count_ones() as u64)) & 1
                                != 0
                            {
                                solution |= 1u64 << i;
                            }
                        }
                        for row in values.into_iter().take(used) {
                            assert_eq!((row & solution).count_ones() as u64 & 1, row >> width);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn packed_prefix_preserves_generic_first_contradiction_and_unique_result() {
        let mut seed = 0x10_2026_1124u64;
        for n in [83, 131] {
            let c = adjoint_curve(n);
            let field = WordField::new(&c);
            for d in [1, 4, 12, 17] {
                let xs: Vec<_> = (1..=d)
                    .map(|i| F2mElement::from_bit_positions(&[i], n))
                    .collect();
                let ws = word_basis(
                    xs.iter().enumerate().flat_map(|(i, x)| {
                        xs[i..].iter().map(|y| words(&x.mul(y, &c.irreducible)))
                    }),
                    n,
                );
                let fast = ProductProjection::new_with_field(&xs, &ws, &c, &field);
                let mut scalar = ProductProjection::new_with_field(&xs, &ws, &c, &field);
                scalar.row_map = None;
                for _ in 0..128 {
                    let mut draw = || {
                        seed ^= seed << 13;
                        seed ^= seed >> 7;
                        seed ^= seed << 17;
                        seed
                    };
                    let z = words(&F2mElement::from_words(&[draw(), draw(), draw()], n));
                    let y = words(&F2mElement::from_words(&[draw(), draw(), draw()], n));
                    assert_eq!(
                        fast.solve_compact(&xs, &c, z, y, &field),
                        scalar.solve_compact(&xs, &c, z, y, &field),
                        "n{n}/d{d}"
                    );
                }
            }
        }
    }

    #[test]
    fn four_bit_half_combinations_preserve_all_small_coefficient_vectors() {
        for n in [83, 131] {
            let c = adjoint_curve(n);
            let field = WordField::new(&c);
            for d in [3, 4, 7, 12] {
                let xs: Vec<_> = (1..=d)
                    .map(|i| F2mElement::from_bit_positions(&[i], n))
                    .collect();
                let p = ProductProjection::new_with_field(&xs, &[], &c, &field);
                for coefficient in 0usize..1 << d {
                    let mut actual = [0; 3];
                    for (chunk, table) in p.half_combinations.iter().enumerate() {
                        let term = table[(coefficient >> (4 * chunk)) & 15];
                        for i in 0..3 {
                            actual[i] ^= term[i];
                        }
                    }
                    let mut expected = F2mElement::zero(n);
                    for (i, h) in p.halves.iter().enumerate() {
                        if coefficient >> i & 1 != 0 {
                            expected = expected.add(h);
                        }
                    }
                    assert_eq!(actual, words(&expected));
                }
            }
        }
    }

    #[test]
    fn packed_row_maps_match_all_input_basis_vectors_and_independent_random_pairs() {
        let mut seed = 20261117u64;
        let mut draw = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for n in [5u32, 83, 131] {
            for width in [1, 13, 16] {
                let equations: Vec<Vec<WordMask>> = (0..21)
                    .map(|_| (0..width).map(|_| [draw(), draw(), draw()]).collect())
                    .collect();
                let rhs: Vec<_> = (0..21).map(|_| [draw(), draw(), draw()]).collect();
                let map = PackedRowMap::new(&equations, &rhs, n).unwrap();
                assert_eq!(map.rows, 16);
                assert_eq!(
                    map.coefficients.len(),
                    (n as usize).div_ceil(map.bits) * (1 << map.bits)
                );
                let mut pairs = vec![([0; 3], [0; 3])];
                for bit in 0..n as usize {
                    let mut e = [0; 3];
                    e[bit / 64] = 1u64 << (bit % 64);
                    pairs.push((e, [0; 3]));
                    pairs.push(([0; 3], e));
                }
                for _ in 0..128 {
                    let z = words(&F2mElement::from_words(&[draw(), draw(), draw()], n));
                    let t = words(&F2mElement::from_words(&[draw(), draw(), draw()], n));
                    pairs.push((z, t));
                }
                for (z, target) in pairs {
                    let (packed, targets) = map.evaluate(z, target);
                    for j in 0..map.rows {
                        let actual = (packed[j / 4] >> ((j % 4) * 16)) & 0xffff;
                        let expected = equations[j]
                            .iter()
                            .enumerate()
                            .fold(0, |r, (i, col)| r | (parity(col, &z) << i));
                        assert_eq!(actual, expected, "n{n}/width{width}/row{j}");
                        assert_eq!(((targets >> j) & 1) as u64, parity(&rhs[j], &target));
                    }
                }
            }
            assert!(PackedRowMap::new(&vec![vec![[0; 3]; 17]; 21], &[[0; 3]; 21], n).is_none());
        }
        assert!(PackedRowMap::new(&[], &[], 5).is_none());
    }

    #[test]
    fn packed_basis_preserves_reference_pivots_and_numeric_coordinate_order() {
        let mut seed = 202611153u64;
        for n in [5, 83, 131] {
            let mut values = Vec::new();
            for _ in 0..160 {
                let mut w = [0; 3];
                for word in &mut w {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    *word = seed;
                }
                values.push(F2mElement::from_words(&w, n));
            }
            values.sort_by_key(x_key);
            assert!(values
                .windows(2)
                .all(|v| v[0].to_biguint() <= v[1].to_biguint()));
            let packed = word_basis(values.iter().map(words), n);
            let reference = basis(values.iter().map(F2mElement::to_biguint), n);
            assert_eq!(packed, reference);
        }
    }

    fn small_curve() -> BinaryCurve {
        BinaryCurve {
            m: 5,
            irreducible: IrreduciblePoly {
                degree: 5,
                low_terms: vec![0, 2],
            },
            a: F2mElement::zero(5),
            b: F2mElement::one(5),
            generator: BinaryPoint::Infinity,
            order: 11u32.into(),
            cofactor: 4u32.into(),
        }
    }

    #[test]
    fn shared_x_admission_rejects_other_ordinates_and_wrong_coordinate_widths() {
        let c = small_curve();
        let v = TraceZeroVariety::new(&c).unwrap();
        let mut point = None;
        for x in 0..32u32 {
            for y in 0..32u32 {
                let p = BinaryPoint::Affine {
                    x: F2mElement::from_biguint(&x.into(), 5),
                    y: F2mElement::from_biguint(&y.into(), 5),
                };
                if v.contains(&p) {
                    point = Some(p);
                    break;
                }
            }
            if point.is_some() {
                break;
            }
        }
        let p = point.unwrap();
        let BinaryPoint::Affine { x, y } = &p else {
            unreachable!()
        };
        for bad in [
            BinaryPoint::Affine {
                x: x.clone(),
                y: y.add(&F2mElement::one(5)),
            },
            BinaryPoint::Affine {
                x: F2mElement::from_biguint(&x.to_biguint(), 7),
                y: y.clone(),
            },
            BinaryPoint::Affine {
                x: x.clone(),
                y: F2mElement::from_biguint(&y.to_biguint(), 7),
            },
        ] {
            let r = decompose_trace_zero_linearized(&v, &[p.clone(), bad], &p, Limits::default());
            assert_eq!(r.relation.disposition, Disposition::Inconclusive);
            assert_eq!(r.relation.reason, Some("invalid linearized target/base"));
        }
    }

    #[test]
    fn projected_unique_inconsistent_and_deficient_results_match_complete_s3_spaces() {
        let c = small_curve();
        let variety = TraceZeroVariety::new(&c).unwrap();
        let mut points = Vec::new();
        for x in 0..32u32 {
            for y in 0..32u32 {
                let p = BinaryPoint::Affine {
                    x: F2mElement::from_biguint(&x.into(), 5),
                    y: F2mElement::from_biguint(&y.into(), 5),
                };
                if variety.contains(&p) {
                    points.push(p);
                }
            }
        }
        let mut classifications = [0usize; 3];
        for count in [1, 2, 3] {
            let xs = basis(
                points
                    .iter()
                    .filter_map(|p| {
                        if let BinaryPoint::Affine { x, .. } = p {
                            Some(x.to_biguint())
                        } else {
                            None
                        }
                    })
                    .take(count),
                5,
            );
            let products: Vec<_> = xs
                .iter()
                .flat_map(|x| xs.iter().map(|y| x.mul(y, &c.irreducible).to_biguint()))
                .collect();
            let ws = basis(products, 5);
            let projector = ProductProjection::new(&xs, &ws, &c);
            let mut scalar = ProductProjection::new(&xs, &ws, &c);
            scalar.row_map = None;
            let mut tail = ProductProjection::new(&xs, &ws, &c);
            tail.equations
                .splice(0..0, vec![vec![[0; 3]; xs.len() + 1]; 16]);
            tail.rhs.splice(0..0, [[0; 3]; 16]);
            tail.row_map = PackedRowMap::new(&tail.equations, &tail.rhs, c.m);
            for p in &points {
                let BinaryPoint::Affine { x: z, y } = p else {
                    unreachable!()
                };
                let mut expected = Vec::new();
                for ubits in 0..1usize << xs.len() {
                    for vbits in 0..1usize << ws.len() {
                        let u = xs
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| ubits >> i & 1 != 0)
                            .fold(F2mElement::zero(5), |a, (_, x)| a.add(x));
                        let product = ws
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| vbits >> i & 1 != 0)
                            .fold(F2mElement::zero(5), |a, (_, x)| a.add(x));
                        if product
                            .square(&c.irreducible)
                            .add(&z.mul(&product, &c.irreducible))
                            .add(
                                &z.square(&c.irreducible)
                                    .mul(&u.square(&c.irreducible), &c.irreducible),
                            )
                            .add(&c.b)
                            .is_zero()
                        {
                            expected.push((u, product));
                        }
                    }
                }
                let answer = projector.solve(&xs, &c, z, y).0;
                assert_eq!(answer, scalar.solve(&xs, &c, z, y).0);
                assert_eq!(answer, tail.solve(&xs, &c, z, y).0);
                match answer {
                    ProjectedResult::Inconsistent => {
                        classifications[0] += 1;
                        assert!(expected.is_empty());
                    }
                    ProjectedResult::Unique(u, v) => {
                        classifications[1] += 1;
                        assert_eq!(expected, vec![(u, v)]);
                    }
                    ProjectedResult::Deficient => {
                        classifications[2] += 1;
                        assert!(expected.len() > 1);
                    }
                }
            }
        }
        assert!(
            classifications.iter().all(|n| *n > 0),
            "exercise every classification: {classifications:?}"
        );
    }

    #[test]
    fn packed_elimination_matches_generic_solutions_kernels_and_inconsistency() {
        let mut seed = 20261115u64;
        for n in [5, 7, 9, 83, 131, 192] {
            for width in [0, 1, 12, 35, 83, 131, 150, 192, 193, 262] {
                let mut next = || {
                    let positions: Vec<_> = (0..n)
                        .filter(|_| {
                            seed ^= seed << 13;
                            seed ^= seed >> 7;
                            seed ^= seed << 17;
                            seed & 1 != 0
                        })
                        .collect();
                    F2mElement::from_bit_positions(&positions, n)
                };
                let columns: Vec<_> = (0..width).map(|_| next()).collect();
                let random = next();
                let consistent = columns
                    .iter()
                    .step_by(3)
                    .fold(F2mElement::zero(n), |s, c| s.add(c));
                for target in [random, consistent] {
                    assert_eq!(
                        linear_solutions(&columns, &target, n),
                        generic_linear_solutions(&columns, &target, n),
                        "n={n}, width={width}"
                    );
                }
            }
        }
    }

    #[test]
    fn batched_residuals_match_scalar_group_law_including_all_exceptions() {
        let c = small_curve();
        let mut points = vec![BinaryPoint::Infinity];
        for x in 0..32u32 {
            for y in 0..32u32 {
                let p = BinaryPoint::Affine {
                    x: F2mElement::from_biguint(&x.into(), 5),
                    y: F2mElement::from_biguint(&y.into(), 5),
                };
                if c.is_on_curve(&p) {
                    points.push(p);
                }
            }
        }
        for target in &points {
            let got = residual_batch(&c, target, &points);
            let want: Vec<_> = points
                .iter()
                .map(|p| point_add(&c, target, &point_neg(p)))
                .collect();
            assert_eq!(got, want);
            assert!(residual_batch(&c, target, &[]).is_empty());
        }
    }

    #[test]
    fn specialized_monic_roots_match_complete_field_enumeration_and_old_solver() {
        let c = small_curve();
        for u in 0..32u32 {
            for v in 0..32u32 {
                let u = F2mElement::from_biguint(&u.into(), 5);
                let v = F2mElement::from_biguint(&v.into(), 5);
                let got = monic_roots(&u, &v, 5, &c.irreducible);
                let old = crate::cryptanalysis::binary_semaev::solve_quadratic_f2m(
                    &F2mElement::one(5),
                    &u,
                    &v,
                    5,
                    &c.irreducible,
                );
                assert_eq!(got, old, "root order must remain unchanged");
                let mut want: Vec<_> = (0..32u32)
                    .map(|x| F2mElement::from_biguint(&x.into(), 5))
                    .filter(|x| {
                        x.square(&c.irreducible)
                            .add(&u.mul(x, &c.irreducible))
                            .add(&v)
                            .is_zero()
                    })
                    .map(|x| x.to_biguint())
                    .collect();
                let mut actual: Vec<_> = got.iter().map(|x| x.to_biguint()).collect();
                want.sort();
                actual.sort();
                assert_eq!(actual, want);
            }
        }
    }
}
