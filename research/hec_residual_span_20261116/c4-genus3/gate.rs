use std::collections::{BTreeMap, BTreeSet};
type Z = (i128, i128);
fn za(a: Z, b: Z) -> Z {
    (a.0 + b.0, a.1 + b.1)
}
fn zm(a: Z, b: Z) -> Z {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}
fn bar(a: Z) -> Z {
    (a.0, -a.1)
}
fn norm(a: Z) -> i128 {
    a.0 * a.0 + a.1 * a.1
}
#[derive(Clone)]
struct Field {
    p: u64,
    n: usize,
    low: [u64; 3],
    q: u64,
}
impl Field {
    fn new(p: u64, n: usize) -> Self {
        let mut low = [0; 3];
        if n == 2 {
            let d = (2..p)
                .find(|&d| modpow(d, (p - 1) / 2, p) == p - 1)
                .unwrap();
            low[0] = p - d;
        }
        if n == 3 {
            let (a, b) = (0..p)
                .flat_map(|a| (1..p).map(move |b| (a, b)))
                .find(|&(a, b)| (0..p).all(|x| (x * x % p * x + a * x + b) % p != 0))
                .unwrap();
            low = [b, a, 0];
        }
        Self {
            p,
            n,
            low,
            q: p.pow(n as u32),
        }
    }
    fn digits(&self, mut a: u64) -> [u64; 3] {
        let mut v = [0; 3];
        for x in v.iter_mut().take(self.n) {
            *x = a % self.p;
            a /= self.p;
        }
        v
    }
    fn pack(&self, a: [u64; 3]) -> u64 {
        let mut v = 0;
        for i in (0..self.n).rev() {
            v = v * self.p + a[i]
        }
        v
    }
    fn add(&self, a: u64, b: u64) -> u64 {
        let a = self.digits(a);
        let b = self.digits(b);
        let mut c = [0; 3];
        for j in 0..self.n {
            c[j] = (a[j] + b[j]) % self.p
        }
        self.pack(c)
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        if self.n == 1 {
            return a * b % self.p;
        }
        let a = self.digits(a);
        let b = self.digits(b);
        let mut c = [0; 5];
        for i in 0..self.n {
            for j in 0..self.n {
                c[i + j] = (c[i + j] + a[i] * b[j]) % self.p
            }
        }
        for k in (self.n..2 * self.n - 1).rev() {
            for j in 0..self.n {
                c[k - self.n + j] =
                    (c[k - self.n + j] + self.p - c[k] * self.low[j] % self.p) % self.p
            }
        }
        self.pack([c[0], c[1], c[2]])
    }
    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut b = 1;
        while e > 0 {
            if e & 1 != 0 {
                b = self.mul(b, a)
            }
            a = self.mul(a, a);
            e >>= 1
        }
        b
    }
    fn eval(&self, x: u64, coeff: &[u64]) -> u64 {
        let mut v = 0;
        for &a in coeff.iter().rev() {
            v = self.add(self.mul(v, x), a)
        }
        v
    }
    fn count(&self, coeff: &[u64]) -> i128 {
        let mut square = vec![0i128; self.q as usize];
        for y in 0..self.q {
            square[self.mul(y, y) as usize] += 1;
        }
        1 + (0..self.q)
            .map(|x| square[self.eval(x, coeff) as usize])
            .sum::<i128>()
    }
    fn twist(&self, coeff: &[u64], i: u64) -> i128 {
        assert_eq!(self.n, 2);
        let z = self.p;
        let mut squares = BTreeMap::<u64, i128>::new();
        for y in 0..self.q {
            if self.pow(y, self.p) == self.mul(self.p - i, y) {
                *squares.entry(self.mul(y, y)).or_default() += 1;
            }
        }
        1 + (0..self.p)
            .map(|t| {
                squares
                    .get(&self.eval(self.mul(z, t), coeff))
                    .copied()
                    .unwrap_or(0)
            })
            .sum::<i128>()
    }
}
fn modpow(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut b = 1;
    while e > 0 {
        if e & 1 != 0 {
            b = b * a % p
        }
        a = a * a % p;
        e >>= 1
    }
    b
}
fn trim(v: &mut Vec<u64>) {
    while v.last() == Some(&0) {
        v.pop();
    }
}
fn smooth(f: &[u64], p: u64) -> bool {
    let mut a = f.to_vec();
    let mut b: Vec<u64> = (1..f.len()).map(|j| f[j] * j as u64 % p).collect();
    trim(&mut b);
    while !b.is_empty() {
        let inv = modpow(*b.last().unwrap(), p - 2, p);
        while a.len() >= b.len() {
            let k = a.len() - b.len();
            let w = a[a.len() - 1] * inv % p;
            for j in 0..b.len() {
                a[k + j] = (a[k + j] + p - w * b[j] % p) % p
            }
            trim(&mut a);
        }
        std::mem::swap(&mut a, &mut b)
    }
    a.len() == 1
}
fn poly_norm(g: &[Z]) -> Vec<i128> {
    let mut o = vec![(0, 0); 2 * g.len() - 1];
    for j in 0..g.len() {
        for k in 0..g.len() {
            o[j + k] = za(o[j + k], zm(g[j], bar(g[k])))
        }
    }
    assert!(o.iter().all(|x| x.1 == 0));
    o.iter().map(|x| x.0).collect()
}
fn main() {
    println!("p\ta5\ti\tsmooth\tC1\tC2\tC3\tN\tcandidates\tfull_polynomial_matches\tG");
    let mut qualified = 0;
    let mut skipped = 0;
    for p in [13u64, 17, 29] {
        for a5 in [0, 1] {
            let f = [0, 2, 0, 1, 0, a5, 0, 1];
            if !smooth(&f, p) {
                println!("{p}\t{a5}\t0\tfalse\t0\t0\t0\t0\t0\t0\tsingular");
                skipped += 1;
                continue;
            }
            let c1 = Field::new(p, 1).count(&f);
            let c2 = Field::new(p, 2).count(&f);
            let c3 = Field::new(p, 3).count(&f);
            let q = p as i128;
            let s1 = q + 1 - c1;
            let s2 = q * q + 1 - c2;
            let s3 = q * q * q + 1 - c3;
            let a1 = -s1;
            assert_eq!((-s2 - a1 * s1) % 2, 0);
            let a2 = (-s2 - a1 * s1) / 2;
            assert_eq!((-s3 - a1 * s2 - a2 * s1) % 3, 0);
            let a3 = (-s3 - a1 * s2 - a2 * s1) / 3;
            let full = vec![q * q * q, q * q * a1, q * a2, a3, a2, a1, 1];
            let n = full.iter().sum::<i128>();
            assert!(n > 0);
            for i in (1..p).filter(|&i| i * i % p == p - 1) {
                let mut previous = None;
                for rep in 0..2 {
                    let twist = Field::new(p, 2).twist(&f, i);
                    let tr = q + 1 - twist;
                    assert_eq!((c1 - q - 1) % 2, 0);
                    assert_eq!(tr % 2, 0);
                    let a = ((c1 - q - 1) / 2, tr / 2);
                    let mut candidates = BTreeSet::new();
                    for r in -(p as i128)..=p as i128 {
                        for im in -(p as i128)..=p as i128 {
                            if r * r + im * im != q {
                                continue;
                            }
                            let d = (q * r, q * im);
                            let b = zm((r, im), bar(a));
                            let g = vec![d, b, a, (1, 0)];
                            let value = g.iter().copied().fold((0, 0), za);
                            if norm(value) == n {
                                candidates.insert(g);
                            }
                        }
                    }
                    let matches: Vec<_> =
                        candidates.iter().filter(|g| poly_norm(g) == full).collect();
                    assert!(
                        !matches.is_empty(),
                        "actual polynomial missing p={p} a5={a5} i={i}"
                    );
                    let sig = (twist, candidates.clone(), full.clone());
                    if let Some(old) = &previous {
                        assert_eq!(old, &sig)
                    } else {
                        previous = Some(sig)
                    }
                    if rep == 0 {
                        println!(
                            "{p}\t{a5}\t{i}\ttrue\t{c1}\t{c2}\t{c3}\t{n}\t{}\t{}\t{:?}",
                            candidates.len(),
                            matches.len(),
                            matches[0]
                        );
                        qualified += 1;
                    }
                }
            }
        }
    }
    eprintln!("qualified_orientations={qualified} singular_models={skipped} independent_extension_counts=true candidate_contains_full_polynomial=true repeats=true complete_solve=false");
}
