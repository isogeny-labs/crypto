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
        assert_eq!(p % 4, 3);
        let r = self.pow(x, (p + 1) / 4, p);
        (self.mul(r, r, p) == x).then_some(r)
    }
    pub fn rhs(&mut self, e: Curve, x: u64) -> u64 {
        let xx = self.mul(x, x, e.p);
        let xxx = self.mul(xx, x, e.p);
        (xxx + self.mul(e.a, x, e.p) + e.b) % e.p
    }
    pub fn on_curve(&mut self, e: Curve, q: Point) -> bool {
        q.is_none_or(|(x, y)| x < e.p && y < e.p && self.mul(y, y, e.p) == self.rhs(e, x))
    }
    pub fn neg(&self, e: Curve, q: Point) -> Point {
        q.map(|(x, y)| (x, (e.p - y) % e.p))
    }
    pub fn add(&mut self, e: Curve, u: Point, v: Point) -> Point {
        self.counts.group_additions += 1;
        let (Some((x, y)), Some((z, w))) = (u, v) else {
            return u.or(v);
        };
        if x == z && (y + w) % e.p == 0 {
            return None;
        }
        let (num, den) = if x == z {
            ((3 * self.mul(x, x, e.p) + e.a) % e.p, 2 * y % e.p)
        } else {
            (sub(w, y, e.p), sub(z, x, e.p))
        };
        let id = self.inv(den, e.p);
        let slope = self.mul(num, id, e.p);
        let xx = sub(sub(self.mul(slope, slope, e.p), x, e.p), z, e.p);
        let yy = sub(self.mul(slope, sub(x, xx, e.p), e.p), y, e.p);
        Some((xx, yy))
    }
    pub fn scalar(&mut self, e: Curve, mut k: u64, mut q: Point) -> Point {
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
