//! Public-input arithmetic only. C++ owns the frozen MT19937 label stream.
use crypto_lib::cryptanalysis::prime_fast::{BatchAddScratch, FastCurve, Pt};
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CPoint {
    pub x: u64,
    pub y: u64,
    pub infinity: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLabel {
    pub point: CPoint,
    pub alpha: u64,
    pub beta: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Counts {
    pub batch_inversions: u64,
    pub exceptional_doubling_inversions: u64,
    pub ordinary_additions: u64,
    pub table_entries: u64,
    pub combinations: u64,
    pub pair_coordinates: u64,
    pub exact_lookups: u64,
    pub uninformative: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Witness {
    pub informative: u64,
    pub i: u64,
    pub j: u64,
    pub k: u64,
    pub second_sign: i64,
    pub third_sign: i64,
    pub scalar: u64,
}
pub struct Context {
    curve: FastCurve,
    g: Pt,
    q: Pt,
    tables: Vec<[Option<Pt>; 16]>,
    scratch: BatchAddScratch,
    counts: Counts,
}
impl Context {
    fn decode(&self, p: CPoint) -> Option<Pt> {
        if p.infinity != 0 {
            None
        } else {
            Some(self.curve.point(p.x, p.y))
        }
    }
    fn encode(&self, p: Option<Pt>) -> CPoint {
        p.map_or(
            CPoint {
                x: 0,
                y: 0,
                infinity: 1,
            },
            |p| {
                let (x, y) = self.curve.canonical(p);
                CPoint { x, y, infinity: 0 }
            },
        )
    }
    fn batch(&mut self, left: &mut [Option<Pt>], right: &[Option<Pt>]) {
        // The reused library reports the shared inversion separately. Its
        // exceptional doubling path performs another inversion per such lane.
        self.counts.exceptional_doubling_inversions += left
            .iter()
            .zip(right)
            .filter(|(a, b)| matches!((a,b), (Some(p),Some(q)) if p == q && p.y != 0))
            .count() as u64;
        let s = self.curve.add_many_batched(left, right, &mut self.scratch);
        self.counts.batch_inversions += s.inversions as u64;
        self.counts.ordinary_additions += s.ordinary as u64;
    }
    fn prepare(&mut self) {
        if !self.tables.is_empty() {
            return;
        }
        let windows = (64 - (self.curve.n - 1).leading_zeros() as usize).div_ceil(2);
        let mut gs = Vec::with_capacity(windows);
        let mut qs = Vec::with_capacity(windows);
        let (mut g, mut q) = (Some(self.g), Some(self.q));
        for j in 0..windows {
            gs.push(g);
            qs.push(q);
            if j + 1 < windows {
                g = self.curve.mul(g, 4);
                q = self.curve.mul(q, 4);
            }
        }
        self.tables.resize(windows, [None; 16]);
        // Entry index is the two base-4 digits a + 4b. Batch across windows.
        for b in 0..4 {
            for a in 0..4 {
                if a == 0 && b == 0 {
                    continue;
                }
                let (parent, right) = if a > 0 {
                    (a - 1 + 4 * b, &gs)
                } else {
                    (4 * (b - 1), &qs)
                };
                let mut left: Vec<_> = self.tables.iter().map(|t| t[parent]).collect();
                self.batch(&mut left, right);
                for (t, p) in self.tables.iter_mut().zip(left) {
                    t[a + 4 * b] = p;
                }
            }
        }
        self.counts.table_entries = (16 * windows) as u64;
    }
    fn combinations(&mut self, labels: &[(u64, u64)], batched: bool) -> Vec<Option<Pt>> {
        self.counts.combinations += labels.len() as u64;
        if !batched {
            return labels
                .iter()
                .map(|&(a, b)| self.curve.combine(self.g, self.q, a, b))
                .collect();
        }
        self.prepare();
        let mut out = vec![None; labels.len()];
        let mut right = vec![None; labels.len()];
        for j in 0..self.tables.len() {
            for (dest, &(a, b)) in right.iter_mut().zip(labels) {
                *dest = self.tables[j][(((a >> (2 * j)) & 3) + 4 * ((b >> (2 * j)) & 3)) as usize];
            }
            self.batch(&mut out, &right);
        }
        out
    }
}

fn mix(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58476d1ce4e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
fn signed(x: u64, s: i64, r: u64) -> u64 {
    if s < 0 && x != 0 {
        r - x
    } else {
        x
    }
}
fn add3(a: u64, b: u64, c: u64, r: u64) -> u64 {
    ((a as u128 + b as u128 + c as u128) % r as u128) as u64
}

impl Context {
    fn support(
        &mut self,
        pools: [&[CLabel]; 3],
        hits: &mut [u8],
        bloom: bool,
        complete: bool,
    ) -> Result<Witness, ()> {
        let f = self.curve.f;
        let third: Vec<_> = pools[2]
            .iter()
            .map(|l| self.decode(l.point).ok_or(()))
            .collect::<Result<_, _>>()?;
        let index: HashMap<_, _> = third.iter().enumerate().map(|(i, p)| (p.x, i)).collect();
        let bits = (third.len().saturating_mul(64).max(64)).next_power_of_two();
        let mut filter = vec![0u64; if bloom { bits / 64 } else { 0 }];
        if bloom {
            for p in &third {
                let h = mix(p.x);
                for bit in [h as usize & (bits - 1), (h >> 32) as usize & (bits - 1)] {
                    filter[bit / 64] |= 1 << (bit % 64);
                }
            }
        }
        let second: Vec<_> = pools[1]
            .iter()
            .map(|l| self.decode(l.point).ok_or(()))
            .collect::<Result<_, _>>()?;
        let mut den = vec![0; second.len()];
        let mut prefix = Vec::new();
        let mut witness = Witness::default();
        for (i, first) in pools[0].iter().enumerate() {
            let p = self.decode(first.point).ok_or(())?;
            for (d, q) in den.iter_mut().zip(&second) {
                *d = f.sub(q.x, p.x);
                if *d == 0 {
                    return Err(());
                }
            }
            f.batch_inv(&mut den, &mut prefix);
            self.counts.batch_inversions += 1;
            for (j, (&q, &inv)) in second.iter().zip(&den).enumerate() {
                for s2 in [1, -1] {
                    let q = if s2 == 1 { q } else { self.curve.neg(q) };
                    let x = self.curve.add_x_with_inv(p, q, inv);
                    self.counts.pair_coordinates += 1;
                    if bloom {
                        let h = mix(x);
                        if [h as usize & (bits - 1), (h >> 32) as usize & (bits - 1)]
                            .iter()
                            .any(|&bit| filter[bit / 64] & (1 << (bit % 64)) == 0)
                        {
                            continue;
                        }
                    }
                    self.counts.exact_lookups += 1;
                    let Some(&k) = index.get(&x) else { continue };
                    hits[k] = 1;
                    let sum = self.curve.add_with_inv(p, q, inv);
                    let s3 = if sum.y == third[k].y {
                        -1
                    } else if sum.y == f.neg(third[k].y) {
                        1
                    } else {
                        return Err(());
                    };
                    let r = self.curve.n;
                    let a = add3(
                        first.alpha,
                        signed(pools[1][j].alpha, s2, r),
                        signed(pools[2][k].alpha, s3, r),
                        r,
                    );
                    let b = add3(
                        first.beta,
                        signed(pools[1][j].beta, s2, r),
                        signed(pools[2][k].beta, s3, r),
                        r,
                    );
                    if b == 0 {
                        if a != 0 {
                            return Err(());
                        }
                        self.counts.uninformative += 1;
                        continue;
                    }
                    if witness.informative == 0 {
                        let sf = self.curve.scalar_field();
                        let scalar = sf.from_mont(sf.mul(
                            sf.to_mont(if a == 0 { 0 } else { r - a }),
                            sf.inv(sf.to_mont(b)),
                        ));
                        if self.curve.mul(Some(self.g), scalar) != Some(self.q) {
                            return Err(());
                        }
                        witness = Witness {
                            informative: 1,
                            i: i as u64,
                            j: j as u64,
                            k: k as u64,
                            second_sign: s2,
                            third_sign: s3,
                            scalar,
                        };
                        if !complete {
                            return Ok(witness);
                        }
                    }
                }
            }
        }
        Ok(witness) // Full traversal even after an informative witness.
    }
}

/// Creates a public-input context; returns null for invalid domain/points.
#[no_mangle]
pub extern "C" fn ti_context_new(
    p: u64,
    a: u64,
    b: u64,
    r: u64,
    g: CPoint,
    q: CPoint,
    prepare: u64,
) -> *mut Context {
    catch_unwind(|| {
        if p <= 3
            || p >= (1 << 62)
            || p % 2 == 0
            || r <= 3
            || r >= (1 << 62)
            || r % 2 == 0
            || a >= p
            || b >= p
            || g.infinity != 0
            || q.infinity != 0
            || g.x >= p
            || g.y >= p
            || q.x >= p
            || q.y >= p
            || !is_prime_u64(p)
            || !is_prime_u64(r)
        {
            return std::ptr::null_mut();
        }
        // Nonsingularity is additionally checked by the C++ input boundary.
        let curve = FastCurve::new("trace-extension", p, a, b, r);
        let (g, q) = (curve.point(g.x, g.y), curve.point(q.x, q.y));
        if !curve.is_on_curve(g)
            || !curve.is_on_curve(q)
            || curve.mul(Some(g), r).is_some()
            || curve.mul(Some(q), r).is_some()
        {
            return std::ptr::null_mut();
        }
        let mut ctx = Context {
            curve,
            g,
            q,
            tables: Vec::new(),
            scratch: BatchAddScratch::default(),
            counts: Counts::default(),
        };
        if prepare != 0 {
            ctx.prepare();
        }
        Box::into_raw(Box::new(ctx))
    })
    .unwrap_or(std::ptr::null_mut())
}
/// # Safety
/// `ctx` must be null or an unfreed result of `ti_context_new`.
#[no_mangle]
pub unsafe extern "C" fn ti_context_free(ctx: *mut Context) {
    if !ctx.is_null() {
        drop(unsafe { Box::from_raw(ctx) });
    }
}
/// # Safety
/// Exclusive valid context, arrays of n entries (2n for labels); no overlap.
#[no_mangle]
pub unsafe extern "C" fn ti_combine(
    ctx: *mut Context,
    labels: *const u64,
    n: usize,
    out: *mut CPoint,
    batched: u64,
) -> i32 {
    if ctx.is_null() || labels.is_null() || out.is_null() || n > 1_000_000 {
        return -1;
    }
    catch_unwind(AssertUnwindSafe(|| {
        let ctx = unsafe { &mut *ctx };
        let raw = unsafe { std::slice::from_raw_parts(labels, 2 * n) };
        let labels: Vec<_> = raw.chunks_exact(2).map(|s| (s[0], s[1])).collect();
        if labels
            .iter()
            .any(|&(a, b)| a >= ctx.curve.n || b >= ctx.curve.n)
        {
            return -1;
        }
        let points = ctx.combinations(&labels, batched != 0);
        for (dest, p) in unsafe { std::slice::from_raw_parts_mut(out, n) }
            .iter_mut()
            .zip(points)
        {
            *dest = ctx.encode(p);
        }
        0
    }))
    .unwrap_or(-2)
}
/// # Safety
/// Valid context, three finite globally x-distinct labelled pools and their
/// lengths; output hits has n2 entries and witness is writable. Labels < r.
#[no_mangle]
pub unsafe extern "C" fn ti_support(
    ctx: *mut Context,
    p0: *const CLabel,
    n0: usize,
    p1: *const CLabel,
    n1: usize,
    p2: *const CLabel,
    n2: usize,
    hits: *mut u8,
    witness: *mut Witness,
    bloom: u64,
    complete: u64,
) -> i32 {
    if ctx.is_null()
        || p0.is_null()
        || p1.is_null()
        || p2.is_null()
        || hits.is_null()
        || witness.is_null()
        || [n0, n1, n2].iter().any(|&n| n == 0 || n > 1_000_000)
    {
        return -1;
    }
    catch_unwind(AssertUnwindSafe(|| {
        let pools = unsafe {
            [
                std::slice::from_raw_parts(p0, n0),
                std::slice::from_raw_parts(p1, n1),
                std::slice::from_raw_parts(p2, n2),
            ]
        };
        let hits = unsafe { std::slice::from_raw_parts_mut(hits, n2) };
        hits.fill(0);
        match unsafe { &mut *ctx }.support(pools, hits, bloom != 0, complete != 0) {
            Ok(w) => {
                unsafe { *witness = w };
                0
            }
            Err(()) => -3,
        }
    }))
    .unwrap_or(-2)
}
/// # Safety
/// Valid context and writable Counts.
#[no_mangle]
pub unsafe extern "C" fn ti_counts(ctx: *const Context, out: *mut Counts) -> i32 {
    if ctx.is_null() || out.is_null() {
        return -1;
    }
    unsafe {
        *out = (*ctx).counts;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    fn context(qneg: bool) -> Context {
        let curve = FastCurve::new("toy", 97, 2, 3, 5);
        let g = curve.point(3, 6);
        let q = if qneg { curve.neg(g) } else { g };
        Context {
            curve,
            g,
            q,
            tables: Vec::new(),
            scratch: BatchAddScratch::default(),
            counts: Counts::default(),
        }
    }
    #[test]
    fn batch_exhaustive_equal_and_opposite_points() {
        for neg in [false, true] {
            let mut c = context(neg);
            let labels: Vec<_> = (0..5).flat_map(|a| (0..5).map(move |b| (a, b))).collect();
            let expected = c.combinations(&labels, false);
            assert_eq!(expected, c.combinations(&labels, true));
            assert!(expected.iter().any(Option::is_none));
        }
    }
    #[test]
    fn batch_wide_labels_with_short_final_window() {
        let curve = FastCurve::new("fixture", 1000003, 2, 3, 1000033);
        let g = (0..1000003)
            .find_map(|x| {
                (0..1000).find_map(|y| {
                    let p = curve.point(x, y);
                    curve.is_on_curve(p).then_some(p)
                })
            })
            .unwrap();
        let q = curve.mul(Some(g), 19).unwrap();
        let mut c = Context {
            curve,
            g,
            q,
            tables: Vec::new(),
            scratch: BatchAddScratch::default(),
            counts: Counts::default(),
        };
        let labels: Vec<_> = (0..311)
            .map(|i| ((i * 7919) % 1000033, (i * 9973) % 1000033))
            .collect();
        let expected = c.combinations(&labels, false);
        assert_eq!(expected, c.combinations(&labels, true));
        assert_eq!(expected, c.combinations(&labels, true));
    }
}
