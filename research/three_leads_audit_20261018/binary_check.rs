// Independent correctness census. No repository arithmetic is imported.
use std::collections::{BTreeMap, HashSet};

#[derive(Clone, Copy)]
struct F {
    d: u32,
    modulus: u32,
}
impl F {
    fn mul(self, mut a: u32, mut b: u32) -> u32 {
        let mut r = 0;
        while b != 0 {
            if b & 1 != 0 {
                r ^= a;
            }
            b >>= 1;
            a <<= 1;
            if a & (1 << self.d) != 0 {
                a ^= self.modulus;
            }
        }
        r
    }
    fn pow(self, mut a: u32, mut e: u32) -> u32 {
        let mut r = 1;
        while e != 0 {
            if e & 1 != 0 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    fn inv(self, a: u32) -> u32 {
        assert_ne!(a, 0);
        self.pow(a, (1 << self.d) - 2)
    }
    fn sq(self, a: u32) -> u32 {
        self.mul(a, a)
    }
    fn trace(self, mut a: u32) -> u32 {
        let mut r = 0;
        for _ in 0..self.d {
            r ^= a;
            a = self.sq(a);
        }
        assert!(r <= 1);
        r
    }
}
fn degree(a: u32) -> u32 {
    31 - a.leading_zeros()
}
fn rem(mut a: u32, b: u32) -> u32 {
    while a != 0 && degree(a) >= degree(b) {
        a ^= b << (degree(a) - degree(b));
    }
    a
}
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = rem(a, b);
        a = b;
        b = r;
    }
    a
}
fn field(d: u32) -> F {
    for modulus in ((1 << d) + 1..(1 << (d + 1))).step_by(2) {
        let f = F { d, modulus };
        let mut t = 2;
        let mut ok = true;
        for i in 1..=d {
            t = f.sq(t);
            if i <= d / 2 && gcd(t ^ 2, modulus) != 1 {
                ok = false;
                break;
            }
        }
        if ok && t == 2 {
            return f;
        }
    }
    panic!("no modulus");
}
type P = Option<(u32, u32)>;
#[derive(Clone, Copy)]
struct E {
    f: F,
    a: u32,
    b: u32,
}
impl E {
    fn valid(self, p: P) -> bool {
        match p {
            None => true,
            Some((x, y)) => {
                self.f.sq(y) ^ self.f.mul(x, y)
                    == self.f.mul(self.f.sq(x), x) ^ self.f.mul(self.a, self.f.sq(x)) ^ self.b
            }
        }
    }
    fn add(self, p: P, q: P) -> P {
        let (x, y) = match p {
            None => return q,
            Some(p) => p,
        };
        let (u, v) = match q {
            None => return p,
            Some(q) => q,
        };
        if x == u {
            if y != v || x == 0 {
                return None;
            }
            let l = x ^ self.f.mul(y, self.f.inv(x));
            let z = self.f.sq(l) ^ l ^ self.a;
            Some((z, self.f.sq(x) ^ self.f.mul(l ^ 1, z)))
        } else {
            let l = self.f.mul(y ^ v, self.f.inv(x ^ u));
            let z = self.f.sq(l) ^ l ^ x ^ u ^ self.a;
            Some((z, self.f.mul(l, x ^ z) ^ z ^ y))
        }
    }
    fn scalar(self, mut k: usize, mut p: P) -> P {
        let mut r = None;
        while k > 0 {
            if k & 1 != 0 {
                r = self.add(r, p);
            }
            p = self.add(p, p);
            k >>= 1;
        }
        r
    }
    fn bit(self, p: P) -> u32 {
        match p {
            None => 0,
            Some((x, _)) => self.f.trace(x ^ self.a),
        }
    }
    fn points(self) -> Vec<P> {
        let n = 1 << self.f.d;
        let mut roots = vec![None; n as usize];
        for z in 0..n {
            roots[(self.f.sq(z) ^ z) as usize] = Some(z);
        }
        let mut r = vec![None, Some((0, self.f.pow(self.b, 1 << (self.f.d - 1))))];
        for x in 1..n {
            let t = x ^ self.a ^ self.f.mul(self.b, self.f.sq(self.f.inv(x)));
            if let Some(z) = roots[t as usize] {
                let y = self.f.mul(x, z);
                r.push(Some((x, y)));
                r.push(Some((x, y ^ x)));
            }
        }
        r
    }
}
fn sumset(e: E, a: &[P], b: &[P]) -> HashSet<P> {
    let mut s = HashSet::new();
    for &p in a {
        for &q in b {
            s.insert(e.add(p, q));
        }
    }
    s
}
fn check(m: u32, n: u32, a: u32, b: u32, kind: &str) {
    let f = field(m * n);
    let e = E { f, a, b };
    assert_ne!(b, 0);
    let points = e.points();
    let population: HashSet<_> = points.iter().copied().collect();
    assert_eq!(population.len(), points.len());
    for &p in &points {
        assert!(e.valid(p));
    }
    if f.d <= 8 {
        let mut direct = HashSet::from([None]);
        for x in 0..(1 << f.d) {
            for y in 0..(1 << f.d) {
                let p = Some((x, y));
                if e.valid(p) {
                    direct.insert(p);
                }
            }
        }
        assert_eq!(direct, population);
    }
    let sub: Vec<_> = (0..(1 << f.d)).filter(|&x| f.pow(x, 1 << m) == x).collect();
    assert_eq!(sub.len(), 1 << m);
    for &x in &sub {
        assert_eq!(f.trace(x), 0);
        if x != 0 {
            assert_eq!(f.mul(x, f.inv(x)), 1);
        }
    }
    let fb: Vec<_> = points
        .iter()
        .copied()
        .filter(|p| p.is_some_and(|(x, _)| sub.contains(&x)))
        .collect();
    for &p in &fb {
        assert_eq!(e.bit(p), f.trace(a));
    }
    let twice: HashSet<_> = points.iter().map(|&p| e.add(p, p)).collect();
    let kernel: HashSet<_> = points.iter().copied().filter(|&p| e.bit(p) == 0).collect();
    assert_eq!(twice, kernel);
    assert_eq!(2 * kernel.len(), points.len());
    let mut odd = points.len();
    while odd & 1 == 0 {
        odd >>= 1;
    }
    for &p in &points {
        if e.scalar(odd, p).is_none() {
            assert_eq!(e.bit(p), 0);
        }
    }
    let mut sums = HashSet::new();
    let mut multiplicities = BTreeMap::new();
    for (i, &p) in fb.iter().enumerate() {
        for &q in &fb[i..] {
            let t = e.add(p, q);
            assert!(population.contains(&t));
            assert_eq!(e.bit(t), e.bit(p) ^ e.bit(q));
            sums.insert(t);
            *multiplicities.entry(t).or_insert(0usize) += 1;
        }
    }
    if n == 4 {
        let pairs: Vec<_> = sums.into_iter().collect();
        sums = sumset(e, &pairs, &pairs);
    }
    for t in &sums {
        assert!(kernel.contains(t));
    }
    // S3 is an independent x-coordinate predicate. Check all rational targets
    // against all actual usable base x pairs on the smallest quadratic cases.
    if n == 2 && m <= 4 {
        let xs: Vec<_> = sub
            .iter()
            .copied()
            .filter(|&x| fb.iter().any(|p| p.unwrap().0 == x))
            .collect();
        for &p in &points {
            if let Some((t, _)) = p {
                let mut predicted = false;
                let coefficient_basis = if f.pow(t, 1 << m) != t {
                    let trace_t = t ^ f.pow(t, 1 << m);
                    let norm_t = f.mul(t, f.pow(t, 1 << m));
                    let b1 = f.mul(b ^ f.pow(b, 1 << m), f.inv(trace_t));
                    let b0 = b ^ f.mul(b1, t);
                    assert_eq!(f.pow(b0, 1 << m), b0);
                    assert_eq!(f.pow(b1, 1 << m), b1);
                    Some((trace_t, norm_t, b0, b1))
                } else {
                    None
                };
                for (i, &x) in xs.iter().enumerate() {
                    for &y in &xs[i..] {
                        let v = f.mul(x, y);
                        let u = x ^ y;
                        let s3 = f.sq(v) ^ f.mul(t, v) ^ f.mul(f.sq(t), f.sq(u)) ^ b;
                        if let Some((trace_t, norm_t, b0, b1)) = coefficient_basis {
                            let w = f.sq(u);
                            let c1 = v ^ f.mul(trace_t, w) ^ b1;
                            let c0 = f.sq(v) ^ f.mul(norm_t, w) ^ b0;
                            assert_eq!(s3, c0 ^ f.mul(c1, t));
                            if c1 == 0 {
                                assert_eq!(
                                    c0,
                                    f.mul(f.sq(trace_t), f.sq(w))
                                        ^ f.mul(norm_t, w)
                                        ^ f.sq(b1)
                                        ^ b0
                                );
                            }
                        }
                        predicted |= s3 == 0;
                    }
                }
                assert_eq!(predicted, sums.contains(&p), "S3 mismatch");
            }
        }
    }
    let hit = sums.len();
    let size = points.len();
    let expected = 1.0 - (-((fb.len() * fb.len()) as f64) / (2.0 * size as f64)).exp();
    let nonid: Vec<_> = multiplicities.iter().filter(|(p, _)| p.is_some()).collect();
    let max_nonid = nonid.iter().map(|(_, c)| **c).max().unwrap_or(0);
    let one = nonid.iter().filter(|(_, c)| **c == 1).count();
    let two = nonid.iter().filter(|(_, c)| **c == 2).count();
    println!(
        "{kind},{m},{n},{},{a},{b},{},{},{hit},{},{:.9},{:.9},{:.9},{max_nonid},{one},{two}",
        f.modulus,
        size,
        fb.len(),
        kernel.len(),
        hit as f64 / size as f64,
        hit as f64 / kernel.len() as f64,
        expected
    );
}
fn main() {
    println!("kind,m,extension,modulus,a,b,group_size,fb_size,reachable,kernel_size,coverage,conditional_coverage,poisson_model,max_nonidentity_pair_multiplicity,one_representation,two_representations");
    for m in 3..=7 {
        let f = field(2 * m);
        let q = 1 << m;
        let bs: Vec<_> = (1..(1 << (2 * m)))
            .filter(|&b| f.pow(b, q) != b)
            .take(7)
            .collect();
        for (i, b) in bs.into_iter().enumerate() {
            check(m, 2, (17 * i as u32 + 3) % (1 << (2 * m)), b, "generic");
        }
    }
    for m in [4, 6] {
        let f = field(2 * m);
        let a0 = (0..(1 << (2 * m))).find(|&a| f.trace(a) == 0).unwrap();
        let a1 = (0..(1 << (2 * m))).find(|&a| f.trace(a) == 1).unwrap();
        check(m, 2, a0, 1, "descended_b_trace0");
        check(m, 2, a1, 1, "descended_b_trace1");
    }
    for m in [2, 3] {
        check(m, 4, 3, 2, "even_extension_four");
    }
}
