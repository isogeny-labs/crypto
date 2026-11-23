//! #J of a Picard curve y^3 = x^4 + a2 x^2 + a1 x + a0 from point counts over F_p, F_p^2, F_p^3.
use crate::fp::Fp;

struct Ext {
    p: u64,
    k: usize,
    c: u64,
}
type E = [u64; 3];
impl Ext {
    fn new(p: u64, k: usize) -> Ext {
        let fp = Fp::new(p);
        let mut c = 2u64;
        while fp.pow_u(c, (p - 1) / k as u64) == 1 {
            c += 1;
        }
        Ext { p, k, c }
    }
    fn mul(&self, a: &E, b: &E) -> E {
        let k = self.k;
        let mut t = [0u64; 5];
        for i in 0..k {
            for j in 0..k {
                t[i + j] = (t[i + j] + a[i] * b[j]) % self.p;
            }
        }
        let mut r = [0u64; 3];
        for i in 0..k {
            r[i] = t[i];
        }
        for i in k..2 * k - 1 {
            r[i - k] = (r[i - k] + t[i] * self.c) % self.p;
        }
        r
    }
    fn add(&self, a: &E, b: &E) -> E {
        let mut r = [0u64; 3];
        for i in 0..self.k {
            r[i] = (a[i] + b[i]) % self.p;
        }
        r
    }
    fn from_index(&self, mut i: u64) -> E {
        let mut r = [0u64; 3];
        for d in 0..self.k {
            r[d] = i % self.p;
            i /= self.p;
        }
        r
    }
    fn to_index(&self, a: &E) -> u64 {
        let mut i = 0u64;
        for d in (0..self.k).rev() {
            i = i * self.p + a[d];
        }
        i
    }
    fn size(&self) -> u64 {
        self.p.pow(self.k as u32)
    }
}

/// N_k = 1 + sum_x cnt[f(x)].
pub fn count_ext(p: u32, a2: u32, a1: u32, a0: u32, k: usize) -> u64 {
    let pp = p as u64;
    if k == 1 {
        let mut hist = vec![0u64; p as usize];
        for y in 0..pp {
            hist[(y * y % pp * y % pp) as usize] += 1;
        }
        let mut n = 1u64;
        for x in 0..pp {
            let x2 = x * x % pp;
            let v = (x2 * x2 + a2 as u64 * x2 + a1 as u64 * x + a0 as u64) % pp;
            n += hist[v as usize];
        }
        return n;
    }
    let ext = Ext::new(pp, k);
    let sz = ext.size();
    let mut hist = vec![0u32; sz as usize];
    for i in 0..sz {
        let y = ext.from_index(i);
        let y3 = ext.mul(&ext.mul(&y, &y), &y);
        hist[ext.to_index(&y3) as usize] += 1;
    }
    let ca = |c: u32| -> E {
        let mut e = [0u64; 3];
        e[0] = c as u64;
        e
    };
    let mut n = 1u64;
    for i in 0..sz {
        let x = ext.from_index(i);
        let x2 = ext.mul(&x, &x);
        let x4 = ext.mul(&x2, &x2);
        let t1 = ext.mul(&ca(a2), &x2);
        let t2 = ext.mul(&ca(a1), &x);
        let v = ext.add(&ext.add(&x4, &t1), &ext.add(&t2, &ca(a0)));
        n += hist[ext.to_index(&v) as usize] as u64;
    }
    n
}

pub fn jac_from_counts(p: u32, n1: u64, n2: u64, n3: u64) -> i128 {
    let pp = p as i128;
    let s1 = pp + 1 - n1 as i128;
    let s2 = pp.pow(2) + 1 - n2 as i128;
    let s3 = pp.pow(3) + 1 - n3 as i128;
    let e1 = s1;
    let t = s1 * s1 - s2;
    assert!(t % 2 == 0);
    let e2 = t / 2;
    let t3 = s1 * s1 * s1 - 3 * s1 * s2 + 2 * s3;
    assert!(t3 % 6 == 0);
    let e3 = t3 / 6;
    1 - e1 + e2 - e3 + pp * e2 - pp * pp * e1 + pp * pp * pp
}

pub fn jac_order(p: u32, a2: u32, a1: u32, a0: u32) -> (u64, u64, u64, i128) {
    let n1 = count_ext(p, a2, a1, a0, 1);
    let n2 = count_ext(p, a2, a1, a0, 2);
    let n3 = count_ext(p, a2, a1, a0, 3);
    (n1, n2, n3, jac_from_counts(p, n1, n2, n3))
}

pub fn factorize(mut n: u128) -> Vec<(u128, u32)> {
    let mut f = Vec::new();
    let mut d = 2u128;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 {
                n /= d;
                e += 1;
            }
            f.push((d, e));
        }
        d += 1;
    }
    if n > 1 {
        f.push((n, 1));
    }
    f
}
