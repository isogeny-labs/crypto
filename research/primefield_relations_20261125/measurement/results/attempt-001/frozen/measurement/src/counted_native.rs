//! Counted derivative of shared-terminal-gate/src/reused_native.rs.
//! Base source SHA256 e01d41e24bdd650a0440a52f8782ece3c69c188cfb7b292479a9c8cfdb3c3986.
//! Underlying crypto commit b60f801660eb423901ddea0c03c629089a1b5bfc.
//! Arithmetic expressions/control flow are preserved; only counter updates and
//! an explicit local result for sqrt success accounting have been added.
//! Modular-product counters count helper calls, including exponentiation squares;
//! they do not count direct small-constant integer products, additions, or GAE.
#![allow(dead_code)]
pub type Point = Option<(u64, u64)>;

#[derive(Clone, Copy, Debug)]
pub struct Curve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub n: u64,
    pub g: (u64, u64),
}

pub const PRIME31: Curve = Curve {
    p: 2_147_483_647,
    a: 847_514_846,
    b: 2_087_444_724,
    n: 2_147_446_831,
    g: (625_037_715, 2_100_627_251),
};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Counts {
    pub group_additions: u64,
    pub fp_multiplications: u64,
    pub fp_inversions: u64,
    pub scalar_multiplications: u64,
    pub scalar_inversions: u64,
    pub polynomial_evaluations: u64,
    pub extraction_nodes: u64,
    pub point_scalar_calls: u64,
    pub sqrt_attempts: u64,
    pub sqrt_successes: u64,
    pub rhs_calls: u64,
    pub on_curve_calls: u64,
    pub identity_add_calls: u64,
    pub inverse_pair_add_calls: u64,
    pub double_calls: u64,
    pub general_add_calls: u64,
    pub base_seed_trials: u64,
    pub base_chain_steps: u64,
}

#[derive(Default)]
pub struct Arithmetic {
    pub counts: Counts,
}

impl Arithmetic {
    fn mul(&mut self, x: u64, y: u64, p: u64) -> u64 {
        self.counts.fp_multiplications += 1;
        ((x as u128 * y as u128) % p as u128) as u64
    }
    fn pow(&mut self, mut x: u64, mut e: u64, p: u64) -> u64 {
        let mut out = 1;
        while e > 0 {
            if e & 1 != 0 {
                out = self.mul(out, x, p);
            }
            x = self.mul(x, x, p);
            e >>= 1;
        }
        out
    }
    fn inv(&mut self, x: u64, p: u64) -> u64 {
        assert!(x > 0 && x < p);
        self.counts.fp_inversions += 1;
        self.pow(x, p - 2, p)
    }
    pub fn sqrt(&mut self, x: u64, p: u64) -> Option<u64> {
        self.counts.sqrt_attempts += 1;
        assert_eq!(p % 4, 3);
        let r = self.pow(x, (p + 1) / 4, p);
        let result = (self.mul(r, r, p) == x).then_some(r);
        self.counts.sqrt_successes += u64::from(result.is_some());
        result
    }
    pub fn rhs(&mut self, e: Curve, x: u64) -> u64 {
        self.counts.rhs_calls += 1;
        let xx = self.mul(x, x, e.p);
        let xxx = self.mul(xx, x, e.p);
        (xxx + self.mul(e.a, x, e.p) + e.b) % e.p
    }
    pub fn on_curve(&mut self, e: Curve, q: Point) -> bool {
        self.counts.on_curve_calls += 1;
        q.is_none_or(|(x, y)| x < e.p && y < e.p && self.mul(y, y, e.p) == self.rhs(e, x))
    }
    pub fn neg(&self, e: Curve, q: Point) -> Point {
        q.map(|(x, y)| (x, (e.p - y) % e.p))
    }
    pub fn add(&mut self, e: Curve, u: Point, v: Point) -> Point {
        self.counts.group_additions += 1;
        let (Some((x, y)), Some((z, w))) = (u, v) else {
            self.counts.identity_add_calls += 1;
            return u.or(v);
        };
        if x == z && (y + w) % e.p == 0 {
            self.counts.inverse_pair_add_calls += 1;
            return None;
        }
        let (num, den) = if x == z {
            self.counts.double_calls += 1;
            ((3 * self.mul(x, x, e.p) + e.a) % e.p, 2 * y % e.p)
        } else {
            self.counts.general_add_calls += 1;
            (sub(w, y, e.p), sub(z, x, e.p))
        };
        let id = self.inv(den, e.p);
        let slope = self.mul(num, id, e.p);
        let xx = sub(sub(self.mul(slope, slope, e.p), x, e.p), z, e.p);
        let yy = sub(self.mul(slope, sub(x, xx, e.p), e.p), y, e.p);
        Some((xx, yy))
    }
    pub fn scalar(&mut self, e: Curve, mut k: u64, mut q: Point) -> Point {
        self.counts.point_scalar_calls += 1;
        let mut out = None;
        while k > 0 {
            if k & 1 != 0 {
                out = self.add(e, out, q);
            }
            q = self.add(e, q, q);
            k >>= 1;
        }
        out
    }
    fn smul(&mut self, x: u64, y: u64, n: u64) -> u64 {
        self.counts.scalar_multiplications += 1;
        ((x as u128 * y as u128) % n as u128) as u64
    }
    fn sinv(&mut self, x: u64, n: u64) -> u64 {
        assert_ne!(x, 0);
        self.counts.scalar_inversions += 1;
        let (mut z, mut b, mut k) = (1, x, n - 2);
        while k > 0 {
            if k & 1 != 0 {
                z = self.smul(z, b, n);
            }
            b = self.smul(b, b, n);
            k >>= 1;
        }
        z
    }
}

fn sub(x: u64, y: u64, p: u64) -> u64 {
    (x + p - y) % p
}

fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d <= n / d {
        if n.is_multiple_of(d) {
            return n == d;
        }
        d += 1;
    }
    true
}

pub fn check_curve(e: Curve, ar: &mut Arithmetic) -> Result<(), String> {
    if e.p >= 1 << 32
        || e.n >= 1 << 32
        || e.a >= e.p
        || e.b >= e.p
        || e.p % 4 != 3
        || !prime(e.p)
        || !prime(e.n)
    {
        return Err("requires prime p<2^32, p=3 mod 4 and prime subgroup order".into());
    }
    let a2 = ar.mul(e.a, e.a, e.p);
    let a3 = ar.mul(a2, e.a, e.p);
    let b2 = ar.mul(e.b, e.b, e.p);
    if (4 * a3 + 27 * b2).is_multiple_of(e.p)
        || !ar.on_curve(e, Some(e.g))
        || ar.scalar(e, e.n, Some(e.g)).is_some()
    {
        return Err("singular curve, invalid generator or subgroup certificate".into());
    }
    // A prime n annihilating a nonzero G with 2n above the Hasse upper
    // bound certifies #E=n. Use a conservative integer sqrt upper bound.
    let root = (e.p as f64).sqrt() as u64 + 1;
    if 2 * e.n <= e.p + 1 + 2 * root {
        return Err("full prime-order certificate not established".into());
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Base {
    pub depth: usize,
    pub terminal: u64,
    pub values: Vec<u64>,
    pub points: Vec<(u64, u64)>,
}

impl Base {
    pub fn build(e: Curve, depth: usize, ar: &mut Arithmetic) -> Result<Self, String> {
        if !(1..=10).contains(&depth) || !(e.p + 1).is_multiple_of(1 << depth) {
            return Err("Dickson fibre requires 2^depth | p+1, depth=1..10".into());
        }
        for seed in 2..e.p.min(10_000) {
            ar.counts.base_seed_trials += 1;
            let mut c = seed;
            for _ in 0..depth {
                ar.counts.base_chain_steps += 1;
                c = sub(ar.mul(c, c, e.p), 2, e.p);
            }
            if c == 2 || c == e.p - 2 {
                continue;
            }
            let mut values = vec![c];
            for _ in 0..depth {
                let mut next = Vec::new();
                for y in values {
                    if let Some(x) = ar.sqrt((y + 2) % e.p, e.p) {
                        next.push(x);
                        next.push((e.p - x) % e.p);
                    }
                }
                next.sort_unstable();
                next.dedup();
                values = next;
            }
            if values.len() != 1 << depth {
                continue;
            }
            let mut points = Vec::new();
            for &x in &values {
                let rhs = ar.rhs(e, x);
                if let Some(y) = ar.sqrt(rhs, e.p) {
                    if y != 0 {
                        points.push((x, y.min(e.p - y)));
                    }
                }
            }
            if points.len() >= 2 {
                return Ok(Self {
                    depth,
                    terminal: c,
                    values,
                    points,
                });
            }
        }
        Err("no fully split fibre with >=2 curve points in frozen seed scan".into())
    }
}
