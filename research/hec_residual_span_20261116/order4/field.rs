use hec_native_ic::{fp::Fp, poly::Poly};
#[path = "../multi/algebra.rs"]
mod algebra;
pub type E = [u64; 4];
pub struct Field {
    pub p: u64,
    pub d: usize,
    pub q: u64,
    pub modulus: Poly,
}
impl Field {
    pub fn new(p: u64, d: usize) -> Self {
        let q = p.pow(d as u32);
        let modulus = if d == 1 {
            Poly::x()
        } else {
            let f = Fp::new(p);
            (1..q)
                .find_map(|code| {
                    let mut x = code;
                    let mut v = vec![0; d + 1];
                    for c in &mut v[..d] {
                        *c = x % p;
                        x /= p
                    }
                    v[d] = 1;
                    let m = Poly::from_coeffs(&v);
                    (v[0] != 0 && algebra::irreducible(&m, &f)).then_some(m)
                })
                .unwrap()
        };
        Self { p, d, q, modulus }
    }
    pub fn element(&self, mut n: u64) -> E {
        let mut a = [0; 4];
        for c in &mut a[..self.d] {
            *c = n % self.p;
            n /= self.p
        }
        a
    }
    pub fn index(&self, a: E) -> usize {
        let mut out = 0;
        for j in (0..self.d).rev() {
            out = out * self.p + a[j]
        }
        out as usize
    }
    pub fn mul(&self, a: E, b: E) -> E {
        let mut v = [0u64; 7];
        for i in 0..self.d {
            for j in 0..self.d {
                v[i + j] += a[i] * b[j]
            }
        }
        for x in &mut v {
            *x %= self.p
        }
        for k in (self.d..2 * self.d - 1).rev() {
            let t = v[k];
            v[k] = 0;
            for j in 0..self.d {
                v[k - self.d + j] =
                    (v[k - self.d + j] + self.p * self.p - t * self.modulus.c[j]) % self.p
            }
        }
        let mut out = [0; 4];
        out[..self.d].copy_from_slice(&v[..self.d]);
        out
    }
    pub fn pow(&self, mut a: E, mut n: u64) -> E {
        let mut out = [1, 0, 0, 0];
        while n > 0 {
            if n & 1 != 0 {
                out = self.mul(out, a)
            }
            a = self.mul(a, a);
            n >>= 1
        }
        out
    }
    pub fn add_const(&self, mut a: E, b: u64) -> E {
        a[0] = (a[0] + b) % self.p;
        a
    }
    pub fn validate(&self) {
        let f = Fp::new(self.p);
        for j in 0..64 {
            let a = self.element((37 * j + 5) % self.q);
            let b = self.element((51 * j + 7) % self.q);
            let reference = Poly::from_coeffs(&a[..self.d])
                .mul(&Poly::from_coeffs(&b[..self.d]), &f)
                .rem(&self.modulus, &f);
            let got = self.mul(a, b);
            for k in 0..self.d {
                assert_eq!(got[k], reference.c[k])
            }
            let nonzero = self.element((17 * j) % (self.q - 1) + 1);
            assert_eq!(self.pow(nonzero, self.q - 1), [1, 0, 0, 0]);
        }
    }
    pub fn count(&self, a: [u64; 4]) -> u64 {
        let mut squares = vec![false; self.q as usize];
        for y in 0..self.q {
            let v = self.element(y);
            squares[self.index(self.mul(v, v))] = true
        }
        assert_eq!(
            squares.iter().filter(|&&x| x).count() as u64,
            (self.q + 1) / 2
        );
        for j in 0..64 {
            let n = (29 * j + 3) % self.q;
            if n != 0 {
                let v = self.pow(self.element(n), (self.q - 1) / 2);
                assert_eq!(squares[n as usize], v == [1, 0, 0, 0]);
                assert!(v == [1, 0, 0, 0] || v == [self.p - 1, 0, 0, 0])
            }
        }
        let mut points = 1;
        for x in 0..self.q {
            let x = self.element(x);
            let z = self.mul(x, x);
            let mut v = [1, 0, 0, 0];
            for c in a {
                v = self.add_const(self.mul(v, z), c)
            }
            v = self.mul(v, x);
            let n = self.index(v);
            points += if n == 0 {
                1
            } else if squares[n] {
                2
            } else {
                0
            };
        }
        points
    }
}
