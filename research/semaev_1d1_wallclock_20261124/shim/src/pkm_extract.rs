// Verbatim extracts of crypto/src/cryptanalysis/pkm_smooth_tower.rs (field primitives + Curve).
// The upstream `opcount` module and its calls are #[cfg(test)], absent in non-test builds; here they are removed likewise.
use std::collections::HashMap;

#[inline]
pub fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
#[inline]
pub fn addm(a: u64, b: u64, p: u64) -> u64 {
    let s = a + b;
    if s >= p {
        s - p
    } else {
        s
    }
}
#[inline]
pub fn subm(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + p - b
    }
}
pub fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
pub fn invm(a: u64, p: u64) -> u64 {
    assert!(!a.is_multiple_of(p), "inverse of zero");
    powm(a, p - 2, p)
}
pub fn is_square(a: u64, p: u64) -> bool {
    a.is_multiple_of(p) || powm(a, (p - 1) / 2, p) == 1
}
/// Tonelli–Shanks; `None` for a non-residue.
pub fn sqrtm(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if p == 2 {
        return Some(a);
    }
    if !is_square(a, p) {
        return None;
    }
    let (mut q, mut s) = (p - 1, 0u32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while is_square(z, p) {
        z += 1;
    }
    let (mut m, mut c, mut t, mut r) = (s, powm(z, q, p), powm(a, q, p), powm(a, q.div_ceil(2), p));
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = mulm(tt, tt, p);
            i += 1;
        }
        let b = powm(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mulm(b, b, p);
        t = mulm(t, c, p);
        r = mulm(r, b, p);
    }
    Some(r)
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n.is_multiple_of(q) {
            return n == q;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powm(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulm(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

/// Trial division of `n` by primes `≤ bound`: `(smooth factors with
/// multiplicity, cofactor)`.
/// `y² = x³ + ax + b` over `F_p`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Curve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}

/// Affine point or the point at infinity (`None`).
pub type Pt = Option<(u64, u64)>;

impl Curve {
    pub fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        addm(
            addm(mulm(mulm(x, x, p), x, p), mulm(self.a, x, p), p),
            self.b,
            p,
        )
    }
    pub fn is_singular(&self) -> bool {
        let p = self.p;
        let d = addm(
            mulm(4, powm(self.a, 3, p), p),
            mulm(27, mulm(self.b, self.b, p), p),
            p,
        );
        d == 0
    }
    pub fn on_curve(&self, pt: Pt) -> bool {
        match pt {
            None => true,
            Some((x, y)) => mulm(y, y, self.p) == self.rhs(x),
        }
    }
    pub fn neg(&self, pt: Pt) -> Pt {
        pt.map(|(x, y)| (x, subm(0, y, self.p)))
    }
    pub fn add(&self, u: Pt, v: Pt) -> Pt {
        let p = self.p;
        let ((x1, y1), (x2, y2)) = match (u, v) {
            (None, _) => return v,
            (_, None) => return u,
            (Some(a), Some(b)) => (a, b),
        };
        let l = if x1 == x2 {
            if addm(y1, y2, p) == 0 {
                return None;
            }
            mulm(
                addm(mulm(3, mulm(x1, x1, p), p), self.a, p),
                invm(mulm(2, y1, p), p),
                p,
            )
        } else {
            mulm(subm(y2, y1, p), invm(subm(x2, x1, p), p), p)
        };
        let x3 = subm(subm(mulm(l, l, p), x1, p), x2, p);
        let y3 = subm(mulm(l, subm(x1, x3, p), p), y1, p);
        Some((x3, y3))
    }
    pub fn mul(&self, mut k: u64, pt: Pt) -> Pt {
        let (mut r, mut b) = (None, pt);
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(r, b);
            }
            b = self.add(b, b);
            k >>= 1;
        }
        r
    }
    pub fn random_point(&self, rng: &mut Rng) -> Pt {
        loop {
            let x = rng.below(self.p);
            if let Some(y) = sqrtm(self.rhs(x), self.p) {
                let y = if rng.next_u64() & 1 == 1 {
                    subm(0, y, self.p)
                } else {
                    y
                };
                return Some((x, y));
            }
        }
    }
    /// Every `k ∈ [lo, hi]` with `[k]P = O`, by baby-step giant-step.
    fn annihilators_in(&self, pt: Pt, lo: u64, hi: u64) -> Vec<u64> {
        let w = hi - lo + 1;
        let m = ((w as f64).sqrt() as u64).max(1) + 1;
        let mut baby: HashMap<Pt, Vec<u64>> = HashMap::new();
        let mut cur = None;
        for j in 0..m {
            baby.entry(cur).or_default().push(j);
            cur = self.add(cur, pt);
        }
        // [lo + i m + j] P = O  ⇔  [j]P = −[lo + i m]P
        let step = self.mul(m, pt);
        let mut g = self.mul(lo, pt);
        let mut out = vec![];
        let mut i = 0;
        while lo + i * m <= hi {
            if let Some(js) = baby.get(&self.neg(g)) {
                for &j in js {
                    let k = lo + i * m + j;
                    if k <= hi {
                        out.push(k);
                    }
                }
            }
            g = self.add(g, step);
            i += 1;
        }
        out.sort_unstable();
        out.dedup();
        out
    }
    /// `#E(F_p)`: exhaustive below 2^16, otherwise the Hasse-interval
    /// candidates annihilating several random points, until one is left.
    pub fn order(&self, rng: &mut Rng) -> Option<u64> {
        let p = self.p;
        if p < 1 << 16 {
            let mut n = 1;
            for x in 0..p {
                let r = self.rhs(x);
                n += if r == 0 {
                    1
                } else if is_square(r, p) {
                    2
                } else {
                    0
                };
            }
            return Some(n);
        }
        let s = 2 * (p as f64).sqrt().ceil() as u64 + 2;
        let (lo, hi) = (p + 1 - s, p + 1 + s);
        let mut cand: Option<Vec<u64>> = None;
        for _ in 0..24 {
            let pt = self.random_point(rng);
            let hits = match &cand {
                None => self.annihilators_in(pt, lo, hi),
                Some(c) => c
                    .iter()
                    .copied()
                    .filter(|&k| self.mul(k, pt).is_none())
                    .collect(),
            };
            if hits.len() == 1 {
                return Some(hits[0]);
            }
            cand = Some(hits);
        }
        None
    }
}
/// SplitMix64, so every search is a function of its seed.
#[derive(Clone, Debug)]
pub struct Rng(pub u64);
impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

// ---------------------------------------------------------------- curves
