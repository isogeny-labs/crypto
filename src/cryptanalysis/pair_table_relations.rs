//! Signed pair-table relation source for prime-field elliptic curves.
//!
//! For a factor base of rational points `P_0..P_{B-1}` on
//! `E : y^2 = x^3 + a*x + b` over `F_p`, the table stores, for each unordered
//! pair `i < j`, the two points `S = P_i + P_j` and `D = P_i - P_j`, both
//! computed from one shared inverse of `x_j - x_i` and Montgomery batch
//! inversion. It is indexed by abscissa only: a point `C` with `x(C) = x(E)`
//! is `+E` or `-E`, decided by the ordinate, so `+-S` and `+-D` share one key.
//!
//! A query point `R` is probed against a window of table pairs. For each
//! stored `L` in `{S, D}` the complements `R - L` and `R + L` again share one
//! inverse `1/(x_L - x_R)`. Every complement found in the table yields a
//! signed four-term relation `R = (+-L) + (+-E)` with its signs known from
//! construction, so callers need no sign lift. Every hit in the window is
//! reported, not just the first.
//!
//! This is a generic (enumerative) source: its cold table-plus-probe work is
//! bounded below by about `sqrt(B * n)` for a fixed relation target, so it
//! improves constants, not the exponent of rho.

/// Prime-field multiplication strategy, monomorphized into the hot loops.
pub trait FieldMul: Copy + Send + Sync {
    fn p(&self) -> u64;
    fn mul(&self, a: u64, b: u64) -> u64;
}

/// `p = 2^31 - 1`: two-fold Mersenne reduction.
#[derive(Clone, Copy, Debug)]
pub struct Mersenne31;

impl FieldMul for Mersenne31 {
    #[inline(always)]
    fn p(&self) -> u64 {
        0x7fff_ffff
    }
    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        const P: u64 = 0x7fff_ffff;
        let x = a * b;
        let x = (x & P) + (x >> 31);
        let x = (x & P) + (x >> 31);
        if x >= P {
            x - P
        } else {
            x
        }
    }
}

/// Any odd prime below `2^63`: 128-bit product and remainder.
#[derive(Clone, Copy, Debug)]
pub struct GenericPrime(pub u64);

impl FieldMul for GenericPrime {
    #[inline(always)]
    fn p(&self) -> u64 {
        self.0
    }
    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.0 as u128) as u64
    }
}

#[inline(always)]
fn sub(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + p - b
    }
}

fn pow<F: FieldMul>(f: &F, mut x: u64, mut e: u64) -> u64 {
    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = f.mul(r, x);
        }
        x = f.mul(x, x);
        e >>= 1;
    }
    r
}

fn inv<F: FieldMul>(f: &F, x: u64) -> u64 {
    assert!(x != 0);
    pow(f, x, f.p() - 2)
}

/// Montgomery simultaneous inversion of nonzero `xs` in place.
fn batch_inv<F: FieldMul>(f: &F, xs: &mut [u64], prefix: &mut Vec<u64>) {
    if xs.is_empty() {
        return;
    }
    prefix.clear();
    let mut acc = 1u64;
    for &x in xs.iter() {
        prefix.push(acc);
        acc = f.mul(acc, x);
    }
    let mut a = inv(f, acc);
    for k in (0..xs.len()).rev() {
        let x = xs[k];
        xs[k] = f.mul(a, prefix[k]);
        a = f.mul(a, x);
    }
}

/// `(U + V, U - V)` for `x(U) != x(V)` given `d = 1/(x_V - x_U)`.
#[inline(always)]
fn sum_diff<F: FieldMul>(f: &F, u: (u64, u64), v: (u64, u64), d: u64) -> ((u64, u64), (u64, u64)) {
    let p = f.p();
    let (x, y) = u;
    let (z, w) = v;
    let lp = f.mul(sub(w, y, p), d);
    let lm = f.mul(sub(p - w, y, p) % p, d);
    let xp = sub(sub(f.mul(lp, lp), x, p), z, p);
    let yp = sub(f.mul(lp, sub(x, xp, p)), y, p);
    let xm = sub(sub(f.mul(lm, lm), x, p), z, p);
    let ym = sub(f.mul(lm, sub(x, xm, p)), y, p);
    ((xp, yp), (xm, ym))
}

const NIL: u32 = u32::MAX;
const BLOCK: usize = 256;

/// A signed four-term relation `R = sum s_k * P_{i_k}` with distinct indices.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SignedFourSum {
    pub terms: [(u32, i8); 4],
}

/// Counters for one table build or a run of probes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PairTableStats {
    pub points_computed: u64,
    pub field_inversions: u64,
    pub probes: u64,
    pub hits: u64,
    pub degenerate_skips: u64,
}

pub struct PairTable<F: FieldMul> {
    f: F,
    points: Vec<(u64, u64)>,
    /// For pair `k`: `[S_k, D_k]`.
    reps: Vec<[(u64, u64); 2]>,
    pair: Vec<(u32, u32)>,
    /// Low 32 bits of the abscissa; a slot is empty when its head is `NIL`.
    keys: Vec<u32>,
    heads: Vec<u32>,
    next: Vec<u32>,
    mask: usize,
    /// One bit per hashed abscissa (about 2^23 bits): rejects most misses in cache.
    filter: Vec<u64>,
    filter_shift: u32,
    pub build_stats: PairTableStats,
}

#[inline(always)]
fn slot_of(x: u64, mask: usize) -> usize {
    (x.wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 20) as usize & mask
}

impl<F: FieldMul> PairTable<F> {
    /// Build from distinct-abscissa rational points (checked on the curve by the caller).
    pub fn build(f: F, points: &[(u64, u64)]) -> Self {
        Self::build_threads(f, points, 1)
    }

    /// As [`PairTable::build`], computing the pair sums on `threads` threads.
    /// The table is identical for every thread count.
    pub fn build_threads(f: F, points: &[(u64, u64)], threads: usize) -> Self {
        let p = f.p();
        let n = points.len();
        assert!(n >= 4 && n < u32::MAX as usize / 2);
        let npairs = n * (n - 1) / 2;
        let mut pair = Vec::with_capacity(npairs);
        for i in 0..n {
            for j in i + 1..n {
                assert_ne!(points[i].0, points[j].0, "distinct abscissae required");
                pair.push((i as u32, j as u32));
            }
        }
        // Fixed pseudo-random pair order, so that a contiguous probe window is
        // a uniform subset: the six split pairs of one relation then fall in
        // a window independently, instead of together (pairs sharing `i` are
        // adjacent in lexicographic order).
        let mut state = 0x5041_4952_5f54_4142u64;
        for k in (1..npairs).rev() {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let r = ((state >> 33) as usize) % (k + 1);
            pair.swap(k, r);
        }
        let mut stats = PairTableStats::default();
        let compute = |pairs: &[(u32, u32)]| -> Vec<[(u64, u64); 2]> {
            let mut out = Vec::with_capacity(pairs.len());
            let mut dens = Vec::with_capacity(BLOCK);
            let mut prefix = Vec::with_capacity(BLOCK);
            for chunk in pairs.chunks(BLOCK) {
                dens.clear();
                dens.extend(
                    chunk
                        .iter()
                        .map(|&(i, j)| sub(points[j as usize].0, points[i as usize].0, p)),
                );
                batch_inv(&f, &mut dens, &mut prefix);
                for (t, &(i, j)) in chunk.iter().enumerate() {
                    let (s, d) = sum_diff(&f, points[i as usize], points[j as usize], dens[t]);
                    out.push([s, d]);
                }
            }
            out
        };
        let threads = threads.max(1);
        let reps: Vec<[(u64, u64); 2]> = if threads == 1 {
            compute(&pair)
        } else {
            let per = npairs.div_ceil(threads).div_ceil(BLOCK) * BLOCK;
            std::thread::scope(|sc| {
                let hs: Vec<_> = pair
                    .chunks(per)
                    .map(|c| sc.spawn(move || compute(c)))
                    .collect();
                hs.into_iter().flat_map(|h| h.join().unwrap()).collect()
            })
        };
        stats.field_inversions = npairs.div_ceil(BLOCK) as u64;
        stats.points_computed = 2 * npairs as u64;
        let size = (4 * npairs).next_power_of_two();
        let filter_bits = (16 * npairs).next_power_of_two().max(64);
        let mut table = Self {
            f,
            points: points.to_vec(),
            reps,
            pair,
            keys: vec![0; size],
            heads: vec![NIL; size],
            next: vec![NIL; 2 * npairs],
            mask: size - 1,
            filter: vec![0; filter_bits / 64],
            filter_shift: 64 - filter_bits.trailing_zeros(),
            build_stats: stats,
        };
        for k in 0..npairs {
            for w in 0..2 {
                let x = table.reps[k][w].0;
                table.insert(x, (2 * k + w) as u32);
            }
        }
        table
    }

    #[inline(always)]
    fn x_of(&self, id: u32) -> u64 {
        self.reps[(id >> 1) as usize][(id & 1) as usize].0
    }

    #[inline(always)]
    fn filter_bit(&self, x: u64) -> usize {
        (x.wrapping_mul(0xd6e8_feb8_6659_fd93) >> self.filter_shift) as usize
    }

    #[inline(always)]
    fn maybe(&self, x: u64) -> bool {
        let b = self.filter_bit(x);
        self.filter[b >> 6] >> (b & 63) & 1 == 1
    }

    fn insert(&mut self, x: u64, id: u32) {
        let b = self.filter_bit(x);
        self.filter[b >> 6] |= 1 << (b & 63);
        let mut s = slot_of(x, self.mask);
        loop {
            if self.heads[s] == NIL {
                self.keys[s] = x as u32;
                self.heads[s] = id;
                return;
            }
            if self.keys[s] == x as u32 && self.x_of(self.heads[s]) == x {
                self.next[id as usize] = self.heads[s];
                self.heads[s] = id;
                return;
            }
            s = (s + 1) & self.mask;
        }
    }

    #[inline(always)]
    fn find(&self, x: u64) -> u32 {
        let mut s = slot_of(x, self.mask);
        loop {
            let h = self.heads[s];
            if h == NIL {
                return NIL;
            }
            if self.keys[s] == x as u32 && self.x_of(h) == x {
                return h;
            }
            s = (s + 1) & self.mask;
        }
    }

    pub fn pairs(&self) -> usize {
        self.pair.len()
    }

    pub fn points(&self) -> &[(u64, u64)] {
        &self.points
    }

    /// `+-P_i +-P_j` terms of stored entry `id = 2k + w`, multiplied by `sign`.
    #[inline(always)]
    fn terms(&self, id: u32, sign: i8) -> [(u32, i8); 2] {
        let (i, j) = self.pair[(id >> 1) as usize];
        let sj = if id & 1 == 0 { 1 } else { -1 };
        [(i, sign), (j, sign * sj)]
    }

    /// Probe pairs `start, start+1, ..., start+len-1` (mod pairs) against `r`
    /// and call `emit` for every signed four-term relation found.
    pub fn probe_window(
        &self,
        r: (u64, u64),
        start: usize,
        len: usize,
        stats: &mut PairTableStats,
        mut emit: impl FnMut(SignedFourSum),
    ) {
        let f = &self.f;
        let p = f.p();
        let npairs = self.pair.len();
        let len = len.min(npairs);
        let mut ids: Vec<u32> = Vec::with_capacity(2 * BLOCK);
        let mut dens: Vec<u64> = Vec::with_capacity(2 * BLOCK);
        let mut prefix = Vec::with_capacity(2 * BLOCK);
        // Per complement pair: (x(R+L), slope(R+L), x(R-L), slope(R-L)); the
        // ordinate is computed only for abscissae that pass the filter.
        let mut comps: Vec<[u64; 4]> = Vec::with_capacity(2 * BLOCK);
        let (rx, ry) = r;
        let mut done = 0;
        let mut k = start % npairs.max(1);
        while done < len {
            let take = BLOCK.min(len - done);
            ids.clear();
            dens.clear();
            for _ in 0..take {
                let rep = &self.reps[k];
                for (w, l) in rep.iter().enumerate() {
                    if l.0 == rx {
                        // L = +-R: no four-term relation through this entry.
                        stats.degenerate_skips += 1;
                        continue;
                    }
                    ids.push((2 * k + w) as u32);
                    dens.push(sub(l.0, rx, p));
                }
                k += 1;
                if k == npairs {
                    k = 0;
                }
            }
            done += take;
            batch_inv(f, &mut dens, &mut prefix);
            stats.field_inversions += 1;
            comps.clear();
            for (t, &id) in ids.iter().enumerate() {
                let (lx, ly) = self.reps[(id >> 1) as usize][(id & 1) as usize];
                let d = dens[t];
                // R + L: slope (yL - yR)/(xL - xR); R - L: slope (-yL - yR)/(xL - xR).
                let lp = f.mul(sub(ly, ry, p), d);
                let lm = f.mul(sub((p - ly) % p, ry, p), d);
                let xp = sub(sub(f.mul(lp, lp), rx, p), lx, p);
                let xm = sub(sub(f.mul(lm, lm), rx, p), lx, p);
                comps.push([xp, lp, xm, lm]);
                #[cfg(target_arch = "x86_64")]
                unsafe {
                    use std::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};
                    for x in [xp, xm] {
                        let b = self.filter_bit(x);
                        _mm_prefetch(self.filter.as_ptr().add(b >> 6) as *const i8, _MM_HINT_T0);
                    }
                }
            }
            stats.points_computed += 2 * ids.len() as u64;
            stats.probes += 2 * ids.len() as u64;
            for (t, &id) in ids.iter().enumerate() {
                let [xp, lp, xm, lm] = comps[t];
                // left = -L, complement = R + L ; left = +L, complement = R - L.
                for (cx, slope, left_sign) in [(xp, lp, -1i8), (xm, lm, 1i8)] {
                    if !self.maybe(cx) {
                        continue;
                    }
                    let mut e = self.find(cx);
                    if e == NIL {
                        continue;
                    }
                    let cy = sub(f.mul(slope, sub(rx, cx, p)), ry, p);
                    while e != NIL {
                        let stored = self.reps[(e >> 1) as usize][(e & 1) as usize];
                        let right_sign = if stored.1 == cy { 1 } else { -1 };
                        let [a, b] = self.terms(id, left_sign);
                        let [cc, dd] = self.terms(e, right_sign);
                        if a.0 != cc.0 && a.0 != dd.0 && b.0 != cc.0 && b.0 != dd.0 {
                            let mut terms = [a, b, cc, dd];
                            terms.sort_unstable_by_key(|t| t.0);
                            stats.hits += 1;
                            emit(SignedFourSum { terms });
                        }
                        e = self.next[e as usize];
                    }
                }
            }
        }
    }
}

/// Affine group law helpers for callers and checks (one inversion per addition).
pub fn affine_add<F: FieldMul>(
    f: &F,
    a: i64,
    u: Option<(u64, u64)>,
    v: Option<(u64, u64)>,
) -> Option<(u64, u64)> {
    let p = f.p();
    let (Some((x1, y1)), Some((x2, y2))) = (u, v) else {
        return u.or(v);
    };
    if x1 == x2 && (y1 + y2) % p == 0 {
        return None;
    }
    let a = a.rem_euclid(p as i64) as u64;
    let l = if x1 == x2 {
        let num = (3 * f.mul(x1, x1) % p + a) % p;
        f.mul(num, inv(f, 2 * y1 % p))
    } else {
        f.mul(sub(y2, y1, p), inv(f, sub(x2, x1, p)))
    };
    let x3 = sub(sub(f.mul(l, l), x1, p), x2, p);
    let y3 = sub(f.mul(l, sub(x1, x3, p)), y1, p);
    Some((x3, y3))
}

pub fn affine_neg(p: u64, u: Option<(u64, u64)>) -> Option<(u64, u64)> {
    u.map(|(x, y)| (x, (p - y) % p))
}

pub fn affine_mul<F: FieldMul>(
    f: &F,
    a: i64,
    mut k: u64,
    mut u: Option<(u64, u64)>,
) -> Option<(u64, u64)> {
    let mut r = None;
    while k > 0 {
        if k & 1 == 1 {
            r = affine_add(f, a, r, u);
        }
        u = affine_add(f, a, u, u);
        k >>= 1;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// Rational points with distinct abscissae on `y^2 = x^3 + a x + b`, `p = 3 mod 4`.
    fn points<F: FieldMul>(f: &F, a: u64, b: u64, count: usize) -> Vec<(u64, u64)> {
        let p = f.p();
        assert_eq!(p % 4, 3);
        let mut out = Vec::new();
        let mut x = 1u64;
        while out.len() < count {
            let rhs = (f.mul(f.mul(x, x), x) + f.mul(a, x) + b) % p;
            let y = pow(f, rhs, (p + 1) / 4);
            if rhs != 0 && f.mul(y, y) == rhs {
                out.push((x, y));
            }
            x += 1;
        }
        out
    }

    fn signed_sum<F: FieldMul>(
        f: &F,
        a: i64,
        pts: &[(u64, u64)],
        terms: &[(u32, i8)],
    ) -> Option<(u64, u64)> {
        let mut s = None;
        for &(i, sg) in terms {
            let pt = Some(pts[i as usize]);
            s = affine_add(f, a, s, if sg > 0 { pt } else { affine_neg(f.p(), pt) });
        }
        s
    }

    #[test]
    fn full_window_equals_brute_force_four_sums() {
        let f = GenericPrime(1019);
        let (a, b) = (2u64, 3u64);
        let pts = points(&f, a, b, 18);
        let table = PairTable::build(f, &pts);
        let n = pts.len() as u32;
        for target_terms in [
            [(0u32, 1i8), (3, -1), (7, 1), (11, 1)],
            [(1, 1), (2, 1), (5, -1), (16, -1)],
        ] {
            let r = signed_sum(&f, a as i64, &pts, &target_terms).unwrap();
            let mut brute = BTreeSet::new();
            for i in 0..n {
                for j in i + 1..n {
                    for k in j + 1..n {
                        for l in k + 1..n {
                            for mask in 0..16u8 {
                                let sg = |t: u8| if mask >> t & 1 == 1 { -1 } else { 1 };
                                let terms = [(i, sg(0)), (j, sg(1)), (k, sg(2)), (l, sg(3))];
                                if signed_sum(&f, a as i64, &pts, &terms) == Some(r) {
                                    brute.insert(terms);
                                }
                            }
                        }
                    }
                }
            }
            let mut found = BTreeSet::new();
            let mut st = PairTableStats::default();
            table.probe_window(r, 0, table.pairs(), &mut st, |rel| {
                assert_eq!(signed_sum(&f, a as i64, &pts, &rel.terms), Some(r));
                found.insert(rel.terms);
            });
            assert!(brute.contains(&target_terms));
            assert_eq!(found, brute);
        }
    }

    #[test]
    fn threaded_build_is_identical() {
        let f = Mersenne31;
        let pts = points(&f, 847_514_846, 2_087_444_724, 300);
        let one = PairTable::build_threads(f, &pts, 1);
        let four = PairTable::build_threads(f, &pts, 4);
        assert_eq!(one.reps, four.reps);
        assert_eq!(one.pair, four.pair);
        assert_eq!(one.keys, four.keys);
        assert_eq!(one.heads, four.heads);
        assert_eq!(one.next, four.next);
        assert_eq!(one.filter, four.filter);
    }

    #[test]
    fn mersenne_and_generic_fields_agree() {
        let m = Mersenne31;
        let g = GenericPrime(0x7fff_ffff);
        let (a, b) = (847_514_846u64, 2_087_444_724u64);
        let pts = points(&m, a, b, 200);
        let tm = PairTable::build(m, &pts);
        let tg = PairTable::build(g, &pts);
        assert_eq!(tm.reps, tg.reps);
        let r = signed_sum(&m, a as i64, &pts, &[(4, 1), (9, -1), (50, 1), (120, 1)]).unwrap();
        let (mut vm, mut vg) = (Vec::new(), Vec::new());
        let (mut sm, mut sg) = (PairTableStats::default(), PairTableStats::default());
        tm.probe_window(r, 0, tm.pairs(), &mut sm, |rel| vm.push(rel));
        tg.probe_window(r, 0, tg.pairs(), &mut sg, |rel| vg.push(rel));
        assert_eq!(vm, vg);
        assert_eq!(sm, sg);
        assert!(vm
            .iter()
            .any(|rel| rel.terms == [(4, 1), (9, -1), (50, 1), (120, 1)]));
    }

    #[test]
    fn windows_partition_the_full_probe() {
        let f = Mersenne31;
        let (a, b) = (847_514_846u64, 2_087_444_724u64);
        let pts = points(&f, a, b, 120);
        let t = PairTable::build(f, &pts);
        let r = signed_sum(&f, a as i64, &pts, &[(1, 1), (2, 1), (3, 1), (4, -1)]).unwrap();
        let mut whole = BTreeSet::new();
        let mut st = PairTableStats::default();
        t.probe_window(r, 0, t.pairs(), &mut st, |rel| {
            whole.insert(rel);
        });
        let mut parts = BTreeSet::new();
        let w = t.pairs() / 7 + 1;
        let mut start = 123;
        let mut covered = 0;
        while covered < t.pairs() {
            let len = w.min(t.pairs() - covered);
            t.probe_window(r, start, len, &mut st, |rel| {
                parts.insert(rel);
            });
            start += len;
            covered += len;
        }
        assert_eq!(whole, parts);
    }
}
