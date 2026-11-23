//! Exact prime-field elliptic-curve arithmetic, factor-base construction,
//! modular linear algebra, shared BSGS, and matched negation-map Pollard's rho
//! for `PRIME31` (`p = 2^31 - 1`) and prime-order control curves.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

pub type Point = Option<(u64, u64)>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Curve {
    pub name: &'static str,
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub n: u64,
    pub g: (u64, u64),
}

pub const PRIME31: Curve = Curve {
    name: "PRIME31",
    p: 2_147_483_647,
    a: 847_514_846,
    b: 2_087_444_724,
    n: 2_147_446_831,
    g: (625_037_715, 2_100_627_251),
};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArithCounts {
    pub group_additions: u64,
    pub fp_muls: u64,
    pub fp_invs: u64,
    pub scalar_muls: u64,
    pub scalar_invs: u64,
}

#[derive(Clone, Default)]
pub struct CurveArith {
    pub counts: ArithCounts,
}

#[inline]
pub fn fp_add(a: u64, b: u64, p: u64) -> u64 {
    let s = a + b;
    if s >= p {
        s - p
    } else {
        s
    }
}

#[inline]
pub fn fp_sub(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + p - b
    }
}

#[inline]
pub fn fp_neg(a: u64, p: u64) -> u64 {
    if a == 0 {
        0
    } else {
        p - (a % p)
    }
}

#[inline]
pub fn fp_mul(a: u64, b: u64, p: u64) -> u64 {
    if p == PRIME31.p {
        let x = a * b;
        let y = (x & PRIME31.p) + (x >> 31);
        let z = (y & PRIME31.p) + (y >> 31);
        if z >= PRIME31.p {
            z - PRIME31.p
        } else {
            z
        }
    } else {
        ((a as u128 * b as u128) % p as u128) as u64
    }
}

#[inline]
pub fn fp_pow(mut base: u64, mut exp: u64, p: u64) -> u64 {
    let mut acc = 1u64;
    base %= p;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = fp_mul(acc, base, p);
        }
        base = fp_mul(base, base, p);
        exp >>= 1;
    }
    acc
}

#[inline]
pub fn fp_inv(a: u64, p: u64) -> u64 {
    let a = a % p;
    assert_ne!(a, 0, "zero inversion mod {p}");
    // Extended Euclidean algorithm is ~5x faster than binary exponentiation
    // for 31-bit primes while remaining exact.
    let (mut r0, mut r1) = (p as i64, a as i64);
    let (mut t0, mut t1) = (0i64, 1i64);
    while r1 != 0 {
        let q = r0 / r1;
        let r2 = r0 - q * r1;
        r0 = r1;
        r1 = r2;
        let t2 = t0 - q * t1;
        t0 = t1;
        t1 = t2;
    }
    debug_assert_eq!(r0, 1);
    t0.rem_euclid(p as i64) as u64
}

pub fn fp_sqrt(x: u64, p: u64) -> Option<u64> {
    let x = x % p;
    if x == 0 {
        return Some(0);
    }
    if p % 4 == 3 {
        let r = fp_pow(x, (p + 1) / 4, p);
        return (fp_mul(r, r, p) == x).then_some(r);
    }
    if fp_pow(x, (p - 1) / 2, p) != 1 {
        return None;
    }
    // Tonelli-Shanks for general odd primes.
    let mut q = p - 1;
    let mut s = 0u32;
    while q.is_multiple_of(2) {
        q /= 2;
        s += 1;
    }
    let mut z = 2u64;
    while fp_pow(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = fp_pow(z, q, p);
    let mut t = fp_pow(x, q, p);
    let mut r = fp_pow(x, q.div_ceil(2), p);
    while t != 1 {
        let mut i = 1u32;
        let mut tt = fp_mul(t, t, p);
        while tt != 1 {
            tt = fp_mul(tt, tt, p);
            i += 1;
        }
        let b = fp_pow(c, 1u64 << (m - i - 1), p);
        m = i;
        c = fp_mul(b, b, p);
        t = fp_mul(t, c, p);
        r = fp_mul(r, b, p);
    }
    Some(r)
}

pub fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n.is_multiple_of(2) {
        return n == 2;
    }
    let mut d = 3u64;
    while d <= n / d {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 2;
    }
    true
}

impl CurveArith {
    pub fn rhs(&mut self, c: Curve, x: u64) -> u64 {
        self.counts.fp_muls += 3;
        let x2 = fp_mul(x, x, c.p);
        let x3 = fp_mul(x2, x, c.p);
        let ax = fp_mul(c.a, x, c.p);
        fp_add(fp_add(x3, ax, c.p), c.b, c.p)
    }

    pub fn on_curve(&mut self, c: Curve, pt: Point) -> bool {
        match pt {
            None => true,
            Some((x, y)) => {
                if x >= c.p || y >= c.p {
                    return false;
                }
                self.counts.fp_muls += 1;
                fp_mul(y, y, c.p) == self.rhs(c, x)
            }
        }
    }

    #[inline]
    pub fn neg(&self, c: Curve, pt: Point) -> Point {
        pt.map(|(x, y)| (x, fp_neg(y, c.p)))
    }

    pub fn add(&mut self, c: Curve, u: Point, v: Point) -> Point {
        self.counts.group_additions += 1;
        let (Some((x1, y1)), Some((x2, y2))) = (u, v) else {
            return u.or(v);
        };
        if x1 == x2 && fp_add(y1, y2, c.p) == 0 {
            return None;
        }
        let slope = if x1 == x2 {
            self.counts.fp_muls += 3;
            self.counts.fp_invs += 1;
            let num = fp_add(fp_mul(3, fp_mul(x1, x1, c.p), c.p), c.a, c.p);
            let den = fp_inv(fp_add(y1, y1, c.p), c.p);
            fp_mul(num, den, c.p)
        } else {
            self.counts.fp_muls += 1;
            self.counts.fp_invs += 1;
            let num = fp_sub(y2, y1, c.p);
            let den = fp_inv(fp_sub(x2, x1, c.p), c.p);
            fp_mul(num, den, c.p)
        };
        self.counts.fp_muls += 2;
        let x3 = fp_sub(fp_sub(fp_mul(slope, slope, c.p), x1, c.p), x2, c.p);
        let y3 = fp_sub(fp_mul(slope, fp_sub(x1, x3, c.p), c.p), y1, c.p);
        Some((x3, y3))
    }

    pub fn scalar(&mut self, c: Curve, mut k: u64, mut pt: Point) -> Point {
        let mut acc = None;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(c, acc, pt);
            }
            k >>= 1;
            if k > 0 {
                pt = self.add(c, pt, pt);
            }
        }
        acc
    }

    pub fn verify_curve(&mut self, c: Curve) -> Result<(), String> {
        if !is_prime_u64(c.p) || !is_prime_u64(c.n) {
            return Err("field p and order n must be prime".into());
        }
        let disc = fp_add(
            fp_mul(4, fp_pow(c.a, 3, c.p), c.p),
            fp_mul(27, fp_mul(c.b, c.b, c.p), c.p),
            c.p,
        );
        if disc == 0 {
            return Err("singular Weierstrass curve".into());
        }
        if !self.on_curve(c, Some(c.g)) {
            return Err("generator off curve".into());
        }
        if self.scalar(c, c.n, Some(c.g)).is_some() {
            return Err("[n]G != O".into());
        }
        let hasse = 2 * ((c.p as f64).sqrt() as u64 + 1);
        if (c.n as i128 - (c.p as i128 + 1)).unsigned_abs() > hasse as u128 {
            return Err("order violates Hasse bound".into());
        }
        Ok(())
    }
}

#[inline]
pub fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Canonical factor base `F_B`: first `b_size` rational affine points with `y != 0`,
/// oriented by `y = min(y, p - y)`. Returns `(points, H_B(X) ascending)`.
pub fn build_factor_base(c: Curve, b_size: usize, ar: &mut CurveArith) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut pts = Vec::with_capacity(b_size);
    for x in 0..c.p {
        let rhs = ar.rhs(c, x);
        if let Some(y) = fp_sqrt(rhs, c.p) {
            if y == 0 {
                continue;
            }
            pts.push((x, y.min(c.p - y)));
            if pts.len() == b_size {
                break;
            }
        }
    }
    assert_eq!(pts.len(), b_size, "fewer than {b_size} rational points on {}", c.name);
    let mut h = vec![1u64];
    for &(x, _) in &pts {
        h = poly_mul(&h, &[fp_neg(x, c.p), 1], c.p);
    }
    (pts, h)
}

pub fn poly_mul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![0u64; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        if ai == 0 {
            continue;
        }
        for (j, &bj) in b.iter().enumerate() {
            if bj != 0 {
                out[i + j] = fp_add(out[i + j], fp_mul(ai, bj, p), p);
            }
        }
    }
    out
}

pub fn poly_rem_monic(dividend: &[u64], monic_divisor: &[u64], p: u64) -> Vec<u64> {
    let d = monic_divisor.len().checked_sub(1).expect("empty divisor");
    assert_eq!(monic_divisor[d], 1, "divisor must be monic");
    let mut r = dividend.to_vec();
    while r.len() > d {
        let lead = *r.last().unwrap();
        let shift = r.len() - 1 - d;
        r.pop();
        if lead != 0 {
            for i in 0..d {
                let sub_term = fp_mul(lead, monic_divisor[i], p);
                r[shift + i] = fp_sub(r[shift + i], sub_term, p);
            }
        }
    }
    r.resize(d, 0);
    r
}

pub fn poly_eval(poly: &[u64], x: u64, p: u64) -> u64 {
    let mut acc = 0u64;
    for &c in poly.iter().rev() {
        acc = fp_add(fp_mul(acc, x, p), c, p);
    }
    acc
}

/// Online modular linear-algebra rank accumulator over `Z/nZ` (`B` base logs + `1` target log).
pub struct ModLinearSystem {
    pivots: Vec<Option<Vec<u64>>>,
    pub rank: usize,
    pub dependent: usize,
    n: u64,
}

impl ModLinearSystem {
    pub fn new(num_vars: usize, n: u64) -> Self {
        Self {
            pivots: vec![None; num_vars],
            rank: 0,
            dependent: 0,
            n,
        }
    }

    pub fn insert(&mut self, mut row: Vec<u64>, ar: &mut CurveArith) -> Result<bool, String> {
        let cols = self.pivots.len();
        assert_eq!(row.len(), cols + 1);
        for c in 0..cols {
            if row[c] == 0 {
                continue;
            }
            if let Some(ref piv) = self.pivots[c] {
                let factor = row[c];
                for j in c..=cols {
                    ar.counts.scalar_muls += 1;
                    let prod = fp_mul(factor, piv[j], self.n);
                    row[j] = fp_sub(row[j], prod, self.n);
                }
            } else {
                ar.counts.scalar_invs += 1;
                let inv = fp_inv(row[c], self.n);
                for val in &mut row[c..=cols] {
                    ar.counts.scalar_muls += 1;
                    *val = fp_mul(*val, inv, self.n);
                }
                self.pivots[c] = Some(row);
                self.rank += 1;
                return Ok(true);
            }
        }
        if row[cols] != 0 {
            return Err("inconsistent relation row mod n".into());
        }
        self.dependent += 1;
        Ok(false)
    }

    pub fn solve(&self, ar: &mut CurveArith) -> Option<Vec<u64>> {
        let cols = self.pivots.len();
        if self.rank != cols {
            return None;
        }
        let mut sol = vec![0u64; cols];
        for i in (0..cols).rev() {
            let row = self.pivots[i].as_ref()?;
            let mut rhs = row[cols];
            for j in (i + 1)..cols {
                ar.counts.scalar_muls += 1;
                rhs = fp_sub(rhs, fp_mul(row[j], sol[j], self.n), self.n);
            }
            sol[i] = rhs;
        }
        Some(sol)
    }
}

/// Negation-folded shared BSGS comparator: stores `x([j]G) -> (j, y([j]G))` for `j = 1..=m`
/// and steps by `[2m]G`, testing both `Q - i[2m]G = ±[j]G` per giant step!
/// Expected/worst-case group additions for cold 1-target at `m = ceil(sqrt(n/2))`: `~1.5 sqrt(n/2) ~ 1.06 sqrt(n)`.
pub struct NegationSharedBsgs {
    pub curve: Curve,
    pub m: u64,
    table: HashMap<u64, (u64, u64)>,
    giant_stride_neg: Point,
    pub setup_counts: ArithCounts,
    pub setup_ns: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BsgsQueryReceipt {
    pub scalar: u64,
    pub verified: bool,
    pub giant_steps: u64,
    pub online_counts: ArithCounts,
    pub online_ns: u128,
}

impl NegationSharedBsgs {
    pub fn build(curve: Curve, m: u64) -> Self {
        let t0 = Instant::now();
        let mut ar = CurveArith::default();
        let mut table = HashMap::with_capacity(m as usize + 1);
        let mut cur = None;
        for j in 1..=m {
            cur = ar.add(curve, cur, Some(curve.g));
            if let Some((x, y)) = cur {
                table.entry(x).or_insert((j, y));
            }
        }
        let two_m_g = ar.add(curve, cur, cur);
        let giant_stride_neg = ar.neg(curve, two_m_g);
        let setup_ns = t0.elapsed().as_nanos();
        Self {
            curve,
            m,
            table,
            giant_stride_neg,
            setup_counts: ar.counts,
            setup_ns,
        }
    }

    pub fn solve(&self, q: Point) -> Result<BsgsQueryReceipt, String> {
        let t0 = Instant::now();
        let mut ar = CurveArith::default();
        if q.is_none() {
            return Ok(BsgsQueryReceipt {
                scalar: 0,
                verified: true,
                giant_steps: 0,
                online_counts: ar.counts,
                online_ns: t0.elapsed().as_nanos(),
            });
        }
        let max_giant = self.curve.n.div_ceil(2 * self.m) + 1;
        let mut gamma = q;
        for i in 0..=max_giant {
            match gamma {
                None => {
                    let k = ((i as u128 * 2 * self.m as u128) % self.curve.n as u128) as u64;
                    let ok = ar.scalar(self.curve, k, Some(self.curve.g)) == q;
                    if !ok {
                        return Err("BSGS zero-hit replay mismatch".into());
                    }
                    return Ok(BsgsQueryReceipt {
                        scalar: k,
                        verified: true,
                        giant_steps: i + 1,
                        online_counts: ar.counts,
                        online_ns: t0.elapsed().as_nanos(),
                    });
                }
                Some((gx, gy)) => {
                    if let Some(&(j, by)) = self.table.get(&gx) {
                        let base = (i as u128 * 2 * self.m as u128) % self.curve.n as u128;
                        let k = if gy == by {
                            ((base + j as u128) % self.curve.n as u128) as u64
                        } else {
                            ((base + self.curve.n as u128 - j as u128) % self.curve.n as u128) as u64
                        };
                        let ok = ar.scalar(self.curve, k, Some(self.curve.g)) == q;
                        if !ok {
                            return Err("BSGS hit scalar replay failed".into());
                        }
                        return Ok(BsgsQueryReceipt {
                            scalar: k,
                            verified: true,
                            giant_steps: i + 1,
                            online_counts: ar.counts,
                            online_ns: t0.elapsed().as_nanos(),
                        });
                    }
                }
            }
            gamma = ar.add(self.curve, gamma, self.giant_stride_neg);
        }
        Err("BSGS failed to find target within group order".into())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RhoSolveReceipt {
    pub scalar: u64,
    pub verified: bool,
    pub walk_steps: u64,
    pub distinguished_points: usize,
    pub restarts: u64,
    pub counts: ArithCounts,
    pub wall_ns: u128,
}

/// Matched negation-map Pollard's rho over `E(F_p) / {±1}` using canonical `y = min(y, p - y)`,
/// 32-branch r-adding walk (`M_r = u_r G + v_r Q`), and distinguished-point collision detection
/// (handling fruitless 2-cycles via deterministic jump perturbation), fully charging setup,
/// fruitless-cycle restarts, and final `[d]G == Q` verification.
pub fn negation_pollard_rho(curve: Curve, q: Point, seed: u64) -> Result<RhoSolveReceipt, String> {
    let t0 = Instant::now();
    let mut ar = CurveArith::default();
    if q.is_none() {
        return Ok(RhoSolveReceipt {
            scalar: 0,
            verified: true,
            walk_steps: 0,
            distinguished_points: 0,
            restarts: 0,
            counts: ar.counts,
            wall_ns: t0.elapsed().as_nanos(),
        });
    }

    const BRANCHES: usize = 32;
    let mut rng = seed ^ 0x5248_4f5f_4e45_4731;
    let mut jumps_pt = Vec::with_capacity(BRANCHES);
    let mut jumps_a = [0u64; BRANCHES];
    let mut jumps_b = [0u64; BRANCHES];
    for r in 0..BRANCHES {
        loop {
            let ua = 1 + splitmix64(&mut rng) % (curve.n - 1);
            let ub = 1 + splitmix64(&mut rng) % (curve.n - 1);
            let ga = ar.scalar(curve, ua, Some(curve.g));
            let qb = ar.scalar(curve, ub, q);
            let pt = ar.add(curve, ga, qb);
            if pt.is_some() {
                jumps_a[r] = ua;
                jumps_b[r] = ub;
                jumps_pt.push(pt);
                break;
            }
        }
    }

    // Distinguished point mask scaled to ~sqrt(pi * n / 4) / 64 so DP table stays tiny
    // while tail overhead is < 2% of walk length.
    let expected_steps = ((std::f64::consts::PI * curve.n as f64 / 4.0).sqrt()) as u64;
    let dp_bits = ((expected_steps / 64).max(1).ilog2()).min(16);
    let dp_mask = (1u64 << dp_bits) - 1;

    let mut dp_table: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut walk_steps = 0u64;
    let mut restarts = 0u64;
    let step_cap = expected_steps.saturating_mul(40).max(10_000);

    'outer: while walk_steps < step_cap {
        let mut a = 1 + splitmix64(&mut rng) % (curve.n - 1);
        let mut b = 1 + splitmix64(&mut rng) % (curve.n - 1);
        let ga0 = ar.scalar(curve, a, Some(curve.g));
        let qb0 = ar.scalar(curve, b, q);
        let mut cur = ar.add(curve, ga0, qb0);

        let canon_ab = |pt: Point, aa: u64, bb: u64| -> Option<((u64, u64), u64, u64)> {
            let (x, y) = pt?;
            if y > curve.p - y {
                Some(((x, curve.p - y), fp_neg(aa, curve.n), fp_neg(bb, curve.n)))
            } else {
                Some(((x, y), aa, bb))
            }
        };

        let branch_of = |x: u64| -> usize {
            ((x.wrapping_mul(0x517c_c1b7_2722_0a95) >> 59) as usize) % BRANCHES
        };

        // Brent's 4-register cycle guard (0 extra curve ops, 100% stateless walk map `f`):
        // catches any fruitless cycle on `E/{±1}` that contains no distinguished point in `< 2*L` steps!
        let mut brent_power = 1u64;
        let mut brent_lam = 0u64;
        let mut brent_x = u64::MAX;
        let mut brent_a = 0u64;
        let mut brent_b = 0u64;

        while walk_steps < step_cap {
            let Some(((cx, cy), na, nb)) = canon_ab(cur, a, b) else {
                if b % curve.n != 0 {
                    ar.counts.scalar_invs += 1;
                    ar.counts.scalar_muls += 1;
                    let d = fp_mul(fp_neg(a, curve.n), fp_inv(b, curve.n), curve.n);
                    if ar.scalar(curve, d, Some(curve.g)) == q {
                        return Ok(RhoSolveReceipt {
                            scalar: d,
                            verified: true,
                            walk_steps,
                            distinguished_points: dp_table.len(),
                            restarts,
                            counts: ar.counts,
                            wall_ns: t0.elapsed().as_nanos(),
                        });
                    }
                }
                restarts += 1;
                continue 'outer;
            };

            let mixed_x = cx.wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 32;
            if (mixed_x & dp_mask) == 0 {
                if let Some(&(pa, pb)) = dp_table.get(&cx) {
                    if (pa, pb) != (na, nb) {
                        let db = fp_sub(nb, pb, curve.n);
                        if db != 0 {
                            ar.counts.scalar_invs += 1;
                            ar.counts.scalar_muls += 1;
                            let da = fp_sub(pa, na, curve.n);
                            let d = fp_mul(da, fp_inv(db, curve.n), curve.n);
                            if ar.scalar(curve, d, Some(curve.g)) == q {
                                return Ok(RhoSolveReceipt {
                                    scalar: d,
                                    verified: true,
                                    walk_steps,
                                    distinguished_points: dp_table.len(),
                                    restarts,
                                    counts: ar.counts,
                                    wall_ns: t0.elapsed().as_nanos(),
                                });
                            }
                        }
                    }
                    restarts += 1;
                    continue 'outer;
                } else {
                    dp_table.insert(cx, (na, nb));
                }
            }

            if cx == brent_x {
                if (brent_a, brent_b) != (na, nb) {
                    let db = fp_sub(nb, brent_b, curve.n);
                    if db != 0 {
                        ar.counts.scalar_invs += 1;
                        ar.counts.scalar_muls += 1;
                        let da = fp_sub(brent_a, na, curve.n);
                        let d = fp_mul(da, fp_inv(db, curve.n), curve.n);
                        if ar.scalar(curve, d, Some(curve.g)) == q {
                            return Ok(RhoSolveReceipt {
                                scalar: d,
                                verified: true,
                                walk_steps,
                                distinguished_points: dp_table.len(),
                                restarts,
                                counts: ar.counts,
                                wall_ns: t0.elapsed().as_nanos(),
                            });
                        }
                    }
                }
                restarts += 1;
                continue 'outer;
            }
            if brent_lam == brent_power {
                brent_x = cx;
                brent_a = na;
                brent_b = nb;
                brent_power <<= 1;
                brent_lam = 0;
            }
            brent_lam += 1;

            // Stateless step function on `E / {±1}` with exact zero-lookahead Wiener-Zuccherato
            // 1- and 2-cycle breaking (`x(P + M_idx) == x(P)` OR `y(P + M_idx) > p - y` AND
            // `branch(x(P + M_idx)) == idx`, which detects `canon(canon(P + M_idx) + M_idx) == P`
            // in 1 integer compare with 0 extra curve additions!):
            let idx = branch_of(cx);
            let canon_pt = Some((cx, cy));
            let raw_next = ar.add(curve, canon_pt, jumps_pt[idx]);
            walk_steps += 1;

            let take_double = match raw_next {
                None => false,
                Some((nx, ny)) => nx == cx || (ny > curve.p - ny && branch_of(nx) == idx),
            };

            if take_double {
                restarts += 1;
                cur = ar.add(curve, canon_pt, canon_pt);
                a = fp_add(na, na, curve.n);
                b = fp_add(nb, nb, curve.n);
                walk_steps += 1;
            } else {
                cur = raw_next;
                a = fp_add(na, jumps_a[idx], curve.n);
                b = fp_add(nb, jumps_b[idx], curve.n);
            }
        }
    }
    Err(format!("negation Pollard rho exceeded step cap {step_cap}"))
}
