use serde::{Deserialize, Serialize};
pub type Point = Option<[u64; 2]>;
pub fn pow(mut x: u64, mut e: u64, p: u64) -> u64 {
    let mut z = 1;
    while e > 0 {
        if e & 1 != 0 {
            z = z * x % p;
        }
        x = x * x % p;
        e >>= 1;
    }
    z
}
pub fn inv(x: u64, p: u64) -> u64 {
    assert!(x % p != 0);
    pow(x, p - 2, p)
}
pub fn prime(n: u64) -> bool {
    n >= 2 && (2..).take_while(|x| x * x <= n).all(|x| n % x != 0)
}
pub fn factors(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = vec![];
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 {
                n /= d;
                e += 1;
            }
            out.push((d, e));
        }
        d += 1;
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}
pub fn isqrt(n: u64) -> u64 {
    let mut x = (n as f64).sqrt() as u64;
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    while x * x > n {
        x -= 1;
    }
    x
}
pub fn sqrt(x: u64, p: u64) -> Option<u64> {
    if x == 0 {
        return Some(0);
    }
    if pow(x, (p - 1) / 2, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(pow(x, (p + 1) / 4, p));
    }
    let s = (p - 1).trailing_zeros();
    let q = (p - 1) >> s;
    let z = (2..p).find(|z| pow(*z, (p - 1) / 2, p) == p - 1).unwrap();
    let (mut c, mut y, mut t, mut m) = (pow(z, q, p), pow(x, (q + 1) / 2, p), pow(x, q, p), s);
    while t != 1 {
        let (mut v, mut i) = (t, 0);
        while v != 1 {
            v = v * v % p;
            i += 1;
            assert!(i < m);
        }
        let b = pow(c, 1 << (m - i - 1), p);
        y = y * b % p;
        c = b * b % p;
        t = t * c % p;
        m = i;
    }
    Some(y)
}
pub fn mix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Curve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}
impl Curve {
    pub fn rhs(self, x: u64) -> u64 {
        (x * x % self.p * x + self.a * x + self.b) % self.p
    }
    pub fn on(self, p: Point) -> bool {
        p.is_none_or(|[x, y]| x < self.p && y < self.p && y * y % self.p == self.rhs(x))
    }
    pub fn neg(self, p: Point) -> Point {
        p.map(|[x, y]| [x, (self.p - y) % self.p])
    }
    pub fn canon(self, p: Point) -> Point {
        p.map(|[x, y]| [x, y.min(self.p - y)])
    }
    pub fn add(self, p: Point, q: Point) -> Point {
        let (Some([x, y]), Some([u, v])) = (p, q) else {
            return p.or(q);
        };
        if x == u && (y + v) % self.p == 0 {
            return None;
        }
        let m = if x == u {
            (3 * x % self.p * x + self.a) % self.p * inv(2 * y % self.p, self.p) % self.p
        } else {
            (v + self.p - y) % self.p * inv((u + self.p - x) % self.p, self.p) % self.p
        };
        let z = (m * m % self.p + 2 * self.p - x - u) % self.p;
        Some([
            z,
            (m * ((x + self.p - z) % self.p) % self.p + self.p - y) % self.p,
        ])
    }
    pub fn mul(self, mut p: Point, mut n: u64) -> Point {
        let mut z = None;
        while n > 0 {
            if n & 1 != 0 {
                z = self.add(z, p);
            }
            p = self.add(p, p);
            n >>= 1;
        }
        z
    }
    pub fn lift(self, x: u64) -> Point {
        sqrt(self.rhs(x), self.p).map(|y| [x, y.min(self.p - y)])
    }
    pub fn order(self, p: Point, n: u64) -> u64 {
        assert!(p.is_some() && self.mul(p, n).is_none());
        let mut z = n;
        for (l, _) in factors(n) {
            while z % l == 0 && self.mul(p, z / l).is_none() {
                z /= l;
            }
        }
        z
    }
    pub fn card(self) -> u64 {
        1 + (0..self.p)
            .map(|x| match sqrt(self.rhs(x), self.p) {
                None => 0,
                Some(0) => 1,
                Some(_) => 2,
            })
            .sum::<u64>()
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fixture {
    pub id: String,
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub order: u64,
    pub r: u64,
    pub d: u64,
    pub generator: [u64; 2],
    pub family: String,
    pub role: String,
}
impl Fixture {
    pub fn curve(&self) -> Curve {
        Curve {
            p: self.p,
            a: self.a,
            b: self.b,
        }
    }
    pub fn twist(&self) -> Curve {
        let p = self.p;
        Curve {
            p,
            a: self.a * self.d % p * self.d % p,
            b: self.b * self.d % p * self.d % p * self.d % p,
        }
    }
    pub fn tn(&self) -> u64 {
        2 * (self.p + 1) - self.order
    }
    pub fn g(&self) -> Point {
        Some(self.generator)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Alpha {
    pub scale: u64,
    pub lambda: u64,
    pub roots: Vec<u64>,
    pub t: Vec<u64>,
    pub w: Vec<u64>,
    pub quotient: Curve,
}
impl Alpha {
    pub fn construct(f: &Fixture, h: Point, hn: u64) -> Option<Self> {
        if f.tn() % 17 != 0 {
            return None;
        }
        let m = isqrt(f.tn() / 17);
        if m < 2 || 17 * m * m != f.tn() || m % f.r == 0 || hn % 17 != 0 {
            return None;
        }
        let e = f.curve();
        let tw = f.twist();
        let k = tw.mul(h, hn / 17);
        if k.is_none() || tw.mul(k, 17).is_some() {
            return None;
        }
        let di = inv(f.d, f.p);
        let mut q = k;
        let mut roots = vec![];
        for _ in 1..=8 {
            roots.push(q.unwrap()[0] * di % f.p);
            q = tw.add(q, k);
        }
        roots.sort();
        roots.dedup();
        if roots.len() != 8 {
            return None;
        }
        let t: Vec<_> = roots
            .iter()
            .map(|x| (6 * x % f.p * x + 2 * f.a) % f.p)
            .collect();
        let w: Vec<_> = roots.iter().map(|x| 4 * e.rhs(*x) % f.p).collect();
        let st = t.iter().fold(0, |s, x| (s + x) % f.p);
        let sw = roots.iter().fold(0, |s, x| {
            (s + 10 * x % f.p * x % f.p * x + 6 * f.a % f.p * x + 4 * f.b) % f.p
        });
        let quotient = Curve {
            p: f.p,
            a: (f.a + f.p - 5 * st % f.p) % f.p,
            b: (f.b + f.p - 7 * sw % f.p) % f.p,
        };
        let m2 = m * m % f.p;
        let m3 = m2 * m % f.p;
        if quotient.a * m2 % f.p * m2 % f.p != f.a || quotient.b * m3 % f.p * m3 % f.p != f.b {
            return None;
        }
        let alpha = Self {
            scale: m,
            lambda: 2 * inv(m % f.r, f.r) % f.r,
            roots,
            t,
            w,
            quotient,
        };
        if alpha.eval(e, f.g()) != e.mul(f.g(), alpha.lambda) {
            return None;
        }
        Some(alpha)
    }
    pub fn eval(&self, e: Curve, p: Point) -> Point {
        let [x, y] = p?;
        let p = e.p;
        let mut ds = vec![];
        let mut products = vec![1];
        for u in &self.roots {
            let d = (x + p - u) % p;
            if d == 0 {
                return None;
            }
            ds.push(d);
            products.push(products.last().unwrap() * d % p);
        }
        let mut z = inv(*products.last().unwrap(), p);
        let (mut rx, mut sy) = (x, 1);
        for i in (0..ds.len()).rev() {
            let d = z * products[i] % p;
            z = z * ds[i] % p;
            let d2 = d * d % p;
            let d3 = d2 * d % p;
            rx = (rx + self.t[i] * d % p + self.w[i] * d2 % p) % p;
            sy = (sy + 2 * p - self.t[i] * d2 % p - 2 * self.w[i] % p * d3 % p) % p;
        }
        let u2 = self.scale * self.scale % p;
        let u3 = u2 * self.scale % p;
        Some([u2 * rx % p, u3 * y % p * sy % p])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roots_all_small_fields() {
        for p in [7, 13, 59, 101, 827] {
            for x in 0..p {
                let s = sqrt(x, p);
                assert_eq!(s.is_some(), (0..p).any(|y| y * y % p == x));
                if let Some(y) = s {
                    assert_eq!(y * y % p, x);
                }
            }
        }
    }
    #[test]
    fn source_group_and_kernel_map() {
        let f = Fixture {
            id: "control".into(),
            p: 827,
            a: 1,
            b: 15,
            order: 823,
            r: 823,
            d: 2,
            generator: [2, 5],
            family: "control".into(),
            role: "control".into(),
        };
        let e = f.curve();
        assert_eq!(e.card(), 823);
        let a = Alpha::construct(&f, Some([2, 386]), 119).unwrap();
        assert_eq!(a.lambda, 353);
        for i in 0..823 {
            let p = e.mul(f.g(), i);
            assert_eq!(a.eval(e, p), e.mul(p, 353));
        }
        assert_ne!(a.eval(e, f.g()), e.mul(f.g(), 354));
    }
}
